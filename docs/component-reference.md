# Passman Component Reference

A complete map of every source file in the Passman project, organized by Go package and frontend module.

---

## pkg/crypto

Go package implementing cryptographic primitives: Argon2id key derivation, AES-256-GCM encryption/decryption, and security level presets.

### `pkg/crypto/crypto.go`
Cryptographic primitives and security level presets. Defines `SecurityLevel` (Low / Medium / Secure / Best), `KdfParams`, `KdfParamsJSON`, `Ciphertext`, `CryptoError`.

- **Key functions**: `DeriveKey`, `Encrypt`, `Decrypt`, `RandomBytes`, `ParseSecurityLevel`, `KdfParamsToJSON`, `KdfParamsFromJSON`
- **Constants**: `KeySize` = 32, `NonceSize` = 12, `TagSize` = 16, `SaltSize` = 16
- **Security levels**: `SecurityLevelLow` (32 MiB / 2 iter / 2 lanes), `SecurityLevelMedium` (64 MiB / 3 iter / 4 lanes), `SecurityLevelSecure` (128 MiB / 4 iter / 4 lanes), `SecurityLevelBest` (256 MiB / 6 iter / 8 lanes)
- **Dependencies**: `golang.org/x/crypto/argon2`, `crypto/aes`, `crypto/cipher`, `crypto/rand`

---

## pkg/vault

Go package implementing the PMV file format, data models, and group/entry operations.

### `pkg/vault/types.go`
Core vault data structures. Defines `VaultHeader`, `VaultPayload`, `VaultEntry`, `Group`, `Trash`, `CustomField`, `HistoryItem`, `VaultFile`, `VaultFileDTO`, `VaultError`.

- **Key types**: `VaultFile` (Header + Payload + Path + NeedsSave), `VaultFileDTO` (serializable to frontend)
- **Key functions**: `VaultToDTO`, `VaultPayload.Touch`
- **Constants**: `Magic` = `"PMV "`, `Version` = 1, `PayloadFormatVersion` = 1
- **Dependencies**: `pkg/crypto`

### `pkg/vault/format.go`
Binary file I/O for the PMV format. Reads and writes the binary wrapper around the JSON header and encrypted payload.

- **Key functions**: `ReadVaultFile`, `WriteVaultFile`, `VaultExists`
- **Write safety**: Atomic writes via temp file + rename
- **Dependencies**: `encoding/binary`, `encoding/json`, `os`, `path/filepath`

### `pkg/vault/vault.go`
High-level vault operations: create, open, save, and KDF parameter changes.

- **Key functions**: `CreateVaultFile`, `CreateVaultFileWithKey`, `CreateVaultFileWithLevel`, `OpenVaultFile`, `OpenVaultFileWithKey`, `SaveVaultFile`, `SaveVaultFileWithKey`, `ChangeKdfParams`
- **Dependencies**: `pkg/crypto`, `encoding/base64`, `encoding/json`, `time`

### `pkg/vault/operations.go`
Group and entry operations: deletion, merging, move/copy within and across vaults, trash management.

- **Key functions**: `DeleteGroupWithChildren`, `MergeGroupsInVault`, `MoveGroupToParent`, `MoveEntriesToTrash`, `MoveGroupToTrash`, `PrepareMoveFromSource`, `ApplyMoveToTarget`, `PrepareCopyFromSource`, `ApplyCopyToTarget`, `CollectChildIDs`, `IsDescendant`, `RandomEntryID`
- **Key types**: `GroupDeletionResult`, `PreparedGroupMove`, `PreparedGroupCopy`
- **Dependencies**: `crypto/rand`, `encoding/hex`, `time`

### `pkg/vault/vault_test.go`
Unit tests for vault create/open/save operations.

---

## pkg/buttercup

Go package for decrypting and parsing Buttercup `.bcup` vault files.

### `pkg/buttercup/buttercup.go`
Entry point for Buttercup decryption. Detects format (A or B) and dispatches to the appropriate parser.

- **Key functions**: `DecryptButtercupFile`
- **Dependencies**: `pkg/buttercup/decrypt`, `pkg/buttercup/parse`, `pkg/buttercup/format_a`

