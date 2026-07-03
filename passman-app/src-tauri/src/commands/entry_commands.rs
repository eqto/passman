use passman_core::{TrashGroup, VaultEntry};
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
    pub trash: Vec<TrashGroup>,
}

#[derive(Serialize)]
pub(crate) struct TrashMutationResult {
    pub group: String,
    pub groups: Vec<String>,
    pub entries: Vec<VaultEntry>,
    pub trash: Vec<TrashGroup>,
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
    group: String,
    state: State<'_, AppState>,
) -> Result<EntryDeletionResult, String> {
    state.with_open_vault_save(&path, |open_vault| {
        let mut deleted_from_trash = false;
        for trash_group in &mut open_vault.vault.payload.trash {
            if let Some(pos) = trash_group.entries.iter().position(|e| e.id == id) {
                trash_group.entries.remove(pos);
                deleted_from_trash = true;
                break;
            }
        }
        if deleted_from_trash {
            open_vault
                .vault
                .payload
                .trash
                .retain(|tg| !tg.entries.is_empty());
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
            move_entries_to_trash(&mut open_vault.vault.payload, group, vec![entry]);
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
        let mut restored: Option<(String, VaultEntry)> = None;
        for trash_group in &mut open_vault.vault.payload.trash {
            if let Some(pos) = trash_group.entries.iter().position(|e| e.id == id) {
                let mut entry = trash_group.entries.remove(pos);
                entry.tags = vec![trash_group.group.clone()];
                entry.updated_at = now;
                restored = Some((trash_group.group.clone(), entry));
                break;
            }
        }
        if let Some((group, entry)) = restored {
            open_vault
                .vault
                .payload
                .trash
                .retain(|tg| !tg.entries.is_empty());
            if !open_vault.vault.payload.groups.contains(&group) {
                open_vault.vault.payload.groups.push(group.clone());
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
                group,
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
) -> Result<Vec<TrashGroup>, String> {
    state.with_open_vault_save(&path, |open_vault| {
        for trash_group in &mut open_vault.vault.payload.trash {
            trash_group.entries.retain(|e| e.id != id);
        }
        open_vault
            .vault
            .payload
            .trash
            .retain(|tg| !tg.entries.is_empty());
        open_vault.vault.payload.touch();
        Ok(open_vault.vault.payload.trash.clone())
    })
}

#[tauri::command]
pub fn list_trash(path: String, state: State<'_, AppState>) -> Result<Vec<TrashGroup>, String> {
    state.with_open_vault(&path, |open_vault| {
        Ok(open_vault.vault.payload.trash.clone())
    })
}
