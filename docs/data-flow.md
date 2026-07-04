# Passman Data Flow Guide

Textual diagrams showing how data moves through the Passman application.

---

## 1. Frontend State Management

```
┌─────────────────────────────────────────────────────────────┐
│                     Svelte Stores                           │
│                                                             │
│  stores/vaults.js                                           │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │ vaults      │  │ currentVault │  │ vaultData    │       │
│  │ (writable)  │  │ (writable)   │  │ (writable)   │       │
│  │ [VaultCfg]  │  │ VaultCfg|null│  │ {path: {     │       │
│  │             │  │              │  │   unlocked,  │       │
│  │             │  │              │  │   groups,    │       │
│  │             │  │              │  │   tags,      │       │
│  │             │  │              │  │   entries,   │       │
│  │             │  │              │  │   viewState  │       │
│  │             │  │              │  │ }}           │       │
│  └──────┬──────┘  └──────┬───────┘  └──────┬───────┘       │
│         │                │                 │                │
│         │     ┌──────────┴───────────┐     │                │
│         │     │ Derived Stores       │     │                │
│         │     │                      │     │                │
│         │     │ isUnlocked           │◄────┘                │
│         │     │ groups               │                      │
│         │     │ entries              │                      │
│         │     │ tags                 │                      │
│         │     └──────────────────────┘                      │
│         │                                                   │
│  ┌──────┴──────┐                                           │
│  │ saveStatus  │  ◄── updated via Tauri event listener     │
│  │ (writable)  │      "save-status" → "saving"/"saved"     │
│  └─────────────┘                                           │
│                                                            │
│  Derived stores also include `trash`.
│                                                             │
│  stores/entries.js          stores/groups.js                │
│  ┌────────────────┐        ┌──────────────────┐             │
│  │ addEntry       │        │ addGroup         │             │
│  │ updateEntry    │        │ addTag           │             │
│  │ deleteEntry    │        │ deleteGroup      │             │
│  │ moveEntryToGrp │        │ reorderGroups    │             │
│  │ moveEntryToVlt │        └──────────────────┘             │
│  │ copyEntryToGrp │                                         │
│  │ copyEntryToVlt │                                         │
│  │ generatePasswd │                                         │
│  └────────────────┘                                        │
│                                                             │
│  All functions: invoke() → Tauri backend → updateVaultData()│
└─────────────────────────────────────────────────────────────┘
```

**Key pattern**: Every store mutation function follows the same flow:
1. Read `currentVault` to get the vault path
2. Call `invoke("command_name", { path, ...params })`
3. On success, call `updateVaultData(path, { field: result })` to update the `vaultData` store
4. Svelte reactivity propagates changes to all derived stores and components

---

## 2. Tauri IPC Call Map

```
Frontend (Svelte)                          Backend (Rust)
───────────────                            ──────────────

stores/vaults.js:
  loadVaults()          ──invoke──►  list_vaults()
  createVault()         ──invoke──►  create_vault()
  openVault()           ──invoke──►  open_vault()
  registerAndOpenVault()──invoke──►  register_and_open_vault()
  closeVault()          ──invoke──►  close_vault()
  lockVaultByPath()     ──invoke──►  close_vault()
  unlockVault()         ──invoke──►  open_vault()
  deleteVault()         ──invoke──►  delete_vault()
  renameVault()         ──invoke──►  rename_vault()
  reorderVaults()       ──invoke──►  reorder_vaults()

stores/groups.js:
  addGroup()            ──invoke──►  add_group()
  addTag()              ──invoke──►  add_tag()
  deleteGroup()         ──invoke──►  delete_group()
  reorderGroups()       ──invoke──►  reorder_groups()

stores/entries.js:
  addEntry()            ──invoke──►  add_entry()
  updateEntry()         ──invoke──►  update_entry()
  deleteEntry()         ──invoke──►  delete_entry()
  moveEntryToGroup()    ──invoke──►  update_entry()        (modifies tags)
  moveEntryToVault()    ──invoke──►  delete_entry() + add_entry()  (cross-vault)
  copyEntryToGroup()    ──invoke──►  add_entry()           (new ID, modified tags)
  copyEntryToVault()    ──invoke──►  add_entry()           (cross-vault, new ID)
  generatePassword()    ──invoke──►  generate_password()

Backend events (Rust → Svelte):
  "save-status"         ──emit───►  saveStatus store
    payload: "saving" | "saved" | "error"
```

