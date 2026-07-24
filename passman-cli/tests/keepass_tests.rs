use assert_cmd::Command;
use keepass::db::fields;
use keepass::{Database, DatabaseKey};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn create_test_kdbx(path: &str, password: &str) {
    let mut db = Database::new();
    db.meta.database_name = Some("Test KeePass DB".to_string());

    let mut root = db.root_mut();
    root.name = "Test KeePass DB".to_string();

    // Add a subgroup
    let mut group = root.add_group();
    group.name = "General".to_string();
    let _group_id = group.id().uuid().to_string();

    // Add an entry in the subgroup
    {
        let mut entry = group.add_entry();
        entry.set_unprotected(fields::TITLE, "Test Entry");
        entry.set_unprotected(fields::USERNAME, "testuser");
        entry.set_protected(fields::PASSWORD, "testpass");
        entry.set_unprotected(fields::URL, "https://example.com");
        entry.set_unprotected(fields::NOTES, "Some notes");
        entry.set_unprotected("CustomField", "custom-value");
        entry.tags.push("tag1".to_string());
    }

    // Add an entry in root
    {
        let mut entry = root.add_entry();
        entry.set_unprotected(fields::TITLE, "Root Entry");
        entry.set_unprotected(fields::USERNAME, "rootuser");
        entry.set_protected(fields::PASSWORD, "rootpass");
    }

    let mut file = std::fs::File::create(path).unwrap();
    db.save(&mut file, DatabaseKey::new().with_password(password))
        .unwrap();
}

fn copy_test_kdbx(path: &str) -> bool {
    let fixture = Path::new("../fixtures/keepass/sample.kdbx");
    if fixture.exists() {
        fs::copy(fixture, path).expect("failed to copy keepass test fixture");
        return true;
    }
    // Generate one on the fly if no fixture exists
    create_test_kdbx(path, "testpass");
    true
}

#[test]
fn test_import_keepass_creates_pmv() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test.kdbx").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    if !copy_test_kdbx(&input) {
        return;
    }

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("import-kee-pass")
        .arg(&input)
        .arg(&output)
        .env("KDBX_PASSWORD", "testpass")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(output.exists());
    assert!(fs::metadata(&output).unwrap().len() > 0);

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "testpass").unwrap();
    assert_eq!(opened.payload.name, "Test KeePass DB");
    assert!(opened.payload.groups.iter().any(|g| g.name == "General"));
    assert!(opened.payload.entries.iter().any(|e| e.title == "Test Entry"));
    assert!(opened.payload.entries.iter().any(|e| e.title == "Root Entry"));

    let entry = opened
        .payload
        .entries
        .iter()
        .find(|e| e.title == "Test Entry")
        .unwrap();
    assert_eq!(entry.username, "testuser");
    assert_eq!(entry.password, "testpass");
    assert!(entry.tags.contains(&"tag1".to_string()));

    // URL should be in custom fields
    let url_field = entry.fields.iter().find(|f| f.label == "URL");
    assert!(url_field.is_some());
    assert_eq!(url_field.unwrap().value, "https://example.com");

    // Custom field should be preserved
    let custom = entry.fields.iter().find(|f| f.label == "CustomField");
    assert!(custom.is_some());
    assert_eq!(custom.unwrap().value, "custom-value");
}

#[test]
fn test_import_keepass_fails_for_missing_file() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("missing.kdbx");
    let output = dir.path().join("vault.pmv");

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("import-kee-pass")
        .arg(&input)
        .arg(&output)
        .env("KDBX_PASSWORD", "testpass")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().failure();
}

#[test]
fn test_import_keepass_fails_for_wrong_password() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test.kdbx").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    if !copy_test_kdbx(&input) {
        return;
    }

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("import-kee-pass")
        .arg(&input)
        .arg(&output)
        .env("KDBX_PASSWORD", "wrongpassword")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().failure();
}

#[test]
fn test_import_keepass_uses_name_flag() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test.kdbx").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    if !copy_test_kdbx(&input) {
        return;
    }

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("import-kee-pass")
        .arg(&input)
        .arg(&output)
        .arg("--name")
        .arg("Custom KeePass Vault")
        .env("KDBX_PASSWORD", "testpass")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(output.exists());

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "testpass").unwrap();
    assert_eq!(opened.payload.name, "Custom KeePass Vault");
}
