# Passman Architecture Overview

Passman is an offline-first password manager. It is built with Go (Wails v3) for the backend and Svelte 5 for the frontend, with a command-line interface for power users and migration.

## Project Layout

```text
Passman/
├── main.go                    # Wails v3 application entry point
├── go.mod                     # Go module definition
├── wails.json                 # Wails application configuration
├── Taskfile.yml               # Task runner for dev/build/test
├── docs/
│   ├── format.md              # PMV file format specification
│   └── architecture.md        # This document
├── pkg/                       # Core Go packages (reusable, no app logic)
│   ├── crypto/                # Argon2id, AES-256-GCM, security levels
│   ├── vault/                 # PMV format: types, file I/O, operations
│   ├── buttercup/             # Buttercup .bcup import
│   └── keepass/               # KeePass .kdbx import
├── internal/                  # Application services (not importable)
│   ├── app/                   # Wails services: vault, group, entry, password
│   ├── state/                 # In-memory vault state and save queue
│   ├── config/                # App config (~/.config/passman/vaults.json)
│   └── vimport/               # Import pipeline (Buttercup/KeePass → PMV)
├── cmd/passman-cli/           # CLI tool: create, import, export, convert, extract
└── frontend/                  # Svelte 5 frontend (Wails v3 webview)
    ├── src/                   # Svelte components, features, stores, styles
    ├── bindings/              # Auto-generated Wails Go→JS bindings
    └── package.json           # Frontend dependencies (Bun)
```

## Components

### `pkg/crypto`

Cryptographic primitives: Argon2id key derivation, AES-256-GCM encryption/decryption, random byte generation, and security level presets.

- `crypto.go` — `DeriveKey`, `Encrypt`, `Decrypt`, `RandomBytes`, `SecurityLevel` presets (Low / Medium / Secure / Best).
- Constants: `KeySize` = 32, `NonceSize` = 12, `TagSize` = 16, `SaltSize` = 16.
- Dependencies: `golang.org/x/crypto/argon2`, `crypto/aes`, `crypto/cipher`, `crypto/rand`.

### `pkg/vault`

The PMV (Passman Vault) file format: create, open, save, data models, and group/entry operations.

- `types.go` — `VaultHeader`, `VaultPayload`, `VaultEntry`, `Group`, `Trash`, `CustomField`, `HistoryItem`, `VaultFileDTO`.
- `format.go` — Binary file I/O: `ReadVaultFile`, `WriteVaultFile` (atomic via temp + rename).
- `vault.go` — High-level operations: `CreateVaultFile`, `OpenVaultFile`, `SaveVaultFile`, `ChangeKdfParams`.
- `operations.go` — Group/entry operations: `DeleteGroupWithChildren`, `MergeGroupsInVault`, `MoveGroupToParent`, cross-vault move/copy, `MoveEntriesToTrash`.
- `vault_test.go` — Unit tests for vault create/open/save.
- Constants: `Magic` = `"PMV "`, `Version` = 1.

### `pkg/buttercup`

Buttercup vault format parser for import. Supports Format A and Format B, CBC and GCM decryption.

- `buttercup.go` — `DecryptButtercupFile` entry point.
- `decrypt.go` — CBC+HMAC and GCM+AAD decryption.
- `format_a.go` — Format A parsing (legacy `$`-delimited format).
- `parse.go` — Format B JSON parsing and group/entry extraction.
- `types.go` — `ButtercupVault`, `ButtercupGroup`, `ButtercupEntry`, `HistoryItem`.

### `pkg/keepass`

KeePass `.kdbx` database parser for import.

- `keepass.go` — `DecryptKeePassFile`, group/entry extraction, `KeePassVault` types.
- Dependencies: `github.com/tobischo/gokeepasslib/v3`.

### `internal/app`

Wails v3 services exposed to the frontend. Each service is registered as a Wails service in `main.go`.

- `vault_service.go` — `VaultService`: `ListVaults`, `CreateVault`, `OpenVault`, `RegisterAndOpenVault`, `CloseVault`, `DeleteVault`, `RenameVault`, `ReorderVaults`, `ConvertButtercupVault`, `ConvertKeepassVault`, `ChangeSecurityLevel`.
- `group_service.go` — `GroupService`: `ListGroups`, `AddGroup`, `DeleteGroup`, `ReorderGroups`, `MergeGroups`, `MoveGroupToVault`, `CopyGroupToVault`, `MoveGroupToParent`, `AddTag`.
- `entry_service.go` — `EntryService`: `ListEntries`, `AddEntry`, `UpdateEntry`, `DeleteEntry`, `RestoreTrashEntry`, `DeleteTrashEntry`, `ListTrash`.
- `password_service.go` — `PasswordService`: `GeneratePassword` with configurable charset and length.

### `internal/state`

In-memory application state. Manages open vaults and background saves.

- `state.go` — `AppState` with `sync.RWMutex`, `openVaults` map, `saveCh` channel. Methods: `InsertVault`, `RemoveVault`, `IsOpen`, `GetVault`, `WithOpenVault`, `WithOpenVaultSave`, `ScheduleSave`.
- `OpenVault` holds `*vault.VaultFile` and `[]byte` key.
- `SaveJob` sent to background save goroutine in `main.go`.

### `internal/config`

Application config file (`~/.config/passman/vaults.json`) read/write.

- `config.go` — `LoadConfig`, `SaveConfig`, `AddVault`, `RemoveVault`, `UpdateVault`, `ConfigDir`, `ConfigPath`.

### `internal/vimport`

