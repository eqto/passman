import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

import { currentVault, vaultData, updateVaultData, groups, entries } from "./vaults";
import { splitTags } from "../lib/tags.js";
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

export async function deleteEntry(id, group) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entries, trash } = await invoke("delete_entry", { path: vault.path, id, group });
  updateVaultData(vault.path, { entries, trash });
}

export async function restoreEntry(id) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { group, groups, entries, trash } = await invoke("restore_trash_entry", { path: vault.path, id });
  updateVaultData(vault.path, { groups, entries, trash });
  return group;
}

export async function deleteTrashEntry(id) {
  const vault = get(currentVault);
  if (!vault) return [];
  const trash = await invoke("delete_trash_entry", { path: vault.path, id });
  updateVaultData(vault.path, { trash });
}

export async function moveEntryToGroup(entry, group) {
  const vault = get(currentVault);
  if (!vault) return;
  const $groups = get(groups);
  const { groupTags, freeTags } = splitTags(entry.tags, $groups);
  if (groupTags.includes(group)) return;
  const updated = {
    ...entry,
    tags: [...freeTags, group],
    updated_at: new Date().toISOString(),
  };
  await invoke("update_entry", { path: vault.path, entry: updated });
  updateVaultData(vault.path, {
    entries: (get(entries) || []).map((e) =>
      e.id === updated.id ? updated : e
    ),
  });
}

export async function moveEntryToVault(entry, targetPath, targetGroup) {
  const vault = get(currentVault);
  if (!vault || vault.path === targetPath) return;
  const $groups = get(groups);
  const { freeTags } = splitTags(entry.tags, $groups);
  let newTags = freeTags;
  if (targetGroup && !newTags.includes(targetGroup)) {
    newTags = [...newTags, targetGroup];
  }
  const newEntry = { ...entry, tags: newTags, updated_at: new Date().toISOString() };
  const { groupTags } = splitTags(entry.tags, $groups);
  const sourceGroup = groupTags[0] || "";
  const { entries: sourceEntries, trash } = await invoke("delete_entry", { path: vault.path, id: entry.id, group: sourceGroup });
  const { entry: added } = await invoke("add_entry", { path: targetPath, entry: newEntry });
  updateVaultData(vault.path, { entries: sourceEntries, trash });
  const targetEntries = (get(vaultData)[targetPath] || {}).entries || [];
  updateVaultData(targetPath, {
    entries: [...targetEntries, added],
  });
}

export async function moveEntriesWithTagToGroup(tag, group) {
  const vault = get(currentVault);
  if (!vault || !tag || !group) return;
  const $entries = get(entries);
  const targets = $entries.filter((e) => (e.tags || []).includes(tag));
  for (const entry of targets) {
    await moveEntryToGroup(entry, group);
  }
}

export async function moveEntriesInGroupToTag(group, tag) {
  const vault = get(currentVault);
  if (!vault || !group || !tag) return;
  const $groups = get(groups);
  const $entries = get(entries);
  const targets = $entries.filter((e) =>
    splitTags(e.tags, $groups).groupTags.includes(group)
  );
  for (const entry of targets) {
    const { freeTags } = splitTags(entry.tags, $groups);
    const updated = { ...entry, tags: [...freeTags, tag], updated_at: new Date().toISOString() };
    await invoke("update_entry", { path: vault.path, entry: updated });
  }
  const list = await invoke("list_entries", { path: vault.path });
  updateVaultData(vault.path, { entries: list });
}

export async function copyEntryToGroup(entry, group) {
  const vault = get(currentVault);
  if (!vault) return;
  const $groups = get(groups);
  const { freeTags } = splitTags(entry.tags, $groups);
  const now = new Date().toISOString();
  const copy = {
    ...entry,
    id: crypto.randomUUID(),
    tags: [...freeTags, group],
    created_at: now,
    updated_at: now,
  };
  const { entry: added } = await invoke("add_entry", { path: vault.path, entry: copy });
  updateVaultData(vault.path, {
    entries: [...(get(entries) || []), added],
  });
}

export async function copyEntryToVault(entry, targetPath, targetGroup) {
  const vault = get(currentVault);
  if (!vault || vault.path === targetPath) return;
  const $groups = get(groups);
  const { freeTags } = splitTags(entry.tags, $groups);
  const now = new Date().toISOString();
  const copy = {
    ...entry,
    id: crypto.randomUUID(),
    tags: freeTags,
    created_at: now,
    updated_at: now,
  };
  if (targetGroup && !copy.tags.includes(targetGroup)) {
    copy.tags = [...copy.tags, targetGroup];
  }
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
