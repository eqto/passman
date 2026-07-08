pub mod buttercup;
pub mod config;
pub mod crypto;
pub mod import;
pub mod vault;
pub mod vault_operations;

pub use buttercup::*;
pub use config::*;
pub use crypto::*;
pub use import::*;
pub use vault::*;
pub use vault_operations::*;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_open_vault() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.pmv").to_string_lossy().to_string();

        let vault = create_vault_file(&path, "Test", "password").unwrap();
        assert_eq!(vault.payload.name, "Test");
        assert!(vault_exists(&path));

        let opened = open_vault_file(&path, "password").unwrap();
        assert_eq!(opened.payload.name, "Test");

        let wrong = open_vault_file(&path, "wrong");
        match wrong {
            Err(e) => assert_eq!(e.to_string(), "incorrect password"),
            Ok(_) => panic!("expected error for wrong password"),
        }
    }

    #[test]
    fn test_save_and_load_entries() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.pmv").to_string_lossy().to_string();

        let mut vault = create_vault_file(&path, "Test", "password").unwrap();
        vault.payload.entries.push(VaultEntry {
            id: "1".to_string(),
            title: "Example".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: "https://example.com".to_string(),
            notes: "".to_string(),
            tags: vec![],
            group_id: None,
            fields: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            history: vec![],
        });
        save_vault_file(&vault, "password").unwrap();

        let opened = open_vault_file(&path, "password").unwrap();
        assert_eq!(opened.payload.entries.len(), 1);
        assert_eq!(opened.payload.entries[0].title, "Example");
    }

    #[test]
    fn test_save_and_load_trash_with_groups() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("trash.pmv").to_string_lossy().to_string();

        let mut vault = create_vault_file(&path, "TrashTest", "password").unwrap();
        let group = Group {
            id: "g1".to_string(),
            name: "Social".to_string(),
            parent_id: None,
        };
        vault.payload.groups.push(group.clone());
        let mut entry = VaultEntry {
            id: "e1".to_string(),
            title: "Twitter".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: "https://twitter.com".to_string(),
            notes: "".to_string(),
            tags: vec![],
            group_id: Some("g1".to_string()),
            fields: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            history: vec![],
        };
        // Move entry to trash (root level, no group)
        entry.group_id = None;
        vault.payload.trash.entries.push(entry);
        // Add a deleted group to trash
        vault.payload.trash.groups.push(Group {
            id: "tg1".to_string(),
            name: "Old".to_string(),
            parent_id: None,
        });
        save_vault_file(&vault, "password").unwrap();

        let opened = open_vault_file(&path, "password").unwrap();
        assert_eq!(opened.payload.name, "TrashTest");
        assert_eq!(opened.payload.groups.len(), 1);
        assert_eq!(opened.payload.groups[0].id, "g1");
        assert_eq!(opened.payload.trash.groups.len(), 1);
        assert_eq!(opened.payload.trash.groups[0].id, "tg1");
        assert_eq!(opened.payload.trash.entries.len(), 1);
        assert_eq!(opened.payload.trash.entries[0].title, "Twitter");
        assert!(opened.payload.trash.entries[0].group_id.is_none());
    }
}
