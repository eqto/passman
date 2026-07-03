<script>
  import { groups, tags, vaults, currentVault, vaultData, setVaultViewState } from "../../stores/vaults";
  import { showToast } from "../../stores/toast.js";
  import { closeAllContextMenus } from "../../stores/contextMenu.js";
  import { onMount } from "svelte";
  import {
    addGroup,
    addTag,
    deleteGroup,
    reorderGroups,
    mergeGroups,
    moveGroupToVault,
    copyGroupToVault,
  } from "../../stores/groups";
  import { moveEntriesWithTagToGroup } from "../../stores/entries";
  import { AddGroupDialog, DeleteGroupDialog, GroupTagContextMenu, GroupVaultMoveDialog, GroupTitle } from "./index";
  import { createDragList } from "../../lib/dragList.js";
  import Chip from "../form/Chip.svelte";

  export let selectedGroup = "";
  export let selectedTags = [];
  export let selectedTrashGroup = "";
  export let trashMode = false;
  export let trashGroups = [];
  export let onSelectGroup;
  export let onSelectTag = (tag) => {};
  export let onSelectTrashGroup;
  export let onTrashClick;

  let showAdd = false;
  let showAddTag = false;
  let deleteTarget = null;
  let contextMenu = { show: false, x: 0, y: 0, type: "tag", item: "" };
  let moveToVaultTarget = null;
  let moveToVaultGroup = "";
  let moveToVaultAction = "move";

  onMount(() => {
    window.addEventListener('close-all-context-menus', closeContextMenu);
    return () => {
      window.removeEventListener('close-all-context-menus', closeContextMenu);
    };
  });

  function switchToVaultAndGroup(vault, groupName) {
    setVaultViewState(vault.path, {
      selectedGroup: groupName,
      selectedEntry: null,
      editingEntry: null,
      mode: "view",
    });
    currentVault.set(vault);
  }

  $: moveVaults = ($vaults || []).filter((v) => v.path !== $currentVault?.path);

  const drag = createDragList({
    axis: "vertical",
    onReorder: async (items) => {
      reorderGroups(items);
    },
  });
  const { dragItem, dragOver, insertBefore } = drag;

  async function handleAddGroup(name) {
    await addGroup(name);
    showAdd = false;
  }

  async function handleAddTag(name) {
    await addTag(name);
    showAddTag = false;
  }

  async function handleDeleteGroup(group) {
    await deleteGroup(group);
    if (selectedGroup === group) {
      onSelectGroup("");
    }
    deleteTarget = null;
  }

  function openContextMenu(event, type, item) {
    event.preventDefault();
    closeAllContextMenus();
    contextMenu = { show: true, x: event.clientX, y: event.clientY, type, item };
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, type: "tag", item: "" };
  }

  async function handleMoveToGroup(event) {
    const { item, target } = event.detail;
    await moveEntriesWithTagToGroup(item, target);
    closeContextMenu();
  }

  async function handleMergeGroup(event) {
    const { source, target } = event.detail;
    await mergeGroups(source, target);
    if (selectedGroup === source) {
      onSelectGroup(target);
    }
    closeContextMenu();
  }

  async function handleMoveToVault(event) {
    await handleVaultAction(event, "move");
  }

  async function handleCopyToVault(event) {
    await handleVaultAction(event, "copy");
  }

  async function handleVaultAction(event, action) {
    const { source, targetPath } = event.detail;
    const target = $vaults.find((v) => v.path === targetPath);
    const targetGroups = $vaultData[targetPath]?.groups || [];
    if (target && targetGroups.includes(source)) {
      moveToVaultGroup = source;
      moveToVaultTarget = target;
      moveToVaultAction = action;
      closeContextMenu();
    } else if (target) {
      try {
        const fn = action === "copy" ? copyGroupToVault : moveGroupToVault;
        await fn(source, targetPath, source);
        switchToVaultAndGroup(target, source);
        showToast(`${action === "copy" ? "Copied" : "Moved"} "${source}" to ${target.name}`);
      } catch (e) {
        console.error(e);
        alert(`${action === "copy" ? "Copy" : "Move"} failed: ${e}`);
      }
      closeContextMenu();
    }
  }

  async function handleMergeToVault() {
    await handleVaultResolve(moveToVaultGroup);
  }

  async function handleCopyToVaultAsNew(newName) {
    await handleVaultResolve(newName);
  }

  async function handleVaultResolve(targetName) {
    if (moveToVaultTarget && moveToVaultGroup) {
      try {
        const fn = moveToVaultAction === "copy" ? copyGroupToVault : moveGroupToVault;
        await fn(moveToVaultGroup, moveToVaultTarget.path, targetName);
        switchToVaultAndGroup(moveToVaultTarget, targetName);
        const verb = moveToVaultAction === "copy" ? "Copied" : "Moved";
        if (targetName === moveToVaultGroup) {
          showToast(`${verb} "${moveToVaultGroup}" into ${moveToVaultTarget.name}`);
        } else {
          showToast(`${verb} "${moveToVaultGroup}" to ${moveToVaultTarget.name} as "${targetName}"`);
        }
      } catch (e) {
        console.error(e);
        alert(`${moveToVaultAction === "copy" ? "Copy" : "Move"} failed: ${e}`);
      }
    }
    moveToVaultTarget = null;
    moveToVaultGroup = "";
    moveToVaultAction = "move";
    closeContextMenu();
  }

  function cancelMoveToVault() {
    moveToVaultTarget = null;
    moveToVaultGroup = "";
    moveToVaultAction = "move";
  }

