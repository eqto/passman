use crate::crypto::{KdfParams, SALT_SIZE};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
    Crypto(#[from] crate::crypto::CryptoError),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub property: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
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
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub fields: Vec<CustomField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub history: Vec<HistoryItem>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VaultPayload {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
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

