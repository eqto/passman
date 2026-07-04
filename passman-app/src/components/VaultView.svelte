<script>
  import { onMount, onDestroy } from "svelte";
  import { currentVault, vaultData, groups, entries, trash, setVaultViewState } from "../stores/vaults";
  import { deleteEntry, restoreEntry, deleteTrashEntry, moveEntryToGroup, moveEntryToVault, copyEntryToGroup, copyEntryToVault } from "../stores/entries";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { GroupList } from "./group";
  import EntryList from "./EntryList.svelte";
  import EntryDetails from "./EntryDetails.svelte";
  import EntryEditor from "./EntryEditor.svelte";
  import {
    GROUP_PANEL_DEFAULT_WIDTH,
    GROUP_PANEL_MIN_WIDTH,
    ENTRY_PANEL_DEFAULT_WIDTH,
    ENTRY_PANEL_MIN_WIDTH,
    COLUMN_RESIZE_STEP,
  } from "../lib/constants.js";

  const WIDTHS_KEY = "passman.columnWidths";
  const MIN_WIDTH = GROUP_PANEL_MIN_WIDTH;

  let selectedGroup = "";
  let selectedEntry = null;
  let editingEntry = null;
  let mode = "view"; // "view" | "edit"
  let trashMode = false;
  let selectedTrashGroup = "";
  let selectedTags = [];

  let columnWidths = {
    groups: GROUP_PANEL_DEFAULT_WIDTH,
    entries: ENTRY_PANEL_DEFAULT_WIDTH,
  };

  let resizing = null;
  let startX = 0;
  let startWidth = 0;

  onMount(() => {
    resetVaultView();
    try {
      const saved = JSON.parse(localStorage.getItem(WIDTHS_KEY));
      if (saved) {
        columnWidths.groups = Math.max(MIN_WIDTH, saved.groups || GROUP_PANEL_DEFAULT_WIDTH);
        columnWidths.entries = Math.max(MIN_WIDTH, saved.entries || ENTRY_PANEL_DEFAULT_WIDTH);
      }
    } catch {
      // ignore invalid saved config
    }
  });

  onDestroy(() => {
    saveViewState();
  });

  function saveWidths() {
    localStorage.setItem(WIDTHS_KEY, JSON.stringify(columnWidths));
  }

  function startResize(panel, event) {
    resizing = panel;
    startX = event.clientX;
    startWidth = columnWidths[panel];
    window.addEventListener("mousemove", onResize);
    window.addEventListener("mouseup", stopResize);
  }

  function onResize(event) {
    if (!resizing) return;
    const delta = event.clientX - startX;
    columnWidths[resizing] = Math.max(MIN_WIDTH, startWidth + delta);
  }

  function stopResize() {
    resizing = null;
    window.removeEventListener("mousemove", onResize);
    window.removeEventListener("mouseup", stopResize);
    saveWidths();
  }

  function handleKeyResize(panel, event) {
    if (event.key === "ArrowLeft") {
      columnWidths[panel] = Math.max(MIN_WIDTH, columnWidths[panel] - COLUMN_RESIZE_STEP);
      saveWidths();
    } else if (event.key === "ArrowRight") {
      columnWidths[panel] += COLUMN_RESIZE_STEP;
      saveWidths();
    }
  }

  function saveViewState() {
    if (!$currentVault) return;
    setVaultViewState($currentVault.path, {
      selectedGroup,
      selectedEntry,
      editingEntry,
      mode,
      trashMode,
      selectedTrashGroup,
      selectedTags,
    });
  }

  function resetVaultView() {
    selectedGroup = "";
    selectedEntry = null;
    editingEntry = null;
    mode = "view";
    trashMode = false;
    selectedTrashGroup = "";
    selectedTags = [];
  }

  $: if (!$currentVault) {
    resetVaultView();
  }

  $: trashGroups = $trash.groups || [];
  $: trashGroupIds = trashGroups.map((g) => g.id);
  $: hasUngroupedTrashEntries = ($trash.entries || []).some((e) => !e.group_id);

  $: if (trashMode && selectedTrashGroup && !trashGroupIds.includes(selectedTrashGroup)) {
    selectedTrashGroup = trashGroupIds[0] || "";
    saveViewState();
  }

  $: selectedEntryData = selectedEntry
    ? (trashMode
        ? $trash.entries.find((e) => e.id === selectedEntry.id)
        : $entries.find((e) => e.id === selectedEntry.id)) || selectedEntry
    : null;

  $: filteredEntries = trashMode
    ? $trash.entries.filter((e) => {
        if (!selectedTrashGroup) return true;
        if (selectedTrashGroup === "__ungrouped__") return !e.group_id;
        return e.group_id === selectedTrashGroup;
      })
    : $entries.filter((e) => {
        if (selectedGroup && e.group_id !== selectedGroup) {
          return false;
        }
        return true;
      });

  function selectedGroupName() {
    const group = $groups.find((g) => g.id === selectedGroup);
    return group ? group.name : selectedGroup;
  }

  function handleNew() {
    editingEntry = {
      id: crypto.randomUUID(),
      title: "",
      username: "",
      password: "",
      url: "",
      notes: "",
      tags: [],
      group_id: selectedGroup || null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    selectedEntry = null;
    mode = "edit";
    saveViewState();
  }

  function resetSelection() {
    selectedEntry = null;
    editingEntry = null;
    mode = "view";
  }

  function handleSelectGroup(group) {
    selectedGroup = group;
    trashMode = false;
    resetSelection();
    saveViewState();
  }

  function handleSelectTag(tag) {
    if (selectedTags.includes(tag)) {
      selectedTags = selectedTags.filter((t) => t !== tag);
    } else {
      selectedTags = [...selectedTags, tag];
      if (selectedEntry && !(selectedEntry.tags || []).includes(tag)) {
        resetSelection();
      }
    }
    saveViewState();
  }

  function handleClearTags() {
    selectedTags = [];
    saveViewState();
  }

  function handleSelectTrashGroup(groupId) {
    selectedTrashGroup = groupId;
    trashMode = true;
    resetSelection();
    saveViewState();
  }

  function handleTrashClick() {
    trashMode = true;
    const ids = ($trash.groups || []).map((g) => g.id);
    if (!ids.includes(selectedTrashGroup)) {
      selectedTrashGroup = ids.length > 0 ? ids[0] : "__ungrouped__";
    }
    resetSelection();
    saveViewState();
  }

  function getGroupName(groupId) {
    const group = $groups.find((g) => g.id === groupId);
    return group ? group.name : groupId;
  }

  function handleSelect(entry) {
    selectedEntry = entry;
    editingEntry = null;
    mode = "view";
    saveViewState();
  }

  function handleEdit(entry) {
    editingEntry = { ...entry };
    mode = "edit";
    saveViewState();
  }

  async function handleDelete(entry) {
    if (trashMode) {
      if (!confirm(`Permanently delete "${entry.title}"?`)) return;
      await deleteTrashEntry(entry.id);
    } else {
      if (!confirm(`Move "${entry.title}" to Trash?`)) return;
      await deleteEntry(entry.id, entry.group_id, getGroupName(entry.group_id));
    }
    if (selectedEntry?.id === entry.id || editingEntry?.id === entry.id) {
      resetSelection();
    }
    saveViewState();
  }

  async function handleRestore(entry) {
    if (!confirm(`Restore "${entry.title}"?`)) return;
    const groupName = await restoreEntry(entry.id);
    if (groupName) {
      trashMode = false;
      const restored = $entries.find((e) => e.id === entry.id);
      if (restored?.group_id) {
        selectedGroup = restored.group_id;
      } else if (selectedGroup) {
        // keep current group if restored entry has no group
      } else {
        selectedGroup = "";
      }
    }
    if (selectedEntry?.id === entry.id || editingEntry?.id === entry.id) {
      resetSelection();
    }
    saveViewState();
  }

  async function handleMoveToGroup(entry, groupId) {
    await moveEntryToGroup(entry, groupId);
    if (selectedEntry?.id === entry.id || editingEntry?.id === entry.id) {
      resetSelection();
    }
    saveViewState();
  }

  async function handleMoveToVault(entry, vault, groupId) {
    if (!$vaultData[vault.path]?.unlocked) return;
    await moveEntryToVault(entry, vault.path, groupId);
    if (selectedEntry?.id === entry.id || editingEntry?.id === entry.id) {
      resetSelection();
    }
    saveViewState();
  }

  async function handleCopyToGroup(entry, groupId) {
    await copyEntryToGroup(entry, groupId);
    saveViewState();
  }

  async function handleCopyToVault(entry, vault, groupId) {
    if (!$vaultData[vault.path]?.unlocked) return;
    await copyEntryToVault(entry, vault.path, groupId);
    saveViewState();
  }

  function handleCloseEditor() {
    mode = "view";
    editingEntry = null;
    saveViewState();
  }

  async function handleKeydown(event) {
    if (mode === "edit") return;

    const target = event.target;
    const isEditable =
      target &&
      (target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.tagName === "SELECT" ||
        target.isContentEditable);
    if (isEditable) return;

    const copyKey = (event.ctrlKey || event.metaKey) && event.key === "c";
    if (copyKey && selectedEntry && selectedEntry.password) {
      event.preventDefault();
      await writeText(selectedEntry.password);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="vault-view">
  <div class="vault-panels">
    <div class="panel groups" style="width: {columnWidths.groups}px">
      <GroupList
        selectedGroup={selectedGroup}
        selectedTags={selectedTags}
        trashMode={trashMode}
        selectedTrashGroup={selectedTrashGroup}
        trashGroups={trashGroups}
        hasUngroupedTrashEntries={hasUngroupedTrashEntries}
        onSelectGroup={handleSelectGroup}
        onSelectTag={handleSelectTag}
        onSelectTrashGroup={handleSelectTrashGroup}
        onTrashClick={handleTrashClick}
      />
    </div>
    <button
      class="resize-handle"
      type="button"
      aria-label="Resize groups panel"
      on:mousedown={(e) => startResize("groups", e)}
      on:keydown={(e) => handleKeyResize("groups", e)}
    ></button>
    <div class="panel entries" style="width: {columnWidths.entries}px">
      <EntryList
        entries={filteredEntries}
        selectedEntry={selectedEntry}
        selectedTags={selectedTags}
        trashMode={trashMode}
        hideNewButton={mode === "edit"}
        onSelect={handleSelect}
        onNew={handleNew}
        onToggleTag={handleSelectTag}
        onClearTags={handleClearTags}
        onMoveToGroup={handleMoveToGroup}
        onMoveToVault={handleMoveToVault}
        onCopyToGroup={handleCopyToGroup}
        onCopyToVault={handleCopyToVault}
      />
    </div>
    <button
      class="resize-handle"
      type="button"
      aria-label="Resize entries panel"
      on:mousedown={(e) => startResize("entries", e)}
      on:keydown={(e) => handleKeyResize("entries", e)}
    ></button>
    <div class="panel details">
      {#if mode === "edit"}
        <EntryEditor entry={editingEntry} selectedGroup={selectedGroup} onClose={handleCloseEditor} onDelete={handleDelete} />
      {:else}
        <EntryDetails
          entry={selectedEntryData}
          trashMode={trashMode}
          onEdit={handleEdit}
          onRestore={handleRestore}
          onDelete={handleDelete}
        />
      {/if}
    </div>
  </div>
</div>

<style>
  .vault-view {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    overflow: hidden;
  }

  .vault-panels {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  .panel {
    height: 100%;
    overflow: hidden;
  }

  .panel.groups {
    flex-shrink: 0;
  }

  .panel.entries {
    flex-shrink: 0;
  }

  .panel.details {
    flex: 1;
    min-width: 160px;
  }

  .resize-handle {
    width: 4px;
    padding: 0;
    margin: 0;
    border: none;
    cursor: col-resize;
    background-color: transparent;
    flex-shrink: 0;
    transition: background-color 0.15s ease;
  }

  .resize-handle:hover,
  .resize-handle:focus,
  .resize-handle:active {
    background-color: var(--accent-color);
    outline: none;
  }
</style>