</script>

<svelte:window on:click={closeContextMenu} />

<div class="group-list">
  {#if trashMode}
    <div class="trash-header">
      <button
        class="btn-icon"
        title="Back to groups"
        on:click={() => onSelectGroup(selectedGroup || ($groups[0] ?? ""))}
      >
        ←
      </button>
      <span>Trash</span>
    </div>

    {#if trashGroups.length === 0}
      <p class="empty-state">No deleted groups.</p>
    {:else}
      {#each trashGroups as group (group)}
        <div
          class="group-row"
          class:selected={selectedTrashGroup === group}
          role="listitem"
        >
          <div
            class="group-item"
            role="button"
            tabindex="0"
            on:click={() => onSelectTrashGroup(group)}
            on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); onSelectTrashGroup(group); } }}
          >
            <span class="group-icon">📁</span>
            <span class="group-name">{group}</span>
          </div>
        </div>
      {/each}
    {/if}
  {:else}
    <GroupTitle title="Groups" showButton={true} onButtonClick={() => showAdd = true} />

    {#if $groups.length === 0}
      <p class="empty-state">No groups.</p>
    {:else}
      {#each $groups as group (group)}
        <div
          class="group-row"
          class:selected={selectedGroup === group}
          class:dragging={$dragItem === group}
          class:drop-before={$dragOver === group && $insertBefore === true}
          class:drop-after={$dragOver === group && $insertBefore === false}
          role="listitem"
          aria-grabbed={$dragItem === group}
          draggable={true}
          on:dragstart={(e) => drag.dragStart(e, group)}
          on:dragend={drag.dragEnd}
          on:dragover={(e) => drag.handleDragOver(e, group)}
          on:dragleave={drag.dragLeave}
          on:drop={(e) => drag.drop(e, $groups, group)}
        >
          <div
            class="group-item"
            role="button"
            tabindex="0"
            on:click={() => onSelectGroup(group)}
            on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); onSelectGroup(group); } }}
            on:contextmenu={(e) => openContextMenu(e, "group", group)}
          >
            <span class="group-icon">📁</span>
            <span class="group-name">{group}</span>
          </div>
          <button
            class="btn-icon-danger"
            title="Delete group"
            on:click={() => deleteTarget = group}
          >
            ×
          </button>
        </div>
      {/each}
    {/if}

    <GroupTitle title="Tags" showButton={true} onButtonClick={() => showAddTag = true} />

    {#if $tags.length === 0}
      <p class="empty-state">No tags.</p>
    {:else}
      <div class="tags">
        {#each $tags as tag}
          <Chip
            size="medium"
            active={selectedTags.includes(tag)}
            on:click={() => onSelectTag(tag)}
            on:contextmenu={(e) => openContextMenu(e, "tag", tag)}
            on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); onSelectTag(tag); } }}
          >
            {tag}
          </Chip>
        {/each}
      </div>
    {/if}

    <GroupTitle title="Trash" isTrash={true} />
    <div class="group-row trash-row" class:selected={trashMode}>
      <div
        class="group-item"
        role="button"
        tabindex="0"
        on:click={onTrashClick}
        on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); onTrashClick(); } }}
      >
        <span class="group-icon">🗑️</span>
        <span class="group-name">Trash</span>
      </div>
    </div>
  {/if}
