import { get } from "svelte/store";
import * as entryService from "../../../bindings/github.com/eqto/passman/internal/app/entryservice.js";

import { currentVault, vaultData, updateVaultData, groups, entries } from "../vault/index.js";

export async function addEntry(entry) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entry: added } = await entryService.AddEntry(vault.path, entry);
  updateVaultData(vault.path, {
    entries: [...(get(entries) || []), added],
  });
}

export async function updateEntry(entry) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entry: updated } = await entryService.UpdateEntry(vault.path, entry);
  updateVaultData(vault.path, {
    entries: (get(entries) || []).map((e) =>
      e.id === updated.id ? updated : e
    ),
  });
}

export async function deleteEntry(id, groupId, groupName) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { entries, trash } = await entryService.DeleteEntry(vault.path, id);
  updateVaultData(vault.path, { entries, trash });
}

export async function restoreEntry(id) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { group_name, groups, entries, trash } = await entryService.RestoreTrashEntry(vault.path, id);
  updateVaultData(vault.path, { groups, entries, trash });
  return group_name;
}

export async function deleteTrashEntry(id) {
  const vault = get(currentVault);
  if (!vault) return [];
  const trash = await entryService.DeleteTrashEntry(vault.path, id);
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
  await entryService.UpdateEntry(vault.path, updated);
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
  const { entries: sourceEntries, trash } = await entryService.DeleteEntry(vault.path, entry.id);
  const { entry: added } = await entryService.AddEntry(targetPath, newEntry);
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
    const updated = { ...entry, tags: [...(entry.tags || []), tag].sort((a, b) => a.localeCompare(b)), updated_at: new Date().toISOString() };
    await entryService.UpdateEntry(vault.path, updated);
  }
  const list = await entryService.ListEntries(vault.path);
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
  const { entry: added } = await entryService.AddEntry(vault.path, copy);
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
  const { entry: added } = await entryService.AddEntry(targetPath, copy);
  const targetEntries = (get(vaultData)[targetPath] || {}).entries || [];
  updateVaultData(targetPath, {
    entries: [...targetEntries, added],
  });
}
