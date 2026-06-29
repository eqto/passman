use passman_core::{config, vault, AppConfig, VaultConfig};
use std::collections::HashSet;
use tauri::State;
use zeroize::Zeroizing;

use crate::commands::password::{vault_to_dto, VaultFileDTO};
use crate::commands::state::{AppState, OpenVault};

#[tauri::command]
pub fn list_vaults() -> Result<AppConfig, String> {
    config::load_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_vault(
    id: String,
    name: String,
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultConfig, String> {
    if vault::vault_exists(&path) {
        return Err("vault file already exists".to_string());
    }
    let (vault, vault_key) =
        vault::create_vault_file_with_key(&path, &name, &password).map_err(|e| e.to_string())?;
    config::add_vault(&id, &name, &path).map_err(|e| e.to_string())?;

    let mut guard = state.inner.lock().unwrap();
    guard.open_vaults.insert(
        path.clone(),
        OpenVault {
            vault,
            key: Some(Zeroizing::new(vault_key.to_vec())),
        },
    );

    Ok(VaultConfig { id, name, path })
}

#[tauri::command]
pub async fn open_vault(
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    let (vault, vault_key) =
        vault::open_vault_file_with_key(&path, &password).map_err(|e| e.to_string())?;
    let needs_save = vault.needs_save;
    let dto = vault_to_dto(&vault);

    let mut guard = state.inner.lock().unwrap();
    guard.open_vaults.insert(
        path.clone(),
        OpenVault {
            vault,
            key: Some(Zeroizing::new(vault_key.to_vec())),
        },
    );
    drop(guard);
    if needs_save {
        state.schedule_save(&path);
    }

    Ok(dto)
}

#[tauri::command]
pub async fn register_and_open_vault(
    id: String,
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    let (vault, vault_key) =
        vault::open_vault_file_with_key(&path, &password).map_err(|e| e.to_string())?;
    let needs_save = vault.needs_save;
    let name = vault.payload.vault_metadata.name.clone();
    config::add_vault(&id, &name, &path).map_err(|e| e.to_string())?;
    let dto = vault_to_dto(&vault);

    let mut guard = state.inner.lock().unwrap();
    guard.open_vaults.insert(
        path.clone(),
        OpenVault {
            vault,
            key: Some(Zeroizing::new(vault_key.to_vec())),
        },
    );
    drop(guard);
    if needs_save {
        state.schedule_save(&path);
    }

    Ok(dto)
}

#[tauri::command]
pub fn close_vault(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.inner.lock().unwrap();
    guard.open_vaults.remove(&path);
    Ok(())
}

#[tauri::command]
pub fn delete_vault(
    id: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    config::remove_vault(&id).map_err(|e| e.to_string())?;

    {
        let mut guard = state.inner.lock().unwrap();
        guard.open_vaults.remove(&path);
    }

    Ok(())
}

#[tauri::command]
pub fn rename_vault(
    id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<VaultConfig, String> {
    let mut config = config::load_config().map_err(|e| e.to_string())?;
    let path = config
        .vaults
        .iter()
        .find(|v| v.id == id)
        .map(|v| v.path.clone())
        .ok_or_else(|| "vault not found".to_string())?;

    let is_open = {
        let guard = state.inner.lock().unwrap();
        guard.open_vaults.contains_key(&path)
    };

    if !is_open {
        return Err("vault must be unlocked to rename".to_string());
    }

    let updated = if let Some(vault) = config.vaults.iter_mut().find(|v| v.id == id) {
        vault.name = name.clone();
        vault.clone()
    } else {
        return Err("vault not found".to_string());
    };
    config::save_config(&config).map_err(|e| e.to_string())?;

    {
        let mut guard = state.inner.lock().unwrap();
        if let Some(open_vault) = guard.open_vaults.get_mut(&path) {
            open_vault.vault.payload.vault_metadata.name = name;
            open_vault.vault.payload.vault_metadata.updated_at = chrono::Utc::now();
        }
    }

    state.schedule_save(&path);
    Ok(updated)
}

#[tauri::command]
pub fn reorder_vaults(ids: Vec<String>) -> Result<Vec<VaultConfig>, String> {
    let mut config = config::load_config().map_err(|e| e.to_string())?;
    let current_set: HashSet<String> = config.vaults.iter().map(|v| v.id.clone()).collect();
    if ids.len() != current_set.len() {
        return Err("invalid vault list".to_string());
    }
    let new_set: HashSet<String> = ids.iter().cloned().collect();
    if new_set != current_set {
        return Err("invalid vault list".to_string());
    }
    let mut ordered = Vec::new();
    for id in ids {
        if let Some(vault) = config.vaults.iter().find(|v| v.id == id) {
            ordered.push(vault.clone());
        }
    }
    config.vaults = ordered;
    config::save_config(&config).map_err(|e| e.to_string())?;
    Ok(config.vaults)
}
