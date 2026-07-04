import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

import { currentVault, vaultData, updateVaultData, groups, entries } from "./vaults";
import { DEFAULT_PASSWORD_LENGTH } from "../lib/constants.js";

export async function addEntry(entry) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entry: added } = await invoke("add_entry", { path: vault.path, entry });
  updateVaultData(vault.path, {
    entries: [...(get(entries) || []), added],
  });
}

export async function updateEntry(entry) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entry: updated } = await invoke("update_entry", { path: vault.path, entry });
  updateVaultData(vault.path, {
    entries: (get(entries) || []).map((e) =>
      e.id === updated.id ? updated : e
    ),
  });
}

export async function deleteEntry(id, groupId, groupName) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entries, trash } = await invoke("delete_entry", {
    path: vault.path,
    id,
    groupId,
    groupName,
  });
  updateVaultData(vault.path, { entries, trash });
}

export async function restoreEntry(id) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { group_name, groups, entries, trash } = await invoke("restore_trash_entry", { path: vault.path, id });
  updateVaultData(vault.path, { groups, entries, trash });
  return group_name;
}

export async function deleteTrashEntry(id) {
  const vault = get(currentVault);
  if (!vault) return [];
  const trash = await invoke("delete_trash_entry", { path: vault.path, id });
  updateVaultData(vault.path, { trash });
}

export async function moveEntryToGroup(entry, groupId) {
  const vault = get(currentVault);
  if (!vault) return;
  if (entry.group_id === groupId) return;
  const updated = {
    ...entry,
    group_id: groupId,
    updated_at: new Date().toISOString(),
  };
  await invoke("update_entry", { path: vault.path, entry: updated });
  updateVaultData(vault.path, {
    entries: (get(entries) || []).map((e) =>
      e.id === updated.id ? updated : e
    ),
  });
}

export async function moveEntryToVault(entry, targetPath, targetGroupId) {
  const vault = get(currentVault);
  if (!vault || vault.path === targetPath) return;
  const newEntry = { ...entry, group_id: targetGroupId, updated_at: new Date().toISOString() };
  const { entries: sourceEntries, trash } = await invoke("delete_entry", {
    path: vault.path,
    id: entry.id,
    groupId: entry.group_id,
    groupName: entry.group_name || "",
  });
  const { entry: added } = await invoke("add_entry", { path: targetPath, entry: newEntry });
  updateVaultData(vault.path, { entries: sourceEntries, trash });
  const targetEntries = (get(vaultData)[targetPath] || {}).entries || [];
  updateVaultData(targetPath, {
    entries: [...targetEntries, added],
  });
}

export async function moveEntriesWithTagToGroup(tag, groupId) {
  const vault = get(currentVault);
  if (!vault || !tag || !groupId) return;
  const $entries = get(entries);
  const targets = $entries.filter((e) => (e.tags || []).includes(tag));
  for (const entry of targets) {
    await moveEntryToGroup(entry, groupId);
  }
}

export async function moveEntriesInGroupToTag(groupId, tag) {
  const vault = get(currentVault);
  if (!vault || !groupId || !tag) return;
  const $entries = get(entries);
  const targets = $entries.filter((e) => e.group_id === groupId);
  for (const entry of targets) {
    const updated = { ...entry, tags: [...(entry.tags || []), tag], updated_at: new Date().toISOString() };
    await invoke("update_entry", { path: vault.path, entry: updated });
  }
  const list = await invoke("list_entries", { path: vault.path });
  updateVaultData(vault.path, { entries: list });
}

export async function copyEntryToGroup(entry, groupId) {
  const vault = get(currentVault);
  if (!vault) return;
  const now = new Date().toISOString();
  const copy = {
    ...entry,
    id: crypto.randomUUID(),
    group_id: groupId,
    created_at: now,
    updated_at: now,
  };
  const { entry: added } = await invoke("add_entry", { path: vault.path, entry: copy });
  updateVaultData(vault.path, {
    entries: [...(get(entries) || []), added],
  });
}

export async function copyEntryToVault(entry, targetPath, targetGroupId) {
  const vault = get(currentVault);
  if (!vault || vault.path === targetPath) return;
  const now = new Date().toISOString();
  const copy = {
    ...entry,
    id: crypto.randomUUID(),
    group_id: targetGroupId,
    created_at: now,
    updated_at: now,
  };
  const { entry: added } = await invoke("add_entry", { path: targetPath, entry: copy });
  const targetEntries = (get(vaultData)[targetPath] || {}).entries || [];
  updateVaultData(targetPath, {
    entries: [...targetEntries, added],
  });
}

export async function generatePassword(length = DEFAULT_PASSWORD_LENGTH, options = {}) {
  return await invoke("generate_password", {
    options: {
      length,
      uppercase: options.uppercase ?? true,
      lowercase: options.lowercase ?? true,
      digits: options.digits ?? true,
      symbols: options.symbols ?? true,
    },
  });
}
