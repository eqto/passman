# PMV File Format Specification

Version 1.0 — Passman Vault (PMV) is the encrypted, self-contained file format used by Passman to store password vaults. A file with the `.pmv` extension is a single vault containing a JSON header and an encrypted JSON payload.

## 1. Overview

- **Magic:** `PMV ` (4 bytes: ASCII `PMV` followed by a space character, `0x20`; hex `50 4D 56 20`)
- **Format version:** `1` (`uint16` little-endian, stored separately after the magic)
- **File extension:** `.pmv`
- **Encoding:** Binary wrapper around a JSON header and a binary encrypted payload.
- **Encryption model:** A user password is converted into a *vault key* via Argon2id. The vault key encrypts a random *data encryption key* (DEK). The DEK encrypts the actual vault payload.

This two-layer key design allows the password to be changed without re-encrypting the entire payload: only the DEK needs to be re-encrypted with the new vault key.

## 2. File Layout

A PMV file is structured as a sequence of six sections:

| Offset (bytes) | Size (bytes) | Field              | Type / Encoding                              |
|----------------|--------------|--------------------|----------------------------------------------|
| 0              | 4            | Magic              | ASCII `PMV` + space (`PMV `; hex `50 4D 56 20`) |
| 4              | 2            | Version            | `uint16` little-endian                       |
| 6              | 2            | Header length      | `uint16` little-endian                       |
| 8              | N            | Header JSON        | UTF-8 JSON, where `N` = Header length        |
| 8 + N          | 8            | Payload length     | `uint64` little-endian                       |
| 16 + N         | M            | Encrypted payload  | Binary, where `M` = Payload length           |

**Total file size:** `16 + N + M` bytes.

### Layout details

