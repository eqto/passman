use super::types::*;
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
use sha2::Sha256;
use std::io::Read;

pub(super) fn parse_encrypted_components(
    encrypted_text: &str,
) -> Result<EncryptedComponents, ButtercupError> {
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

pub(super) fn decrypt_components(
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

pub(super) fn decompress(compressed: &str) -> Result<String, ButtercupError> {
    let compressed_bytes = BASE64.decode(compressed)?;
    let mut decoder = GzDecoder::new(compressed_bytes.as_slice());
    let mut output = String::new();
    decoder
        .read_to_string(&mut output)
        .map_err(|e| ButtercupError::DecompressionFailed(e.to_string()))?;
    Ok(output)
}