### `pkg/buttercup/decrypt.go`
Decryption logic for Buttercup vaults. Supports CBC+HMAC and GCM+AAD modes.

- **Key functions**: `decryptCBC`, `decryptGCM`, `deriveButtercupKey`
- **Constants**: `PASSWORD_KEY_SIZE` = 32, `HMAC_KEY_SIZE` = 32
- **Dependencies**: `crypto/aes`, `crypto/cipher`, `crypto/hmac`, `crypto/sha2`, `golang.org/x/crypto/pbkdf2`

### `pkg/buttercup/format_a.go`
Parser for Buttercup Format A (legacy `$`-delimited format).

- **Key functions**: `parseFormatA`, `extractFormatAComponents`

### `pkg/buttercup/parse.go`
Parser for Buttercup Format B (JSON-based format). Extracts groups, entries, trash, and history.

- **Key functions**: `parseFormatB`, `mapButtercupGroups`, `mapButtercupEntries`
- **Dependencies**: `compress/flate`, `encoding/json`, `encoding/base64`

### `pkg/buttercup/types.go`
Buttercup data types. Defines `ButtercupVault`, `ButtercupGroup`, `ButtercupEntry`, `ButtercupTrash`, `ButtercupField`, `HistoryItem`.

---

## pkg/keepass

Go package for decrypting and parsing KeePass `.kdbx` database files.

### `pkg/keepass/keepass.go`
KeePass database parser. Decrypts `.kdbx` files and extracts groups, entries, and custom fields.

- **Key functions**: `DecryptKeePassFile`, `mapKeePassGroups`, `mapKeePassEntries`
- **Key types**: `KeePassVault`, `KeePassGroup`, `KeePassEntry`, `KeePassField`
- **Dependencies**: `github.com/tobischo/gokeepasslib/v3`

---

## internal/app

Wails v3 services exposed to the frontend. Each service is registered in `main.go`.

### `internal/app/vault_service.go`
Vault service: CRUD, import, security level management.

- **Key methods**: `ListVaults`, `CreateVault`, `OpenVault`, `RegisterAndOpenVault`, `CloseVault`, `DeleteVault`, `RenameVault`, `ReorderVaults`, `ConvertButtercupVault`, `ConvertKeepassVault`, `ChangeSecurityLevel`
- **Dependencies**: `internal/config`, `internal/state`, `internal/vimport`, `pkg/buttercup`, `pkg/keepass`, `pkg/crypto`, `pkg/vault`

### `internal/app/group_service.go`
Group service: group CRUD, tag management, cross-vault move/copy.

- **Key methods**: `ListGroups`, `AddGroup`, `DeleteGroup`, `ReorderGroups`, `MergeGroups`, `MoveGroupToVault`, `CopyGroupToVault`, `MoveGroupToParent`, `AddTag`
- **Key types**: `GroupDeletionResult`, `MoveGroupToVaultResult`
- **Dependencies**: `internal/state`, `pkg/vault`

### `internal/app/entry_service.go`
Entry service: entry CRUD, trash management, restore.

- **Key methods**: `ListEntries`, `AddEntry`, `UpdateEntry`, `DeleteEntry`, `RestoreTrashEntry`, `DeleteTrashEntry`, `ListTrash`
- **Key types**: `EntryMutationResult`, `EntryDeletionResult`, `TrashMutationResult`
- **Dependencies**: `internal/state`, `pkg/vault`, `time`

### `internal/app/password_service.go`
Password generation service using `crypto/rand`.

- **Key types**: `PasswordOptions` (Length, Uppercase, Lowercase, Digits, Space, UnderscoreDash, Symbols)
- **Key methods**: `GeneratePassword`
- **Dependencies**: `crypto/rand`, `math/big`

---

## internal/state

In-memory application state management.

### `internal/state/state.go`
Thread-safe vault state with `sync.RWMutex`. Manages open vaults and background save queue.

- **Key types**: `AppState`, `OpenVault` (Vault + Key), `SaveJob` (Vault + Key)
- **Key methods**: `NewAppState`, `InsertVault`, `RemoveVault`, `IsOpen`, `GetVault`, `WithOpenVault`, `WithOpenVaultSave`, `ScheduleSave`
- **Dependencies**: `sync`, `pkg/vault`

