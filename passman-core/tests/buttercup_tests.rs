use passman_core::decrypt_buttercup_file;
use tempfile::tempdir;

fn copy_test_bcup(path: &str) -> bool {
    let fixture = std::path::Path::new("../fixtures/buttercup/sample.bcup");
    if !fixture.exists() {
        eprintln!("Skipping test: fixture {} not found", fixture.display());
        return false;
    }
    std::fs::copy(fixture, path).expect("failed to copy buttercup test fixture");
    assert!(
        std::path::Path::new(path).exists(),
        "test bcup file was not copied to {}",
        path
    );
    true
}

#[test]
fn test_decrypt_buttercup_cbc() {
    let dir = tempdir().unwrap();
    let bcup = dir.path().join("test.bcup").to_string_lossy().to_string();
    if !copy_test_bcup(&bcup) {
        return;
    }

    let vault = decrypt_buttercup_file(&bcup, "testpass").unwrap();
    assert!(!vault.entries.is_empty());
    let entry = vault.entries.iter().find(|e| e.title == "Example").unwrap();
    assert_eq!(entry.username, "user");
    assert_eq!(entry.password, "pass");
    let url_field = entry.fields.iter().find(|f| f.label == "URL").unwrap();
    assert_eq!(url_field.value, "https://example.com");
    let notes_field = entry.fields.iter().find(|f| f.label == "Notes").unwrap();
    assert_eq!(notes_field.value, "note");
}

#[test]
fn test_decrypt_buttercup_wrong_password() {
    let dir = tempdir().unwrap();
    let bcup = dir.path().join("test.bcup").to_string_lossy().to_string();
    if !copy_test_bcup(&bcup) {
        return;
    }

    let result = decrypt_buttercup_file(&bcup, "wrong");
    assert!(result.is_err());
}
