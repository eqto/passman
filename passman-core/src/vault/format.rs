use crate::vault::types::*;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn read_vault_file(path: &str) -> Result<(VaultHeader, Vec<u8>), VaultError> {
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

pub fn write_vault_file(
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