---

## 3. Rust Backend State

```
┌──────────────────────────────────────────────────────────────┐
│  AppState (managed by Tauri, cloned per command)             │
│                                                              │
│  ┌────────────────────────────────────────────────┐          │
│  │  inner: Arc<Mutex<AppStateInner>>              │          │
│  │  ┌──────────────────────────────────────────┐  │          │
│  │  │  open_vaults: HashMap<String, OpenVault> │  │          │
│  │  │                                          │  │          │
│  │  │  "path/to/vault.pmv" → OpenVault {       │  │          │
│  │  │    vault: VaultFile {                    │  │          │
│  │  │      path, header, payload, needs_save   │  │          │
│  │  │    }                                     │  │          │
│  │  │    key: Zeroizing<Vec<u8>>                │  │          │
│  │  │  }                                       │  │          │
│  │  └──────────────────────────────────────────┘  │          │
│  └────────────────────────────────────────────────┘          │
│                                                              │
│  ┌────────────────────────────────────────────────┐          │
│  │  save_tx: mpsc::Sender<SaveJob>                │          │
│  │  (sends vault + key to background save thread) │          │
│  └────────────────────────────────────────────────┘          │
│                                                              │
│  Background Save Worker Thread:                              │
│  ┌────────────────────────────────────────────────┐          │
│  │  loop {                                       │          │
│  │    job = save_rx.recv()                       │          │
│  │    emit("save-status", "saving")              │          │
│  │    vault::save_vault_file_with_key(           │          │
│  │      job.vault, job.key)                      │          │
│  │    emit("save-status", "saved"|"error")       │          │
│  │  }                                            │          │
│  └────────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────────┘

Command flow:
  1. Tauri command receives State<AppState>
  2. Lock inner mutex
  3. Get/modify OpenVault from open_vaults map
  4. Drop lock
  5. state.schedule_save(&path) → sends SaveJob on save_tx channel
  6. Worker thread picks it up, saves to disk, emits event
```

---

## 4. Data Model

