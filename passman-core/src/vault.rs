use crate::crypto::{
    decrypt, derive_key, encrypt, random_bytes, CryptoError, KdfParams, KEY_SIZE, SALT_SIZE,
};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const MAGIC: &[u8] = b"PMV ";
pub const VERSION: u32 = 1;
pub const PAYLOAD_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Crypto(#[from] CryptoError),
    #[error("base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid magic bytes")]
    InvalidMagic,
    #[error("unsupported version: {0}")]
    UnsupportedVersion(u32),
    #[error("invalid file format")]
    InvalidFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultHeader {
    pub version: u32,
    pub cipher: String,
    pub kdf: String,
    pub kdf_params: KdfParamsJson,
    pub encrypted_dek: String,
    pub dek_nonce: String,
    pub payload_nonce: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KdfParamsJson {
    pub salt: String,
    pub memory_kb: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

impl From<&KdfParams> for KdfParamsJson {
    fn from(params: &KdfParams) -> Self {
        Self {
            salt: general_purpose::STANDARD.encode(params.salt),
            memory_kb: params.memory_kb,
            iterations: params.iterations,
            parallelism: params.parallelism,
        }
    }
}

impl TryFrom<KdfParamsJson> for KdfParams {
    type Error = VaultError;

    fn try_from(json: KdfParamsJson) -> Result<Self, Self::Error> {
        let salt_bytes = general_purpose::STANDARD.decode(&json.salt)?;
        let salt: [u8; SALT_SIZE] = salt_bytes
            .try_into()
            .map_err(|_| VaultError::InvalidFormat)?;
        Ok(Self {
            salt,
            memory_kb: json.memory_kb,
            iterations: json.iterations,
            parallelism: json.parallelism,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Trash {
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(default)]
    pub entries: Vec<VaultEntry>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CustomField {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VaultEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub fields: Vec<CustomField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VaultPayload {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub groups: Vec<Group>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub entries: Vec<VaultEntry>,
    #[serde(default)]
    pub trash: Trash,
}

impl VaultPayload {
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

#[derive(Clone)]
pub struct VaultFile {
    pub header: VaultHeader,
    pub payload: VaultPayload,
    pub path: String,
    pub needs_save: bool,
}

pub fn create_vault_file(path: &str, name: &str, password: &str) -> Result<VaultFile, VaultError> {
    let (vault, _key) = create_vault_file_with_key(path, name, password)?;
    Ok(vault)
}

pub fn create_vault_file_with_key(
    path: &str,
    name: &str,
    password: &str,
) -> Result<(VaultFile, [u8; KEY_SIZE]), VaultError> {
    let kdf_params = KdfParams::default();
    let vault_key = derive_key(password, &kdf_params)?;
    let dek = random_bytes(KEY_SIZE);
    let dek_array: [u8; KEY_SIZE] = dek.as_slice().try_into().unwrap();

    let encrypted_dek = encrypt(&dek, &vault_key);

    let now = Utc::now();
    let payload = VaultPayload {
        name: name.to_string(),
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

pub fn derive_vault_key(password: &str, header: &VaultHeader) -> Result<Vec<u8>, VaultError> {
    let kdf_params: KdfParams = header.kdf_params.clone().try_into()?;
    Ok(derive_key(password, &kdf_params)?.to_vec())
}

pub fn save_vault_file(vault: &VaultFile, password: &str) -> Result<(), VaultError> {
    let vault_key = derive_vault_key(password, &vault.header)?;
    save_vault_file_with_key(vault, &vault_key)
}

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

fn read_vault_file(path: &str) -> Result<(VaultHeader, Vec<u8>), VaultError> {
    let data = fs::read(path)?;
    let (_, rest) = read_magic_and_version(&data)?;
    let (header, rest) = read_header(rest)?;
    let encrypted_payload = read_payload(rest)?;
    Ok((header, encrypted_payload))
}

fn read_magic_and_version(data: &[u8]) -> Result<(u32, &[u8]), VaultError> {
    let magic_len = MAGIC.len();
    let version_len = 2;
    if data.len() < magic_len + version_len {
        return Err(VaultError::InvalidFormat);
    }
    if &data[0..magic_len] != MAGIC {
        return Err(VaultError::InvalidMagic);
    }
    let file_version = u16::from_le_bytes([data[magic_len], data[magic_len + 1]]) as u32;
    if file_version != VERSION {
        return Err(VaultError::UnsupportedVersion(file_version));
    }
    Ok((file_version, &data[magic_len + version_len..]))
}

fn read_header(data: &[u8]) -> Result<(VaultHeader, &[u8]), VaultError> {
    if data.len() < 2 {
        return Err(VaultError::InvalidFormat);
    }
    let header_len = u16::from_le_bytes([data[0], data[1]]) as usize;
    let header_start = 2;
    let header_end = header_start + header_len;
    if header_end > data.len() {
        return Err(VaultError::InvalidFormat);
    }
    let header: VaultHeader = serde_json::from_slice(&data[header_start..header_end])?;
    if header.version != VERSION {
        return Err(VaultError::UnsupportedVersion(header.version));
    }
    Ok((header, &data[header_end..]))
}

fn read_payload(data: &[u8]) -> Result<Vec<u8>, VaultError> {
    if data.len() < 8 {
        return Err(VaultError::InvalidFormat);
    }
    let payload_len = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]) as usize;
    let payload_start = 8;
    let payload_end = payload_start + payload_len;
    if payload_end > data.len() {
        return Err(VaultError::InvalidFormat);
    }
    Ok(data[payload_start..payload_end].to_vec())
}

fn write_vault_file(
    path: &str,
    header: &VaultHeader,
    encrypted_payload: &[u8],
) -> Result<(), VaultError> {
    let header_json = serde_json::to_vec(header)?;
    let header_len = header_json.len() as u16;
    let payload_len = encrypted_payload.len() as u64;

    let mut data = Vec::new();
    data.extend_from_slice(MAGIC);
    data.extend_from_slice(&(VERSION as u16).to_le_bytes());
    data.extend_from_slice(&header_len.to_le_bytes());
    data.extend_from_slice(&header_json);
    data.extend_from_slice(&payload_len.to_le_bytes());
    data.extend_from_slice(encrypted_payload);

    let temp_path = format!("{}.tmp", path);
    let write_result = (|| -> Result<(), VaultError> {
        let mut temp_file = fs::File::create(&temp_path)?;
        temp_file.write_all(&data)?;
        temp_file.sync_all()?;
        Ok(())
    })();

    if let Err(e) = write_result {
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    fs::rename(&temp_path, path)?;
    Ok(())
}

pub fn vault_exists(path: &str) -> bool {
    Path::new(path).exists()
}