Import pipeline that converts Buttercup and KeePass data into PMV format.

- `import.go` — `ImportJSON` intermediate representation, `FromButtercupVault`, `FromKeePassVault`, `BuildPayload`, `DeriveVaultName`.

### `cmd/passman-cli`

CLI tool using Cobra. Subcommands: `create`, `import`, `export-buttercup`, `import-buttercup`, `import-keepass`, `convert`, `extract`.

- `main.go` — Command definitions, password prompting via `golang.org/x/term`.

### `frontend/` (Svelte 5 + Wails v3)

The desktop UI is built with Svelte 5 (runes mode) and communicates with the Go backend via auto-generated Wails bindings.

- `src/App.svelte` — Root component: loads vaults, initializes save listener.
- `src/features/vault/` — Vault feature: stores, components (`Vaults`, `Topbar`, `VaultView`, `UnlockDialog`, `CreateVaultDialog`, `ImportDialog`, `OpenVaultMenu`, `VaultSettingsDialog`, `VaultContextMenu`, `RemoveVaultDialog`, `SecurityLevelSlider`).
- `src/features/entry/` — Entry feature: stores, actions, components (`EntryList`, `EntryRow`, `EntryDetails`, `EntryEditor`, `EntryContextMenu`, `MoveCopySubmenu`, `TagManager`, `EntryInput`).
- `src/features/group/` — Group feature: stores, components (`GroupList`, `GroupTitle`, `AddGroupDialog`, `DeleteGroupDialog`, `GroupTagContextMenu`, `GroupVaultMoveDialog`, `TagSidebar`, `TrashSidebar`, `TrashRow`).
- `src/components/` — Shared components: `AutoLock`, `PasswordGenerator`, `ThemeToggle`, `Tree`, `TreeItem`, `TagContextMenu`, dialog system (`Dialog`, `Confirm`, `Toast`), form components (`Chip`, `Input`, `Label`), Tab system (`Tab`, `TabHeader`, `Tabs`, `drag.js`).
- `src/stores/` — Global stores: `selection.js` (per-vault view state), `toast.js`, `contextMenu.js`.
- `src/lib/` — Utilities: `columnResize`, `createContextMenu`, `debounce`, `menuPosition`, `tags`, `constants`, `types`.
- `src/styles/` — SCSS design system: theme tokens, colors, buttons, menus, modal, mixins.
- `bindings/` — Auto-generated Wails bindings (Go services → JS functions).

## Data Flows

### PMV Vault Lifecycle

1. **Create** — `pkg/vault` generates a random salt, derives a vault key with Argon2id, generates a random data encryption key (DEK), encrypts the DEK with the vault key, and writes the encrypted payload.
2. **Open** — The file header is read, the vault key is derived from the password, the DEK is decrypted, and the JSON payload is decrypted.
3. **Save** — The payload is re-encrypted with the DEK and a fresh nonce, then written atomically via a temp file + rename.

### Desktop Application Flow

1. The Svelte frontend calls `vaultService.ListVaults()` (auto-generated Wails binding) to populate the vault list.
2. Unlocking calls `vaultService.OpenVault()` or `vaultService.RegisterAndOpenVault()`.
3. The Go backend stores the decrypted vault and derived key in `AppState.openVaults`. The user password is not retained.
4. The frontend uses Svelte stores to display groups and tags. Groups are single-select; tags are multi-select and filter the entry list.
5. Mutations (add/update/delete entry or group) update the in-memory vault and trigger a background save via `ScheduleSave`.
6. `AutoLock.svelte` locks the vault after 5 minutes of inactivity.

### Import Flow

1. **Buttercup** — The CLI or desktop app reads a `.bcup` file, decrypts it (CBC+HMAC or GCM+AAD), parses the JSON, and maps groups/entries to the `ImportJSON` intermediate format. A new PMV file is created with the mapped data.
2. **KeePass** — The CLI or desktop app reads a `.kdbx` file using `gokeepasslib`, extracts groups/entries, and maps them to `ImportJSON`. A new PMV file is created with the mapped data.

## Security Notes

- User password → Argon2id (64 MiB, 3 iterations, 4 lanes by default) → vault key → DEK → payload.
- AES-256-GCM with a random 12-byte nonce and 16-byte authentication tag.
- The derived vault key is held in memory as `[]byte` in `AppState.openVaults` and cleared on lock. The user password is not stored after key derivation.
- IPC between the Svelte frontend and Go backend is via Wails v3 bindings (local, no network exposure).

## Key Files

- `go.mod` — Go module and dependencies.
- `main.go` — Wails v3 app entry point, service registration, background save worker.
- `wails.json` — Wails application configuration.
- `pkg/vault/vault.go` — PMV format high-level operations.
- `pkg/vault/format.go` — PMV binary file I/O.
- `pkg/vault/types.go` — PMV data structures.
- `pkg/vault/operations.go` — Group/entry operations (delete, merge, move, copy).
- `pkg/crypto/crypto.go` — Cryptography primitives and security levels.
- `internal/config/config.go` — App configuration.
- `internal/app/vault_service.go` — Vault Wails service.
- `internal/app/group_service.go` — Group Wails service.
- `internal/app/entry_service.go` — Entry Wails service.
- `internal/state/state.go` — In-memory state management.
- `internal/vimport/import.go` — Import pipeline.
- `cmd/passman-cli/main.go` — CLI entry point.
- `frontend/src/features/vault/store.js` — Frontend vault state store.
- `frontend/src/stores/selection.js` — Per-vault selection state.
- `docs/format.md` — Detailed PMV format spec.
