<script>
  import { groups, tags, vaults, currentVault, vaultData, setVaultViewState } from "../stores/vaults";
  import { showToast } from "../stores/toast.js";
  import {
    addGroup,
    addTag,
    deleteGroup,
    reorderGroups,
    mergeGroups,
    moveGroupToVault,
    copyGroupToVault,
  } from "../stores/groups";
  import { moveEntriesWithTagToGroup } from "../stores/entries";
  import AddGroupDialog from "./AddGroupDialog.svelte";
  import DeleteGroupDialog from "./DeleteGroupDialog.svelte";
  import GroupTagContextMenu from "./GroupTagContextMenu.svelte";
  import GroupVaultMoveDialog from "./GroupVaultMoveDialog.svelte";
  import { createDragList } from "../lib/dragList.js";

  export let selectedGroup = "";
  export let selectedTrashGroup = "";
  export let trashMode = false;
  export let trashGroups = [];
  export let onSelectGroup;
  export let onSelectTrashGroup;
  export let onTrashClick;

  let showAdd = false;
  let showAddTag = false;
  let deleteTarget = null;
  let contextMenu = { show: false, x: 0, y: 0, type: "tag", item: "" };
  let moveToVaultTarget = null;
  let moveToVaultGroup = "";
  let moveToVaultAction = "move";

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
    const { source, targetPath } = event.detail;
    const target = $vaults.find((v) => v.path === targetPath);
    const targetGroups = $vaultData[targetPath]?.groups || [];
    if (target && targetGroups.includes(source)) {
      moveToVaultGroup = source;
      moveToVaultTarget = target;
      moveToVaultAction = "move";
      closeContextMenu();
    } else if (target) {
      try {
        await moveGroupToVault(source, targetPath, source);
        switchToVaultAndGroup(target, source);
        showToast(`Moved "${source}" to ${target.name}`);
      } catch (e) {
        console.error(e);
        alert(`Move failed: ${e}`);
      }
      closeContextMenu();
    }
  }

  async function handleCopyToVault(event) {
    const { source, targetPath } = event.detail;
    const target = $vaults.find((v) => v.path === targetPath);
    const targetGroups = $vaultData[targetPath]?.groups || [];
    if (target && targetGroups.includes(source)) {
      moveToVaultGroup = source;
      moveToVaultTarget = target;
      moveToVaultAction = "copy";
      closeContextMenu();
    } else if (target) {
      try {
        await copyGroupToVault(source, targetPath, source);
        switchToVaultAndGroup(target, source);
        showToast(`Copied "${source}" to ${target.name}`);
      } catch (e) {
        console.error(e);
        alert(`Copy failed: ${e}`);
      }
      closeContextMenu();
    }
  }

  async function handleMergeToVault() {
    if (moveToVaultTarget && moveToVaultGroup) {
      try {
        if (moveToVaultAction === "copy") {
          await copyGroupToVault(moveToVaultGroup, moveToVaultTarget.path, moveToVaultGroup);
          switchToVaultAndGroup(moveToVaultTarget, moveToVaultGroup);
          showToast(`Copied "${moveToVaultGroup}" into ${moveToVaultTarget.name}`);
        } else {
          await moveGroupToVault(moveToVaultGroup, moveToVaultTarget.path, moveToVaultGroup);
          switchToVaultAndGroup(moveToVaultTarget, moveToVaultGroup);
          showToast(`Moved "${moveToVaultGroup}" into ${moveToVaultTarget.name}`);
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

  async function handleCopyToVaultAsNew(newName) {
    if (moveToVaultTarget && moveToVaultGroup) {
      try {
        if (moveToVaultAction === "copy") {
          await copyGroupToVault(moveToVaultGroup, moveToVaultTarget.path, newName);
          switchToVaultAndGroup(moveToVaultTarget, newName);
          showToast(`Copied "${moveToVaultGroup}" to ${moveToVaultTarget.name} as "${newName}"`);
        } else {
          await moveGroupToVault(moveToVaultGroup, moveToVaultTarget.path, newName);
          switchToVaultAndGroup(moveToVaultTarget, newName);
          showToast(`Moved "${moveToVaultGroup}" to ${moveToVaultTarget.name} as "${newName}"`);
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
        class="back-btn"
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
          <button
            class="group-item"
            on:click={() => onSelectTrashGroup(group)}
          >
            <span class="group-icon">📁</span>
            <span class="group-name">{group}</span>
          </button>
        </div>
      {/each}
    {/if}
  {:else}
    <div class="group-header section-header">
      <span>Groups</span>
      <button class="add-group-btn" title="New group" on:click={() => showAdd = true}>
        +
      </button>
    </div>

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
          <button
            class="group-item"
            on:click={() => onSelectGroup(group)}
            on:contextmenu={(e) => openContextMenu(e, "group", group)}
          >
            <span class="group-icon">📁</span>
            <span class="group-name">{group}</span>
          </button>
          <button
            class="delete-group-btn"
            title="Delete group"
            on:click={() => deleteTarget = group}
          >
            ×
          </button>
        </div>
      {/each}
    {/if}

    <div class="tags-header section-header">
      <span>Tags</span>
      <button class="add-group-btn" title="New tag" on:click={() => showAddTag = true}>
        +
      </button>
    </div>

    {#if $tags.length === 0}
      <p class="empty-state">No tags.</p>
    {:else}
      <div class="tags">
        {#each $tags as tag}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <span class="tag-chip" on:contextmenu={(e) => openContextMenu(e, "tag", tag)}>
            {tag}
          </span>
        {/each}
      </div>
    {/if}

    <div class="trash-header section-header">
      <span>Trash</span>
    </div>
    <div class="group-row trash-row" class:selected={trashMode}>
      <button class="group-item" on:click={onTrashClick}>
        <span class="group-icon">🗑️</span>
        <span class="group-name">Trash</span>
      </button>
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
    padding: 1rem 0;
    overflow-y: auto;
  }

  .group-header,
  .tags-header,
  .trash-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1rem;
    margin-bottom: 0.5rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--muted-color);
    letter-spacing: 0.05em;
  }

  .tags-header,
  .trash-header {
    margin-top: 1rem;
  }

  .trash-header {
    justify-content: flex-start;
    gap: 0.5rem;
  }

  .back-btn {
    background: transparent;
    border: none;
    color: var(--muted-color);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0.25rem;
    border-radius: 0.25rem;
  }

  .back-btn:hover {
    color: var(--text-color);
    background-color: var(--hover-bg);
  }

  .add-group-btn {
    background: transparent;
    border: none;
    color: var(--muted-color);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0.25rem;
    border-radius: 0.25rem;
  }

  .add-group-btn:hover {
    color: var(--text-color);
    background-color: var(--hover-bg);
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

  .group-row.dragging .delete-group-btn {
    display: none;
  }

  .group-row.drop-before {
    border-top: 2px solid var(--selected-bg);
  }

  .group-row.drop-after {
    border-bottom: 2px solid var(--selected-bg);
  }

  .group-row.selected .group-item,
  .group-row.selected .delete-group-btn {
    color: var(--selected-text);
  }

  .group-item {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    color: var(--text-color);
    cursor: pointer;
    text-align: left;
    font-size: 0.875rem;
  }

  .delete-group-btn {
    display: none;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    margin-right: 0.75rem;
    padding: 0;
    background: transparent;
    border: none;
    color: var(--muted-color);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    border-radius: 0.25rem;
  }

  .group-row:hover .delete-group-btn {
    display: flex;
  }

  .delete-group-btn:hover {
    background-color: rgba(239, 68, 68, 0.15);
    color: var(--danger-color);
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
    padding: 0 1rem;
  }

  .tag-chip {
    padding: 0.25rem 0.75rem;
  }

  .empty {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }
</style>