</div>

{#if showAdd}
  <AddGroupDialog onAdd={handleAddGroup} onCancel={() => showAdd = false} />
{/if}

{#if showAddTag}
  <AddGroupDialog title="Add Tag" onAdd={handleAddTag} onCancel={() => showAddTag = false} />
{/if}

{#if deleteTarget}
  <DeleteGroupDialog
    group={deleteTarget}
    onDelete={handleDeleteGroup}
    onCancel={() => deleteTarget = null}
  />
{/if}

{#if contextMenu.show}
  <GroupTagContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    type={contextMenu.type}
    item={contextMenu.item}
    groups={$groups}
    vaults={moveVaults}
    on:moveToGroup={handleMoveToGroup}
    on:mergeGroup={handleMergeGroup}
    on:moveToVault={handleMoveToVault}
    on:copyToVault={handleCopyToVault}
  />
{/if}

{#if moveToVaultTarget}
  <GroupVaultMoveDialog
    group={moveToVaultGroup}
    vaultName={moveToVaultTarget.name}
    action={moveToVaultAction}
    onMerge={handleMergeToVault}
    onCopy={handleCopyToVaultAsNew}
    onCancel={cancelMoveToVault}
  />
{/if}

<style>
  .group-list {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--sidebar-bg);
    border-right: 1px solid var(--border-color);
    padding: 0;
    overflow-y: auto;
    text-align: left;
  }

  .group-row {
    display: flex;
    align-items: center;
    cursor: grab;
  }

  .group-row:hover {
    background-color: var(--hover-bg);
  }

  .group-row.selected {
    background-color: var(--selected-bg);
  }

  .group-row.dragging {
    cursor: grabbing;
    opacity: 0.6;
  }

  .group-row.dragging .btn-icon-danger {
    display: none;
  }

  .group-row.drop-before {
    border-top: 2px solid var(--accent-color);
  }

  .group-row.drop-after {
    border-bottom: 2px solid var(--accent-color);
  }

  .group-row.selected .group-item,
  .group-row.selected .btn-icon-danger {
    color: var(--selected-text);
  }

  .group-row:not(.selected) .group-item {
    opacity: 0.85;
  }

  .group-row:not(.selected):hover .group-item {
    opacity: 0.8;
  }

  .group-item {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 0.5rem;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
    background: transparent;
    border: none;
    border-radius: 0;
    cursor: pointer;
    text-align: left;
    font-size: 0.875rem;
    font-weight: 400;
    color: var(--text-color);
    line-height: 1.5;
  }

  .btn-icon-danger {
    display: none;
  }

  .group-row:hover .btn-icon-danger {
    display: inline-flex;
  }

  .group-icon {
    font-size: 1rem;
    opacity: 0.8;
  }

  .group-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0 0.5rem;
  }

  :global(.chip:hover) {
    background-color: var(--accent-color);
    color: #ffffff;
    border-color: var(--accent-color);
  }

  .empty {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }
</style>
