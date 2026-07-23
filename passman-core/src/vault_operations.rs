use crate::random_bytes;
use crate::vault::{Group, Trash, VaultEntry, VaultPayload};
use std::collections::HashSet;

pub fn collect_child_ids(groups: &[Group], parent_id: &str) -> Vec<String> {
    let mut result = Vec::new();
    for group in groups {
        if group.parent_id.as_deref() == Some(parent_id) {
            result.push(group.id.clone());
            result.extend(collect_child_ids(groups, &group.id));
        }
    }
    result
}

pub fn is_descendant(groups: &[Group], group_id: &str, potential_parent_id: &str) -> bool {
    if group_id == potential_parent_id {
        return true;
    }
    let group = groups.iter().find(|g| g.id == group_id);
    if let Some(parent_id) = group.and_then(|g| g.parent_id.as_deref()) {
        return is_descendant(groups, parent_id, potential_parent_id);
    }
    false
}

pub fn move_group_to_parent(
    payload: &mut VaultPayload,
    group_id: &str,
    new_parent_id: Option<&str>,
) -> Result<Vec<Group>, String> {
    if !payload.groups.iter().any(|g| g.id == group_id) {
        return Err("group does not exist".to_string());
    }

    if let Some(parent_id) = new_parent_id {
        if !payload.groups.iter().any(|g| g.id == parent_id) {
            return Err("parent group does not exist".to_string());
        }
        if is_descendant(&payload.groups, parent_id, group_id) {
            return Err("cannot move group into its own descendant".to_string());
        }
    }

    for group in &mut payload.groups {
        if group.id == group_id {
            group.parent_id = new_parent_id.map(|s| s.to_string());
            break;
        }
    }
    payload.touch();
    Ok(payload.groups.clone())
}

pub fn move_entries_to_trash(payload: &mut VaultPayload, entries: Vec<VaultEntry>) {
    if entries.is_empty() {
        return;
    }
    let now = chrono::Utc::now();
    payload
        .trash
        .entries
        .extend(prepare_trash_entries(entries, now, None));
}

pub fn move_group_to_trash(payload: &mut VaultPayload, group: Group, entries: Vec<VaultEntry>) {
    let now = chrono::Utc::now();
    let group_id = group.id.clone();
    payload.trash.groups.push(group);
    payload
        .trash
        .entries
        .extend(prepare_trash_entries(entries, now, Some(&group_id)));
}

fn prepare_trash_entries(
    entries: Vec<VaultEntry>,
    now: chrono::DateTime<chrono::Utc>,
    group_id_override: Option<&str>,
) -> Vec<VaultEntry> {
    entries
        .into_iter()
        .map(|mut e| {
            e.group_id = group_id_override.map(|s| s.to_string());
            e.updated_at = now;
            e
        })
        .collect()
}

pub fn random_entry_id() -> String {
    let bytes = random_bytes(16);
    bytes.iter().map(|b| format!("{:02x}", *b)).collect()
}

pub struct GroupDeletionResult {
    pub groups: Vec<Group>,
    pub entries: Vec<VaultEntry>,
    pub trash: Trash,
}

pub fn delete_group_with_children(
    payload: &mut VaultPayload,
    group_id: &str,
) -> Result<GroupDeletionResult, String> {
    if !payload.groups.iter().any(|g| g.id == group_id) {
        return Err("group does not exist".to_string());
    }

    let mut ids_to_remove: HashSet<String> = HashSet::new();
    ids_to_remove.insert(group_id.to_string());
    ids_to_remove.extend(collect_child_ids(&payload.groups, group_id));

    let group = payload
        .groups
        .iter()
        .find(|g| g.id == group_id)
        .cloned()
        .unwrap_or_else(|| Group {
            id: group_id.to_string(),
            name: group_id.to_string(),
            parent_id: None,
        });

    payload.groups.retain(|g| !ids_to_remove.contains(&g.id));

    let entries_to_trash: Vec<VaultEntry> = payload
        .entries
        .iter()
        .filter(|e| {
            e.group_id
                .as_deref()
                .is_some_and(|gid| ids_to_remove.contains(gid))
        })
        .cloned()
        .collect();
    payload
        .entries
        .retain(|e| !ids_to_remove.contains(e.group_id.as_deref().unwrap_or("")));
    move_group_to_trash(payload, group, entries_to_trash);
    payload.touch();

    Ok(GroupDeletionResult {
        groups: payload.groups.clone(),
        entries: payload.entries.clone(),
        trash: payload.trash.clone(),
    })
}

