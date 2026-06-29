use passman_core::{random_bytes, TrashGroup, VaultEntry};
use serde::Serialize;
use std::collections::HashSet;
use tauri::State;

use crate::commands::state::AppState;

#[derive(Serialize)]
pub(crate) struct GroupDeletionResult {
    pub groups: Vec<String>,
    pub entries: Vec<VaultEntry>,
    pub trash: Vec<TrashGroup>,
}

#[tauri::command]
pub fn list_groups(
    path: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    state.with_open_vault(&path, |open_vault| {
        Ok(open_vault.vault.payload.groups.clone())
    })
}

#[tauri::command]
pub fn add_group(
    path: String,
    group: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        let trimmed = group.trim().to_string();
        if trimmed.is_empty() {
            return Err("group name cannot be empty".to_string());
        }
        if !open_vault
            .vault
            .payload
            .groups
            .iter()
            .any(|g| g == &trimmed)
        {
            open_vault.vault.payload.groups.push(trimmed);
        }
        open_vault.vault.payload.vault_metadata.updated_at = chrono::Utc::now();
        Ok(open_vault.vault.payload.groups.clone())
    })
}

#[tauri::command]
pub fn delete_group(
    path: String,
    group: String,
    state: State<'_, AppState>,
) -> Result<GroupDeletionResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        if !open_vault.vault.payload.groups.contains(&group) {
            return Err("group does not exist".to_string());
        }
        open_vault
            .vault
            .payload
            .groups
            .retain(|g| g != &group);
        let now = chrono::Utc::now();
        let mut entries_to_trash: Vec<VaultEntry> = Vec::new();
        open_vault.vault.payload.entries.retain(|e| {
            if e.tags.iter().any(|t| t == &group) {
                let mut entry = e.clone();
                entry.tags = vec![group.clone()];
                entry.updated_at = now;
                entries_to_trash.push(entry);
                false
            } else {
                true
            }
        });
        if !entries_to_trash.is_empty() {
            match open_vault
                .vault
                .payload
                .trash
                .iter_mut()
                .find(|tg| tg.group == group)
            {
                Some(tg) => tg.entries.extend(entries_to_trash),
                None => open_vault.vault.payload.trash.push(TrashGroup {
                    group,
                    entries: entries_to_trash,
                }),
            }
        }
        open_vault.vault.payload.vault_metadata.updated_at = now;
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
    groups: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        let current_set: HashSet<String> =
            open_vault.vault.payload.groups.iter().cloned().collect();
        if groups.len() != current_set.len() {
            return Err("invalid group list".to_string());
        }
        let new_set: HashSet<String> = groups.iter().cloned().collect();
        if new_set != current_set {
            return Err("invalid group list".to_string());
        }
        open_vault.vault.payload.groups = groups;
        open_vault.vault.payload.vault_metadata.updated_at = chrono::Utc::now();
        Ok(open_vault.vault.payload.groups.clone())
    })
}

#[derive(Serialize)]
pub(crate) struct MoveGroupToVaultResult {
    source_groups: Vec<String>,
    source_entries: Vec<VaultEntry>,
    target_groups: Vec<String>,
    target_entries: Vec<VaultEntry>,
}

#[tauri::command]
pub fn merge_groups(
    path: String,
    source: String,
    target: String,
    state: State<'_, AppState>,
) -> Result<(Vec<String>, Vec<VaultEntry>), String> {
    if source == target {
        return Err("cannot merge a group into itself".to_string());
    }
    state.with_open_vault_save(&path, |open_vault| {
        if !open_vault.vault.payload.groups.contains(&source) {
            return Err("source group does not exist".to_string());
        }
        if !open_vault.vault.payload.groups.contains(&target) {
            return Err("target group does not exist".to_string());
        }
        open_vault.vault.payload.groups.retain(|g| g != &source);
        let now = chrono::Utc::now();
        for entry in &mut open_vault.vault.payload.entries {
            if entry.tags.iter().any(|t| t == &source) {
                entry.tags.retain(|t| t != &source);
                if !entry.tags.iter().any(|t| t == &target) {
                    entry.tags.push(target.clone());
                }
                entry.updated_at = now;
            }
        }
        open_vault.vault.payload.vault_metadata.updated_at = now;
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
    group: String,
    target_group: String,
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
            .filter(|e| e.tags.iter().any(|t| t == &group))
            .cloned()
            .collect(),
        None => return Err("source vault is not open".to_string()),
    };

    let now = chrono::Utc::now();
    let moved_ids: HashSet<String> = entries_to_move.iter().map(|e| e.id.clone()).collect();
    {
        let source = guard
            .open_vaults
            .get_mut(&source_path)
            .ok_or("source vault is not open")?;
        source.vault.payload.entries.retain(|e| !moved_ids.contains(&e.id));
        let group_still_used = source
            .vault
            .payload
            .entries
            .iter()
            .any(|e| e.tags.iter().any(|t| t == &group));
        if !group_still_used {
            source.vault.payload.groups.retain(|g| g != &group);
        }
        source.vault.payload.vault_metadata.updated_at = now;
    }
    {
        let target = guard
            .open_vaults
            .get_mut(&target_path)
            .ok_or("target vault is not open")?;
        if !target.vault.payload.groups.contains(&target_group) {
            target.vault.payload.groups.push(target_group.clone());
        }
        for mut entry in entries_to_move {
            entry.tags.retain(|t| t != &group);
            if !entry.tags.contains(&target_group) {
                entry.tags.push(target_group.clone());
            }
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
        target.vault.payload.vault_metadata.updated_at = now;
    }
    let source = guard.open_vaults.get(&source_path).ok_or("source vault is not open")?;
    let target = guard.open_vaults.get(&target_path).ok_or("target vault is not open")?;
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
    group: String,
    target_group: String,
    state: State<'_, AppState>,
) -> Result<(Vec<String>, Vec<VaultEntry>), String> {
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
            .filter(|e| e.tags.iter().any(|t| t == &group))
            .cloned()
            .collect(),
        None => return Err("source vault is not open".to_string()),
    };

    let now = chrono::Utc::now();
    {
        let target = guard
            .open_vaults
            .get_mut(&target_path)
            .ok_or("target vault is not open")?;
        if !target.vault.payload.groups.contains(&target_group) {
            target.vault.payload.groups.push(target_group.clone());
        }
        for entry in entries_to_copy {
            let mut copy = entry.clone();
            copy.id = random_entry_id();
            copy.tags.retain(|t| t != &group);
            if !copy.tags.contains(&target_group) {
                copy.tags.push(target_group.clone());
            }
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
        target.vault.payload.vault_metadata.updated_at = now;
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
        if !open_vault
            .vault
            .payload
            .tags
            .iter()
            .any(|t| t == &trimmed)
        {
            open_vault.vault.payload.tags.push(trimmed);
        }
        open_vault.vault.payload.vault_metadata.updated_at = chrono::Utc::now();
        Ok(open_vault.vault.payload.tags.clone())
    })
}