```
AppConfig (config.json)
├── vaults: Vec<VaultConfig>
│   └── VaultConfig { id: String, name: String, path: String }

VaultFile (decrypted, in-memory)
├── path: String
├── header: VaultHeader
│   ├── magic: "PMV "
│   ├── version: 1
│   ├── kdf_params: KdfParamsJson
│   │   ├── algorithm: "argon2id"
│   │   ├── salt: base64
│   │   ├── iterations: 3
│   │   ├── memory_kib: 65536
│   │   └── parallelism: 4
│   ├── nonce: base64 (12 bytes)
│   └── tag: base64 (16 bytes)
├── payload: VaultPayload
│   ├── name: String
│   ├── created_at: DateTime
│   ├── updated_at: DateTime
│   ├── groups: Vec<Group>
│   │   └── Group { id: String, name: String, parent_id: Option<String> }
│   ├── tags: Vec<String>
│   ├── entries: Vec<VaultEntry>
│   │   └── VaultEntry {
│   │         id: String (UUID),
│   │         title: String,
│   │         username: String,
│   │         password: String,
│   │         url: String,
│   │         notes: String,
│   │         tags: Vec<String>,
│   │         group_id: Option<String>,
│   │         created_at: DateTime,
│   │         updated_at: DateTime
│   │       }
│   └── trash: Trash
│       ├── groups: Vec<Group>
│       └── entries: Vec<VaultEntry>
└── needs_save: bool

On-disk format (PMV file):
  [4 bytes: "PMV "]
  [2 bytes: version (uint16 LE)]
  [2 bytes: header length (uint16 LE)]
  [N bytes: header JSON (UTF-8)]
  [8 bytes: payload length (uint64 LE)]
  [M bytes: encrypted payload (ciphertext + 16-byte GCM tag)]

Groups vs Tags:
  - Groups are stored in payload.groups[]
  - Tags are stored in payload.tags[]
  - Entry.tags can contain both group names and tag names
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
     (add_entry) (update_  (delete_
                 entry)    entry)
          │        │        │
          ▼        ▼        ▼
     ┌─────────────────────────┐
     │ schedule_save(path)     │
     │ → save_tx channel       │
     │ → worker thread         │
     │ → save_vault_file_      │
     │   with_key()            │
     │ → emit "save-status"    │
     └─────────────────────────┘

Lock flow:
  User clicks lock / Ctrl+L / auto-lock timeout
    → close_vault(path) invoke
    → Rust: open_vaults.remove(path) (drops key from memory)
    → Frontend: vaultData[path].unlocked = false
    → Vault data cleared from derived stores

Unlock flow:
  User enters password
    → open_vault(path, password) invoke
    → Rust: open_vault_file() → derive_vault_key() → store OpenVault
    → Frontend: vaultData[path] = { unlocked, groups, tags, entries, trash }
    → Derived stores propagate to components

Cross-vault move:
  moveEntryToVault(entry, targetPath, targetGroup)
    → delete_entry from source vault (invoke)
    → add_entry to target vault (invoke, new tags)
    → vaultData updated for BOTH vaults

Cross-vault copy:
  copyEntryToVault(entry, targetPath, targetGroup)
    → add_entry to target vault (invoke, new UUID, new tags)
    → vaultData updated for target vault only
```

---

## 6. Component Hierarchy

```
App.svelte
├── VaultList.svelte (top bar)
│   ├── UnlockDialog.svelte (per-vault unlock prompt)
│   ├── CreateVaultDialog.svelte
│   ├── VaultSettingsDialog.svelte (rename)
│   ├── VaultContextMenu.svelte (right-click)
│   └── RemoveVaultDialog.svelte
├── VaultView.svelte (main content, when unlocked)
│   ├── GroupList.svelte (left panel)
│   │   ├── AddGroupDialog.svelte (also used for tags)
│   │   └── GroupTagContextMenu.svelte
│   ├── EntryList.svelte (middle panel)
│   │   └── EntryContextMenu.svelte
│   │       └── MoveCopySubmenu.svelte (×2: move + copy)
│   └── EntryDetails.svelte OR EntryEditor.svelte (right panel)
└── AutoLock.svelte (invisible, timer-based)
```

---

## 7. Save Flow Detail

```
User action (add/edit/delete entry, add/delete group, etc.)
  │
  ▼
Store function (entries.js / groups.js / vaults.js)
  │
  ├──► invoke("command", { path, ... })
  │    │
  │    ▼
  │    Rust command (entry_commands.rs / etc.)
  │    │
  │    ├── lock mutex
  │    ├── modify open_vaults[path].vault.payload
  │    ├── drop lock
  │    ├── state.schedule_save(&path)
  │    │   └── save_tx.send(SaveJob { vault, key })  (non-blocking)
  │    └── return updated data
  │
  ├──► updateVaultData(path, { entries/groups/tags })
  │    └── vaultData store updated → Svelte reactivity
  │
  ▼
Background save worker thread
  │
  ├── save_rx.recv() → SaveJob { vault, key }
  ├── emit("save-status", "saving")
  ├── vault::save_vault_file_with_key(&job.vault, &job.key)
  │   └── encrypt payload with AES-256-GCM
  │       └── write to .pmv file
  └── emit("save-status", "saved" | "error")
      └── Frontend: saveStatus store → UI indicator
          └── auto-reset to "idle" after 2 seconds
```
