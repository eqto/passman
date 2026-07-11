import { writable } from "svelte/store";
import { setVaultViewState } from "../features/vault/store.js";

function createSelectionStore() {
  const { subscribe, set, update } = writable({
    selectedGroup: "",
    selectedEntry: null,
    editingEntry: null,
    mode: "view",
    trashMode: false,
    selectedTrashGroup: "",
    selectedTags: [],
  });

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

  return {
    subscribe,
    setVaultPath(path) {
      currentVaultPath = path;
    },
    reset() {
      set({
        selectedGroup: "",
        selectedEntry: null,
        editingEntry: null,
        mode: "view",
        trashMode: false,
        selectedTrashGroup: "",
        selectedTags: [],
      });
    },
    selectGroup(group) {
      update((s) => {
        const next = {
          ...s,
          selectedGroup: group,
          trashMode: false,
          selectedEntry: null,
          editingEntry: null,
          mode: "view",
        };
        saveViewState(next);
        return next;
      });
    },
    selectTag(tag) {
      update((s) => {
        let selectedTags;
        if (s.selectedTags.includes(tag)) {
          selectedTags = s.selectedTags.filter((t) => t !== tag);
        } else {
          selectedTags = [...s.selectedTags, tag];
        }
        let selectedEntry = s.selectedEntry;
        let editingEntry = s.editingEntry;
        let mode = s.mode;
        if (s.selectedEntry && !(s.selectedEntry.tags || []).includes(tag)) {
          selectedEntry = null;
          editingEntry = null;
          mode = "view";
        }
        const next = {
          ...s,
          selectedTags,
          selectedEntry,
          editingEntry,
          mode,
        };
        saveViewState(next);
        return next;
      });
    },
    clearTags() {
      update((s) => {
        const next = { ...s, selectedTags: [] };
        saveViewState(next);
        return next;
      });
    },
    selectTrashGroup(groupId) {
      update((s) => {
        const next = {
          ...s,
          selectedTrashGroup: groupId,
          trashMode: true,
          selectedEntry: null,
          editingEntry: null,
          mode: "view",
        };
        saveViewState(next);
        return next;
      });
    },
    trashClick(trashGroupIds) {
      update((s) => {
        let selectedTrashGroup = s.selectedTrashGroup;
        if (!trashGroupIds.includes(selectedTrashGroup)) {
          selectedTrashGroup =
            trashGroupIds.length > 0 ? trashGroupIds[0] : "__ungrouped__";
        }
        const next = {
          ...s,
          trashMode: true,
          selectedTrashGroup,
          selectedEntry: null,
          editingEntry: null,
          mode: "view",
        };
        saveViewState(next);
        return next;
      });
    },
    selectEntry(entry) {
      update((s) => {
        const next = {
          ...s,
          selectedEntry: entry,
          editingEntry: null,
          mode: "view",
        };
        saveViewState(next);
        return next;
      });
    },
    editEntry(entry) {
      update((s) => {
        const next = {
          ...s,
          editingEntry: { ...entry },
          mode: "edit",
        };
        saveViewState(next);
        return next;
      });
    },
    newEntry(entry) {
      update((s) => {
        const next = {
          ...s,
          editingEntry: entry,
          selectedEntry: null,
          mode: "edit",
        };
        saveViewState(next);
        return next;
      });
    },
    resetSelection() {
      update((s) => {
        const next = {
          ...s,
          selectedEntry: null,
          editingEntry: null,
          mode: "view",
        };
        saveViewState(next);
        return next;
      });
    },
    closeEditor() {
      update((s) => {
        const next = {
          ...s,
          mode: "view",
          editingEntry: null,
        };
        saveViewState(next);
        return next;
      });
    },
    setTrashMode(trashMode) {
      update((s) => {
        const next = { ...s, trashMode };
        saveViewState(next);
        return next;
      });
    },
    setSelectedGroup(group) {
      update((s) => {
        const next = { ...s, selectedGroup: group };
        saveViewState(next);
        return next;
      });
    },
    setSelectedTrashGroup(groupId) {
      update((s) => {
        const next = { ...s, selectedTrashGroup: groupId };
        saveViewState(next);
        return next;
      });
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