- **Magic:** Must exactly match the bytes `50 4D 56 20` (`PMV `). If the magic does not match, the file is not a valid PMV vault.
- **Version:** The file format version, stored as a 16-bit unsigned integer in little-endian byte order. For the PMV format this value is `1`. Implementations must reject files with an unsupported version before parsing the header.
- **Header length:** The length of the header JSON in bytes, stored as a 16-bit unsigned integer in little-endian byte order. This limits the header to 65,535 bytes, which is far larger than any realistic header.
- **Header JSON:** A UTF-8 JSON object described in [Section 3](#3-header-json-schema). It is not padded or null-terminated; its length is exactly the header length.
- **Payload length:** The length of the encrypted payload in bytes, stored as a 64-bit unsigned integer in little-endian byte order.
- **Encrypted payload:** The ciphertext produced by encrypting the UTF-8 JSON payload described in [Section 5](#5-payload-json-schema) with AES-256-GCM. The ciphertext includes the 16-byte AES-GCM authentication tag appended to the ciphertext.

## 3. Header JSON Schema

The header is a JSON object with the following fields:

```json
{
  "version": 1,
  "cipher": "AES-256-GCM",
  "kdf": "Argon2id",
  "kdf_params": {
    "salt": "base64-encoded-salt",
    "memory_kb": 65536,
    "iterations": 3,
    "parallelism": 4
  },
  "encrypted_dek": "base64-encoded-encrypted-dek",
  "dek_nonce": "base64-encoded-dek-nonce",
  "payload_nonce": "base64-encoded-payload-nonce",
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z"
}
```

### Field definitions

| Field             | Type     | Required | Description                                                  |
|-------------------|----------|----------|--------------------------------------------------------------|
| `version`         | `uint32` | Yes      | Format version. Must be `1` for the PMV format.                       |
| `cipher`          | `string` | Yes      | Symmetric cipher used for the DEK and payload. Value is `AES-256-GCM`. |
| `kdf`             | `string` | Yes      | Key derivation function. Value is `Argon2id`.               |
| `kdf_params`      | `object` | Yes      | Parameters used by the KDF. See [Section 3.1](#31-kdf_params). |
| `encrypted_dek`   | `string` | Yes      | Base64-encoded AES-256-GCM ciphertext of the 32-byte DEK.   |
| `dek_nonce`       | `string` | Yes      | Base64-encoded 12-byte nonce used to encrypt the DEK.         |
| `payload_nonce`   | `string` | Yes      | Base64-encoded 12-byte nonce used to encrypt the payload.     |
| `created_at`      | `string` | Yes      | ISO 8601 UTC timestamp of vault creation.                     |
| `updated_at`      | `string` | Yes      | ISO 8601 UTC timestamp of the last vault modification.        |

All binary data stored in the header (`salt`, `encrypted_dek`, `dek_nonce`, `payload_nonce`) is encoded using **standard Base64** (RFC 4648, with padding).

### 3.1 `kdf_params`

| Field          | Type     | Description                                                |
|----------------|----------|------------------------------------------------------------|
| `salt`         | `string` | Base64-encoded 16-byte salt for Argon2id.                |
| `memory_kb`    | `uint32` | Argon2id memory cost in KiB. Default is `65536`.           |
| `iterations`   | `uint32` | Argon2id time cost (number of iterations). Default is `3`. |
| `parallelism`  | `uint32` | Argon2id parallelism (number of lanes). Default is `4`.    |

## 4. Cryptography

### 4.1 Key derivation (Argon2id)

The user password is converted into a 32-byte vault key using **Argon2id**, version `0x13` (the latest version supported by the `argon2` crate).

Parameters used to derive the vault key are read from `header.kdf_params`:

- **Algorithm:** `Argon2id`
- **Version:** `0x13`
- **Memory cost:** `kdf_params.memory_kb` KiB
- **Time cost:** `kdf_params.iterations` iterations
- **Parallelism:** `kdf_params.parallelism` lanes
- **Output length:** 32 bytes (256 bits)
- **Salt:** `kdf_params.salt` decoded from Base64, exactly 16 bytes

The default parameters used when creating a new vault are:

```text
memory_kb   = 65536
iterations  = 3
parallelism = 4
salt        = 16 random bytes
```

Implementations must be prepared to read non-default values from the header and use them to derive the same vault key.

### 4.2 Encryption (AES-256-GCM)

PMV uses **AES-256-GCM** for both DEK encryption and payload encryption.

- **Key size:** 32 bytes (256 bits)
- **Nonce size:** 12 bytes (96 bits), randomly generated for every encryption operation
- **Tag size:** 16 bytes (128 bits), appended to the ciphertext by AES-GCM

Two separate encryption operations occur:

1. **DEK encryption:** A random 32-byte DEK is encrypted with the vault key. The `dek_nonce` and `encrypted_dek` are stored in the header.
2. **Payload encryption:** The UTF-8 JSON payload is encrypted with the DEK. The `payload_nonce` and the resulting ciphertext are stored after the header.

The AES-256-GCM implementation appends the 16-byte authentication tag to the ciphertext. The output bytes stored in the file are therefore the concatenation of `ciphertext` and `tag`. Decryptors must treat the final 16 bytes of the encrypted payload as the authentication tag.

### 4.3 Decryption flow

To read a PMV file:

1. Verify the magic is `PMV `.
2. Read the 2-byte file version and confirm it is `1`.
3. Read the 2-byte header length and parse the header JSON.
4. Confirm `header.version == 1`.
5. Decode `kdf_params.salt` and derive the vault key from the user password using Argon2id with the parameters in the header.
6. Decode `encrypted_dek` and `dek_nonce`, then decrypt the DEK using the vault key and AES-256-GCM.
7. Read the payload length and encrypted payload bytes.
8. Decode `payload_nonce`, then decrypt the payload using the DEK and AES-256-GCM.
9. Parse the decrypted payload as UTF-8 JSON.

### 4.4 Password changes

Because the payload is encrypted with the DEK, a password change can be performed without re-encrypting the entire payload:

1. Decrypt the DEK with the old password-derived vault key.
2. Derive a new vault key from the new password using a fresh salt and parameters.
3. Re-encrypt the DEK with the new vault key.
4. Update `header.kdf_params`, `header.encrypted_dek`, and `header.dek_nonce`.
5. Rewrite the file with the unchanged magic/version (`PMV ` + version `1`), the updated header, and the unchanged encrypted payload.

## 5. Payload JSON Schema

The decrypted payload is a JSON object representing the contents of the vault.

### 5.1 `VaultPayload`

```json
{
  "name": "My Vault",
  "uuid": "fb31b4a6-1e54-4460-ae03-5441a8083be5",
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z",
  "groups": [ ... ],
  "tags": [ ... ],
  "entries": [ ... ],
  "trash": { ... }
}
```

| Field        | Type     | Required | Description                                          |
|--------------|----------|----------|------------------------------------------------------|
| `name`       | `string` | Yes      | Human-readable vault name.                           |
| `uuid`       | `string` | No       | Optional stable vault UUID (e.g. from Buttercup).    |
| `created_at` | `string` | Yes      | ISO 8601 UTC timestamp of vault creation.            |
| `updated_at` | `string` | Yes      | ISO 8601 UTC timestamp of last content change.         |
| `groups`     | `array`  | Yes      | List of `VaultGroup` objects, in display order.      |
| `tags`       | `array`  | No       | List of free-form tag strings.                       |
| `entries`    | `array`  | Yes      | List of `VaultEntry` objects.                        |
| `trash`      | `object` | No       | `Trash` object holding deleted groups and entries.   |

### 5.3 `VaultGroup`

`groups` is a JSON array of `VaultGroup` objects. Each group has a unique identifier, a display name, and an optional parent identifier for nested groups:

```json
[
  { "id": "g1", "name": "Group A" },
  { "id": "g2", "name": "Group B", "parent_id": "g1" }
]
```

| Field        | Type     | Description                                           |
|--------------|----------|-------------------------------------------------------|
| `id`         | `string` | Unique group identifier.                              |
| `name`       | `string` | Human-readable group name.                          |
| `parent_id`  | `string` | Optional identifier of the parent group.              |

The order of the objects is preserved and used as the display order in the UI. Groups are displayed as a tree, with child groups nested under their parent.

### 5.4 `VaultEntry`

```json
{
  "id": "1",
  "title": "Example",
  "username": "user",
  "password": "pass",
  "url": "https://example.com",
  "notes": "",
  "tags": ["work"],
  "group_id": "g1",
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z",
  "deleted_at": null,
  "history": [
    { "property": "password", "value": "old-pass", "updated_at": "2026-06-25T02:25:00Z" }
  ]
}
```

| Field        | Type     | Required | Description                                           |
|--------------|----------|----------|-------------------------------------------------------|
| `id`         | `string` | Yes      | Unique entry identifier.                              |
| `title`      | `string` | Yes      | Entry title or display name.                          |
| `username`   | `string` | Yes      | Account username.                                     |
| `password`   | `string` | Yes      | Account password.                                     |
| `url`        | `string` | Yes      | Associated website or service URL.                    |
| `notes`      | `string` | Yes      | Free-form notes.                                      |
| `tags`       | `array`  | No       | Array of free-form tag strings.                       |
| `group_id`   | `string` | No       | Optional identifier of the group containing the entry.|
| `created_at` | `string` | Yes      | ISO 8601 UTC timestamp.                               |
| `updated_at` | `string` | Yes      | ISO 8601 UTC timestamp.                               |
| `deleted_at` | `string` | No       | ISO 8601 UTC timestamp when the entry was deleted.    |
| `history`    | `array`  | No       | Array of `HistoryItem` objects for previous values.   |

### 5.4.1 `HistoryItem`

Each `history` entry records a previous value of an entry property:

| Field        | Type     | Required | Description                                           |
|--------------|----------|----------|-------------------------------------------------------|
| `property`   | `string` | Yes      | Name of the property this history item belongs to.    |
| `value`      | `string` | Yes      | The previous value of the property.                   |
| `updated_at` | `string` | Yes      | ISO 8601 UTC timestamp when this value was replaced.  |

### 5.5 `Trash`

The `trash` object mirrors the main payload structure to support nested deleted groups and direct placement of deleted entries:

```json
{
  "groups": [
    { "id": "tg1", "name": "Deleted Group", "parent_id": "tg2" }
  ],
  "entries": [
    { "id": "te1", "title": "Old account", "group_id": "tg1", ... }
  ]
}
```

| Field     | Type    | Description                                                  |
|-----------|---------|--------------------------------------------------------------|
| `groups`  | `array` | List of deleted `VaultGroup` objects. Supports `parent_id`.  |
| `entries` | `array` | List of deleted `VaultEntry` objects.                        |

Deleted entries placed directly in the trash (not inside a deleted group) have `group_id` set to `null` or omitted. Entries inside a deleted group keep their `group_id` pointing to the deleted group in `trash.groups`.

## 6. Example

### 6.1 Hex dump of a minimal PMV file

The following example shows a freshly created empty vault named `Demo`. Values are illustrative; the encrypted sections vary with each creation due to random nonces and salt.

```text
00000000  50 4D 56 20  ┊ 01 00  ┊ 88 01  ┊ { "version": 1, ... }  ┊  ...
         └ magic ───┘  └ ver ┘  └ len  ┘  └ header JSON (N bytes) ┘
00000190  45 01 00 00 00 00 00 00  ┊  <encrypted payload bytes>  ...
         └ payload length (M bytes) ┘  └ ciphertext + 16-byte tag ┘
```

Where:

- `50 4D 56 20` = `PMV ` (space is `0x20`)
- `01 00` = file version `1` (2-byte `uint16` little-endian)
- `88 01` = header length `392` bytes (example only, based on real header size)
- `45 01 00 00 00 00 00 00` = payload length `325` bytes (example only)
- The encrypted payload is the AES-256-GCM ciphertext of the JSON payload, including the 16-byte tag.

### 6.2 Example header JSON

```json
{
  "version": 1,
  "cipher": "AES-256-GCM",
  "kdf": "Argon2id",
  "kdf_params": {
    "salt": "YWJjZGVmZ2hpamtsbW5vcA==",
    "memory_kb": 65536,
    "iterations": 3,
    "parallelism": 4
  },
  "encrypted_dek": "...",
  "dek_nonce": "...",
  "payload_nonce": "...",
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z"
}
```

### 6.3 Example payload JSON

```json
{
  "name": "Demo",
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z",
  "groups": [],
  "tags": [],
  "entries": [],
  "trash": {
    "groups": [],
    "entries": []
  }
}
```

## 7. Versioning

- The binary file version and header `version` remain `1`.
- Implementations must reject files whose binary file version is not `1` and whose `header.version` is not `1`.
- Future versions may introduce incompatible changes to the byte layout, header schema, or encryption scheme. A new version number will be used; the magic `PMV ` may remain the same if only the binary version changes, or a new magic may be introduced for incompatible layouts.

### Version locations in PMV

For PMV, the version number appears in two places:

1. The 2-byte binary file version after the magic.
2. The `version` field inside the JSON header.

The binary file version remains `1`. The header `version` remains `1`. Implementations should treat the binary file version as the authoritative sentinel for rejecting incompatible files, and treat the header version as a consistency check.

## 8. Reference Implementation

The canonical implementation is in `passman-core/src/vault.rs` and `passman-core/src/crypto.rs`. It provides the following relevant functions and constants:

| Symbol         | Location                          | Purpose                                      |
|----------------|-----------------------------------|----------------------------------------------|
| `MAGIC`        | `passman-core/src/vault.rs`       | The byte sequence `b"PMV "`.                 |
| `VERSION`      | `passman-core/src/vault.rs`       | The current format version `1`.             |
| `create_vault_file` | `passman-core/src/vault.rs`  | Creates a new PMV file.                      |
| `open_vault_file`   | `passman-core/src/vault.rs`  | Opens and decrypts an existing PMV file.     |
| `derive_key`   | `passman-core/src/crypto.rs`      | Derives a 32-byte key from a password.       |
| `encrypt`      | `passman-core/src/crypto.rs`      | AES-256-GCM encryption.                      |
| `decrypt`      | `passman-core/src/crypto.rs`      | AES-256-GCM decryption.                      |

## 9. Security Notes

- The Argon2id parameters in `kdf_params` are part of the file format and are stored unencrypted in the header. They are not secret; they only need to be authentic. Changing them after creation will prevent the original password from deriving the same vault key.
- The authentication tag in the AES-256-GCM ciphertext protects both the DEK and the payload from tampering. Implementations must verify this tag before using decrypted data.
- The DEK never changes unless the user explicitly rotates it. Password changes only re-encrypt the DEK.

## 10. Buttercup Import Compatibility

Passman can import Buttercup `.bcup` vaults (Format B). This section documents how Buttercup data is mapped to the PMV payload.

### 10.1 Buttercup Format B structure

A decrypted Buttercup Format B vault is a JSON object with the following relevant fields:

| Field | Description |
|-------|-------------|
| `id`  | Vault UUID. |
| `a`   | Vault attributes (arbitrary key/value map). |
| `g`   | Array of groups. Each group has `id`, `t` (title/name), `g` (parent group id, `"0"` for root), and `a` (attributes). |
| `e`   | Array of entries. Each entry has `id`, `g` (parent group id), `p` (properties such as title, username, password, each with optional `history` and `deleted`), and `a` (attributes). |
| `c`   | Vault creation timestamp. |

### 10.1.1 Field mapping

| Buttercup field | PMV field | Notes |
|-------------------|-----------|-------|
| `id` (vault)      | `uuid`    | Stored as the optional vault UUID. |
| `deleted` (entry) | `deleted_at` | Entry-level deletion timestamp. |
| `p[*].history`    | `history` | All per-property history items are collected into the entry-level `history` array. |

### 10.2 Trash group handling

Buttercup does not store trash in a separate top-level section. Instead, the trash group is a normal group whose attributes contain `bc_group_role = "trash"`. The official Buttercup importer (`buttercup/buttercup-importer`) locates trash using the `Group.Attribute.Role` constant, which maps to this attribute:

```javascript
const trashGroup = vaultFacade.groups.find(
    (group) =>
        group.attributes &&
        group.attributes[Group.Attribute.Role] === "trash"
);
```

In the raw Buttercup Format B JSON, `Group.Attribute.Role` is stored as `bc_group_role`. The official importer removes the trash group entirely when converting from one Buttercup vault to another. Passman, however, preserves the trash group's contents by placing them in the PMV `trash` object:

- The Buttercup trash root itself is **not** added as a `Group` in `trash.groups`; it is the container, not an item inside trash.
- Child groups of the trash group are recursively included in `trash.groups`, preserving their nested structure. Direct children of the trash root become root-level trash groups in PMV.
- Entries inside the trash subtree become `VaultEntry` items in `trash.entries`. Entries directly in the trash root have their `group_id` cleared; entries in child groups retain their `group_id` pointing to the child group.
- Deleted entries (entries with a `deleted` timestamp) are also moved to `trash.entries`; their `group_id` is cleared if they were in the root of the trash group.

Passman detects the trash group **only by the `bc_group_role` attribute** (`bc_group_role = "trash"`), matching the official Buttercup behaviour. A group named `"Trash"` without a `bc_group_role="trash"` attribute is treated as a normal group and placed in `groups`.
