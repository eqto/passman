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
  "groups": [ ... ],
  "vault_metadata": { ... },
  "entries": [ ... ]
}
```

| Field            | Type     | Description                                          |
|------------------|----------|------------------------------------------------------|
| `groups`         | `array`  | List of group name strings, in display order.        |
| `vault_metadata` | `object` | Metadata about the vault.                            |
| `entries`        | `array`  | List of `VaultEntry` objects.                        |

### 5.2 `VaultMetadata`

```json
{
  "name": "My Vault",
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z",
  "format_version": 2
}
```

| Field            | Type     | Description                                      |
|------------------|----------|--------------------------------------------------|
| `name`           | `string` | Human-readable vault name.                       |
| `created_at`     | `string` | ISO 8601 UTC timestamp of vault creation.        |
| `updated_at`     | `string` | ISO 8601 UTC timestamp of last content change.   |
| `format_version` | `uint32` | Payload format version. Currently `2`.           |

### 5.3 `VaultGroup`

`groups` is a JSON array of unique, non-empty group name strings:

```json
["Group A", "Group B"]
```

The order of the strings is preserved and used as the display order in the UI.

### 5.4 `VaultEntry`

```json
{
  "id": "1",
  "title": "Example",
  "username": "user",
  "password": "pass",
  "url": "https://example.com",
  "notes": "",
  "tags": ["Group A", "work"],
  "created_at": "2026-06-26T02:25:00Z",
  "updated_at": "2026-06-26T02:25:00Z"
}
```

| Field        | Type     | Description                                           |
|--------------|----------|-------------------------------------------------------|
| `id`         | `string` | Unique entry identifier.                              |
| `title`      | `string` | Entry title or display name.                          |
| `username`   | `string` | Account username.                                     |
| `password`   | `string` | Account password.                                     |
| `url`        | `string` | Associated website or service URL.                    |
| `notes`      | `string` | Free-form notes.                                      |
| `tags`       | `array`  | Array of tag strings associated with the entry.       |
| `created_at` | `string` | ISO 8601 UTC timestamp.                               |
| `updated_at` | `string`  | ISO 8601 UTC timestamp.                               |

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
  "groups": [],
  "vault_metadata": {
    "name": "Demo",
    "created_at": "2026-06-26T02:25:00Z",
    "updated_at": "2026-06-26T02:25:00Z",
    "format_version": 2
  },
  "entries": []
}
```

## 7. Versioning

- The binary file version and header `version` remain `1`.
- The payload `format_version` is `2` for new vaults. Payload `format_version` `1` files are automatically migrated on open.
- Implementations must reject files whose binary file version is not `1` and whose `header.version` is not `1`.
- Future versions may introduce incompatible changes to the byte layout, header schema, or encryption scheme. A new version number will be used; the magic `PMV ` may remain the same if only the binary version changes, or a new magic may be introduced for incompatible layouts.

### Version redundancy in PMV

For PMV, the version number appears in three places:

1. The 2-byte binary file version after the magic.
2. The `version` field inside the JSON header.
3. The `format_version` field inside `vault_metadata` in the JSON payload.

The binary file version remains `1`. The header `version` remains `1`. The payload `format_version` is `2` for new vaults. Implementations should treat the binary file version as the authoritative sentinel for rejecting incompatible files, and treat the header and payload versions as a consistency check. Old payload format version `1` files are automatically migrated on open.

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
