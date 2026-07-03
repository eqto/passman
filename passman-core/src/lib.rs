pub mod buttercup;
pub mod config;
pub mod crypto;
pub mod import;
pub mod vault;

pub use buttercup::*;
pub use config::*;
pub use crypto::*;
pub use import::*;
pub use vault::*;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_open_vault() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.pmv").to_string_lossy().to_string();

        let vault = create_vault_file(&path, "Test", "password").unwrap();
        assert_eq!(vault.payload.vault_metadata.name, "Test");
        assert!(vault_exists(&path));

        let opened = open_vault_file(&path, "password").unwrap();
        assert_eq!(opened.payload.vault_metadata.name, "Test");

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
            fields: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });
        save_vault_file(&vault, "password").unwrap();

        let opened = open_vault_file(&path, "password").unwrap();
        assert_eq!(opened.payload.entries.len(), 1);
        assert_eq!(opened.payload.entries[0].title, "Example");
    }
}
