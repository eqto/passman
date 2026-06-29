use passman_core::{TrashGroup, VaultEntry, VaultFile};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;

#[derive(serde::Deserialize)]
pub struct PasswordOptions {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub digits: bool,
    pub symbols: bool,
}

#[tauri::command]
pub fn generate_password(options: PasswordOptions) -> Result<String, String> {
    let mut charset = Vec::new();
    if options.uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if options.lowercase {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }
    if options.digits {
        charset.extend_from_slice(b"0123456789");
    }
    if options.symbols {
        charset.extend_from_slice(b"!@#$%^&*-_=+?");
    }
    if charset.is_empty() {
        return Err("at least one character set must be selected".to_string());
    }
    if options.length == 0 {
        return Err("password length must be greater than 0".to_string());
    }

    let mut rng = OsRng;
    let password: Vec<u8> = (0..options.length)
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    Ok(String::from_utf8(password).unwrap())
}

#[derive(Clone, serde::Serialize)]
pub struct VaultFileDTO {
    pub path: String,
    pub name: String,
    pub groups: Vec<String>,
    pub tags: Vec<String>,
    pub entries: Vec<VaultEntry>,
    pub trash: Vec<TrashGroup>,
}

pub fn vault_to_dto(vault: &VaultFile) -> VaultFileDTO {
    VaultFileDTO {
        path: vault.path.clone(),
        name: vault.payload.vault_metadata.name.clone(),
        groups: vault.payload.groups.clone(),
        tags: vault.payload.tags.clone(),
        entries: vault.payload.entries.clone(),
        trash: vault.payload.trash.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() {
        let password = generate_password(PasswordOptions {
            length: 16,
            uppercase: true,
            lowercase: true,
            digits: true,
            symbols: true,
        })
        .unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_respects_charset() {
        let password = generate_password(PasswordOptions {
            length: 20,
            uppercase: false,
            lowercase: true,
            digits: false,
            symbols: false,
        })
        .unwrap();
        assert!(password.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_generate_password_empty_charset_fails() {
        let result = generate_password(PasswordOptions {
            length: 10,
            uppercase: false,
            lowercase: false,
            digits: false,
            symbols: false,
        });
        assert!(result.is_err());
    }
}
