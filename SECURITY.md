# Security Policy

## Reporting a Vulnerability

Passman is a password manager — security is a top priority.

If you discover a security vulnerability, **please do not open a public issue**.

Instead, report it privately by emailing the maintainers. Include:

- A description of the vulnerability and its impact
- Steps to reproduce or a proof of concept
- Suggested fix (if any)

You will receive a response within 72 hours. If the vulnerability is confirmed, a fix will be prioritized and a security advisory will be published.

## Scope

- Vault encryption (AES-256-GCM, Argon2id key derivation)
- Vault file format (PMV) parsing and validation
- In-memory key handling and zeroization
- Tauri IPC boundary and CSP configuration
- Password generator randomness source

## Out of Scope

- Vulnerabilities in third-party dependencies (report upstream)
- Social engineering attacks
- Physical access to an unlocked device

## Security Architecture Summary

- **KDF:** Argon2id (64 MiB, 3 iterations, 4 lanes by default)
- **Cipher:** AES-256-GCM with 12-byte random nonce
- **Key model:** User password → vault key → DEK → payload (two-layer encryption)
- **Key material:** Stored in memory using `Zeroizing<Vec<u8>>`, zeroed on lock
- **File integrity:** GCM authentication tag verified on every decryption

**Note:** Passman has not undergone a formal security audit. Use at your own risk.
