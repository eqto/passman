# Passman Component Reference

A complete map of every source file in the Passman project, organized by crate/module.

---

## passman-core

Rust library implementing vault file format, cryptography, and Buttercup import.

### `passman-core/src/lib.rs`
Re-exports `buttercup`, `config`, `crypto`, `vault` modules. Defines `VaultConfig`, `AppConfig`, `VaultFile`, `VaultEntry` public types. Contains integration tests for create/open/save vaults.

- **Exports**: `VaultConfig`, `AppConfig`, `VaultFile`, `VaultEntry`, `VaultHeader`, `VaultMetadata`, `VaultPayload`, `KdfParams`, `TrashGroup`
- **Dependencies**: `serde`, `serde_json`, `chrono`

### `passman-core/src/vault.rs`
Core vault data structures and file I/O. Defines `VaultHeader`, `KdfParamsJson`, `VaultMetadata`, `VaultEntry`, `VaultPayload`, `VaultFile`, `TrashGroup`. Handles vault creation, opening, saving, key derivation, and v1→v2 payload migration.

- **Key functions**: `create_vault_file`, `create_vault_file_with_key`, `open_vault_file`, `open_vault_file_with_key`, `save_vault_file`, `save_vault_file_with_key`, `derive_vault_key`, `vault_exists`
- **Constants**: `MAGIC` = `"PMV "`, `VERSION` = 1, `PAYLOAD_FORMAT_VERSION` = 2
- **Dependencies**: `crypto` module, `serde`, `serde_json`, `chrono`, `zeroize`

### `passman-core/src/crypto.rs` (129 lines)
Cryptographic primitives: Argon2id key derivation, AES-256-GCM encryption/decryption, random byte generation. Defines `KdfParams`, `CryptoError`.

- **Key functions**: `random_bytes`, `derive_key`, `encrypt`, `decrypt`
- **Constants**: `KEY_SIZE` = 32, `NONCE_SIZE` = 12, `TAG_SIZE` = 16, `SALT_SIZE` = 16
- **Dependencies**: `argon2`, `aes-gcm`, `rand`, `zeroize`

### `passman-core/src/config.rs`
Application config file (`~/.config/passman/vaults.json`) read/write. Defines `VaultConfig`, `AppConfig`.

- **Key functions**: `load_config`, `save_config`, `add_vault`, `remove_vault`, `update_vault`
- **Dependencies**: `serde`, `serde_json`, `dirs`

### `passman-core/src/buttercup.rs`
Buttercup vault format parser for import. Supports CBC and GCM decryption. Defines `ButtercupVault`, `ButtercupEntry`, `ButtercupError`.

- **Key functions**: `decrypt_buttercup_file`, `decrypt_buttercup_vault`
- **Constants**: `FORMAT_B_SIGNATURE` = `"b~>buttercup/b"`, `DEFAULT_ALGORITHM` = `"cbc"`, `PASSWORD_KEY_SIZE` = 32, `HMAC_KEY_SIZE` = 32
- **Dependencies**: `aes`, `cbc`, `hmac`, `sha2`, `flate2`, `pbkdf2`, `serde_json`, `base64`

---

## passman-cli

Rust CLI tool for vault creation, import, export, and conversion.

### `passman-cli/src/main.rs`
Command-line interface using `clap`. Subcommands: `Create`, `Import`, `ExportButtercup`, `ImportButtercup`, `Convert`, `Extract`. Handles password prompting via `rpassword`.

- **Key functions**: `create_and_save_vault`, `build_payload`, `prompt_password`, `resolve_convert_password`, `derive_vault_name`
- **Dependencies**: `clap`, `passman-core`, `rpassword`, `serde`, `serde_json`

---

## passman-app (Tauri Backend)

Rust backend for the desktop app, exposing Tauri IPC commands.

### `passman-app/src-tauri/src/main.rs`
Tauri app entry point. Initializes plugins (dialog, shell, clipboard-manager), manages `AppState`, registers all `invoke_handler` commands.

- **Dependencies**: `tauri`, `commands` module

### `passman-app/src-tauri/src/commands/mod.rs` (11 lines)
Module declarations and re-exports for all command submodules.

