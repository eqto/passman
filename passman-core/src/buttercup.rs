use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::engine::{general_purpose::STANDARD as BASE64, Engine};
use flate2::read::GzDecoder;
use hmac::digest::KeyInit as HmacKeyInit;
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use serde::Deserialize;
use sha2::Sha256;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use thiserror::Error;

const FORMAT_B_SIGNATURE: &str = "b~>buttercup/b";
const DEFAULT_ALGORITHM: &str = "cbc";
const PASSWORD_KEY_SIZE: usize = 32;
const HMAC_KEY_SIZE: usize = 32;

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
    pub groups: Vec<String>,
    pub entries: Vec<ButtercupEntry>,
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
    pub tags: Vec<String>,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub fields: Vec<ButtercupCustomField>,
}

#[derive(Debug, Deserialize)]
struct RawVault {
    #[serde(default)]
    #[allow(dead_code)]
    _id: Option<String>,
    #[serde(default)]
    a: HashMap<String, RawValue>,
    #[serde(default)]
    g: Vec<RawGroup>,
    #[serde(default)]
    e: Vec<RawEntry>,
    #[serde(default)]
    #[allow(dead_code)]
    _c: String,
}

#[derive(Debug, Deserialize)]
struct RawGroup {
    id: String,
    #[serde(default)]
    #[allow(dead_code)]
    g: String,
    #[serde(default)]
    t: String,
    #[serde(default)]
    #[allow(dead_code)]
    _a: HashMap<String, RawValue>,
}

#[derive(Debug, Deserialize)]
struct RawEntry {
    id: String,
    #[serde(default)]
    g: String,
    #[serde(default)]
    p: HashMap<String, RawValue>,
    #[serde(default)]
    a: HashMap<String, RawValue>,
}

#[derive(Debug, Deserialize)]
struct RawValue {
    #[serde(default)]
    value: String,
    #[serde(default)]
    deleted: Option<u64>,
}

struct EncryptedComponents {
    content: String,
    iv: String,
    salt: String,
    auth: String,
    rounds: u32,
    method: String,
}

pub fn decrypt_buttercup_file(
    path: &str,
    password: &str,
) -> Result<ButtercupVault, ButtercupError> {
    let contents = std::fs::read_to_string(path)?;
    decrypt_buttercup_vault(&contents, password)
}

pub fn decrypt_buttercup_vault(
    contents: &str,
    password: &str,
) -> Result<ButtercupVault, ButtercupError> {
    if !contents.starts_with(FORMAT_B_SIGNATURE) {
        return Err(ButtercupError::InvalidSignature);
    }

    let encrypted_text = &contents[FORMAT_B_SIGNATURE.len()..];
    let components = parse_encrypted_components(encrypted_text)?;
    let compressed = decrypt_components(&components, password)?;
    let decompressed = decompress(&compressed)?;
    let raw: RawVault = serde_json::from_str(&decompressed)?;

    let mut vault = ButtercupVault {
        name: raw
            .a
            .get("name")
            .map(|v| v.value.clone())
            .unwrap_or_default(),
        groups: Vec::new(),
        entries: Vec::new(),
    };

    let group_titles: HashMap<String, String> =
        raw.g.iter().map(|g| (g.id.clone(), g.t.clone())).collect();

    let mut seen_groups: HashSet<String> = HashSet::new();
    for group in raw.g {
        if !group.t.is_empty() {
            seen_groups.insert(group.t);
        }
    }
    vault.groups = seen_groups.into_iter().collect();

    for entry in raw.e {
        let mut tags = Vec::new();
        if !entry.g.is_empty() {
            if let Some(title) = group_titles.get(&entry.g) {
                if !title.is_empty() {
                    tags.push(title.clone());
                }
            }
        }

        let standard_properties: HashSet<String> =
            ["title", "username", "password", "URL", "notes"]
                .iter()
                .map(|s| s.to_string())
                .collect();
        let mut fields = Vec::new();
        let mut field_index = 0;
        for (property, raw_value) in &entry.p {
            if standard_properties.contains(property) || raw_value.value.is_empty() {
                continue;
            }
            if raw_value.deleted.is_some() {
                continue;
            }
            let field_type = get_field_type(&entry.a, property);
            fields.push(ButtercupCustomField {
                id: format!("{}-cf-{}", entry.id, field_index),
                label: property.clone(),
                field_type,
                value: raw_value.value.clone(),
            });
            field_index += 1;
        }

        vault.entries.push(ButtercupEntry {
            id: entry.id,
            tags,
            title: get_property(&entry.p, "title"),
            username: get_property(&entry.p, "username"),
            password: get_property(&entry.p, "password"),
            url: get_property(&entry.p, "URL"),
            notes: get_property(&entry.p, "notes"),
            fields,
        });
    }

    Ok(vault)
}

fn get_field_type(attributes: &HashMap<String, RawValue>, property: &str) -> String {
    let key = format!("BC_ENTRY_FIELD_TYPE:{}", property);
    attributes
        .get(&key)
        .map(|v| v.value.clone())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "text".to_string())
}

