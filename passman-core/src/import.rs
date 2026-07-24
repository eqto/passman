use crate::buttercup::{ButtercupEntry, ButtercupGroup, ButtercupVault};
use crate::keepass::{KeePassEntry, KeePassGroup, KeePassVault};
use crate::vault::{CustomField, Group, HistoryItem, Trash, VaultEntry, VaultFile};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportJson {
    #[serde(default = "default_vault_name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub groups: Vec<ImportGroup>,
    #[serde(default)]
    pub entries: Vec<ImportEntry>,
    #[serde(default)]
    pub trash: ImportTrash,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ImportTrash {
    #[serde(default)]
    pub groups: Vec<ImportGroup>,
    #[serde(default)]
    pub entries: Vec<ImportEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportGroup {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportEntry {
    pub id: String,
    #[serde(default)]
    pub group_id: Option<String>,
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
    pub tags: Vec<String>,
    #[serde(default)]
    pub fields: Vec<ImportCustomField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub history: Vec<HistoryItem>,
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

fn map_import_entry_to_vault_entry(e: ImportEntry, now: DateTime<Utc>) -> VaultEntry {
    let mut fields: Vec<CustomField> = e
        .fields
        .into_iter()
        .map(|f| CustomField {
            id: f.id,
            label: f.label,
            field_type: f.field_type,
            value: f.value,
        })
        .collect();

    if !e.url.is_empty() {
        fields.push(CustomField {
            id: format!("{}-cf-url", e.id),
            label: "URL".to_string(),
            field_type: "text".to_string(),
            value: e.url,
        });
    }
    if !e.notes.is_empty() {
        fields.push(CustomField {
            id: format!("{}-cf-notes", e.id),
            label: "Notes".to_string(),
            field_type: "note".to_string(),
            value: e.notes,
        });
    }

    VaultEntry {
        id: e.id,
        title: e.title,
        username: e.username,
        password: e.password,
        tags: e.tags,
        group_id: e.group_id,
        fields,
        created_at: now,
        updated_at: now,
        deleted_at: e.deleted_at,
        history: e.history,
    }
}

fn map_import_group_to_group(g: ImportGroup) -> Group {
    Group {
        id: g.id,
        name: g.name.trim().to_string(),
        parent_id: g.parent_id,
    }
}

pub fn build_payload(vault: &mut VaultFile, imported: ImportJson) {
    let now = chrono::Utc::now();
    vault.payload.name = imported.name;
    vault.payload.uuid = imported.uuid;
    vault.payload.created_at = imported.created_at;
    vault.payload.updated_at = now;

    vault.payload.groups = imported
        .groups
        .into_iter()
        .map(map_import_group_to_group)
        .filter(|g| !g.name.is_empty())
        .collect();

    vault.payload.entries = imported
        .entries
        .into_iter()
        .map(|e| map_import_entry_to_vault_entry(e, now))
        .collect();

    vault.payload.trash = Trash {
        groups: imported
            .trash
            .groups
            .into_iter()
            .map(map_import_group_to_group)
            .filter(|g| !g.name.is_empty())
            .collect(),
        entries: imported
            .trash
            .entries
            .into_iter()
            .map(|e| map_import_entry_to_vault_entry(e, now))
            .collect(),
    };
}

fn map_buttercup_group(g: ButtercupGroup) -> ImportGroup {
    ImportGroup {
        id: g.id,
        name: g.name,
        parent_id: g.parent_id,
    }
}

fn map_buttercup_entry(e: ButtercupEntry) -> ImportEntry {
    ImportEntry {
        id: e.id,
        group_id: e.group_id,
        title: e.title,
        username: e.username,
        password: e.password,
        url: String::new(),
        notes: String::new(),
        tags: Vec::new(),
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
        deleted_at: e.deleted_at,
        history: e.history,
    }
}

fn map_keepass_group(g: KeePassGroup) -> ImportGroup {
    ImportGroup {
        id: g.id,
        name: g.name,
        parent_id: g.parent_id,
    }
}

fn map_keepass_entry(e: KeePassEntry) -> ImportEntry {
    ImportEntry {
        id: e.id,
        group_id: e.group_id,
        title: e.title,
        username: e.username,
        password: e.password,
        url: e.url,
        notes: e.notes,
        tags: e.tags,
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
        deleted_at: e.deleted_at,
        history: Vec::new(),
    }
}

impl From<KeePassVault> for ImportJson {
    fn from(vault: KeePassVault) -> Self {
        let now = chrono::Utc::now();
        ImportJson {
            name: vault.name,
            uuid: vault.uuid,
            created_at: now,
            updated_at: now,
            groups: vault.groups.into_iter().map(map_keepass_group).collect(),
            entries: vault.entries.into_iter().map(map_keepass_entry).collect(),
            trash: ImportTrash {
                groups: vault
                    .trash
                    .groups
                    .into_iter()
                    .map(map_keepass_group)
                    .collect(),
                entries: vault
                    .trash
                    .entries
                    .into_iter()
                    .map(map_keepass_entry)
                    .collect(),
            },
        }
    }
}

impl From<ButtercupVault> for ImportJson {
    fn from(vault: ButtercupVault) -> Self {
        let now = chrono::Utc::now();
        ImportJson {
            name: vault.name,
            uuid: vault.uuid,
            created_at: now,
            updated_at: now,
            groups: vault.groups.into_iter().map(map_buttercup_group).collect(),
            entries: vault.entries.into_iter().map(map_buttercup_entry).collect(),
            trash: ImportTrash {
                groups: vault
                    .trash
                    .groups
                    .into_iter()
                    .map(map_buttercup_group)
                    .collect(),
                entries: vault
                    .trash
                    .entries
                    .into_iter()
                    .map(map_buttercup_entry)
                    .collect(),
            },
        }
    }
}