- **Re-exports**: `entry_commands::*`, `group_commands::*`, `password::*`, `state::*`, `vault_commands::*`

### `passman-app/src-tauri/src/commands/state.rs`
In-memory application state. Defines `SaveJob`, `OpenVault`, `AppStateInner`, `AppState`. Spawns background save worker thread that listens on `save_tx` channel and emits `save-status` events.

- **Key types**: `AppState` (Clone), `AppStateInner`, `OpenVault`, `SaveJob`
- **Key functions**: `AppState::new`, `AppState::schedule_save`, `AppState::with_open_vault`, `AppState::with_open_vault_save`
- **Dependencies**: `passman-core::vault`, `tauri`, `zeroize`, `std::sync`

### `passman-app/src-tauri/src/commands/vault_commands.rs` (184 lines)
Tauri commands for vault CRUD: `list_vaults`, `create_vault`, `open_vault`, `register_and_open_vault`, `close_vault`, `delete_vault`, `rename_vault`, `reorder_vaults`.

- **Dependencies**: `passman-core::{config, vault}`, `state`, `password` (for DTO)

### `passman-app/src-tauri/src/commands/group_commands.rs`
Tauri commands for group/tag management: `list_groups`, `add_group`, `delete_group`, `reorder_groups`, `merge_groups`, `move_group_to_vault`, `copy_group_to_vault`, `add_tag`.

- **Dependencies**: `passman-core::{VaultEntry, TrashGroup, random_bytes}`, `state`

### `passman-app/src-tauri/src/commands/entry_commands.rs`
Tauri commands for entry CRUD and trash: `list_entries`, `add_entry`, `update_entry`, `delete_entry`, `restore_trash_entry`, `delete_trash_entry`, `list_trash`.

- **Dependencies**: `passman-core::{VaultEntry, TrashGroup}`, `state`

### `passman-app/src-tauri/src/commands/password.rs`
Password generation command and DTO types. Defines `PasswordOptions`, `VaultFileDTO`, `vault_to_dto`. Contains unit tests for password generation.

- **Key functions**: `generate_password`, `vault_to_dto`
- **Dependencies**: `rand`, `passman-core::{VaultEntry, VaultFile}`

---

## passman-app (Svelte Frontend)

### `passman-app/src/main.js`
Vite/Svelte entry point. Imports `App.svelte`, mounts to `#app`, imports `app.css`.

### `passman-app/src/app.css`
Global CSS: CSS custom properties for theming (light/dark), reset styles, shared modal classes (`.modal-overlay`, `.modal`, `.modal-actions`, `.modal-cancel-btn`, `.modal-primary-btn`, `.modal-danger-btn`, `.modal-input`, `.modal-error`, `.modal-form`).

### `passman-app/src/App.svelte`
Root component. Sets theme on mount, loads vault list, initializes save listener. Renders `VaultList` (top bar), `VaultView` (main content), `UnlockDialog` (when vault needs unlocking), `AutoLock`. Handles Ctrl/Cmd+L to lock vault. Displays save status indicator.

- **Props**: none
- **Dependencies**: `VaultList`, `VaultView`, `UnlockDialog`, `AutoLock`, `stores/vaults`, `stores/theme`

### `passman-app/src/stores/vaults.js`
Core Svelte store module. Exports writable stores: `vaults`, `currentVault`, `vaultData`, `saveStatus`. Derived stores: `isUnlocked`, `groups`, `entries`, `tags`. Functions: `loadVaults`, `createVault`, `openVault`, `registerAndOpenVault`, `closeVault`, `lockVault`, `lockVaultByPath`, `unlockVault`, `deleteVault`, `renameVault`, `reorderVaults`, `setVaultViewState`, `initSaveListener`, `updateVaultData`.

- **Dependencies**: `svelte/store`, `@tauri-apps/api/core`, `@tauri-apps/api/event`

