# Passman Data Flow Guide

Textual diagrams showing how data moves through the Passman application.

---

## 1. Frontend State Management

```
┌─────────────────────────────────────────────────────────────┐
│                     Svelte Stores                           │
│                                                             │
│  features/vault/store.js                                    │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │ vaults      │  │ currentVault │  │ vaultData    │       │
│  │ (writable)  │  │ (writable)   │  │ (writable)   │       │
│  │ [VaultCfg]  │  │ VaultCfg|null│  │ {path: {     │       │
│  │             │  │              │  │   unlocked,  │       │
│  │             │  │              │  │   groups,    │       │
│  │             │  │              │  │   tags,      │       │
│  │             │  │              │  │   entries,   │       │
│  │             │  │              │  │   trash      │       │
│  │             │  │              │  │ }}           │       │
│  └──────┬──────┘  └──────┬───────┘  └──────┬───────┘       │
│         │                │                 │                │
│         │     ┌──────────┴───────────┐     │                │
│         │     │ Derived Stores       │     │                │
│         │     │                      │     │                │
│         │     │ isUnlocked           │◄────┘                │
│         │     │ groups               │                      │
│         │     │ entries              │                      │
│         │     └──────────────────────┘                      │
│                                                             │
│  stores/selection.js                                        │
│  ┌────────────────────────────────────────────────┐         │
│  │ Per-vault writable stores (Map<path, store>)   │         │
│  │ { selectedGroup, selectedEntry, editingEntry,  │         │
│  │   mode, trashMode, selectedTrashGroup,         │         │
│  │   selectedTags }                               │         │
│  └────────────────────────────────────────────────┘         │
│                                                             │
│  features/entry/store.js    features/group/store.js         │
│  ┌────────────────┐        ┌──────────────────┐             │
│  │ addEntry       │        │ addGroup         │             │
│  │ updateEntry    │        │ addTag           │             │
│  │ deleteEntry    │        │ deleteGroup      │             │
│  │ restoreEntry   │        │ reorderGroups    │             │
│  │ moveEntryToGrp │        │ mergeGroups      │             │
│  │ moveEntryToVlt │        │ moveGroupToVault │             │
│  │ copyEntryToGrp │        │ copyGroupToVault │             │
│  │ copyEntryToVlt │        │ moveGroupToParent│             │
│  └────────────────┘        └──────────────────┘             │
│                                                             │
│  All functions: Wails binding → Go backend → updateVaultData│
│  Save status: Events.On("save-status") → showToast()       │
└─────────────────────────────────────────────────────────────┘
```

**Key pattern**: Every store mutation function follows the same flow:
1. Read `currentVault` to get the vault path
2. Call the auto-generated Wails binding (e.g., `entryService.AddEntry(path, entry)`)
3. On success, call `updateVaultData(path, { field: result })` to update the `vaultData` store
4. Svelte reactivity propagates changes to all derived stores and components

---

## 2. Wails IPC Call Map

```
Frontend (Svelte)                          Backend (Go)
───────────────                            ──────────────

features/vault/store.js:
  vaultService.ListVaults()           ──binding──►  VaultService.ListVaults()
  vaultService.CreateVault()          ──binding──►  VaultService.CreateVault()
  vaultService.OpenVault()            ──binding──►  VaultService.OpenVault()
  vaultService.RegisterAndOpenVault() ──binding──►  VaultService.RegisterAndOpenVault()
  vaultService.CloseVault()           ──binding──►  VaultService.CloseVault()
  vaultService.DeleteVault()          ──binding──►  VaultService.DeleteVault()
  vaultService.RenameVault()          ──binding──►  VaultService.RenameVault()
  vaultService.ReorderVaults()        ──binding──►  VaultService.ReorderVaults()
  vaultService.ConvertButtercupVault()──binding──►  VaultService.ConvertButtercupVault()
  vaultService.ConvertKeepassVault()  ──binding──►  VaultService.ConvertKeepassVault()
  vaultService.ChangeSecurityLevel()  ──binding──►  VaultService.ChangeSecurityLevel()

features/group/store.js:
  groupService.AddGroup()             ──binding──►  GroupService.AddGroup()
  groupService.AddTag()               ──binding──►  GroupService.AddTag()
  groupService.DeleteGroup()          ──binding──►  GroupService.DeleteGroup()
  groupService.ReorderGroups()        ──binding──►  GroupService.ReorderGroups()
  groupService.MergeGroups()          ──binding──►  GroupService.MergeGroups()
  groupService.MoveGroupToVault()     ──binding──►  GroupService.MoveGroupToVault()
  groupService.CopyGroupToVault()     ──binding──►  GroupService.CopyGroupToVault()
  groupService.MoveGroupToParent()    ──binding──►  GroupService.MoveGroupToParent()

features/entry/store.js:
  entryService.AddEntry()             ──binding──►  EntryService.AddEntry()
  entryService.UpdateEntry()          ──binding──►  EntryService.UpdateEntry()
  entryService.DeleteEntry()          ──binding──►  EntryService.DeleteEntry()
  entryService.RestoreTrashEntry()    ──binding──►  EntryService.RestoreTrashEntry()
  entryService.DeleteTrashEntry()     ──binding──►  EntryService.DeleteTrashEntry()
  moveEntryToGroup()                  ──binding──►  EntryService.UpdateEntry()  (modifies group_id)
  moveEntryToVault()                  ──binding──►  DeleteEntry() + AddEntry()  (cross-vault)
  copyEntryToGroup()                  ──binding──►  EntryService.AddEntry()     (new ID)
  copyEntryToVault()                  ──binding──►  EntryService.AddEntry()     (cross-vault, new ID)

  passwordService.GeneratePassword()  ──binding──►  PasswordService.GeneratePassword()

Backend events (Go → Svelte):
  "save-status"         ──emit───►  Events.On listener → showToast()
    payload: "saved" | "error"
```

