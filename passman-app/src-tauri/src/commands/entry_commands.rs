use passman_core::{Group, Trash, VaultEntry};
use serde::Serialize;
use tauri::State;

use passman_core::vault_operations::move_entries_to_trash;

use crate::commands::state::AppState;

#[derive(Serialize)]
pub(crate) struct EntryMutationResult {
    pub entry: VaultEntry,
}

#[derive(Serialize)]
pub(crate) struct EntryDeletionResult {
    pub entries: Vec<VaultEntry>,
    pub trash: Trash,
}

#[derive(Serialize)]
pub(crate) struct TrashMutationResult {
    pub group_id: Option<String>,
    pub group_name: String,
    pub groups: Vec<Group>,
    pub entries: Vec<VaultEntry>,
    pub trash: Trash,
}

#[tauri::command]
pub fn list_entries(path: String, state: State<'_, AppState>) -> Result<Vec<VaultEntry>, String> {
    state.with_open_vault(&path, |open_vault| {
        Ok(open_vault.vault.payload.entries.clone())
    })
}

#[tauri::command]
pub fn add_entry(
    path: String,
    entry: VaultEntry,
    state: State<'_, AppState>,
) -> Result<EntryMutationResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        if open_vault
            .vault
            .payload
            .entries
            .iter()
            .any(|e| e.id == entry.id)
        {
            return Err("an entry with this id already exists".to_string());
        }
        open_vault.vault.payload.entries.push(entry.clone());
        open_vault.vault.payload.touch();
        Ok(EntryMutationResult { entry })
    })
}

#[tauri::command]
pub fn update_entry(
    path: String,
    entry: VaultEntry,
    state: State<'_, AppState>,
) -> Result<EntryMutationResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        if let Some(existing) = open_vault
            .vault
            .payload
            .entries
            .iter_mut()
            .find(|e| e.id == entry.id)
        {
            *existing = entry.clone();
        } else {
            return Err("entry not found".to_string());
        }
        open_vault.vault.payload.touch();
        Ok(EntryMutationResult { entry })
    })
}

#[tauri::command]
pub fn delete_entry(
    path: String,
    id: String,
    state: State<'_, AppState>,
) -> Result<EntryDeletionResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        // First try to delete from trash
        if let Some(pos) = open_vault
            .vault
            .payload
            .trash
            .entries
            .iter()
            .position(|e| e.id == id)
        {
            open_vault.vault.payload.trash.entries.remove(pos);
            open_vault.vault.payload.touch();
            return Ok(EntryDeletionResult {
                entries: open_vault.vault.payload.entries.clone(),
                trash: open_vault.vault.payload.trash.clone(),
            });
        }

        let mut entry_to_trash: Option<VaultEntry> = None;
        open_vault.vault.payload.entries.retain(|e| {
            if e.id == id {
                entry_to_trash = Some(e.clone());
                false
            } else {
                true
            }
        });
        if let Some(entry) = entry_to_trash {
            move_entries_to_trash(&mut open_vault.vault.payload, vec![entry]);
        }
        open_vault.vault.payload.touch();
        Ok(EntryDeletionResult {
            entries: open_vault.vault.payload.entries.clone(),
            trash: open_vault.vault.payload.trash.clone(),
        })
    })
}

#[tauri::command]
pub fn restore_trash_entry(
    path: String,
    id: String,
    state: State<'_, AppState>,
) -> Result<TrashMutationResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        let payload = &mut open_vault.vault.payload;

        let pos = payload
            .trash
            .entries
            .iter()
            .position(|e| e.id == id)
            .ok_or_else(|| "entry not found in trash".to_string())?;

        let mut entry = payload.trash.entries.remove(pos);
        let group_id = entry.group_id.clone();
        let group_name = group_id
            .as_ref()
            .and_then(|gid| {
                payload
                    .trash
                    .groups
                    .iter()
                    .find(|g| &g.id == gid)
                    .map(|g| g.name.clone())
            })
            .unwrap_or_default();
        entry.updated_at = chrono::Utc::now();

        if let Some(ref gid) = group_id {
            restore_group_if_missing(payload, gid, &group_name);
        }

        if !payload.entries.iter().any(|e| e.id == entry.id) {
            payload.entries.push(entry);
        }

        payload.touch();
        Ok(TrashMutationResult {
            group_id,
            group_name,
            groups: payload.groups.clone(),
            entries: payload.entries.clone(),
            trash: payload.trash.clone(),
        })
    })
}

fn restore_group_if_missing(payload: &mut passman_core::VaultPayload, gid: &str, group_name: &str) {
    if payload.groups.iter().any(|g| g.id == gid) {
        return;
    }
    if let Some(trash_group) = payload.trash.groups.iter().find(|g| g.id == gid).cloned() {
        payload.groups.push(trash_group);
    } else {
        payload.groups.push(Group {
            id: gid.to_string(),
            name: group_name.to_string(),
            parent_id: None,
        });
    }
}

#[tauri::command]
pub fn delete_trash_entry(
    path: String,
    id: String,
    state: State<'_, AppState>,
) -> Result<Trash, String> {
    state.with_open_vault_save(&path, |open_vault| {
        open_vault
            .vault
            .payload
            .trash
            .entries
            .retain(|e| e.id != id);
        open_vault.vault.payload.touch();
        Ok(open_vault.vault.payload.trash.clone())
    })
}

#[tauri::command]
pub fn list_trash(path: String, state: State<'_, AppState>) -> Result<Trash, String> {
    state.with_open_vault(&path, |open_vault| {
        Ok(open_vault.vault.payload.trash.clone())
    })
}
