import { writable } from "svelte/store";
import { setVaultViewState } from "../features/vault/store.js";

const DEFAULT_STATE = {
  selectedGroup: "",
  selectedEntry: null,
  editingEntry: null,
  mode: "view",
  trashMode: false,
  selectedTrashGroup: "",
  selectedTags: [],
};

function createSelectionStore() {
  const { subscribe, set, update } = writable({ ...DEFAULT_STATE });

  let currentVaultPath = null;

  function saveViewState(state) {
    if (!currentVaultPath) return;
    setVaultViewState(currentVaultPath, {
      selectedGroup: state.selectedGroup,
      selectedEntry: state.selectedEntry,
      editingEntry: state.editingEntry,
      mode: state.mode,
      trashMode: state.trashMode,
      selectedTrashGroup: state.selectedTrashGroup,
      selectedTags: state.selectedTags,
    });
  }

  function updateAndSave(updater) {
    update((s) => {
      const next = updater(s);
      saveViewState(next);
      return next;
    });
  }

  return {
    subscribe,
    setVaultPath(path) {
      currentVaultPath = path;
    },
    reset() {
      set({ ...DEFAULT_STATE });
    },
    selectGroup(group) {
      updateAndSave((s) => ({
        ...s,
        selectedGroup: group,
        trashMode: false,
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
      }));
    },
    selectTag(tag) {
      updateAndSave((s) => {
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
      updateAndSave((s) => ({ ...s, selectedTags: [] }));
    },
    selectTrashGroup(groupId) {
      updateAndSave((s) => ({
        ...s,
        selectedTrashGroup: groupId,
        trashMode: true,
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
      }));
    },
    trashClick(trashGroupIds) {
      updateAndSave((s) => {
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
      updateAndSave((s) => ({
        ...s,
        selectedEntry: entry,
        editingEntry: null,
        mode: "view",
      }));
    },
    editEntry(entry) {
      updateAndSave((s) => ({
        ...s,
        editingEntry: { ...entry },
        mode: "edit",
      }));
    },
    newEntry(entry) {
      updateAndSave((s) => ({
        ...s,
        editingEntry: entry,
        selectedEntry: null,
        mode: "edit",
      }));
    },
    resetSelection() {
      updateAndSave((s) => ({
        ...s,
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
      }));
    },
    closeEditor() {
      updateAndSave((s) => ({
        ...s,
        mode: "view",
        editingEntry: null,
      }));
    },
    setTrashMode(trashMode) {
      updateAndSave((s) => ({ ...s, trashMode }));
    },
    setSelectedGroup(group) {
      updateAndSave((s) => ({ ...s, selectedGroup: group }));
    },
    setSelectedTrashGroup(groupId) {
      updateAndSave((s) => ({ ...s, selectedTrashGroup: groupId }));
    },
    save() {
      let state;
      update((s) => {
        state = s;
        return s;
      });
      if (state) saveViewState(state);
    },
  };
}

export const selection = createSelectionStore();
