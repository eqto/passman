use passman_core::{buttercup, config, keepass, vault, AppConfig, SecurityLevel, VaultConfig};
use tauri::State;
use zeroize::Zeroizing;

use crate::commands::dto::{vault_to_dto, VaultFileDTO};
use crate::commands::state::{validate_reorder, AppState};

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
    security_level: Option<String>,
    state: State<'_, AppState>,
) -> Result<VaultConfig, String> {
    if vault::vault_exists(&path) {
        return Err("vault file already exists".to_string());
    }
    let level = parse_security_level(security_level)?;
    let (vault, vault_key) =
        vault::create_vault_file_with_level(&path, &name, &password, level)
            .map_err(|e| e.to_string())?;
    config::add_vault(&id, &name, &path).map_err(|e| e.to_string())?;

    state.insert_vault(&path, vault, vault_key);

    Ok(VaultConfig { id, name, path })
}

/// Shared logic for opening a vault file and inserting it into state.
fn open_vault_inner(
    path: &str,
    password: &str,
    state: &AppState,
) -> Result<(VaultFileDTO, [u8; passman_core::KEY_SIZE]), String> {
    let (vault, vault_key) =
        vault::open_vault_file_with_key(path, password).map_err(|e| e.to_string())?;
    let needs_save = vault.needs_save;
    let dto = vault_to_dto(&vault);

    state.insert_vault(path, vault, vault_key);
    if needs_save {
        state.schedule_save(path);
    }

    Ok((dto, vault_key))
}

#[tauri::command]
pub async fn open_vault(
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    let (dto, _) = open_vault_inner(&path, &password, &state)?;
    Ok(dto)
}

#[tauri::command]
pub async fn register_and_open_vault(
    id: String,
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    let (dto, _) = open_vault_inner(&path, &password, &state)?;
    let name = dto.name.clone();
    config::add_vault(&id, &name, &path).map_err(|e| e.to_string())?;
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
            open_vault.vault.payload.name = name;
            open_vault.vault.payload.touch();
        }
    }

    state.schedule_save(&path);
    Ok(updated)
}

#[tauri::command]
pub fn reorder_vaults(ids: Vec<String>) -> Result<Vec<VaultConfig>, String> {
    let mut config = config::load_config().map_err(|e| e.to_string())?;
    let current_ids: Vec<String> = config.vaults.iter().map(|v| v.id.clone()).collect();
    validate_reorder(&current_ids, &ids)?;
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

#[tauri::command]
pub async fn convert_buttercup_vault(
    bcup_path: String,
    password: String,
    output_path: String,
    id: String,
    security_level: Option<String>,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    let bcup =
        buttercup::decrypt_buttercup_file(&bcup_path, &password).map_err(|e| e.to_string())?;

    let import = passman_core::ImportJson::from(bcup);

    let vault_name = passman_core::derive_vault_name(&import.name, &bcup_path);

    let level = parse_security_level(security_level)?;
    let (mut vault, vault_key) =
        vault::create_vault_file_with_level(&output_path, &vault_name, &password, level)
            .map_err(|e| e.to_string())?;

    passman_core::build_payload(&mut vault, import);

    // Save the vault
    vault::save_vault_file(&vault, &password).map_err(|e| e.to_string())?;

    // Register the vault
    config::add_vault(&id, &vault_name, &output_path).map_err(|e| e.to_string())?;

    // Open the vault in state
    let dto = vault_to_dto(&vault);
    state.insert_vault(&output_path, vault, vault_key);

    Ok(dto)
}

#[tauri::command]
pub async fn convert_keepass_vault(
    kdbx_path: String,
    password: String,
    output_path: String,
    id: String,
    security_level: Option<String>,
    state: State<'_, AppState>,
) -> Result<VaultFileDTO, String> {
    let kdbx =
        keepass::decrypt_keepass_file(&kdbx_path, &password).map_err(|e| e.to_string())?;

    let import = passman_core::ImportJson::from(kdbx);

    let vault_name = passman_core::derive_vault_name(&import.name, &kdbx_path);

    let level = parse_security_level(security_level)?;
    let (mut vault, vault_key) =
        vault::create_vault_file_with_level(&output_path, &vault_name, &password, level)
            .map_err(|e| e.to_string())?;

    passman_core::build_payload(&mut vault, import);

    vault::save_vault_file(&vault, &password).map_err(|e| e.to_string())?;

    config::add_vault(&id, &vault_name, &output_path).map_err(|e| e.to_string())?;

    let dto = vault_to_dto(&vault);
    state.insert_vault(&output_path, vault, vault_key);

    Ok(dto)
}

#[tauri::command]
pub async fn change_security_level(
    path: String,
    password: String,
    new_level: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let level = parse_security_level(Some(new_level))?;

    let vault_file = {
        let guard = state.inner.lock().unwrap();
        let open_vault = guard
            .open_vaults
            .get(&path)
            .ok_or_else(|| "vault is not open".to_string())?;
        open_vault.vault.clone()
    };

    let (new_header, new_vault_key) =
        vault::change_kdf_params(&vault_file, &password, level).map_err(|e| e.to_string())?;

    {
        let mut guard = state.inner.lock().unwrap();
        if let Some(open_vault) = guard.open_vaults.get_mut(&path) {
            open_vault.vault.header = new_header;
            open_vault.key = Some(Zeroizing::new(new_vault_key.to_vec()));
        }
    }

    Ok(())
}

fn parse_security_level(level: Option<String>) -> Result<SecurityLevel, String> {
    match level {
        Some(s) => s.parse::<SecurityLevel>().map_err(|e| e.to_string()),
        None => Ok(SecurityLevel::Medium),
    }
}
