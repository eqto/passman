# Passman Architecture Overview

Passman is an offline-first password manager. It is organized as a Rust workspace with a Svelte/Tauri desktop application and a command-line interface.

## Workspace Layout

```text
Passman/
├── Cargo.toml                 # Workspace definition
├── docs/
│   ├── format.md              # PMV file format specification
│   └── architecture.md        # This document
├── passman-core/              # Rust library: crypto, PMV format, Buttercup import
├── passman-cli/               # Rust CLI: create, import, export, convert
└── passman-app/
    ├── src/                   # Svelte 5 frontend
    └── src-tauri/             # Tauri v2 Rust backend
```

## Components

### `passman-core`

The core library owns the vault format and cryptography.

- `crypto.rs` — Argon2id key derivation and AES-256-GCM encryption/decryption.
- `vault.rs` — The PMV (Passman Vault) file format: create, open, save, and data models.
- `config.rs` — Application configuration persisted to the system config directory (`~/.config/passman/vaults.json`).
- `buttercup.rs` — Decrypts and imports legacy Buttercup `.bcup` files.
- `lib.rs` — Re-exports modules and contains integration tests.

### `passman-cli`

A thin CLI wrapper around `passman-core` for power users and migration.

- `main.rs` — `clap` subcommands: `create`, `import`, `export-buttercup`, `import-buttercup`, `convert`, `extract`.
- `tests/import_tests.rs` — Integration tests for import and conversion flows.

### `passman-app` (Tauri + Svelte)

The desktop UI is built with Tauri v2 and Svelte 5.

- `src-tauri/src/main.rs` — Tauri application setup, plugin registration, and command routing.
- `src-tauri/src/commands/` — Tauri command handlers, split into submodules (`vault_commands`, `group_commands`, `entry_commands`, `password`, `state`).
- `src/App.svelte` — Root component: loads vaults, handles unlock/lock, and renders the UI.
- `src/stores/vaults.js` — Svelte stores that wrap `invoke` calls to the Tauri backend.
- `src/components/` — UI components: `VaultList`, `VaultView`, `GroupList`, `EntryList`, `EntryDetails`, `EntryEditor`, `UnlockDialog`, `CreateVaultDialog`, `VaultSettingsDialog`, `VaultContextMenu`, `RemoveVaultDialog`, `AddGroupDialog`, `GroupTagContextMenu`, `EntryContextMenu`, `MoveCopySubmenu`, `AutoLock`.

## Data Flows

### PMV Vault Lifecycle

1. **Create** — `passman-core` generates a random salt, derives a vault key with Argon2id, generates a random data encryption key (DEK), encrypts the DEK with the vault key, and writes the encrypted payload.
2. **Open** — The file header is read, the vault key is derived from the password, the DEK is decrypted, and the JSON payload is decrypted.
3. **Save** — The payload is re-encrypted with the DEK and a fresh nonce, then written atomically via a temp file + rename.

### Desktop Application Flow

1. The Svelte frontend calls `invoke("list_vaults")` to populate the vault list.
2. Unlocking calls `invoke("open_vault")` or `invoke("register_and_open_vault")`.
3. The backend stores the decrypted vault and derived key (wrapped in `Zeroizing<Vec<u8>>`) in `AppState`. The user password is not retained.
4. The frontend uses Svelte stores to display groups and tags. Groups are single-select; tags are multi-select and filter the entry list.
5. Mutations (add/update/delete entry or group) update the in-memory vault and trigger a background save via `schedule_save`.
6. `AutoLock.svelte` locks the vault after 5 minutes of inactivity.

### Buttercup Import Flow

1. The CLI reads a `.bcup` file, parses its `$`-delimited components, and decrypts the content using CBC+HMAC or GCM+AAD.
2. The decrypted JSON is decompressed and mapped to PMV group strings and entry tags.
3. A new PMV file is created with the mapped data. Old PMV files with payload format version `1` are automatically migrated to version `2` on open.

## Security Notes

- User password → Argon2id (64 MiB, 3 iterations, 4 lanes) → vault key → DEK → payload.
- AES-256-GCM with a random 12-byte nonce and 16-byte authentication tag.
- The derived vault key is held in memory using `Zeroizing<Vec<u8>>` and zeroed on lock. The user password is not stored after key derivation.
- IPC between the Svelte frontend and Tauri backend is local; no network exposure.

## Key Files

- `Cargo.toml` — workspace members and shared dependencies.
- `passman-core/src/vault.rs` — PMV format implementation.
- `passman-core/src/crypto.rs` — cryptography primitives.
- `passman-core/src/config.rs` — app configuration.
- `passman-core/src/buttercup.rs` — Buttercup importer.
- `passman-cli/src/main.rs` — CLI entry point.
- `passman-app/src-tauri/src/commands/` — Tauri command handlers (split into submodules).
- `passman-app/src-tauri/tauri.conf.json` — Tauri application configuration.
- `passman-app/src/stores/vaults.js` — frontend state store.
- `docs/format.md` — detailed PMV format spec.
