use passman_core::{buttercup, config, vault, AppConfig, VaultConfig, VaultEntry, VaultFile, VaultMetadata, PAYLOAD_FORMAT_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
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
pub fn delete_vault(id: String, path: String, state: State<'_, AppState>) -> Result<(), String> {
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

#[derive(Debug, Deserialize, Serialize)]
struct ImportJson {
    #[serde(default = "default_vault_name")]
    name: String,
    #[serde(default)]
    groups: Vec<String>,
    #[serde(default)]
    entries: Vec<ImportEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportEntry {
    id: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    title: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    notes: String,
}

fn default_vault_name() -> String {
    "Imported Vault".to_string()
}

fn derive_vault_name(source_name: &str, input_path: &str) -> String {
    if !source_name.is_empty() {
        source_name.to_string()
    } else {
        Path::new(input_path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Imported Buttercup Vault".to_string())
    }
}

fn build_payload(vault: &mut VaultFile, imported: ImportJson) {
    let now = chrono::Utc::now();
    vault.payload.vault_metadata = VaultMetadata {
        name: imported.name,
        created_at: now,
        updated_at: now,
        format_version: PAYLOAD_FORMAT_VERSION,
    };

    vault.payload.groups = imported
        .groups
        .into_iter()
        .map(|g| g.trim().to_string())
        .filter(|g| !g.is_empty())
        .collect();

    vault.payload.entries = imported
        .entries
        .into_iter()
        .map(|e| VaultEntry {
            id: e.id,
            title: e.title,
            username: e.username,
            password: e.password,
            url: e.url,
            notes: e.notes,
            tags: e.tags,
            created_at: now,
            updated_at: now,
        })
        .collect();
}

#[tauri::command]
pub async fn convert_buttercup_vault(
    bcup_path: String,
    password: String,
    output_path: String,
    id: String,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    // Decrypt buttercup file
    let bcup = buttercup::decrypt_buttercup_file(&bcup_path, &password)
        .map_err(|e| e.to_string())?;
    
    // Convert to ImportJson format
    let import = ImportJson {
        name: bcup.name.clone(),
        groups: bcup.groups,
        entries: bcup
            .entries
            .into_iter()
            .map(|e| ImportEntry {
                id: e.id,
                tags: e.tags,
                title: e.title,
                username: e.username,
                password: e.password,
                url: e.url,
                notes: e.notes,
            })
            .collect(),
    };
    
    // Derive vault name
    let vault_name = derive_vault_name(&import.name, &bcup_path);
    
    // Create new vault with same password
    let (mut vault, vault_key) = vault::create_vault_file_with_key(&output_path, &vault_name, &password)
        .map_err(|e| e.to_string())?;
    
    // Build payload from buttercup data
    build_payload(&mut vault, import);
    
    // Save the vault
    vault::save_vault_file(&vault, &password).map_err(|e| e.to_string())?;
    
    // Register the vault
    config::add_vault(&id, &vault_name, &output_path).map_err(|e| e.to_string())?;
    
    // Open the vault in state
    let dto = vault_to_dto(&vault);
    let mut guard = state.inner.lock().unwrap();
    guard.open_vaults.insert(
        output_path.clone(),
        OpenVault {
            vault,
            key: Some(Zeroizing::new(vault_key.to_vec())),
        },
    );
    
    Ok(dto)
}