### `passman-app/src/stores/entries.js`
Entry operation functions: `addEntry`, `updateEntry`, `deleteEntry`, `moveEntryToGroup`, `moveEntryToVault`, `moveEntriesWithTagToGroup`, `moveEntriesInGroupToTag`, `copyEntryToGroup`, `copyEntryToVault`, `generatePassword`. All call Tauri `invoke` and update `vaultData` store.

- **Dependencies**: `svelte/store`, `@tauri-apps/api/core`, `stores/vaults`

### `passman-app/src/stores/groups.js`
Group/tag operation functions: `addGroup`, `addTag`, `deleteGroup`, `reorderGroups`. All call Tauri `invoke` and update `vaultData` store.

- **Dependencies**: `svelte/store`, `@tauri-apps/api/core`, `stores/vaults`

### `passman-app/src/stores/theme.js`
Theme application utility. Exports `applyTheme(value)` which toggles `.dark` class on `<html>`. Listens to system color scheme changes.

- **Dependencies**: none (plain DOM API)

### `passman-app/src/lib/dragList.js`
Reusable Svelte drag-and-drop reorder utility. Exports `createDragList({ axis, getKey, onReorder })` returning stores (`dragItem`, `dragOver`, `insertBefore`) and handler functions (`dragStart`, `dragEnd`, `dragOver`, `dragLeave`, `drop`).

- **Dependencies**: `svelte/store`

### `passman-app/src/components/VaultList.svelte`
Top tab bar showing all vaults. Handles vault selection, unlock, lock, create, rename (settings), delete. Drag-to-reorder using `createDragList`. Right-click context menu via `VaultContextMenu`.

- **Props**: none
- **Dependencies**: `stores/vaults`, `UnlockDialog`, `CreateVaultDialog`, `VaultSettingsDialog`, `VaultContextMenu`, `RemoveVaultDialog`, `lib/dragList`, `@tauri-apps/plugin-dialog`

### `passman-app/src/components/VaultView.svelte`
Main 3-panel layout: groups sidebar, entry list, entry details/editor. Manages selection state (`selectedGroup`, `selectedTags`, `selectedEntry`, `editingEntry`, `mode`). Resizable panels with persisted widths. Keyboard shortcut Ctrl+C copies selected entry password. `resetSelection()` helper for cleanup.

- **Props**: none
- **Dependencies**: `stores/vaults`, `stores/entries`, `GroupList`, `EntryList`, `EntryDetails`, `EntryEditor`, `@tauri-apps/plugin-clipboard-manager`

### `passman-app/src/components/VaultContextMenu.svelte`
Right-click menu for vault tabs. Actions: Settings (rename), Remove. Dispatches `settings` and `remove` events.

- **Props**: `x`, `y`, `vault`
- **Events**: `settings`, `remove`

### `passman-app/src/components/UnlockDialog.svelte`
Modal dialog for entering vault password. Shows vault name, password input, error messages, indeterminate progress bar during unlock. Dispatches `onUnlock(path, password)` and `onCancel`.

- **Props**: `path`, `name`, `onUnlock`, `onCancel`
- **Dependencies**: shared modal CSS from `app.css`

### `passman-app/src/components/CreateVaultDialog.svelte`
Modal dialog for creating a new vault. Fields: name, file path (with Browse button via `save` dialog), password. Calls `createVault` from stores.

- **Props**: none (dispatches `created`, `cancel` events)
- **Dependencies**: `stores/vaults`, `@tauri-apps/plugin-dialog`, shared modal CSS

### `passman-app/src/components/VaultSettingsDialog.svelte`
Modal dialog for renaming a vault. Shows name input and read-only file path. Calls `renameVault` from stores.

- **Props**: `vault`
- **Events**: `renamed`, `cancel`
- **Dependencies**: `stores/vaults`, shared modal CSS

### `passman-app/src/components/RemoveVaultDialog.svelte`
Confirmation modal for removing a vault from Passman (unregister, not delete file). Shows vault name. Escape key cancels.

- **Props**: `vault`, `onRemove`, `onCancel`
- **Dependencies**: shared modal CSS

### `passman-app/src/components/AddGroupDialog.svelte`
Modal dialog for adding a group or tag (title prop switches label). Enter to add, Escape to cancel. Validates non-empty name.

