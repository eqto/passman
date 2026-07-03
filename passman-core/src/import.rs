use crate::buttercup::ButtercupVault;
use crate::vault::{CustomField, VaultEntry, VaultFile, VaultMetadata, PAYLOAD_FORMAT_VERSION};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportJson {
    #[serde(default = "default_vault_name")]
    pub name: String,
    #[serde(default)]
    pub groups: Vec<String>,
    #[serde(default)]
    pub entries: Vec<ImportEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportEntry {
    pub id: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub fields: Vec<ImportCustomField>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportCustomField {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub value: String,
}

pub fn default_vault_name() -> String {
    "Imported Vault".to_string()
}

pub fn derive_vault_name(source_name: &str, input_path: &str) -> String {
    if !source_name.is_empty() {
        source_name.to_string()
    } else {
        Path::new(input_path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Imported Buttercup Vault".to_string())
    }
}

pub fn build_payload(vault: &mut VaultFile, imported: ImportJson) {
    let now = chrono::Utc::now();
    vault.payload.vault_metadata = VaultMetadata {
        name: imported.name,
        created_at: now,
        updated_at: now,
        format_version: PAYLOAD_FORMAT_VERSION,
    };

    vault.payload.groups = imported
        .groups
        .into_iter()
        .map(|g| g.trim().to_string())
        .filter(|g| !g.is_empty())
        .collect();

    vault.payload.entries = imported
        .entries
        .into_iter()
        .map(|e| VaultEntry {
            id: e.id,
            title: e.title,
            username: e.username,
            password: e.password,
            url: e.url,
            notes: e.notes,
            tags: e.tags,
            fields: e
                .fields
                .into_iter()
                .map(|f| CustomField {
                    id: f.id,
                    label: f.label,
                    field_type: f.field_type,
                    value: f.value,
                })
                .collect(),
            created_at: now,
            updated_at: now,
        })
        .collect();
}

impl From<ButtercupVault> for ImportJson {
    fn from(vault: ButtercupVault) -> Self {
        ImportJson {
            name: vault.name,
            groups: vault.groups,
            entries: vault
                .entries
                .into_iter()
                .map(|e| ImportEntry {
                    id: e.id,
                    tags: e.tags,
                    title: e.title,
                    username: e.username,
                    password: e.password,
                    url: e.url,
                    notes: e.notes,
                    fields: e
                        .fields
                        .into_iter()
                        .map(|f| ImportCustomField {
                            id: f.id,
                            label: f.label,
                            field_type: f.field_type,
                            value: f.value,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
