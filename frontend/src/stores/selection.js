import { writable } from "svelte/store";

const DEFAULT_STATE = {
  selectedGroup: "",
  selectedEntry: null,
  editingEntry: null,
  mode: "view",
  trashMode: false,
  selectedTrashGroup: "",
  selectedTags: [],
};

const vaultStores = new Map();

function getVaultStore(path) {
  if (!vaultStores.has(path)) {
    vaultStores.set(path, writable({ ...DEFAULT_STATE }));
  }
  return vaultStores.get(path);
}

function deleteVaultStore(path) {
  vaultStores.delete(path);
}

function createVaultSelection(vaultPath) {
  const store = getVaultStore(vaultPath);
  const { subscribe, update, set } = store;

  return {
    subscribe,
    vaultPath,
    reset() {
      set({ ...DEFAULT_STATE });
    },
    selectGroup(group) {
      update((s) => ({
        ...s,
        selectedGroup: group,
        trashMode: false,
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
      }));
    },
    selectTag(tag) {
      update((s) => {
        const selectedTags = s.selectedTags.includes(tag)
          ? s.selectedTags.filter((t) => t !== tag)
          : [...s.selectedTags, tag];
        const shouldClearEntry = s.selectedEntry && !(s.selectedEntry.tags || []).includes(tag);
        return {
          ...s,
          selectedTags,
          selectedEntry: shouldClearEntry ? null : s.selectedEntry,
          editingEntry: shouldClearEntry ? null : s.editingEntry,
          mode: shouldClearEntry ? "view" : s.mode,
        };
      });
    },
    clearTags() {
      update((s) => ({ ...s, selectedTags: [] }));
    },
    selectTrashGroup(groupId) {
      update((s) => ({
        ...s,
        selectedTrashGroup: groupId,
        trashMode: true,
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
      }));
    },
    trashClick(trashGroupIds) {
      update((s) => {
        let selectedTrashGroup = s.selectedTrashGroup;
        if (!trashGroupIds.includes(selectedTrashGroup)) {
          selectedTrashGroup = trashGroupIds.length > 0 ? trashGroupIds[0] : "__ungrouped__";
        }
        return {
          ...s,
          trashMode: true,
          selectedTrashGroup,
          selectedEntry: null,
          editingEntry: null,
          mode: "view",
        };
      });
    },
    selectEntry(entry) {
      update((s) => ({
        ...s,
        selectedEntry: entry,
        editingEntry: null,
        mode: "view",
      }));
    },
    editEntry(entry) {
      update((s) => ({
        ...s,
        editingEntry: { ...entry },
        mode: "edit",
      }));
    },
    newEntry(entry) {
      update((s) => ({
        ...s,
        editingEntry: entry,
        selectedEntry: null,
        mode: "edit",
      }));
    },
    resetSelection() {
      update((s) => ({
        ...s,
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
      }));
    },
    closeEditor() {
      update((s) => ({
        ...s,
        mode: "view",
        editingEntry: null,
      }));
    },
    setTrashMode(trashMode) {
      update((s) => ({ ...s, trashMode }));
    },
    setSelectedGroup(group) {
      update((s) => ({ ...s, selectedGroup: group }));
    },
    setSelectedTrashGroup(groupId) {
      update((s) => ({ ...s, selectedTrashGroup: groupId }));
    },
  };
}

export { createVaultSelection, deleteVaultStore };
