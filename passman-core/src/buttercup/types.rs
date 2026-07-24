use crate::vault::HistoryItem;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

pub const FORMAT_A_SIGNATURE: &str = "b~>buttercup/a";
pub const FORMAT_B_SIGNATURE: &str = "b~>buttercup/b";
pub const DEFAULT_ALGORITHM: &str = "cbc";
pub const PASSWORD_KEY_SIZE: usize = 32;
pub const HMAC_KEY_SIZE: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    A,
    B,
}

#[derive(Debug, Error)]
pub enum ButtercupError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid encrypted format")]
    InvalidFormat,
    #[error("invalid base64: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    #[error("invalid hex: {0}")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("invalid rounds: {0}")]
    InvalidRounds(String),
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("authentication failed")]
    AuthenticationFailed,
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("decompression failed: {0}")]
    DecompressionFailed(String),
}

#[derive(Debug, Clone)]
pub struct ButtercupVault {
    pub name: String,
    pub uuid: Option<String>,
    pub groups: Vec<ButtercupGroup>,
    pub entries: Vec<ButtercupEntry>,
    pub trash: ButtercupTrash,
}

#[derive(Debug, Clone, Default)]
pub struct ButtercupTrash {
    pub groups: Vec<ButtercupGroup>,
    pub entries: Vec<ButtercupEntry>,
}

#[derive(Debug, Clone)]
pub struct ButtercupGroup {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ButtercupCustomField {
    pub id: String,
    pub label: String,
    pub field_type: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ButtercupEntry {
    pub id: String,
    pub group_id: Option<String>,
    pub title: String,
    pub username: String,
    pub password: String,
    pub fields: Vec<ButtercupCustomField>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub history: Vec<HistoryItem>,
}

#[derive(Debug, Deserialize)]
pub struct RawVault {
    #[serde(default)]
    #[allow(dead_code)]
    pub _id: Option<String>,
    #[serde(default)]
    pub a: HashMap<String, RawValue>,
    #[serde(default)]
    pub g: Vec<RawGroup>,
    #[serde(default)]
    pub e: Vec<RawEntry>,
    #[serde(default)]
    #[allow(dead_code)]
    pub _c: String,
}

#[derive(Debug, Deserialize)]
pub struct RawGroup {
    pub id: String,
    #[serde(default)]
    pub g: String,
    #[serde(default)]
    pub t: String,
    #[serde(default, rename = "a")]
    pub a: HashMap<String, RawValue>,
}

#[derive(Debug, Deserialize)]
pub struct RawEntry {
    pub id: String,
    #[serde(default)]
    pub g: String,
    #[serde(default)]
    pub p: HashMap<String, RawValue>,
    #[serde(default)]
    pub a: HashMap<String, RawValue>,
    #[serde(default)]
    pub deleted: Option<u64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct RawValue {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub deleted: Option<u64>,
    #[serde(default)]
    pub history: Vec<RawHistoryItem>,
}

#[derive(Debug, Deserialize, Default)]
pub struct RawHistoryItem {
    pub value: String,
    pub updated: u64,
}

pub struct EncryptedComponents {
    pub content: String,
    pub iv: String,
    pub salt: String,
    pub auth: String,
    pub rounds: u32,
    pub method: String,
}