---

## 3. Go Backend State

```
┌──────────────────────────────────────────────────────────────┐
│  AppState (internal/state/state.go)                          │
│                                                              │
│  ┌────────────────────────────────────────────────┐          │
│  │  mu: sync.RWMutex                             │          │
│  │  ┌──────────────────────────────────────────┐  │          │
│  │  │  openVaults: map[string]*OpenVault       │  │          │
│  │  │                                          │  │          │
│  │  │  "path/to/vault.pmv" → OpenVault {      │  │          │
│  │  │    Vault: *vault.VaultFile {             │  │          │
│  │  │      Path, Header, Payload, NeedsSave    │  │          │
│  │  │    }                                     │  │          │
│  │  │    Key: []byte                           │  │          │
│  │  │  }                                       │  │          │
│  │  └──────────────────────────────────────────┘  │          │
│  └────────────────────────────────────────────────┘          │
│                                                              │
│  ┌────────────────────────────────────────────────┐          │
│  │  saveCh: chan state.SaveJob                   │          │
│  │  (sends vault + key to background save goroutine)│       │
│  └────────────────────────────────────────────────┘          │
│                                                              │
│  Background Save Worker Goroutine (started in main.go):      │
│  ┌────────────────────────────────────────────────┐          │
│  │  for {                                       │          │
│  │    job := <-saveCh                           │          │
│  │    application.EmitEvent("save-status", ...) │          │
│  │    vault.SaveVaultFileWithKey(               │          │
│  │      job.Vault, job.Key)                     │          │
│  │    application.EmitEvent("save-status", ...)│          │
│  │  }                                           │          │
│  └────────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────────┘

Service method flow:
  1. Service method receives *state.AppState (via struct reference)
  2. RLock or Lock mutex
  3. Get/modify OpenVault from openVaults map
  4. RUnlock or Unlock
  5. state.ScheduleSave(path) → sends SaveJob on saveCh channel
  6. Worker goroutine picks it up, saves to disk, emits event
```

---

## 4. Data Model

```
AppConfig (internal/config/config.go)
├── Vaults: []VaultConfig
│   └── VaultConfig { ID: string, Name: string, Path: string }

VaultFile (decrypted, in-memory, pkg/vault/types.go)
├── Path: string
├── Header: VaultHeader
│   ├── Magic: "PMV "
│   ├── Version: 1
│   ├── KdfParams: KdfParamsJSON
│   │   ├── Algorithm: "argon2id"
│   │   ├── Salt: base64
│   │   ├── Iterations: 3
│   │   ├── MemoryKIB: 65536
│   │   └── Parallelism: 4
│   ├── Nonce: base64 (12 bytes)
│   └── Tag: base64 (16 bytes)
├── Payload: VaultPayload
│   ├── Name: string
│   ├── CreatedAt: time.Time
│   ├── UpdatedAt: time.Time
│   ├── Groups: []Group
│   │   └── Group { ID: string, Name: string, ParentID: string }
│   ├── Tags: []string
│   ├── Entries: []VaultEntry
│   │   └── VaultEntry {
│   │         ID: string (UUID),
│   │         Title: string,
│   │         Username: string,
│   │         Password: string,
│   │         URL: string,
│   │         Notes: string,
│   │         Tags: []string,
│   │         GroupID: string,
│   │         CustomFields: []CustomField,
│   │         History: []HistoryItem,
│   │         CreatedAt: time.Time,
│   │         UpdatedAt: time.Time
│   │       }
│   └── Trash: Trash
│       ├── Groups: []Group
│       └── Entries: []VaultEntry
└── NeedsSave: bool

On-disk format (PMV file):
  [4 bytes: "PMV "]
  [2 bytes: version (uint16 LE)]
  [2 bytes: header length (uint16 LE)]
  [N bytes: header JSON (UTF-8)]
  [8 bytes: payload length (uint64 LE)]
  [M bytes: encrypted payload (ciphertext + 16-byte GCM tag)]

Groups vs Tags:
  - Groups are stored in payload.Groups[]
  - Tags are stored in payload.Tags[]
  - Entry.Tags can contain both group names and tag names
  - Frontend filters: tags = all entry tags NOT in groups
  - Selecting a group filters entries by tag membership
```

