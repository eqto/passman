import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

import { currentVault, updateVaultData } from "../vault/store.js";

export async function addGroup(group) {
  const vault = get(currentVault);
  if (!vault) return [];
  const list = await invoke("add_group", { path: vault.path, group });
  updateVaultData(vault.path, { groups: list });
  return list;
}

export async function addTag(tag) {
  const vault = get(currentVault);
  if (!vault) return [];
  const list = await invoke("add_tag", { path: vault.path, tag });
  updateVaultData(vault.path, { tags: list });
  return list;
}

export async function deleteGroup(groupId) {
  const vault = get(currentVault);
  if (!vault) return [];
  const { groups, entries, trash } = await invoke("delete_group", {
    path: vault.path,
    groupId: groupId,
  });
  updateVaultData(vault.path, { groups, entries, trash });
  return groups;
}

export async function reorderGroups(orderedGroups) {
  const vault = get(currentVault);
  if (!vault) return [];
  const list = await invoke("reorder_groups", {
    path: vault.path,
    groups: orderedGroups,
  });
  updateVaultData(vault.path, { groups: list });
  return list;
}

export async function mergeGroups(sourceId, targetId) {
  const vault = get(currentVault);
  if (!vault || !sourceId || !targetId || sourceId === targetId) return;
  try {
    const [groups, entries] = await invoke("merge_groups", {
      path: vault.path,
      sourceId,
      targetId,
    });
    updateVaultData(vault.path, { groups, entries });
    return groups;
  } catch (e) {
    console.error("mergeGroups failed:", e);
    throw e;
  }
}

export async function moveGroupToVault(sourceId, targetPath, targetGroupId) {
  const vault = get(currentVault);
  if (!vault || !sourceId || !targetPath || !targetGroupId || vault.path === targetPath) return;
  try {
    const result = await invoke("move_group_to_vault", {
      sourcePath: vault.path,
      targetPath,
      groupId: sourceId,
      targetGroupId,
    });
    updateVaultData(vault.path, {
      groups: result.source_groups,
      entries: result.source_entries,
    });
    updateVaultData(targetPath, {
      groups: result.target_groups,
      entries: result.target_entries,
    });
    return result;
  } catch (e) {
    console.error("moveGroupToVault failed:", e);
    throw e;
  }
}

export async function copyGroupToVault(sourceId, targetPath, targetGroupId) {
  const vault = get(currentVault);
  if (!vault || !sourceId || !targetPath || !targetGroupId || vault.path === targetPath) return;
  try {
    const [groups, entries] = await invoke("copy_group_to_vault", {
      sourcePath: vault.path,
      targetPath,
      groupId: sourceId,
      targetGroupId,
    });
    updateVaultData(targetPath, { groups, entries });
    return { groups, entries };
  } catch (e) {
    console.error("copyGroupToVault failed:", e);
    throw e;
  }
}

export async function moveGroupToParent(groupId, newParentId) {
  const vault = get(currentVault);
  if (!vault || !groupId) return;
  try {
    const groups = await invoke("move_group_to_parent_cmd", {
      path: vault.path,
      groupId,
      newParentId,
    });
    updateVaultData(vault.path, { groups });
    return groups;
  } catch (e) {
    console.error("moveGroupToParent failed:", e);
    throw e;
  }
}
