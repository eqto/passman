use passman_core::vault_operations::{
    apply_copy_to_target, apply_move_to_target, delete_group_with_children, merge_groups_in_vault,
    move_group_to_parent, prepare_copy_from_source, prepare_move_from_source,
    GroupDeletionResult as CoreGroupDeletionResult,
};
use passman_core::{Group, Trash, VaultEntry};
use serde::Serialize;
use tauri::State;

use crate::commands::state::{validate_reorder, AppState};

fn validate_different_vaults(source: &str, target: &str) -> Result<(), String> {
    if source == target {
        Err("source and target vault must be different".to_string())
    } else {
        Ok(())
    }
}

#[derive(Serialize)]
pub(crate) struct GroupDeletionResult {
    pub groups: Vec<Group>,
    pub entries: Vec<VaultEntry>,
    pub trash: Trash,
}

impl From<CoreGroupDeletionResult> for GroupDeletionResult {
    fn from(r: CoreGroupDeletionResult) -> Self {
        Self {
            groups: r.groups,
            entries: r.entries,
            trash: r.trash,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct MoveGroupToVaultResult {
    source_groups: Vec<Group>,
    source_entries: Vec<VaultEntry>,
    target_groups: Vec<Group>,
    target_entries: Vec<VaultEntry>,
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
        let result = delete_group_with_children(&mut open_vault.vault.payload, &group_id)?;
        Ok(GroupDeletionResult::from(result))
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

#[tauri::command]
pub fn merge_groups(
    path: String,
    source_id: String,
    target_id: String,
    state: State<'_, AppState>,
) -> Result<(Vec<Group>, Vec<VaultEntry>), String> {
    state.with_open_vault_save(&path, |open_vault| {
        merge_groups_in_vault(&mut open_vault.vault.payload, &source_id, &target_id)
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
    validate_different_vaults(&source_path, &target_path)?;
    let mut guard = state.inner.lock().unwrap();
    let prepared = {
        let source = guard
            .open_vaults
            .get_mut(&source_path)
            .ok_or("source vault is not open")?;
        prepare_move_from_source(&mut source.vault.payload, &group_id, &target_group_id)
    };
    let (target_groups, target_entries) = {
        let target = guard
            .open_vaults
            .get_mut(&target_path)
            .ok_or("target vault is not open")?;
        apply_move_to_target(&mut target.vault.payload, &target_group_id, prepared)
    };
    let (source_groups, source_entries) = {
        let source = guard
            .open_vaults
            .get(&source_path)
            .ok_or("source vault is not open")?;
        (
            source.vault.payload.groups.clone(),
            source.vault.payload.entries.clone(),
        )
    };
    drop(guard);
    state.schedule_save(&source_path);
    state.schedule_save(&target_path);
    Ok(MoveGroupToVaultResult {
        source_groups,
        source_entries,
        target_groups,
        target_entries,
    })
}

#[tauri::command]
pub fn copy_group_to_vault(
    source_path: String,
    target_path: String,
    group_id: String,
    target_group_id: String,
    state: State<'_, AppState>,
) -> Result<(Vec<Group>, Vec<VaultEntry>), String> {
    validate_different_vaults(&source_path, &target_path)?;
    let mut guard = state.inner.lock().unwrap();
    let prepared = {
        let source = guard
            .open_vaults
            .get(&source_path)
            .ok_or("source vault is not open")?;
        prepare_copy_from_source(&source.vault.payload, &group_id, &target_group_id)
    };
    let result = {
        let target = guard
            .open_vaults
            .get_mut(&target_path)
            .ok_or("target vault is not open")?;
        apply_copy_to_target(&mut target.vault.payload, &target_group_id, prepared)
    };
    drop(guard);
    state.schedule_save(&target_path);
    Ok(result)
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
        open_vault.vault.payload.tags.sort();
        open_vault.vault.payload.touch();
        Ok(open_vault.vault.payload.tags.clone())
    })
}

#[tauri::command]
pub fn move_group_to_parent_cmd(
    path: String,
    group_id: String,
    new_parent_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Group>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        move_group_to_parent(
            &mut open_vault.vault.payload,
            &group_id,
            new_parent_id.as_deref(),
        )
    })
}
