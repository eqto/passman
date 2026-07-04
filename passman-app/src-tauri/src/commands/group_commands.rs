use passman_core::{random_bytes, Group, Trash, VaultEntry};
use serde::Serialize;
use std::collections::HashSet;
use tauri::State;

use crate::commands::state::{move_group_to_trash, validate_reorder, AppState};

#[derive(Serialize)]
pub(crate) struct GroupDeletionResult {
    pub groups: Vec<Group>,
    pub entries: Vec<VaultEntry>,
    pub trash: Trash,
}

fn collect_child_ids(groups: &[Group], parent_id: &str) -> Vec<String> {
    let mut result = Vec::new();
    for group in groups {
        if group.parent_id.as_deref() == Some(parent_id) {
            result.push(group.id.clone());
            result.extend(collect_child_ids(groups, &group.id));
        }
    }
    result
}

#[tauri::command]
pub fn list_groups(path: String, state: State<'_, AppState>) -> Result<Vec<Group>, String> {
    state.with_open_vault(&path, |open_vault| {
        Ok(open_vault.vault.payload.groups.clone())
    })
}

#[tauri::command]
pub fn add_group(
    path: String,
    group: Group,
    state: State<'_, AppState>,
) -> Result<Vec<Group>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        let trimmed = group.name.trim().to_string();
        if trimmed.is_empty() {
            return Err("group name cannot be empty".to_string());
        }
        if group.id.is_empty() {
            return Err("group id cannot be empty".to_string());
        }
        if !open_vault
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g.id == group.id)
        {
            open_vault.vault.payload.groups.push(Group {
                id: group.id,
                name: trimmed,
                parent_id: group.parent_id,
            });
        }
        open_vault.vault.payload.touch();
        Ok(open_vault.vault.payload.groups.clone())
    })
}

#[tauri::command]
pub fn delete_group(
    path: String,
    group_id: String,
    state: State<'_, AppState>,
) -> Result<GroupDeletionResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        if !open_vault
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g.id == group_id)
        {
            return Err("group does not exist".to_string());
        }

        let mut ids_to_remove: HashSet<String> = HashSet::new();
        ids_to_remove.insert(group_id.clone());
        ids_to_remove.extend(collect_child_ids(
            &open_vault.vault.payload.groups,
            &group_id,
        ));

        let group = open_vault
            .vault
            .payload
            .groups
            .iter()
            .find(|g| g.id == group_id)
            .cloned()
            .unwrap_or_else(|| Group {
                id: group_id.clone(),
                name: group_id.clone(),
                parent_id: None,
            });

        open_vault
            .vault
            .payload
            .groups
            .retain(|g| !ids_to_remove.contains(&g.id));

        let mut entries_to_trash: Vec<VaultEntry> = Vec::new();
        open_vault.vault.payload.entries.retain(|e| {
            if e.group_id
                .as_deref()
                .map_or(false, |gid| ids_to_remove.contains(gid))
            {
                entries_to_trash.push(e.clone());
                false
            } else {
                true
            }
        });
        move_group_to_trash(&mut open_vault.vault.payload, group, entries_to_trash);
        open_vault.vault.payload.touch();
        Ok(GroupDeletionResult {
            groups: open_vault.vault.payload.groups.clone(),
            entries: open_vault.vault.payload.entries.clone(),
            trash: open_vault.vault.payload.trash.clone(),
        })
    })
}

#[tauri::command]
pub fn reorder_groups(
    path: String,
    groups: Vec<Group>,
    state: State<'_, AppState>,
) -> Result<Vec<Group>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        validate_reorder(&open_vault.vault.payload.groups, &groups)?;
        open_vault.vault.payload.groups = groups;
        open_vault.vault.payload.touch();
        Ok(open_vault.vault.payload.groups.clone())
    })
}

#[derive(Serialize)]
pub(crate) struct MoveGroupToVaultResult {
    source_groups: Vec<Group>,
    source_entries: Vec<VaultEntry>,
    target_groups: Vec<Group>,
    target_entries: Vec<VaultEntry>,
}

#[tauri::command]
pub fn merge_groups(
    path: String,
    source_id: String,
    target_id: String,
    state: State<'_, AppState>,
) -> Result<(Vec<Group>, Vec<VaultEntry>), String> {
    if source_id == target_id {
        return Err("cannot merge a group into itself".to_string());
    }
    state.with_open_vault_save(&path, |open_vault| {
        if !open_vault
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g.id == source_id)
        {
            return Err("source group does not exist".to_string());
        }
        if !open_vault
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g.id == target_id)
        {
            return Err("target group does not exist".to_string());
        }
        open_vault
            .vault
            .payload
            .groups
            .retain(|g| g.id != source_id);
        let now = chrono::Utc::now();
        for entry in &mut open_vault.vault.payload.entries {
            if entry.group_id.as_deref() == Some(&source_id) {
                entry.group_id = Some(target_id.clone());
                entry.updated_at = now;
            }
        }
        open_vault.vault.payload.touch();
        Ok((
            open_vault.vault.payload.groups.clone(),
            open_vault.vault.payload.entries.clone(),
        ))
    })
}

