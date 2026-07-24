import { get } from "svelte/store";
import { Clipboard } from "@wailsio/runtime";
import {
  deleteEntry,
  restoreEntry,
  deleteTrashEntry,
  moveEntryToGroup,
  moveEntryToVault,
  copyEntryToGroup,
  copyEntryToVault,
} from "./store.js";
import { vaultData } from "../vault/store.js";

export function createEntryActions(selection, vaultPath) {
  function getGroupName(groupId) {
    const groups = get(vaultData)[vaultPath]?.groups || [];
    const group = groups.find((g) => g.id === groupId);
    return group ? group.name : groupId;
  }

  function resetSelectionIfCurrent(entry) {
    const s = get(selection);
    if (
      s.selectedEntry?.id === entry.id ||
      s.editingEntry?.id === entry.id
    ) {
      selection.resetSelection();
    }
  }

  function handleNew() {
    const s = get(selection);
    const entry = {
      id: crypto.randomUUID(),
      title: "",
      username: "",
      password: "",
      url: "",
      notes: "",
      tags: [],
      group_id: s.selectedGroup || null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    selection.newEntry(entry);
  }

  async function handleDelete(entry) {
    const s = get(selection);
    if (s.trashMode) {
      if (!confirm(`Permanently delete "${entry.title}"?`)) return;
      await deleteTrashEntry(entry.id);
    } else {
      if (!confirm(`Move "${entry.title}" to Trash?`)) return;
      await deleteEntry(entry.id, entry.group_id, getGroupName(entry.group_id));
    }
    resetSelectionIfCurrent(entry);
  }

  async function handleRestore(entry) {
    if (!confirm(`Restore "${entry.title}"?`)) return;
    const groupName = await restoreEntry(entry.id);
    if (groupName) {
      selection.setTrashMode(false);
      const entries = get(vaultData)[vaultPath]?.entries || [];
      const restored = entries.find((e) => e.id === entry.id);
      if (restored?.group_id) {
        selection.setSelectedGroup(restored.group_id);
      } else {
        const s = get(selection);
        if (!s.selectedGroup) {
          selection.setSelectedGroup("");
        }
      }
    }
    resetSelectionIfCurrent(entry);
  }

  async function handleMoveToGroup(entry, groupId) {
    await moveEntryToGroup(entry, groupId);
    resetSelectionIfCurrent(entry);
  }

  async function handleMoveToVault(entry, vault, groupId) {
    const vd = get(vaultData);
    if (!vd[vault.path]?.unlocked) return;
    await moveEntryToVault(entry, vault.path, groupId);
    resetSelectionIfCurrent(entry);
  }

  async function handleCopyToGroup(entry, groupId) {
    await copyEntryToGroup(entry, groupId);
  }

  async function handleCopyToVault(entry, vault, groupId) {
    const vd = get(vaultData);
    if (!vd[vault.path]?.unlocked) return;
    await copyEntryToVault(entry, vault.path, groupId);
  }

  async function handleCopyPassword() {
    const s = get(selection);
    if (s.selectedEntry?.password) {
      await Clipboard.SetText(s.selectedEntry.password);
    }
  }

  return {
    handleNew,
    handleDelete,
    handleRestore,
    handleMoveToGroup,
    handleMoveToVault,
    handleCopyToGroup,
    handleCopyToVault,
    handleCopyPassword,
    resetSelectionIfCurrent,
  };
}
