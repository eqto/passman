<script>
  import { onMount, onDestroy } from "svelte";
  import { currentVault, vaultData, groups, entries, trash } from "../store.js";
  import {
    deleteEntry,
    restoreEntry,
    deleteTrashEntry,
    moveEntryToGroup,
    moveEntryToVault,
    copyEntryToGroup,
    copyEntryToVault,
  } from "../../entry/store.js";
  import { selection } from "../../../stores/selection.js";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { GroupList } from "../../group";
  import EntryList from "../../entry/components/EntryList.svelte";
  import EntryDetails from "../../entry/components/EntryDetails.svelte";
  import EntryEditor from "../../entry/components/EntryEditor.svelte";
  import { createColumnResize } from "../../../lib/columnResize.js";

  const { columnWidths, loadWidths, startResize, handleKeyResize } =
    createColumnResize();

  onDestroy(() => {
    selection.save();
  });

  onMount(() => {
    selection.reset();
    loadWidths();
  });

  $: if (!$currentVault) {
    selection.reset();
  }
  $: selection.setVaultPath($currentVault?.path || null);

  $: trashGroups = $trash.groups || [];
  $: trashGroupIds = trashGroups.map((g) => g.id);
  $: hasUngroupedTrashEntries = ($trash.entries || []).some((e) => !e.group_id);

  $: if (
    $selection.trashMode &&
    $selection.selectedTrashGroup &&
    !trashGroupIds.includes($selection.selectedTrashGroup)
  ) {
    selection.setSelectedTrashGroup(trashGroupIds[0] || "");
  }

  $: selectedEntryData = $selection.selectedEntry
    ? ($selection.trashMode
        ? $trash.entries.find((e) => e.id === $selection.selectedEntry.id)
        : $entries.find((e) => e.id === $selection.selectedEntry.id)) ||
      $selection.selectedEntry
    : null;

  $: filteredEntries = $selection.trashMode
    ? $trash.entries.filter((e) => {
        if (!$selection.selectedTrashGroup) return true;
        if ($selection.selectedTrashGroup === "__ungrouped__")
          return !e.group_id;
        return e.group_id === $selection.selectedTrashGroup;
      })
    : $entries.filter((e) => {
        if (
          $selection.selectedGroup &&
          e.group_id !== $selection.selectedGroup
        ) {
          return false;
        }
        return true;
      });

  function getGroupName(groupId) {
    const group = $groups.find((g) => g.id === groupId);
    return group ? group.name : groupId;
  }

  function handleNew() {
    const entry = {
      id: crypto.randomUUID(),
      title: "",
      username: "",
      password: "",
      url: "",
      notes: "",
      tags: [],
      group_id: $selection.selectedGroup || null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    selection.newEntry(entry);
  }

  async function handleDelete(entry) {
    if ($selection.trashMode) {
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
      const restored = $entries.find((e) => e.id === entry.id);
      if (restored?.group_id) {
        selection.setSelectedGroup(restored.group_id);
      } else if (!$selection.selectedGroup) {
        selection.setSelectedGroup("");
      }
    }
    resetSelectionIfCurrent(entry);
  }

  async function handleMoveToGroup(entry, groupId) {
    await moveEntryToGroup(entry, groupId);
    resetSelectionIfCurrent(entry);
  }

  async function handleMoveToVault(entry, vault, groupId) {
    if (!$vaultData[vault.path]?.unlocked) return;
    await moveEntryToVault(entry, vault.path, groupId);
    resetSelectionIfCurrent(entry);
  }

  async function handleCopyToGroup(entry, groupId) {
    await copyEntryToGroup(entry, groupId);
  }

  async function handleCopyToVault(entry, vault, groupId) {
    if (!$vaultData[vault.path]?.unlocked) return;
    await copyEntryToVault(entry, vault.path, groupId);
  }

  function resetSelectionIfCurrent(entry) {
    if (
      $selection.selectedEntry?.id === entry.id ||
      $selection.editingEntry?.id === entry.id
    ) {
      selection.resetSelection();
    }
  }

  async function handleKeydown(event) {
    if ($selection.mode === "edit") return;

    const target = event.target;
    const isEditable =
      target &&
      (target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.tagName === "SELECT" ||
        target.isContentEditable);
    if (isEditable) return;

    const copyKey = (event.ctrlKey || event.metaKey) && event.key === "c";
    if (
      copyKey &&
      $selection.selectedEntry &&
      $selection.selectedEntry.password
    ) {
      event.preventDefault();
      await writeText($selection.selectedEntry.password);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="vault-view">
  <div class="vault-panels">
    <div class="panel groups" style="width: {columnWidths.groups}px">
      <GroupList
        selectedGroup={$selection.selectedGroup}
        selectedTags={$selection.selectedTags}
        trashMode={$selection.trashMode}
        selectedTrashGroup={$selection.selectedTrashGroup}
        {trashGroups}
        {hasUngroupedTrashEntries}
        onSelectGroup={selection.selectGroup}
        onSelectTag={selection.selectTag}
        onSelectTrashGroup={selection.selectTrashGroup}
        onTrashClick={() => selection.trashClick(trashGroupIds)}
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
        selectedEntry={$selection.selectedEntry}
        selectedTags={$selection.selectedTags}
        trashMode={$selection.trashMode}
        hideNewButton={$selection.mode === "edit"}
        onSelect={selection.selectEntry}
        onNew={handleNew}
        onToggleTag={selection.selectTag}
        onClearTags={selection.clearTags}
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
      {#if $selection.mode === "edit"}
        <EntryEditor
          entry={$selection.editingEntry}
          selectedGroup={$selection.selectedGroup}
          onClose={selection.closeEditor}
          onDelete={handleDelete}
        />
      {:else}
        <EntryDetails
          entry={selectedEntryData}
          trashMode={$selection.trashMode}
          onEdit={selection.editEntry}
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
