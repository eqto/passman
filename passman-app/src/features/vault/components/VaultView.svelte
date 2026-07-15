<script>
  import { onMount } from "svelte";
  import { vaultData } from "../store.js";
  import { createEntryActions } from "../../entry/actions.js";
  import { createVaultSelection } from "../../../stores/selection.js";
  import { GroupList } from "../../group";
  import EntryList from "../../entry/components/EntryList.svelte";
  import EntryDetails from "../../entry/components/EntryDetails.svelte";
  import EntryEditor from "../../entry/components/EntryEditor.svelte";
  import { createColumnResize } from "../../../lib/columnResize.js";

  let { vault } = $props();

  const vaultPath = vault.path;
  const selection = createVaultSelection(vaultPath);
  const entryActions = createEntryActions(selection, vaultPath);

  const { columnWidths, loadWidths, startResize, handleKeyResize } =
    createColumnResize();

  onMount(() => {
    loadWidths();
  });

  let vaultEntries = $derived($vaultData[vaultPath]?.entries || []);
  let vaultGroups = $derived($vaultData[vaultPath]?.groups || []);
  let vaultTrash = $derived(
    $vaultData[vaultPath]?.trash || { groups: [], entries: [] },
  );

  $effect(() => {
    if (
      vaultGroups.length > 0 &&
      !$selection.selectedGroup &&
      !$selection.trashMode
    ) {
      selection.selectGroup(vaultGroups[0].id);
    }
  });

  let trashGroups = $derived(vaultTrash.groups || []);
  let trashGroupIds = $derived(trashGroups.map((g) => g.id));
  let hasUngroupedTrashEntries = $derived(
    (vaultTrash.entries || []).some((e) => !e.group_id),
  );

  $effect(() => {
    if (
      $selection.trashMode &&
      $selection.selectedTrashGroup &&
      !trashGroupIds.includes($selection.selectedTrashGroup)
    ) {
      selection.setSelectedTrashGroup(trashGroupIds[0] || "");
    }
  });

  let selectedEntryData = $derived(
    $selection.selectedEntry
      ? ($selection.trashMode
          ? vaultTrash.entries.find((e) => e.id === $selection.selectedEntry.id)
          : vaultEntries.find((e) => e.id === $selection.selectedEntry.id)) ||
          $selection.selectedEntry
      : null,
  );

  let filteredEntries = $derived(
    $selection.trashMode
      ? vaultTrash.entries.filter((e) => {
          if (!$selection.selectedTrashGroup) return true;
          if ($selection.selectedTrashGroup === "__ungrouped__")
            return !e.group_id;
          return e.group_id === $selection.selectedTrashGroup;
        })
      : vaultEntries.filter((e) => {
          if (
            $selection.selectedGroup &&
            e.group_id !== $selection.selectedGroup
          ) {
            return false;
          }
          return true;
        }),
  );

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
      await entryActions.handleCopyPassword();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="vault-view">
  <div class="vault-panels">
    <div class="panel groups" style="width: {columnWidths.groups}px">
      <GroupList
        {vault}
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
      onmousedown={(e) => startResize("groups", e)}
      onkeydown={(e) => handleKeyResize("groups", e)}
    ></button>
    <div class="panel entries" style="width: {columnWidths.entries}px">
      <EntryList
        entries={filteredEntries}
        selectedEntry={$selection.selectedEntry}
        selectedTags={$selection.selectedTags}
        trashMode={$selection.trashMode}
        hideNewButton={$selection.mode === "edit"}
        onSelect={selection.selectEntry}
        onNew={entryActions.handleNew}
        onToggleTag={selection.selectTag}
        onClearTags={selection.clearTags}
        onMoveToGroup={entryActions.handleMoveToGroup}
        onMoveToVault={entryActions.handleMoveToVault}
        onCopyToGroup={entryActions.handleCopyToGroup}
        onCopyToVault={entryActions.handleCopyToVault}
      />
    </div>
    <button
      class="resize-handle"
      type="button"
      aria-label="Resize entries panel"
      onmousedown={(e) => startResize("entries", e)}
      onkeydown={(e) => handleKeyResize("entries", e)}
    ></button>
    <div class="panel details">
      {#if $selection.mode === "edit"}
        <EntryEditor
          entry={$selection.editingEntry}
          selectedGroup={$selection.selectedGroup}
          onClose={selection.closeEditor}
          onDelete={entryActions.handleDelete}
        />
      {:else}
        <EntryDetails
          entry={selectedEntryData}
          trashMode={$selection.trashMode}
          onEdit={selection.editEntry}
          onRestore={entryActions.handleRestore}
          onDelete={entryActions.handleDelete}
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
    min-width: 320px;
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