---

## internal/config

Application configuration persisted to `~/.config/passman/vaults.json`.

### `internal/config/config.go`
Read/write app config. Defines `VaultConfig`, `AppConfig`.

- **Key functions**: `LoadConfig`, `SaveConfig`, `AddVault`, `RemoveVault`, `UpdateVault`, `ConfigDir`, `ConfigPath`
- **Dependencies**: `encoding/json`, `os`, `path/filepath`

---

## internal/vimport

Import pipeline that converts Buttercup and KeePass data into PMV format.

### `internal/vimport/import.go`
Intermediate `ImportJSON` representation and mapping functions from Buttercup/KeePass to PMV.

- **Key types**: `ImportJSON`, `ImportGroup`, `ImportEntry`, `ImportTrash`, `ImportCustomField`
- **Key functions**: `FromButtercupVault`, `FromKeePassVault`, `BuildPayload`, `DeriveVaultName`, `DefaultVaultName`
- **Dependencies**: `pkg/buttercup`, `pkg/keepass`, `pkg/vault`, `path/filepath`, `time`

---

## cmd/passman-cli

CLI tool using Cobra for vault creation, import, export, and conversion.

### `cmd/passman-cli/main.go`
Command-line interface with subcommands: `create`, `import`, `export-buttercup`, `import-buttercup`, `import-keepass`, `convert`, `extract`. Password prompting via `golang.org/x/term`.

- **Key functions**: `createCmd`, `importCmd`, `exportButtercupCmd`, `importButtercupCmd`, `importKeePassCmd`, `convertCmd`, `extractCmd`, `createAndSaveVault`, `promptPassword`, `promptPasswordEnv`, `resolveConvertPassword`
- **Dependencies**: `cobra`, `golang.org/x/term`, `pkg/buttercup`, `pkg/keepass`, `pkg/vault`, `pkg/crypto`, `internal/vimport`

---

## main.go

Wails v3 application entry point. Initializes `AppState` with a background save worker goroutine, creates services (`VaultService`, `GroupService`, `EntryService`, `PasswordService`), registers them as Wails services, and creates the main window.

- **Dependencies**: `wailsapp/wails/v3/pkg/application`, `internal/app`, `internal/state`, `pkg/vault`

---

## frontend/ (Svelte 5 + Wails v3)

### `frontend/src/main.js`
Vite/Svelte entry point. Imports `App.svelte`, mounts to `#app`, imports `styles/app.scss`.

### `frontend/src/App.svelte`
Root component. Loads vault list on mount, initializes save event listener, renders `Vaults` and `AutoLock`. Displays load errors.

- **Dependencies**: `features/vault`, `components/AutoLock`, `components/dialog`, `bindings/.../vaultservice.js`

### `frontend/src/features/vault/store.js`
Core vault state store. Exports writable stores: `vaults`, `currentVault`, `vaultData`. Derived stores: `isUnlocked`, `groups`, `entries`. Functions: `createVault`, `openVault`, `registerAndOpenVault`, `closeVault`, `lockVault`, `lockVaultByPath`, `unlockVault`, `deleteVault`, `renameVault`, `reorderVaults`, `convertButtercupVault`, `convertKeepassVault`, `changeSecurityLevel`, `initSaveListener`, `updateVaultData`.

- **Dependencies**: `svelte/store`, `@wailsio/runtime`, `bindings/.../vaultservice.js`, `stores/toast.js`, `stores/selection.js`

### `frontend/src/features/vault/components/Vaults.svelte`
Top-level vault tab bar. Handles vault selection, unlock, create, import, open. Drag-to-reorder. Right-click context menu.

### `frontend/src/features/vault/components/Topbar.svelte`
Top bar with vault actions and theme toggle.

### `frontend/src/features/vault/components/VaultView.svelte`
Main 3-panel layout: groups sidebar, entry list, entry details/editor. Resizable panels.

### `frontend/src/features/vault/components/UnlockDialog.svelte`
Modal dialog for entering vault password.

