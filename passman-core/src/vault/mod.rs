mod format;
mod types;

pub use types::*;

use crate::crypto::{
    decrypt, derive_key, encrypt, random_bytes, KdfParams, SecurityLevel, KEY_SIZE,
};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use format::{read_vault_file, write_vault_file};

pub use format::vault_exists;

pub fn create_vault_file(path: &str, name: &str, password: &str) -> Result<VaultFile, VaultError> {
    let (vault, _key) = create_vault_file_with_key(path, name, password)?;
    Ok(vault)
}

pub fn create_vault_file_with_key(
    path: &str,
    name: &str,
    password: &str,
) -> Result<(VaultFile, [u8; KEY_SIZE]), VaultError> {
    create_vault_file_with_level(path, name, password, SecurityLevel::Medium)
}

pub fn create_vault_file_with_level(
    path: &str,
    name: &str,
    password: &str,
    level: SecurityLevel,
) -> Result<(VaultFile, [u8; KEY_SIZE]), VaultError> {
    let kdf_params = level.kdf_params();
    let vault_key = derive_key(password, &kdf_params)?;
    let dek = random_bytes(KEY_SIZE);
    let dek_array: [u8; KEY_SIZE] = dek.as_slice().try_into().unwrap();

    let encrypted_dek = encrypt(&dek, &vault_key);

    let now = Utc::now();
    let payload = VaultPayload {
        name: name.to_string(),
        uuid: None,
        created_at: now,
        updated_at: now,
        groups: Vec::new(),
        tags: Vec::new(),
        entries: Vec::new(),
        trash: Trash::default(),
    };
    let payload_json = serde_json::to_vec(&payload)?;
    let encrypted_payload = encrypt(&payload_json, &dek_array);

    let header = VaultHeader {
        version: VERSION,
        cipher: "AES-256-GCM".to_string(),
        kdf: "Argon2id".to_string(),
        kdf_params: KdfParamsJson::from(&kdf_params),
        encrypted_dek: general_purpose::STANDARD.encode(&encrypted_dek.bytes),
        dek_nonce: general_purpose::STANDARD.encode(&encrypted_dek.nonce),
        payload_nonce: general_purpose::STANDARD.encode(&encrypted_payload.nonce),
        created_at: now,
        updated_at: now,
    };

    write_vault_file(path, &header, &encrypted_payload.bytes)?;

    let vault = VaultFile {
        header,
        payload,
        path: path.to_string(),
        needs_save: false,
    };
    Ok((vault, vault_key))
}

pub fn open_vault_file(path: &str, password: &str) -> Result<VaultFile, VaultError> {
    let (vault, _key) = open_vault_file_with_key(path, password)?;
    Ok(vault)
}

pub fn open_vault_file_with_key(
    path: &str,
    password: &str,
) -> Result<(VaultFile, [u8; KEY_SIZE]), VaultError> {
    let (header, encrypted_payload) = read_vault_file(path)?;
    let kdf_params: KdfParams = header.kdf_params.clone().try_into()?;
    let vault_key = derive_key(password, &kdf_params)?;

    let encrypted_dek = general_purpose::STANDARD.decode(&header.encrypted_dek)?;
    let dek_nonce = general_purpose::STANDARD.decode(&header.dek_nonce)?;
    let dek = decrypt(&encrypted_dek, &vault_key, &dek_nonce)?;
    let dek_array: [u8; KEY_SIZE] = dek
        .as_slice()
        .try_into()
        .map_err(|_| VaultError::InvalidFormat)?;

    let payload_nonce = general_purpose::STANDARD.decode(&header.payload_nonce)?;
    let payload_json = decrypt(&encrypted_payload, &dek_array, &payload_nonce)?;
    let payload: VaultPayload = serde_json::from_slice(&payload_json)?;

    let vault = VaultFile {
        header,
        payload,
        path: path.to_string(),
        needs_save: false,
    };
    Ok((vault, vault_key))
}

pub fn save_vault_file(vault: &VaultFile, password: &str) -> Result<(), VaultError> {
    let kdf_params: KdfParams = vault.header.kdf_params.clone().try_into()?;
    let vault_key = derive_key(password, &kdf_params)?;
    save_vault_file_with_key(vault, &vault_key)
}

/// Save a vault file using a pre-derived key.
///
/// Flow: decrypt the stored DEK → re-encrypt the payload → atomically write the file.
pub fn save_vault_file_with_key(vault: &VaultFile, vault_key: &[u8]) -> Result<(), VaultError> {
    let vault_key_array: [u8; KEY_SIZE] = vault_key
        .try_into()
        .map_err(|_| VaultError::InvalidFormat)?;
    let encrypted_dek = general_purpose::STANDARD.decode(&vault.header.encrypted_dek)?;
    let dek_nonce = general_purpose::STANDARD.decode(&vault.header.dek_nonce)?;
    let dek = decrypt(&encrypted_dek, &vault_key_array, &dek_nonce)?;
    let dek_array: [u8; KEY_SIZE] = dek
        .as_slice()
        .try_into()
        .map_err(|_| VaultError::InvalidFormat)?;

    let payload_json = serde_json::to_vec(&vault.payload)?;
    let encrypted_payload = encrypt(&payload_json, &dek_array);

    let mut header = vault.header.clone();
    header.updated_at = Utc::now();
    header.payload_nonce = general_purpose::STANDARD.encode(&encrypted_payload.nonce);

    write_vault_file(&vault.path, &header, &encrypted_payload.bytes)?;
    Ok(())
}

/// Change the KDF parameters of an existing vault by re-deriving the vault key
/// with new parameters and re-encrypting the DEK.
///
/// Flow: decrypt DEK with old key → derive new vault key with new params + fresh salt →
/// re-encrypt DEK → update header → rewrite file.
pub fn change_kdf_params(
    vault: &VaultFile,
    password: &str,
    new_level: SecurityLevel,
) -> Result<(VaultHeader, [u8; KEY_SIZE]), VaultError> {
    let old_kdf_params: KdfParams = vault.header.kdf_params.clone().try_into()?;
    let old_vault_key = derive_key(password, &old_kdf_params)?;

    let encrypted_dek = general_purpose::STANDARD.decode(&vault.header.encrypted_dek)?;
    let dek_nonce = general_purpose::STANDARD.decode(&vault.header.dek_nonce)?;
    let dek = decrypt(&encrypted_dek, &old_vault_key, &dek_nonce)?;
    let _dek_array: [u8; KEY_SIZE] = dek
        .as_slice()
        .try_into()
        .map_err(|_| VaultError::InvalidFormat)?;

    let new_kdf_params = new_level.kdf_params();
    let new_vault_key = derive_key(password, &new_kdf_params)?;
    let re_encrypted_dek = encrypt(&dek, &new_vault_key);

    let mut new_header = vault.header.clone();
    new_header.kdf_params = KdfParamsJson::from(&new_kdf_params);
    new_header.encrypted_dek = general_purpose::STANDARD.encode(&re_encrypted_dek.bytes);
    new_header.dek_nonce = general_purpose::STANDARD.encode(&re_encrypted_dek.nonce);
    new_header.updated_at = Utc::now();

    let (header_ref, encrypted_payload) = read_vault_file(&vault.path)?;
    let _ = header_ref;
    write_vault_file(&vault.path, &new_header, &encrypted_payload)?;

    Ok((new_header, new_vault_key))
}
