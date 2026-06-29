use passman_core::vault;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use tauri::{AppHandle, Emitter};
use zeroize::Zeroizing;

use passman_core::VaultFile;

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