fn parse_encrypted_components(encrypted_text: &str) -> Result<EncryptedComponents, ButtercupError> {
    let parts: Vec<&str> = encrypted_text.split('$').collect();
    if parts.len() < 5 {
        return Err(ButtercupError::InvalidFormat);
    }

    let rounds = parts[4]
        .parse::<u32>()
        .map_err(|_| ButtercupError::InvalidRounds(parts[4].to_string()))?;

    let method = if parts.len() == 5 {
        DEFAULT_ALGORITHM.to_string()
    } else {
        parts[5].to_string().to_lowercase()
    };

    Ok(EncryptedComponents {
        content: parts[0].to_string(),
        iv: parts[1].to_string(),
        salt: parts[2].to_string(),
        auth: parts[3].to_string(),
        rounds,
        method,
    })
}

fn decrypt_components(
    components: &EncryptedComponents,
    password: &str,
) -> Result<String, ButtercupError> {
    match components.method.as_str() {
        "cbc" => decrypt_cbc(components, password),
        "gcm" => decrypt_gcm(components, password),
        other => Err(ButtercupError::UnsupportedAlgorithm(other.to_string())),
    }
}

type DecodedComponents = (Vec<u8>, Vec<u8>, Vec<u8>);

fn decode_common_components(
    components: &EncryptedComponents,
) -> Result<DecodedComponents, ButtercupError> {
    let content_bytes = BASE64.decode(&components.content)?;
    let iv_bytes = hex::decode(&components.iv)?;
    let auth_bytes = hex::decode(&components.auth)?;
    Ok((content_bytes, iv_bytes, auth_bytes))
}

fn decrypt_cbc(components: &EncryptedComponents, password: &str) -> Result<String, ButtercupError> {
    let salt_bytes = components.salt.as_bytes();
    let derived = derive_key(
        password,
        salt_bytes,
        components.rounds,
        PASSWORD_KEY_SIZE + HMAC_KEY_SIZE,
    );
    let (key, hmac_key) = derived.split_at(PASSWORD_KEY_SIZE);

    let (content_bytes, iv_bytes, auth_bytes) = decode_common_components(components)?;

    let mut mac = <Hmac<Sha256> as HmacKeyInit>::new_from_slice(hmac_key)
        .map_err(|e| ButtercupError::DecryptionFailed(e.to_string()))?;
    mac.update(components.content.as_bytes());
    mac.update(components.iv.as_bytes());
    mac.update(components.salt.as_bytes());
    let result = mac.finalize();
    let expected_hmac = result.into_bytes();
    if expected_hmac.as_slice() != auth_bytes.as_slice() {
        return Err(ButtercupError::AuthenticationFailed);
    }

    let cipher = cbc::Decryptor::<aes::Aes256>::new(key.into(), iv_bytes.as_slice().into());
    let mut buffer = content_bytes.to_vec();
    let decrypted = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|e| ButtercupError::DecryptionFailed(e.to_string()))?;

    String::from_utf8(decrypted.to_vec()).map_err(ButtercupError::Utf8)
}

fn decrypt_gcm(components: &EncryptedComponents, password: &str) -> Result<String, ButtercupError> {
    let salt_bytes = components.salt.as_bytes();
    let key = derive_key(password, salt_bytes, components.rounds, PASSWORD_KEY_SIZE);

    let (content_bytes, iv_bytes, auth_tag_bytes) = decode_common_components(components)?;

    let cipher_key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(cipher_key);
    let nonce = Nonce::from_slice(iv_bytes.as_slice());

    let mut full_ciphertext = content_bytes;
    full_ciphertext.extend_from_slice(&auth_tag_bytes);

    let aad = format!("{}{}", components.iv, components.salt).into_bytes();

    let decrypted = cipher
        .decrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: &full_ciphertext,
                aad: &aad,
            },
        )
        .map_err(|e| ButtercupError::DecryptionFailed(e.to_string()))?;

    String::from_utf8(decrypted).map_err(ButtercupError::Utf8)
}

fn derive_key(password: &str, salt: &[u8], rounds: u32, length: usize) -> Vec<u8> {
    let mut output = vec![0u8; length];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, rounds, &mut output);
    output
}

fn decompress(compressed: &str) -> Result<String, ButtercupError> {
    let compressed_bytes = BASE64.decode(compressed)?;
    let mut decoder = GzDecoder::new(compressed_bytes.as_slice());
    let mut output = String::new();
    decoder
        .read_to_string(&mut output)
        .map_err(|e| ButtercupError::DecompressionFailed(e.to_string()))?;
    Ok(output)
}

fn get_property(properties: &HashMap<String, RawValue>, name: &str) -> String {
    properties
        .get(name)
        .map(|v| v.value.clone())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_encrypted_components() {
        let text = "content$iv$salt$auth$125000$cbc";
        let components = parse_encrypted_components(text).unwrap();
        assert_eq!(components.content, "content");
        assert_eq!(components.iv, "iv");
        assert_eq!(components.salt, "salt");
        assert_eq!(components.auth, "auth");
        assert_eq!(components.rounds, 125000);
        assert_eq!(components.method, "cbc");
    }

    #[test]
    fn test_parse_encrypted_components_legacy() {
        let text = "content$iv$salt$auth$125000";
        let components = parse_encrypted_components(text).unwrap();
        assert_eq!(components.method, "cbc");
    }
}
