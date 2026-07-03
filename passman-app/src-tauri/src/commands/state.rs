use passman_core::vault;
use passman_core::{TrashGroup, VaultEntry, VaultPayload};
use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, Arc, Mutex};
use tauri::{AppHandle, Emitter};
use zeroize::Zeroizing;

use passman_core::VaultFile;

/// Validate that a reordered list contains exactly the same items as the current list.
pub fn validate_reorder<T: std::hash::Hash + Eq + Clone>(current: &[T], reordered: &[T]) -> Result<(), String> {
    let current_set: HashSet<&T> = current.iter().collect();
    if reordered.len() != current_set.len() {
        return Err("invalid list".to_string());
    }
    let new_set: HashSet<&T> = reordered.iter().collect();
    if new_set != current_set {
        return Err("invalid list".to_string());
    }
    Ok(())
}

/// Move entries into the trash group with the given name, creating the group if it doesn't exist.
pub fn move_entries_to_trash(payload: &mut VaultPayload, group: String, entries: Vec<VaultEntry>) {
    if entries.is_empty() {
        return;
    }
    let now = chrono::Utc::now();
    let entries: Vec<VaultEntry> = entries
        .into_iter()
        .map(|mut e| {
            e.tags = vec![group.clone()];
            e.updated_at = now;
            e
        })
        .collect();
    match payload.trash.iter_mut().find(|tg| tg.group == group) {
        Some(tg) => tg.entries.extend(entries),
        None => payload.trash.push(TrashGroup { group, entries }),
    }
}

/// Data required to save a vault without holding the global state lock.
pub(crate) struct SaveJob {
    pub vault: VaultFile,
    pub key: Zeroizing<Vec<u8>>,
}

/// An unlocked vault kept in memory.
pub struct OpenVault {
    pub vault: VaultFile,
    pub key: Option<Zeroizing<Vec<u8>>>,
}

/// In-memory application state for the Tauri backend.
#[derive(Default)]
pub struct AppStateInner {
    pub open_vaults: HashMap<String, OpenVault>,
}

/// Shared application state passed to every Tauri command.
#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<Mutex<AppStateInner>>,
    pub save_tx: mpsc::Sender<SaveJob>,
}

impl AppState {
    pub fn new(app_handle: AppHandle) -> Self {
        let inner = Arc::new(Mutex::new(AppStateInner::default()));
        let (save_tx, save_rx) = mpsc::channel();

        let state = Self { inner, save_tx };

        std::thread::spawn(move || {
            while let Ok(job) = save_rx.recv() {
                let _ = app_handle.emit("save-status", "saving");

                let result = vault::save_vault_file_with_key(&job.vault, &job.key);

                let status = match result {
                    Ok(_) => "saved",
                    Err(_) => "error",
                };
                let _ = app_handle.emit("save-status", status);
            }
        });

        state
    }

    /// Request a background save of the vault at `path`.
    pub fn schedule_save(&self, path: &str) {
        let job = {
            let guard = self.inner.lock().unwrap();
            match guard.open_vaults.get(path) {
                Some(open_vault) => match (&open_vault.key, &open_vault.vault) {
                    (Some(key), vault) => Some(SaveJob {
                        vault: vault.clone(),
                        key: key.clone(),
                    }),
                    _ => None,
                },
                None => None,
            }
        };
        if let Some(job) = job {
            let _ = self.save_tx.send(job);
        }
    }

    /// Run a closure with mutable access to an open vault, returning an error if the vault is not open.
    pub fn with_open_vault<T, F>(&self, path: &str, f: F) -> Result<T, String>
    where
        F: FnOnce(&mut OpenVault) -> Result<T, String>,
    {
        let mut guard = self.inner.lock().unwrap();
        let open_vault = guard
            .open_vaults
            .get_mut(path)
            .ok_or_else(|| "no vault is open".to_string())?;
        f(open_vault)
    }

    /// Like `with_open_vault`, but schedules a background save when the closure succeeds.
    pub fn with_open_vault_save<T, F>(&self, path: &str, f: F) -> Result<T, String>
    where
        F: FnOnce(&mut OpenVault) -> Result<T, String>,
    {
        let result = self.with_open_vault(path, f);
        if result.is_ok() {
            self.schedule_save(path);
        }
        result
    }
}