#[tauri::command]
pub fn move_group_to_vault(
    source_path: String,
    target_path: String,
    group_id: String,
    target_group_id: String,
    state: State<'_, AppState>,
) -> Result<MoveGroupToVaultResult, String> {
    if source_path == target_path {
        return Err("source and target vault must be different".to_string());
    }
    let mut guard = state.inner.lock().unwrap();
    let entries_to_move: Vec<VaultEntry> = match guard.open_vaults.get(&source_path) {
        Some(source) => source
            .vault
            .payload
            .entries
            .iter()
            .filter(|e| e.group_id.as_deref() == Some(&group_id))
            .cloned()
            .collect(),
        None => return Err("source vault is not open".to_string()),
    };

    let now = chrono::Utc::now();
    let moved_ids: HashSet<String> = entries_to_move.iter().map(|e| e.id.clone()).collect();
    let source_group = guard
        .open_vaults
        .get(&source_path)
        .and_then(|s| s.vault.payload.groups.iter().find(|g| g.id == group_id))
        .cloned()
        .unwrap_or_else(|| Group {
            id: target_group_id.clone(),
            name: target_group_id.clone(),
            parent_id: None,
        });
    {
        let source = guard
            .open_vaults
            .get_mut(&source_path)
            .ok_or("source vault is not open")?;
        source
            .vault
            .payload
            .entries
            .retain(|e| !moved_ids.contains(&e.id));
        let group_still_used = source
            .vault
            .payload
            .entries
            .iter()
            .any(|e| e.group_id.as_deref() == Some(&group_id));
        if !group_still_used {
            source.vault.payload.groups.retain(|g| g.id != group_id);
        }
        source.vault.payload.touch();
    }
    {
        let target = guard
            .open_vaults
            .get_mut(&target_path)
            .ok_or("target vault is not open")?;
        if !target
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g.id == target_group_id)
        {
            target.vault.payload.groups.push(source_group);
        }
        for mut entry in entries_to_move {
            entry.group_id = Some(target_group_id.clone());
            entry.updated_at = now;
            if let Some(existing) = target
                .vault
                .payload
                .entries
                .iter_mut()
                .find(|e| e.id == entry.id)
            {
                *existing = entry;
            } else {
                target.vault.payload.entries.push(entry);
            }
        }
        target.vault.payload.touch();
    }
    let source = guard
        .open_vaults
        .get(&source_path)
        .ok_or("source vault is not open")?;
    let target = guard
        .open_vaults
        .get(&target_path)
        .ok_or("target vault is not open")?;
    let result = MoveGroupToVaultResult {
        source_groups: source.vault.payload.groups.clone(),
        source_entries: source.vault.payload.entries.clone(),
        target_groups: target.vault.payload.groups.clone(),
        target_entries: target.vault.payload.entries.clone(),
    };
    drop(guard);
    state.schedule_save(&source_path);
    state.schedule_save(&target_path);
    Ok(result)
}

fn random_entry_id() -> String {
    let bytes = random_bytes(16);
    bytes.iter().map(|b| format!("{:02x}", *b)).collect()
}

#[tauri::command]
pub fn copy_group_to_vault(
    source_path: String,
    target_path: String,
    group_id: String,
    target_group_id: String,
    state: State<'_, AppState>,
) -> Result<(Vec<Group>, Vec<VaultEntry>), String> {
    if source_path == target_path {
        return Err("source and target vault must be different".to_string());
    }
    let mut guard = state.inner.lock().unwrap();
    let entries_to_copy: Vec<VaultEntry> = match guard.open_vaults.get(&source_path) {
        Some(source) => source
            .vault
            .payload
            .entries
            .iter()
            .filter(|e| e.group_id.as_deref() == Some(&group_id))
            .cloned()
            .collect(),
        None => return Err("source vault is not open".to_string()),
    };

    let now = chrono::Utc::now();
    let source_group = guard
        .open_vaults
        .get(&source_path)
        .and_then(|s| s.vault.payload.groups.iter().find(|g| g.id == group_id))
        .cloned()
        .unwrap_or_else(|| Group {
            id: target_group_id.clone(),
            name: target_group_id.clone(),
            parent_id: None,
        });
    {
        let target = guard
            .open_vaults
            .get_mut(&target_path)
            .ok_or("target vault is not open")?;
        if !target
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g.id == target_group_id)
        {
            target.vault.payload.groups.push(source_group);
        }
        for entry in entries_to_copy {
            let mut copy = entry.clone();
            copy.id = random_entry_id();
            copy.group_id = Some(target_group_id.clone());
            copy.created_at = now;
            copy.updated_at = now;
            if let Some(existing) = target
                .vault
                .payload
                .entries
                .iter_mut()
                .find(|e| e.id == copy.id)
            {
                *existing = copy;
            } else {
                target.vault.payload.entries.push(copy);
            }
        }
        target.vault.payload.touch();
        let groups = target.vault.payload.groups.clone();
        let entries = target.vault.payload.entries.clone();
        drop(guard);
        state.schedule_save(&target_path);
        Ok((groups, entries))
    }
}

#[tauri::command]
pub fn add_tag(
    path: String,
    tag: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        let trimmed = tag.trim().to_string();
        if trimmed.is_empty() {
            return Err("tag name cannot be empty".to_string());
        }
        if !open_vault.vault.payload.tags.iter().any(|t| t == &trimmed) {
            open_vault.vault.payload.tags.push(trimmed);
        }
        open_vault.vault.payload.touch();
        Ok(open_vault.vault.payload.tags.clone())
    })
}