pub fn merge_groups_in_vault(
    payload: &mut VaultPayload,
    source_id: &str,
    target_id: &str,
) -> Result<(Vec<Group>, Vec<VaultEntry>), String> {
    if source_id == target_id {
        return Err("cannot merge a group into itself".to_string());
    }
    if !payload.groups.iter().any(|g| g.id == source_id) {
        return Err("source group does not exist".to_string());
    }
    if !payload.groups.iter().any(|g| g.id == target_id) {
        return Err("target group does not exist".to_string());
    }
    payload.groups.retain(|g| g.id != source_id);
    let now = chrono::Utc::now();
    for entry in &mut payload.entries {
        if entry.group_id.as_deref() == Some(source_id) {
            entry.group_id = Some(target_id.to_string());
            entry.updated_at = now;
        }
    }
    payload.touch();
    Ok((payload.groups.clone(), payload.entries.clone()))
}

pub struct MoveGroupToVaultResult {
    pub source_groups: Vec<Group>,
    pub source_entries: Vec<VaultEntry>,
    pub target_groups: Vec<Group>,
    pub target_entries: Vec<VaultEntry>,
}

pub struct PreparedGroupMove {
    pub entries: Vec<VaultEntry>,
    pub group: Option<Group>,
    pub source_groups: Vec<Group>,
    pub source_entries: Vec<VaultEntry>,
}

pub fn prepare_move_from_source(
    source: &mut VaultPayload,
    group_id: &str,
    target_group_id: &str,
) -> PreparedGroupMove {
    let entries_to_move: Vec<VaultEntry> = source
        .entries
        .iter()
        .filter(|e| e.group_id.as_deref() == Some(group_id))
        .cloned()
        .collect();

    let moved_ids: HashSet<String> = entries_to_move.iter().map(|e| e.id.clone()).collect();
    let source_group = source
        .groups
        .iter()
        .find(|g| g.id == group_id)
        .cloned()
        .unwrap_or_else(|| Group {
            id: target_group_id.to_string(),
            name: target_group_id.to_string(),
            parent_id: None,
        });

    source.entries.retain(|e| !moved_ids.contains(&e.id));
    let group_still_used = source
        .entries
        .iter()
        .any(|e| e.group_id.as_deref() == Some(group_id));
    if !group_still_used {
        source.groups.retain(|g| g.id != group_id);
    }
    source.touch();

    PreparedGroupMove {
        entries: entries_to_move,
        group: Some(source_group),
        source_groups: source.groups.clone(),
        source_entries: source.entries.clone(),
    }
}

pub fn apply_move_to_target(
    target: &mut VaultPayload,
    target_group_id: &str,
    prepared: PreparedGroupMove,
) -> (Vec<Group>, Vec<VaultEntry>) {
    let now = chrono::Utc::now();

    if !target.groups.iter().any(|g| g.id == target_group_id) {
        if let Some(group) = prepared.group {
            target.groups.push(group);
        }
    }
    for mut entry in prepared.entries {
        entry.group_id = Some(target_group_id.to_string());
        entry.updated_at = now;
        if let Some(existing) = target.entries.iter_mut().find(|e| e.id == entry.id) {
            *existing = entry;
        } else {
            target.entries.push(entry);
        }
    }
    target.touch();

    (target.groups.clone(), target.entries.clone())
}

pub struct PreparedGroupCopy {
    pub entries: Vec<VaultEntry>,
    pub group: Option<Group>,
}

pub fn prepare_copy_from_source(
    source: &VaultPayload,
    group_id: &str,
    target_group_id: &str,
) -> PreparedGroupCopy {
    let entries_to_copy: Vec<VaultEntry> = source
        .entries
        .iter()
        .filter(|e| e.group_id.as_deref() == Some(group_id))
        .cloned()
        .collect();

    let source_group = source
        .groups
        .iter()
        .find(|g| g.id == group_id)
        .cloned()
        .unwrap_or_else(|| Group {
            id: target_group_id.to_string(),
            name: target_group_id.to_string(),
            parent_id: None,
        });

    PreparedGroupCopy {
        entries: entries_to_copy,
        group: Some(source_group),
    }
}

pub fn apply_copy_to_target(
    target: &mut VaultPayload,
    target_group_id: &str,
    prepared: PreparedGroupCopy,
) -> (Vec<Group>, Vec<VaultEntry>) {
    let now = chrono::Utc::now();

    if !target.groups.iter().any(|g| g.id == target_group_id) {
        if let Some(group) = prepared.group {
            target.groups.push(group);
        }
    }
    for entry in prepared.entries {
        let mut copy = entry.clone();
        copy.id = random_entry_id();
        copy.group_id = Some(target_group_id.to_string());
        copy.created_at = now;
        copy.updated_at = now;
        target.entries.push(copy);
    }
    target.touch();

    (target.groups.clone(), target.entries.clone())
}
