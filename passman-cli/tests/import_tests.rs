use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn copy_test_bcup(path: &str) {
    let fixture = Path::new("../fixtures/buttercup/sample.bcup");
    fs::copy(fixture, path).expect("failed to copy buttercup test fixture");
    assert!(Path::new(path).exists(), "test bcup file was not copied");
}

#[test]
fn test_import_creates_pmv() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("import.json");
    let output = dir.path().join("vault.pmv");

    let json = serde_json::json!({
        "name": "Imported",
        "groups": ["General"],
        "entries": [
            {
                "id": "e1",
                "tags": ["General"],
                "title": "Example",
                "username": "user",
                "password": "pass",
                "url": "https://example.com",
                "notes": ""
            }
        ]
    });
    fs::write(&input, json.to_string()).unwrap();

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("import")
        .arg(&input)
        .arg(&output)
        .arg("--name")
        .arg("Imported")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(output.exists());
    assert!(fs::metadata(&output).unwrap().len() > 0);

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "testpass").unwrap();
    assert_eq!(opened.payload.groups, vec!["General"]);
    assert_eq!(opened.payload.trash.len(), 0);
    assert_eq!(opened.payload.entries.len(), 1);
    assert_eq!(opened.payload.entries[0].tags, vec!["General"]);
}

#[test]
fn test_export_buttercup_fails_for_missing_file() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("missing.bcup");
    let output = dir.path().join("export.json");

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("export-buttercup")
        .arg(&input)
        .arg(&output)
        .env("BCUP_PASSWORD", "testpass");
    cmd.assert().failure();
}

#[test]
fn test_convert_creates_pmv_from_bcup() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test.bcup").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    copy_test_bcup(&input);

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("convert")
        .arg(&input)
        .arg(&output)
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(output.exists());
    assert!(fs::metadata(&output).unwrap().len() > 0);

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "testpass").unwrap();
    assert_eq!(opened.payload.vault_metadata.name, "test");
}

#[test]
fn test_import_buttercup_uses_name_flag() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test.bcup").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    copy_test_bcup(&input);

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("import-buttercup")
        .arg(&input)
        .arg(&output)
        .arg("--name")
        .arg("Custom Vault")
        .env("BCUP_PASSWORD", "testpass")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(output.exists());

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "testpass").unwrap();
    assert_eq!(opened.payload.vault_metadata.name, "Custom Vault");
}

#[test]
fn test_extract_creates_header_and_payload_json() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("diva.pmv");

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("create")
        .arg(&input)
        .arg("--name")
        .arg("Diva Vault")
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(input.exists());

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("extract")
        .arg(&input)
        .env("PASSMAN_PASSWORD", "testpass")
        .current_dir(dir.path());
    cmd.assert().success();

    let extracted_dir = dir.path().join("diva");
    let header_path = extracted_dir.join("header.json");
    let payload_path = extracted_dir.join("payload.json");
    assert!(extracted_dir.exists());
    assert!(header_path.exists());
    assert!(payload_path.exists());

    let header: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&header_path).unwrap()).unwrap();
    assert_eq!(header["version"], 1);
    assert_eq!(header["cipher"], "AES-256-GCM");
    assert_eq!(header["kdf"], "Argon2id");

    let payload: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&payload_path).unwrap()).unwrap();
    assert_eq!(payload["vault_metadata"]["name"], "Diva Vault");
    assert_eq!(payload["entries"], serde_json::json!([]));
}
