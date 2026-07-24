use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, Params};
use rand::RngCore;
use thiserror::Error;

pub const KEY_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 12;
pub const TAG_SIZE: usize = 16;
pub const SALT_SIZE: usize = 16;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("argon2 error: {0}")]
    Argon2(String),
    #[error("encryption error: {0}")]
    Encrypt(String),
    #[error("incorrect password")]
    Decrypt,
}

#[derive(Debug, Clone)]
pub struct KdfParams {
    pub salt: [u8; SALT_SIZE],
    pub memory_kb: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        SecurityLevel::Medium.kdf_params()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecurityLevel {
    Low,
    Medium,
    Secure,
    Best,
}

impl SecurityLevel {
    pub fn kdf_params(self) -> KdfParams {
        let salt: [u8; SALT_SIZE] = random_bytes(SALT_SIZE)
            .try_into()
            .expect("salt generated with correct length");
        let (memory_kb, iterations, parallelism) = match self {
            SecurityLevel::Low => (32_768, 2, 2),
            SecurityLevel::Medium => (65_536, 3, 4),
            SecurityLevel::Secure => (131_072, 4, 4),
            SecurityLevel::Best => (262_144, 6, 8),
        };
        KdfParams {
            salt,
            memory_kb,
            iterations,
            parallelism,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SecurityLevel::Low => "low",
            SecurityLevel::Medium => "medium",
            SecurityLevel::Secure => "secure",
            SecurityLevel::Best => "best",
        }
    }
}

impl std::str::FromStr for SecurityLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "secure" => Ok(Self::Secure),
            "best" => Ok(Self::Best),
            other => Err(format!("unknown security level: {other}")),
        }
    }
}

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    OsRng.fill_bytes(&mut buf);
    buf
}

pub fn derive_key(password: &str, params: &KdfParams) -> Result<[u8; KEY_SIZE], CryptoError> {
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::new(
            params.memory_kb,
            params.iterations,
            params.parallelism,
            Some(KEY_SIZE),
        )
        .map_err(|e| CryptoError::Argon2(e.to_string()))?,
    );

    let mut key = [0u8; KEY_SIZE];
    argon2
        .hash_password_into(password.as_bytes(), &params.salt, &mut key)
        .map_err(|e| CryptoError::Argon2(e.to_string()))?;
    Ok(key)
}

/// Result of an AES-256-GCM encryption: the random nonce and the ciphertext bytes.
pub struct Ciphertext {
    pub nonce: Vec<u8>,
    pub bytes: Vec<u8>,
}

pub fn encrypt(plaintext: &[u8], key: &[u8; KEY_SIZE]) -> Ciphertext {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let bytes = cipher
        .encrypt(&nonce, plaintext)
        .expect("encryption should not fail");
    Ciphertext {
        nonce: nonce.to_vec(),
        bytes,
    }
}

pub fn decrypt(
    ciphertext: &[u8],
    key: &[u8; KEY_SIZE],
    nonce: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(nonce);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::Decrypt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let params = KdfParams::default();
        let key1 = derive_key("password", &params).unwrap();
        let key2 = derive_key("password", &params).unwrap();
        let key3 = derive_key("different", &params).unwrap();
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = random_bytes(KEY_SIZE);
        let key: &[u8; KEY_SIZE] = key.as_slice().try_into().unwrap();
        let plaintext = b"hello world";
        let encrypted = encrypt(plaintext, key);
        let decrypted = decrypt(&encrypted.bytes, key, &encrypted.nonce).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