### `frontend/src/features/vault/components/CreateVaultDialog.svelte`
Modal dialog for creating a new vault with security level selection.

### `frontend/src/features/vault/components/ImportDialog.svelte`
Modal dialog for importing Buttercup/KeePass vaults.

### `frontend/src/features/vault/components/OpenVaultMenu.svelte`
Menu for opening an existing vault file from disk.

### `frontend/src/features/vault/components/VaultSettingsDialog.svelte`
Modal dialog for renaming a vault and changing security level.

### `frontend/src/features/vault/components/VaultContextMenu.svelte`
Right-click menu for vault tabs. Actions: Settings, Remove.

### `frontend/src/features/vault/components/RemoveVaultDialog.svelte`
Confirmation modal for removing a vault from Passman.

### `frontend/src/features/vault/components/SecurityLevelSlider.svelte`
Slider component for selecting Argon2id security level.

### `frontend/src/features/entry/store.js`
Entry state and operations. Functions: `addEntry`, `updateEntry`, `deleteEntry`, `restoreEntry`, `deleteTrashEntry`, `moveEntryToGroup`, `moveEntryToVault`, `moveEntriesWithTagToGroup`, `moveEntriesInGroupToTag`, `copyEntryToGroup`, `copyEntryToVault`.

- **Dependencies**: `svelte/store`, `bindings/.../entryservice.js`, `features/vault`

### `frontend/src/features/entry/actions.js`
Entry action helpers, created per-vault via `createEntryActions`.

### `frontend/src/features/entry/components/EntryList.svelte`
Middle panel showing filtered entry list with search.

### `frontend/src/features/entry/components/EntryRow.svelte`
Single entry row in the entry list.

### `frontend/src/features/entry/components/EntryDetails.svelte`
Right panel showing entry details in view mode. Copy buttons for fields.

### `frontend/src/features/entry/components/EntryEditor.svelte`
Right panel for creating/editing entries. Fields: title, username, password (with Generate), custom fields, tags.

### `frontend/src/features/entry/components/EntryInput.svelte`
Reusable input component for entry fields.

### `frontend/src/features/entry/components/EntryContextMenu.svelte`
Right-click menu for entries: Copy Password, Move to, Copy to.

### `frontend/src/features/entry/components/MoveCopySubmenu.svelte`
Reusable submenu for move/copy operations across groups and vaults.

### `frontend/src/features/entry/components/TagManager.svelte`
Tag input and management component for entries.

### `frontend/src/features/group/store.js`
Group state and operations. Functions: `addGroup`, `addTag`, `deleteGroup`, `reorderGroups`, `mergeGroups`, `moveGroupToVault`, `copyGroupToVault`, `moveGroupToParent`.

- **Dependencies**: `svelte/store`, `bindings/.../groupservice.js`, `features/vault`

### `frontend/src/features/group/groupTree.js`
Tree builder for group hierarchy. Exports `buildTree`.

### `frontend/src/features/group/groupActions.js`
Group action helpers.

### `frontend/src/features/group/groupVaultMove.svelte.js`
Cross-vault group move/copy logic.

### `frontend/src/features/group/components/GroupList.svelte`
Sidebar showing groups and tags with tree structure. Drag-and-drop, context menu.

### `frontend/src/features/group/components/GroupTitle.svelte`
Group title display component.

### `frontend/src/features/group/components/AddGroupDialog.svelte`
Modal dialog for adding a group or tag.

### `frontend/src/features/group/components/DeleteGroupDialog.svelte`
Confirmation dialog for deleting a group.

### `frontend/src/features/group/components/GroupTagContextMenu.svelte`
Right-click menu for groups/tags: Move to group, Move to vault, Delete.

### `frontend/src/features/group/components/GroupVaultMoveDialog.svelte`
Dialog for moving/copying a group to another vault.

### `frontend/src/features/group/components/TagSidebar.svelte`
Sidebar showing tags for filtering.

### `frontend/src/features/group/components/TrashSidebar.svelte`
Sidebar showing trash groups and entries.

### `frontend/src/features/group/components/TrashRow.svelte`
Single trash entry row.

