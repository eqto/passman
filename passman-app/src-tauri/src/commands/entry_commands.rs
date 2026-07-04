use passman_core::{Group, Trash, VaultEntry};
use serde::Serialize;
use tauri::State;

use crate::commands::state::{move_entries_to_trash, AppState};

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
    _group_id: Option<String>,
    _group_name: String,
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
        let now = chrono::Utc::now();
        let mut restored: Option<(Option<String>, String, VaultEntry)> = None;
        if let Some(pos) = open_vault
            .vault
            .payload
            .trash
            .entries
            .iter()
            .position(|e| e.id == id)
        {
            let mut entry = open_vault.vault.payload.trash.entries.remove(pos);
            let group_id = entry.group_id.clone();
            let group_name = group_id
                .as_ref()
                .and_then(|gid| {
                    open_vault
                        .vault
                        .payload
                        .trash
                        .groups
                        .iter()
                        .find(|g| &g.id == gid)
                        .map(|g| g.name.clone())
                })
                .unwrap_or_default();
            entry.updated_at = now;
            restored = Some((group_id, group_name, entry));
        }
        if let Some((group_id, group_name, mut entry)) = restored {
            // Restore the group if it was deleted with the entry
            if let Some(ref gid) = group_id {
                if !open_vault
                    .vault
                    .payload
                    .groups
                    .iter()
                    .any(|g| g.id == *gid)
                {
                    if let Some(trash_group) = open_vault
                        .vault
                        .payload
                        .trash
                        .groups
                        .iter()
                        .find(|g| g.id == *gid)
                        .cloned()
                    {
                        open_vault.vault.payload.groups.push(trash_group);
                    } else {
                        open_vault.vault.payload.groups.push(Group {
                            id: gid.clone(),
                            name: group_name.clone(),
                            parent_id: None,
                        });
                    }
                }
                entry.group_id = Some(gid.clone());
            }
            if !open_vault
                .vault
                .payload
                .entries
                .iter()
                .any(|e| e.id == entry.id)
            {
                open_vault.vault.payload.entries.push(entry);
            }
            open_vault.vault.payload.touch();
            Ok(TrashMutationResult {
                group_id,
                group_name,
                groups: open_vault.vault.payload.groups.clone(),
                entries: open_vault.vault.payload.entries.clone(),
                trash: open_vault.vault.payload.trash.clone(),
            })
        } else {
            Err("entry not found in trash".to_string())
        }
    })
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