---

## 5. Vault Lifecycle

```
┌─────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Create │───►│  Open    │───►│  Locked  │───►│  Closed  │
│  Vault  │    │  Vault   │    │  (in     │    │  (data   │
│         │    │          │    │  memory) │    │  cleared)│
└─────────┘    └──────────┘    └──────────┘    └──────────┘
                    │               │
                    │               │
                    ▼               │
              ┌──────────┐          │
              │ Unlocked │◄─────────┘
              │ (key in  │   unlock (re-enter password)
              │ memory)  │
              └────┬─────┘
                   │
          ┌────────┼────────┐
          │        │        │
          ▼        ▼        ▼
     Add Entry  Edit     Delete Entry
     (AddEntry) (Update-  (Delete-
                 Entry)   Entry)
          │        │        │
          ▼        ▼        ▼
     ┌─────────────────────────┐
     │ ScheduleSave(path)      │
     │ → saveCh channel        │
     │ → worker goroutine      │
     │ → SaveVaultFileWithKey()│
     │ → EmitEvent "save-status"│
     └─────────────────────────┘

Lock flow:
  User clicks lock / auto-lock timeout
    → vaultService.CloseVault(path) binding call
    → Go: openVaults[path] removed (key dropped from memory)
    → Frontend: vaultData[path] cleared, deleteVaultStore(path)
    → Vault data cleared from derived stores

Unlock flow:
  User enters password
    → vaultService.OpenVault(path, password) binding call
    → Go: OpenVaultFile() → DeriveKey() → store OpenVault in AppState
    → Frontend: vaultData[path] = { unlocked, groups, tags, entries, trash }
    → Derived stores propagate to components

Cross-vault move:
  moveEntryToVault(entry, targetPath, targetGroup)
    → entryService.DeleteEntry from source vault (binding)
    → entryService.AddEntry to target vault (binding, new group_id)
    → vaultData updated for BOTH vaults

Cross-vault copy:
  copyEntryToVault(entry, targetPath, targetGroup)
    → entryService.AddEntry to target vault (binding, new UUID, new group_id)
    → vaultData updated for target vault only
```

---

## 6. Component Hierarchy

```
App.svelte
├── Vaults.svelte (top tab bar)
│   ├── UnlockDialog.svelte (per-vault unlock prompt)
│   ├── CreateVaultDialog.svelte (with SecurityLevelSlider)
│   ├── ImportDialog.svelte (Buttercup/KeePass import)
│   ├── OpenVaultMenu.svelte (open vault from disk)
│   ├── VaultSettingsDialog.svelte (rename, change security level)
│   ├── VaultContextMenu.svelte (right-click)
│   └── RemoveVaultDialog.svelte
├── VaultView.svelte (main content, when unlocked)
│   ├── GroupList.svelte (left panel, tree structure)
│   │   ├── AddGroupDialog.svelte (also used for tags)
│   │   ├── DeleteGroupDialog.svelte
│   │   └── GroupTagContextMenu.svelte
│   ├── EntryList.svelte (middle panel)
│   │   └── EntryContextMenu.svelte
│   │       └── MoveCopySubmenu.svelte (×2: move + copy)
│   └── EntryDetails.svelte OR EntryEditor.svelte (right panel)
│       └── TagManager.svelte (within editor)
├── AutoLock.svelte (invisible, timer-based)
└── Toast.svelte (notification display)
```

---

## 7. Save Flow Detail

```
User action (add/edit/delete entry, add/delete group, etc.)
  │
  ▼
Store function (features/entry/store.js / features/group/store.js / features/vault/store.js)
  │
  ├──► Wails binding call (e.g., entryService.AddEntry(path, entry))
  │    │
  │    ▼
  │    Go service method (entry_service.go / etc.)
  │    │
  │    ├── RLock mutex
  │    ├── modify openVaults[path].Vault.Payload
  │    ├── RUnlock mutex
  │    ├── state.ScheduleSave(path)
  │    │   └── saveCh <- SaveJob{Vault, Key}  (non-blocking)
  │    └── return updated data
  │
  ├──► updateVaultData(path, { entries/groups/tags })
  │    └── vaultData store updated → Svelte reactivity
  │
  ▼
Background save worker goroutine (started in main.go)
  │
  ├── job := <-saveCh  →  SaveJob{Vault, Key}
  ├── EmitEvent("save-status", "saving")
  ├── vault.SaveVaultFileWithKey(job.Vault, job.Key)
  │   └── encrypt payload with AES-256-GCM
  │       └── write to .pmv file (atomic: temp + rename)
  └── EmitEvent("save-status", "saved" | "error")
      └── Frontend: Events.On("save-status") → showToast()
```
