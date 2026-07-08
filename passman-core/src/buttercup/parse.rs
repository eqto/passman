use super::types::*;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use crate::vault::HistoryItem;

pub(super) fn is_trash_group(group: &RawGroup) -> bool {
    group
        .a
        .get("bc_group_role")
        .map(|v| v.value.eq_ignore_ascii_case("trash"))
        .unwrap_or(false)
}

fn datetime_from_millis(ts: u64) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(ts as i64)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap())
}

fn get_field_type(attributes: &HashMap<String, RawValue>, property: &str) -> String {
    let key = format!("BC_ENTRY_FIELD_TYPE:{}", property);
    attributes
        .get(&key)
        .map(|v| v.value.clone())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "text".to_string())
}

fn get_property(properties: &HashMap<String, RawValue>, name: &str) -> String {
    properties
        .get(name)
        .map(|v| v.value.clone())
        .unwrap_or_default()
}

pub(super) fn identify_trash_groups(raw: &RawVault) -> (Option<String>, HashSet<String>) {
    let mut trash_group_id: Option<String> = None;
    for group in &raw.g {
        if is_trash_group(group) {
            trash_group_id = Some(group.id.clone());
            break;
        }
    }

    let mut trash_group_ids: HashSet<String> = HashSet::new();
    if let Some(tid) = &trash_group_id {
        trash_group_ids.insert(tid.clone());
        loop {
            let mut changed = false;
            for group in &raw.g {
                if !trash_group_ids.contains(&group.id)
                    && !group.g.is_empty()
                    && trash_group_ids.contains(&group.g)
                {
                    trash_group_ids.insert(group.id.clone());
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }

    (trash_group_id, trash_group_ids)
}

pub(super) fn build_groups(
    raw_groups: Vec<RawGroup>,
    trash_group_id: &Option<String>,
    trash_group_ids: &HashSet<String>,
) -> (Vec<ButtercupGroup>, Vec<ButtercupGroup>) {
    let mut groups = Vec::new();
    let mut trash_groups = Vec::new();
    let mut seen_group_ids: HashSet<String> = HashSet::new();

    for group in raw_groups {
        if group.t.is_empty() || !seen_group_ids.insert(group.id.clone()) {
            continue;
        }

        let parent_id = if group.g.is_empty() {
            None
        } else {
            Some(group.g)
        };

        let buttercup_group = ButtercupGroup {
            id: group.id,
            name: group.t,
            parent_id: parent_id.clone(),
        };

        if trash_group_id.as_ref() == Some(&buttercup_group.id) {
            continue;
        }

        if trash_group_ids.contains(&buttercup_group.id) {
            let mut trash_group = buttercup_group;
            if parent_id.as_ref() == trash_group_id.as_ref() {
                trash_group.parent_id = None;
            }
            trash_groups.push(trash_group);
        } else {
            groups.push(buttercup_group);
        }
    }

    (groups, trash_groups)
}

pub(super) fn build_entries(
    raw_entries: Vec<RawEntry>,
    trash_group_ids: &HashSet<String>,
    trash_group_id: &Option<String>,
) -> (Vec<ButtercupEntry>, Vec<ButtercupEntry>) {
    let mut entries = Vec::new();
    let mut trash_entries = Vec::new();

    for entry in raw_entries {
        let group_id = if entry.g.is_empty() {
            None
        } else {
            Some(entry.g)
        };

        let standard_properties: HashSet<String> =
            ["title", "username", "password", "URL", "notes"]
                .iter()
                .map(|s| s.to_string())
                .collect();
        let mut fields = Vec::new();
        let mut field_index = 0;
        for (property, raw_value) in &entry.p {
            if standard_properties.contains(property) || raw_value.value.is_empty() {
                continue;
            }
            if raw_value.deleted.is_some() {
                continue;
            }
            let field_type = get_field_type(&entry.a, property);
            fields.push(ButtercupCustomField {
                id: format!("{}-cf-{}", entry.id, field_index),
                label: property.clone(),
                field_type,
                value: raw_value.value.clone(),
            });
            field_index += 1;
        }

        let mut history = Vec::new();
        for (property, raw_value) in &entry.p {
            for hist in &raw_value.history {
                history.push(HistoryItem {
                    property: property.clone(),
                    value: hist.value.clone(),
                    updated_at: datetime_from_millis(hist.updated),
                });
            }
        }

        let mut buttercup_entry = ButtercupEntry {
            id: entry.id,
            group_id: group_id.clone(),
            title: get_property(&entry.p, "title"),
            username: get_property(&entry.p, "username"),
            password: get_property(&entry.p, "password"),
            url: get_property(&entry.p, "URL"),
            notes: get_property(&entry.p, "notes"),
            fields,
            deleted_at: entry.deleted.map(datetime_from_millis),
            history,
        };

        let is_in_trash_group = group_id
            .as_ref()
            .is_some_and(|gid| trash_group_ids.contains(gid));
        let is_trash = entry.deleted.is_some() || is_in_trash_group;

        if is_trash {
            if group_id.as_ref() == trash_group_id.as_ref() {
                buttercup_entry.group_id = None;
            }
            trash_entries.push(buttercup_entry);
        } else {
            entries.push(buttercup_entry);
        }
    }

    (entries, trash_entries)
}