### `frontend/src/components/AutoLock.svelte`
Invisible component that auto-locks the vault after 5 minutes of inactivity.

- **Constants**: `LOCK_TIMEOUT_MS` = 300000

### `frontend/src/components/PasswordGenerator.svelte`
Password generator dialog with configurable charset and length.

### `frontend/src/components/ThemeToggle.svelte`
Theme toggle component (light/dark/auto).

### `frontend/src/components/Tree.svelte`
Reusable tree component for hierarchical lists.

### `frontend/src/components/TreeItem.svelte`
Single tree item with collapse/expand support.

### `frontend/src/components/TagContextMenu.svelte`
Right-click menu for tags.

### `frontend/src/components/dialog/Dialog.svelte`
Base dialog wrapper component.

### `frontend/src/components/dialog/Confirm.svelte`
Confirmation dialog component.

### `frontend/src/components/dialog/Toast.svelte`
Toast notification component.

### `frontend/src/components/dialog/DialogHeader.svelte`
Dialog header component.

### `frontend/src/components/dialog/DialogBody.svelte`
Dialog body component.

### `frontend/src/components/dialog/DialogFooter.svelte`
Dialog footer component.

### `frontend/src/components/dialog/DialogActions.svelte`
Dialog action buttons component.

### `frontend/src/components/form/Chip.svelte`
Chip component for tags and UI chips.

### `frontend/src/components/form/Input.svelte`
Reusable input component.

### `frontend/src/components/form/Label.svelte`
Reusable label component.

### `frontend/src/components/Tab/Tab.svelte`
Individual tab component.

### `frontend/src/components/Tab/TabHeader.svelte`
Tab header with close button and drag support.

### `frontend/src/components/Tab/Tabs.svelte`
Tab container component.

### `frontend/src/components/Tab/drag.js`
Tab drag-and-drop reorder utility.

### `frontend/src/components/Tab/tab-id.js`
Tab ID generation utility.

### `frontend/src/stores/selection.js`
Per-vault selection state. Creates isolated stores for each vault with `selectedGroup`, `selectedEntry`, `editingEntry`, `mode`, `trashMode`, `selectedTrashGroup`, `selectedTags`.

- **Key exports**: `createVaultSelection`, `deleteVaultStore`

### `frontend/src/stores/toast.js`
Toast notification store. Exports `showToast`.

### `frontend/src/stores/contextMenu.js`
Global context menu state store.

### `frontend/src/lib/columnResize.svelte.js`
Resizable panel utility using Svelte runes.

### `frontend/src/lib/createContextMenu.svelte.js`
Context menu creation utility.

### `frontend/src/lib/debounce.js`
Debounce utility function.

### `frontend/src/lib/menuPosition.js`
Menu positioning utility to keep menus within viewport.

### `frontend/src/lib/tags.js`
Tag utility functions.

### `frontend/src/lib/constants.js`
Shared constants (e.g., `SAVE_LISTENER_TIMEOUT_MS`).

### `frontend/src/lib/types.js`
Shared TypeScript-like type definitions for JS.

### `frontend/src/styles/`
SCSS design system files:

- `app.scss` — Entry point, imports all partials.
- `theme.scss` — Theme token definitions (light/dark).
- `_colors.scss` — Color variables.
- `_var.scss` — CSS custom properties.
- `_buttons.scss` — Button styles.
- `_menus.scss` — Menu and context menu styles.
- `_modal.scss` — Modal/dialog styles.
- `_base.scss` — Base reset and typography.
- `_mixins.scss` — SCSS mixins.

---

## Test Files

### `pkg/vault/vault_test.go`
Unit tests for vault create/open/save operations.

---

## Configuration Files

### `wails.json`
Wails v3 application configuration: name, output filename, frontend settings (dir, install/build/dev commands, dev server URL).

### `Taskfile.yml`
Task runner with commands: `dev` (wails3 dev), `build` (frontend + Go build), `build:frontend`, `build:app`, `generate` (wails3 generate bindings), `test` (go test ./...).

### `frontend/bindings/`
Auto-generated Wails bindings. Generated by `wails3 generate bindings`. Maps Go service methods to JavaScript functions importable from the frontend.
