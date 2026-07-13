use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn copy_test_bcup(path: &str) -> bool {
    let fixture = Path::new("../fixtures/buttercup/sample.bcup");
    if !fixture.exists() {
        eprintln!("Skipping test: fixture {} not found", fixture.display());
        return false;
    }
    fs::copy(fixture, path).expect("failed to copy buttercup test fixture");
    assert!(Path::new(path).exists(), "test bcup file was not copied");
    true
}

#[test]
fn test_import_creates_pmv() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("import.json");
    let output = dir.path().join("vault.pmv");

    let json = serde_json::json!({
        "name": "Imported",
        "uuid": "vault-uuid-1",
        "groups": [{"id": "g1", "name": "General"}],
        "entries": [
            {
                "id": "e1",
                "group_id": "g1",
                "title": "Example",
                "username": "user",
                "password": "pass",
                "url": "https://example.com",
                "notes": "",
                "tags": [],
                "deleted_at": "2026-06-25T00:00:00Z",
                "history": [
                    {
                        "property": "password",
                        "value": "old-pass",
                        "updated_at": "2026-06-24T00:00:00Z"
                    }
                ],
                "fields": [
                    {
                        "id": "f1",
                        "label": "PIN",
                        "type": "password",
                        "value": "1234"
                    }
                ]
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
    assert_eq!(opened.payload.groups.len(), 1);
    assert_eq!(opened.payload.groups[0].id, "g1");
    assert_eq!(opened.payload.groups[0].name, "General");
    assert_eq!(opened.payload.groups[0].parent_id, None);
    assert_eq!(opened.payload.trash.entries.len(), 0);
    assert_eq!(opened.payload.trash.groups.len(), 0);
    assert_eq!(opened.payload.entries.len(), 1);
    assert_eq!(opened.payload.uuid, Some("vault-uuid-1".to_string()));
    assert_eq!(opened.payload.entries[0].group_id, Some("g1".to_string()));
    assert!(opened.payload.entries[0].tags.is_empty());
    assert_eq!(opened.payload.entries[0].fields.len(), 2);
    assert_eq!(opened.payload.entries[0].fields[0].label, "PIN");
    assert_eq!(opened.payload.entries[0].fields[0].field_type, "password");
    assert_eq!(opened.payload.entries[0].fields[0].value, "1234");
    assert_eq!(opened.payload.entries[0].fields[1].label, "URL");
    assert_eq!(opened.payload.entries[0].fields[1].field_type, "text");
    assert_eq!(opened.payload.entries[0].fields[1].value, "https://example.com");
    assert!(opened.payload.entries[0].deleted_at.is_some());
    assert_eq!(opened.payload.entries[0].history.len(), 1);
    assert_eq!(opened.payload.entries[0].history[0].property, "password");
    assert_eq!(opened.payload.entries[0].history[0].value, "old-pass");
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

    if !copy_test_bcup(&input) {
        return;
    }

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("convert")
        .arg(&input)
        .arg(&output)
        .env("PASSMAN_PASSWORD", "testpass");
    cmd.assert().success();

    assert!(output.exists());
    assert!(fs::metadata(&output).unwrap().len() > 0);

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "testpass").unwrap();
    assert_eq!(opened.payload.name, "test");
    // The sample fixture uses the Buttercup bc_group_role="trash" attribute.
    // Its trash root has no children, so PMV trash should be empty.
    assert_eq!(opened.payload.trash.groups.len(), 0);
    assert_eq!(opened.payload.trash.entries.len(), 0);
    assert!(
        !opened.payload.groups.iter().any(|g| g.name == "Trash"),
        "Buttercup trash root should not appear in regular groups"
    );
}

#[test]
fn test_import_buttercup_uses_name_flag() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("test.bcup").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    if !copy_test_bcup(&input) {
        return;
    }

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
    assert_eq!(opened.payload.name, "Custom Vault");
}

#[test]
fn test_convert_trash_bcup_promotes_child_groups_to_trash() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("trash.bcup").to_string_lossy().to_string();
    let output = dir.path().join("vault.pmv");

    let fixture = Path::new("../fixtures/buttercup/trash.bcup");
    if !fixture.exists() {
        eprintln!("Skipping test: fixture {} not found", fixture.display());
        return;
    }
    fs::copy(fixture, &input).expect("failed to copy trash bcup fixture");

    let mut cmd = Command::cargo_bin("passman-cli").unwrap();
    cmd.arg("convert")
        .arg(&input)
        .arg(&output)
        .env("BCUP_PASSWORD", "test")
        .env("PASSMAN_PASSWORD", "test");
    cmd.assert().success();

    let opened = passman_core::open_vault_file(output.to_str().unwrap(), "test").unwrap();
    // The Buttercup trash root itself should not appear in PMV trash.
    assert!(
        !opened
            .payload
            .trash
            .groups
            .iter()
            .any(|g| g.name == "Trash"),
        "Buttercup trash root should not appear as a group inside PMV trash"
    );
    // The child group inside the Buttercup trash root should be promoted to a root trash group.
    assert_eq!(opened.payload.trash.groups.len(), 1);
    assert_eq!(opened.payload.trash.groups[0].name, "groupt");
    assert_eq!(opened.payload.trash.groups[0].parent_id, None);
    // The entry inside the child group should be in trash.entries with its group_id preserved.
    assert_eq!(opened.payload.trash.entries.len(), 1);
    assert_eq!(
        opened.payload.trash.entries[0].group_id,
        Some(opened.payload.trash.groups[0].id.clone())
    );
    // Neither the trash root nor the child group should appear in regular groups.
    assert!(!opened
        .payload
        .groups
        .iter()
        .any(|g| g.name == "Trash" || g.name == "groupt"));
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
    assert_eq!(payload["name"], "Diva Vault");
    assert_eq!(payload["entries"], serde_json::json!([]));
}