- **Props**: `title`, `onAdd`, `onCancel`
- **Dependencies**: shared modal CSS

### `passman-app/src/components/AutoLock.svelte`
Invisible component that auto-locks the vault after 5 minutes of inactivity. Resets timer on mousemove, keydown, click. Calls `lockVault` on timeout.

- **Props**: none
- **Dependencies**: `stores/vaults`, `svelte`
- **Constants**: `LOCK_TIMEOUT_MS` = 300000

### `passman-app/src/components/GroupList.svelte`
Sidebar showing groups and tags. Add/delete groups, add tags. Click to select group, click tag to toggle filter. Drag-to-reorder groups using `createDragList`. Right-click context menu via `GroupTagContextMenu`.

- **Props**: `selectedGroup`, `selectedTags`, `onSelectGroup`, `onToggleTag`
- **Dependencies**: `stores/vaults`, `stores/groups`, `stores/entries`, `AddGroupDialog`, `GroupTagContextMenu`, `lib/dragList`

### `passman-app/src/components/GroupTagContextMenu.svelte`
Right-click menu for groups/tags. Shows "Move to group" or "Move to tag" submenu listing all other groups/tags. Dispatches `moveToGroup` or `moveToTag` events.

- **Props**: `x`, `y`, `type` ("group"|"tag"), `item`, `groups`, `tags`
- **Events**: `moveToGroup`, `moveToTag`

### `passman-app/src/components/EntryList.svelte`
Middle panel showing filtered entry list. Search box, "New" button. Each entry row shows title, username, tags. Right-click context menu via `EntryContextMenu`. Click to select, double-click to edit.

- **Props**: `entries`, `selectedEntry`, `onSelect`, `onNew`, `onMoveToGroup`, `onMoveToVault`, `onCopyToGroup`, `onCopyToVault`
- **Dependencies**: `stores/vaults`, `EntryContextMenu`, `@tauri-apps/plugin-clipboard-manager`

### `passman-app/src/components/EntryContextMenu.svelte`
Right-click menu for entries. Actions: Copy Password, Move to (groups + other vaults), Copy to (groups + other vaults). Uses `MoveCopySubmenu` for the move/copy submenus. Auto-positions within viewport.

- **Props**: `x`, `y`, `entry`
- **Events**: `copyPassword`, `moveToGroup`, `moveToVault`, `copyToGroup`, `copyToVault`
- **Dependencies**: `stores/vaults`, `MoveCopySubmenu`

### `passman-app/src/components/MoveCopySubmenu.svelte`
Reusable submenu component for move/copy operations. Shows list of groups and vaults (with locked badge). Hovering a vault shows its groups as a nested submenu. Dispatches `selectGroup` and `selectVaultGroup` events.

- **Props**: `label`, `groups`, `vaults`, `left`, `top`, `menuWidth`
- **Events**: `selectGroup`, `selectVaultGroup`
- **Dependencies**: `stores/vaults` (for `vaultData`)

### `passman-app/src/components/EntryDetails.svelte`
Right panel showing entry details when in view mode. Displays title, URL, username, password (with show/hide toggle), notes, tags. Copy buttons for each field. Edit and Delete buttons.

- **Props**: `entry`, `onEdit`, `onClose`
- **Dependencies**: `stores/entries`, `stores/vaults`, `@tauri-apps/plugin-clipboard-manager`

### `passman-app/src/components/EntryEditor.svelte`
Right panel for creating/editing entries. Fields: title, username, password (with Generate button), URL, notes, tags. Tag input with comma/enter to add. Save calls `addEntry` or `updateEntry` depending on whether entry already has a title.

- **Props**: `entry`, `selectedGroup`, `onClose`
- **Dependencies**: `stores/entries`, `stores/vaults`

---

## Test Files

### `passman-core/tests/buttercup_tests.rs`
Integration tests for Buttercup vault import (CBC and GCM formats).

### `passman-cli/tests/import_tests.rs`
Integration tests for CLI import functionality.

### `passman-app/src-tauri/src/commands/password.rs` (inline tests)
Unit tests for `generate_password`: length, charset restriction, empty charset error.
