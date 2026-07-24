pub mod decrypt;
pub mod format_a;
pub mod parse;
pub mod types;

pub use types::{
    ButtercupCustomField, ButtercupEntry, ButtercupError, ButtercupGroup, ButtercupTrash,
    ButtercupVault,
};

use types::{Format, RawVault, FORMAT_A_SIGNATURE, FORMAT_B_SIGNATURE};

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
    let format = if contents.starts_with(FORMAT_B_SIGNATURE) {
        Format::B
    } else if contents.starts_with(FORMAT_A_SIGNATURE) {
        Format::A
    } else {
        return Err(ButtercupError::InvalidSignature);
    };

    let sig_len = match format {
        Format::A => FORMAT_A_SIGNATURE.len(),
        Format::B => FORMAT_B_SIGNATURE.len(),
    };

    let encrypted_text = &contents[sig_len..];
    let components = decrypt::parse_encrypted_components(encrypted_text)?;
    let compressed = decrypt::decrypt_components(&components, password)?;
    let decompressed = decrypt::decompress(&compressed)?;

    match format {
        Format::A => format_a::parse_format_a(&decompressed),
        Format::B => {
            let raw: RawVault = serde_json::from_str(&decompressed)?;

            let (trash_group_id, trash_group_ids) = parse::identify_trash_groups(&raw);

            let (groups, trash_groups) =
                parse::build_groups(raw.g, &trash_group_id, &trash_group_ids);
            let (entries, trash_entries) =
                parse::build_entries(raw.e, &trash_group_ids, &trash_group_id);

            Ok(ButtercupVault {
                name: raw
                    .a
                    .get("name")
                    .map(|v| v.value.clone())
                    .unwrap_or_default(),
                uuid: raw._id,
                groups,
                entries,
                trash: ButtercupTrash {
                    groups: trash_groups,
                    entries: trash_entries,
                },
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse::is_trash_group;
    use std::collections::HashMap;
    use types::{RawGroup, RawValue};

    #[test]
    fn test_parse_encrypted_components() {
        let text = "content$iv$salt$auth$125000$cbc";
        let components = decrypt::parse_encrypted_components(text).unwrap();
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
        let components = decrypt::parse_encrypted_components(text).unwrap();
        assert_eq!(components.method, "cbc");
    }

    #[test]
    fn test_is_trash_group_detects_by_bc_group_role() {
        let mut role_attrs = HashMap::new();
        role_attrs.insert(
            "bc_group_role".to_string(),
            RawValue {
                value: "trash".to_string(),
                ..Default::default()
            },
        );
        let role_group = RawGroup {
            id: "g1".to_string(),
            g: "0".to_string(),
            t: "Trash".to_string(),
            a: role_attrs,
        };
        assert!(is_trash_group(&role_group));

        let name_only_group = RawGroup {
            id: "g2".to_string(),
            g: "0".to_string(),
            t: "Trash".to_string(),
            a: HashMap::new(),
        };
        assert!(
            !is_trash_group(&name_only_group),
            "groups named 'Trash' without a bc_group_role='trash' attribute must not be treated as trash"
        );

        let normal_group = RawGroup {
            id: "g3".to_string(),
            g: "0".to_string(),
            t: "General".to_string(),
            a: HashMap::new(),
        };
        assert!(!is_trash_group(&normal_group));
    }
}
