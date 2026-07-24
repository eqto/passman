use chrono::{DateTime, Utc};
use keepass::db::fields;
use keepass::{Database, DatabaseKey};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeePassError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("KeePass database error: {0}")]
    Database(#[from] keepass::error::DatabaseOpenError),
    #[error("invalid database: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone)]
pub struct KeePassVault {
    pub name: String,
    pub uuid: Option<String>,
    pub groups: Vec<KeePassGroup>,
    pub entries: Vec<KeePassEntry>,
    pub trash: KeePassTrash,
}

#[derive(Debug, Clone, Default)]
pub struct KeePassTrash {
    pub groups: Vec<KeePassGroup>,
    pub entries: Vec<KeePassEntry>,
}

#[derive(Debug, Clone)]
pub struct KeePassGroup {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeePassCustomField {
    pub id: String,
    pub label: String,
    pub field_type: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct KeePassEntry {
    pub id: String,
    pub group_id: Option<String>,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub tags: Vec<String>,
    pub fields: Vec<KeePassCustomField>,
    pub deleted_at: Option<DateTime<Utc>>,
}

const STANDARD_FIELDS: &[&str] = &[
    fields::TITLE,
    fields::USERNAME,
    fields::PASSWORD,
    fields::URL,
    fields::NOTES,
    fields::OTP,
];

pub fn decrypt_keepass_file(path: &str, password: &str) -> Result<KeePassVault, KeePassError> {
    let mut file = std::fs::File::open(path)?;
    let key = DatabaseKey::new().with_password(password);
    let db = Database::open(&mut file, key)?;

    let name = db
        .meta
        .database_name
        .clone()
        .unwrap_or_else(|| "Imported KeePass".to_string());

    let root = db.root();
    let root_uuid = root.id().uuid().to_string();

    let recycle_bin_group_ids: HashSet<String> = if let Some(rb) = db.recycle_bin() {
        let mut ids = HashSet::new();
        collect_group_ids(&rb, &mut ids);
        ids
    } else {
        HashSet::new()
    };

    let mut groups = Vec::new();
    let mut entries = Vec::new();
    let mut trash_groups = Vec::new();
    let mut trash_entries = Vec::new();

    for subgroup in root.groups() {
        let gid = subgroup.id().uuid().to_string();
        let is_trash = recycle_bin_group_ids.contains(&gid);

        if is_trash {
            collect_trash(
                &subgroup,
                None,
                &recycle_bin_group_ids,
                &mut trash_groups,
                &mut trash_entries,
            );
        } else {
            collect_normal(
                &subgroup,
                None,
                &recycle_bin_group_ids,
                &mut groups,
                &mut entries,
            );
        }
    }

    for entry in root.entries() {
        let eid = entry.id().uuid().to_string();
        entries.push(map_entry(&entry, None, &eid));
    }

    Ok(KeePassVault {
        name,
        uuid: Some(root_uuid),
        groups,
        entries,
        trash: KeePassTrash {
            groups: trash_groups,
            entries: trash_entries,
        },
    })
}

fn collect_group_ids(group: &keepass::db::GroupRef, ids: &mut HashSet<String>) {
    let gid = group.id().uuid().to_string();
    ids.insert(gid);
    for subgroup in group.groups() {
        collect_group_ids(&subgroup, ids);
    }
}

fn collect_normal(
    group: &keepass::db::GroupRef,
    parent_id: Option<String>,
    recycle_bin_ids: &HashSet<String>,
    groups: &mut Vec<KeePassGroup>,
    entries: &mut Vec<KeePassEntry>,
) {
    let gid = group.id().uuid().to_string();

    groups.push(KeePassGroup {
        id: gid.clone(),
        name: group.name.clone(),
        parent_id,
    });

    for entry in group.entries() {
        let eid = entry.id().uuid().to_string();
        entries.push(map_entry(&entry, Some(&gid), &eid));
    }

    for subgroup in group.groups() {
        let subgid = subgroup.id().uuid().to_string();
        if recycle_bin_ids.contains(&subgid) {
            continue;
        }
        collect_normal(&subgroup, Some(gid.clone()), recycle_bin_ids, groups, entries);
    }
}

fn collect_trash(
    group: &keepass::db::GroupRef,
    parent_id: Option<String>,
    recycle_bin_ids: &HashSet<String>,
    groups: &mut Vec<KeePassGroup>,
    entries: &mut Vec<KeePassEntry>,
) {
    let gid = group.id().uuid().to_string();

    groups.push(KeePassGroup {
        id: gid.clone(),
        name: group.name.clone(),
        parent_id,
    });

    for entry in group.entries() {
        let eid = entry.id().uuid().to_string();
        entries.push(map_entry(&entry, Some(&gid), &eid));
    }

    for subgroup in group.groups() {
        let subgid = subgroup.id().uuid().to_string();
        if recycle_bin_ids.contains(&subgid) {
            collect_trash(&subgroup, Some(gid.clone()), recycle_bin_ids, groups, entries);
        } else {
            collect_trash(&subgroup, Some(gid.clone()), recycle_bin_ids, groups, entries);
        }
    }
}

fn map_entry(
    entry: &keepass::db::EntryRef,
    group_id: Option<&str>,
    eid: &str,
) -> KeePassEntry {
    let title = entry.get(fields::TITLE).unwrap_or("").to_string();
    let username = entry.get(fields::USERNAME).unwrap_or("").to_string();
    let password = entry.get(fields::PASSWORD).unwrap_or("").to_string();
    let url = entry.get(fields::URL).unwrap_or("").to_string();
    let notes = entry.get(fields::NOTES).unwrap_or("").to_string();

    let standard: HashSet<&str> = STANDARD_FIELDS.iter().copied().collect();

    let mut fields = Vec::new();
    for (key, _) in &entry.fields {
        if standard.contains(key.as_str()) {
            continue;
        }
        let value = entry.get(key).unwrap_or("").to_string();
        if value.is_empty() {
            continue;
        }
        let field_type = if key == fields::OTP { "otp" } else { "text" };
        fields.push(KeePassCustomField {
            id: format!("{}-cf-{}", eid, fields.len()),
            label: key.clone(),
            field_type: field_type.to_string(),
            value,
        });
    }

    KeePassEntry {
        id: eid.to_string(),
        group_id: group_id.map(|s| s.to_string()),
        title,
        username,
        password,
        url,
        notes,
        tags: entry.tags.clone(),
        fields,
        deleted_at: None,
    }
}
