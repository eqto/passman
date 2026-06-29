use passman_core::decrypt_buttercup_file;
use tempfile::tempdir;

fn copy_test_bcup(path: &str) {
    let fixture = std::path::Path::new("../fixtures/buttercup/sample.bcup");
    std::fs::copy(fixture, path).expect("failed to copy buttercup test fixture");
    assert!(
        std::path::Path::new(path).exists(),
        "test bcup file was not copied to {}",
        path
    );
}

#[test]
fn test_decrypt_buttercup_cbc() {
    let dir = tempdir().unwrap();
    let bcup = dir.path().join("test.bcup").to_string_lossy().to_string();
    copy_test_bcup(&bcup);

    let vault = decrypt_buttercup_file(&bcup, "testpass").unwrap();
    assert!(!vault.entries.is_empty());
    let entry = vault.entries.iter().find(|e| e.title == "Example").unwrap();
    assert_eq!(entry.username, "user");
    assert_eq!(entry.password, "pass");
    assert_eq!(entry.url, "https://example.com");
    assert_eq!(entry.notes, "note");
}

#[test]
fn test_decrypt_buttercup_wrong_password() {
    let dir = tempdir().unwrap();
    let bcup = dir.path().join("test.bcup").to_string_lossy().to_string();
    copy_test_bcup(&bcup);

    let result = decrypt_buttercup_file(&bcup, "wrong");
    assert!(result.is_err());
}
