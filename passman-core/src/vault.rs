use crate::crypto::{decrypt, derive_key, encrypt, random_bytes, KdfParams, CryptoError, KEY_SIZE, SALT_SIZE};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const MAGIC: &[u8] = b"PMV ";
pub const VERSION: u32 = 1;
pub const PAYLOAD_FORMAT_VERSION: u32 = 2;

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
            salt: general_purpose::STANDARD.encode(&params.salt),
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
pub struct VaultMetadata {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub format_version: u32,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VaultPayload {
    pub groups: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub vault_metadata: VaultMetadata,
    pub entries: Vec<VaultEntry>,
    #[serde(default)]
    pub trash: Vec<TrashGroup>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TrashGroup {
    pub group: String,
    pub entries: Vec<VaultEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LegacyVaultGroup {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LegacyVaultEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    #[serde(default)]
    pub group_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LegacyVaultPayload {
    pub vault_metadata: VaultMetadata,
    pub groups: Vec<LegacyVaultGroup>,
    pub entries: Vec<LegacyVaultEntry>,
}

fn migrate_v1_to_v2(legacy: LegacyVaultPayload) -> VaultPayload {
    let now = Utc::now();
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut id_to_name: HashMap<String, String> = HashMap::new();
    for group in &legacy.groups {
        if group.deleted || group.id == "default" || group.name == "All Entries" {
            continue;
        }
        if seen_names.insert(group.name.clone()) {
            id_to_name.insert(group.id.clone(), group.name.clone());
        }
    }
    let group_names: Vec<String> = seen_names.into_iter().collect();
    let entries = legacy
        .entries
        .into_iter()
        .map(|e| {
            let mut tags = Vec::new();
            if !e.group_id.is_empty() && e.group_id != "default" {
                if let Some(name) = id_to_name.get(&e.group_id) {
                    tags.push(name.clone());
                }
            }
            VaultEntry {
                id: e.id,
                title: e.title,
                username: e.username,
                password: e.password,
                url: e.url,
                notes: e.notes,
                tags,
                created_at: e.created_at,
                updated_at: e.updated_at,
            }
        })
        .collect();
    VaultPayload {
        groups: group_names,
        tags: Vec::new(),
        vault_metadata: VaultMetadata {
            name: legacy.vault_metadata.name,
            created_at: legacy.vault_metadata.created_at,
            updated_at: now,
            format_version: PAYLOAD_FORMAT_VERSION,
        },
        entries,
        trash: Vec::new(),
    }
}

#[derive(Clone)]
pub struct VaultFile {
    pub header: VaultHeader,
    pub payload: VaultPayload,
    pub path: String,
    pub needs_save: bool,
}

pub fn create_vault_file(
    path: &str,
    name: &str,
    password: &str,
) -> Result<VaultFile, VaultError> {
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
    let metadata = VaultMetadata {
        name: name.to_string(),
        created_at: now,
        updated_at: now,
        format_version: PAYLOAD_FORMAT_VERSION,
    };
    let payload = VaultPayload {
        groups: Vec::new(),
        tags: Vec::new(),
        vault_metadata: metadata,
        entries: Vec::new(),
        trash: Vec::new(),
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
    let dek_array: [u8; KEY_SIZE] = dek.as_slice().try_into().map_err(|_| VaultError::InvalidFormat)?;

    let payload_nonce = general_purpose::STANDARD.decode(&header.payload_nonce)?;
    let payload_json = decrypt(&encrypted_payload, &dek_array, &payload_nonce)?;
    let (payload, migrated) = match serde_json::from_slice(&payload_json) {
        Ok(p) => (p, false),
        Err(_) => {
            let legacy: LegacyVaultPayload = serde_json::from_slice(&payload_json)?;
            (migrate_v1_to_v2(legacy), true)
        }
    };

    let vault = VaultFile {
        header,
        payload,
        path: path.to_string(),
        needs_save: migrated,
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
    let vault_key_array: [u8; KEY_SIZE] = vault_key.try_into().map_err(|_| VaultError::InvalidFormat)?;
    let encrypted_dek = general_purpose::STANDARD.decode(&vault.header.encrypted_dek)?;
    let dek_nonce = general_purpose::STANDARD.decode(&vault.header.dek_nonce)?;
    let dek = decrypt(&encrypted_dek, &vault_key_array, &dek_nonce)?;
    let dek_array: [u8; KEY_SIZE] = dek.as_slice().try_into().map_err(|_| VaultError::InvalidFormat)?;

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

#[cfg(test)]
mod migration_tests {
    use super::*;

    #[test]
    fn test_migrate_v1_to_v2() {
        let now = Utc::now();
        let legacy = LegacyVaultPayload {
            vault_metadata: VaultMetadata {
                name: "Test".to_string(),
                created_at: now,
                updated_at: now,
                format_version: 1,
            },
            groups: vec![
                LegacyVaultGroup {
                    id: "default".to_string(),
                    name: "All Entries".to_string(),
                    deleted: false,
                    created_at: now,
                    updated_at: now,
                },
                LegacyVaultGroup {
                    id: "g1".to_string(),
                    name: "Group A".to_string(),
                    deleted: false,
                    created_at: now,
                    updated_at: now,
                },
                LegacyVaultGroup {
                    id: "g2".to_string(),
                    name: "Group B".to_string(),
                    deleted: true,
                    created_at: now,
                    updated_at: now,
                },
            ],
            entries: vec![
                LegacyVaultEntry {
                    id: "e1".to_string(),
                    title: "Entry 1".to_string(),
                    username: "".to_string(),
                    password: "".to_string(),
                    url: "".to_string(),
                    notes: "".to_string(),
                    group_id: "g1".to_string(),
                    created_at: now,
                    updated_at: now,
                },
                LegacyVaultEntry {
                    id: "e2".to_string(),
                    title: "Entry 2".to_string(),
                    username: "".to_string(),
                    password: "".to_string(),
                    url: "".to_string(),
                    notes: "".to_string(),
                    group_id: "default".to_string(),
                    created_at: now,
                    updated_at: now,
                },
            ],
        };
        let migrated = migrate_v1_to_v2(legacy);
        assert_eq!(migrated.groups, vec!["Group A"]);
        assert_eq!(migrated.entries.len(), 2);
        assert_eq!(migrated.entries[0].tags, vec!["Group A"]);
        assert!(migrated.entries[1].tags.is_empty());
        assert_eq!(migrated.vault_metadata.format_version, PAYLOAD_FORMAT_VERSION);
    }
}
