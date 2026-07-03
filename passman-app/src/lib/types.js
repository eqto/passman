/**
 * Shared type definitions for IPC data structures.
 * Use these JSDoc types with `@type` annotations for type-safe invoke() calls.
 */

/**
 * @typedef {Object} CustomField
 * @property {string} id
 * @property {string} label
 * @property {string} type
 * @property {string} value
 */

/**
 * @typedef {Object} VaultEntry
 * @property {string} id
 * @property {string} title
 * @property {string} username
 * @property {string} password
 * @property {string} url
 * @property {string} notes
 * @property {string[]} tags
 * @property {CustomField[]} fields
 * @property {string} created_at
 * @property {string} updated_at
 */

/**
 * @typedef {Object} TrashGroup
 * @property {string} group
 * @property {VaultEntry[]} entries
 */

/**
 * @typedef {Object} VaultFileDTO
 * @property {string} path
 * @property {string} name
 * @property {string[]} groups
 * @property {string[]} tags
 * @property {VaultEntry[]} entries
 * @property {TrashGroup[]} trash
 */

/**
 * @typedef {Object} VaultConfig
 * @property {string} id
 * @property {string} name
 * @property {string} path
 */

/**
 * @typedef {Object} AppConfig
 * @property {VaultConfig[]} vaults
 */

/**
 * @typedef {Object} EntryMutationResult
 * @property {VaultEntry} entry
 */

/**
 * @typedef {Object} EntryDeletionResult
 * @property {VaultEntry[]} entries
 * @property {TrashGroup[]} trash
 */

/**
 * @typedef {Object} TrashMutationResult
 * @property {string} group
 * @property {string[]} groups
 * @property {VaultEntry[]} entries
 * @property {TrashGroup[]} trash
 */

/**
 * @typedef {Object} GroupDeletionResult
 * @property {string[]} groups
 * @property {VaultEntry[]} entries
 * @property {TrashGroup[]} trash
 */

/**
 * @typedef {Object} MoveGroupToVaultResult
 * @property {string[]} source_groups
 * @property {VaultEntry[]} source_entries
 * @property {string[]} target_groups
 * @property {VaultEntry[]} target_entries
 */

/**
 * @typedef {Object} PasswordOptions
 * @property {number} length
 * @property {boolean} uppercase
 * @property {boolean} lowercase
 * @property {boolean} digits
 * @property {boolean} symbols
 */

export { };
