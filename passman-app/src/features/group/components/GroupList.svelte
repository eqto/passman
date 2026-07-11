<script>
  import {
    groups,
    tags,
    vaults,
    currentVault,
    vaultData,
    setVaultViewState,
  } from "../../vault/store.js";
  import { showToast } from "../../../stores/toast.js";
  import { closeAllContextMenus } from "../../../stores/contextMenu.js";
  import { onMount } from "svelte";
  import {
    addGroup,
    addTag,
    deleteGroup,
    reorderGroups,
    mergeGroups,
    moveGroupToVault,
    copyGroupToVault,
    moveGroupToParent,
  } from "../store.js";
  import { moveEntriesWithTagToGroup } from "../../entry/store.js";
  import {
    AddGroupDialog,
    DeleteGroupDialog,
    GroupTagContextMenu,
    GroupVaultMoveDialog,
    GroupTitle,
    TrashSidebar,
    TagSidebar,
  } from "../index";
  import Tree from "../../../components/Tree.svelte";
  import { buildTree } from "../groupTree.js";

  export let selectedGroup = "";
  export let selectedTags = [];
  export let selectedTrashGroup = "";
  export let trashMode = false;
  export let trashGroups = [];
  export let hasUngroupedTrashEntries = false;
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
    window.addEventListener("close-all-context-menus", closeContextMenu);
    return () => {
      window.removeEventListener("close-all-context-menus", closeContextMenu);
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

  async function onReorder(items, { source, target } = {}) {
    const normalizeParent = (id) => (id && id !== "0" ? id : null);
    const sourceParent = normalizeParent(source?.parent_id);
    const targetParent = normalizeParent(target?.parent_id);
    if (source && target && sourceParent !== targetParent) {
      if (isDescendant(target.id, source.id, items)) {
        showToast("Cannot move a group into its own descendant");
        return;
      }
      // Step 1: update parent_id in backend first
      const updatedGroups = await moveGroupToParent(source.id, targetParent);
      if (!updatedGroups) return;
      // Step 2: reorder to match desired position (items is already in desired order)
      const reordered = items.map(
        (item) => updatedGroups.find((g) => g.id === item.id) ?? item,
      );
      await reorderGroups(reordered);
      return;
    }
    reorderGroups(items);
  }

  function onDropInto({ source, target }) {
    handleDropInto(source, target);
  }

  async function handleAddGroup(name) {
    await addGroup({ id: crypto.randomUUID(), name, parent_id: null });
    showAdd = false;
  }

  async function handleAddTag(name) {
    await addTag(name);
    showAddTag = false;
  }

  async function handleDeleteGroup(group) {
    await deleteGroup(group.id);
    if (selectedGroup === group.id) {
      onSelectGroup("");
    }
    deleteTarget = null;
  }

  function openContextMenu(event, type, item) {
    event.preventDefault();
    closeAllContextMenus();
    contextMenu = {
      show: true,
      x: event.clientX,
      y: event.clientY,
      type,
      item,
    };
  }

  function getGroupName(groupId) {
    const group = $groups.find((g) => g.id === groupId);
    return group ? group.name : groupId;
  }

  $: groupTree = buildTree($groups);

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, type: "tag", item: "" };
  }

  async function handleMoveToGroup(event) {
    const { item, target } = event.detail;
    await moveEntriesWithTagToGroup(item, target);
    closeContextMenu();
  }

  async function handleMergeGroup(event) {
    const { sourceId, targetId } = event.detail;
    await mergeGroups(sourceId, targetId);
    if (selectedGroup === sourceId) {
      onSelectGroup(targetId);
    }
    closeContextMenu();
  }

  async function handleMoveToVault(event) {
    await handleVaultAction(event, "move");
  }

  async function handleCopyToVault(event) {
    await handleVaultAction(event, "copy");
  }

  function handleMoveToTrash(event) {
    const group = $groups.find((g) => g.id === event.detail.groupId);
    if (group) {
      deleteTarget = group;
    }
    closeContextMenu();
  }

  async function handleVaultAction(event, action) {
    const { sourceId, targetPath } = event.detail;
    const target = $vaults.find((v) => v.path === targetPath);
    const targetGroups = ($vaultData[targetPath]?.groups || []).map(
      (g) => g.id,
    );
    if (target && targetGroups.includes(sourceId)) {
      moveToVaultGroup = sourceId;
      moveToVaultTarget = target;
      moveToVaultAction = action;
      closeContextMenu();
    } else if (target) {
      try {
        const fn = action === "copy" ? copyGroupToVault : moveGroupToVault;
        await fn(sourceId, targetPath, sourceId);
        switchToVaultAndGroup(target, sourceId);
        showToast(
          `${action === "copy" ? "Copied" : "Moved"} "${getGroupName(sourceId)}" to ${target.name}`,
        );
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

  async function handleVaultResolve(targetId) {
    if (moveToVaultTarget && moveToVaultGroup) {
      try {
        const fn =
          moveToVaultAction === "copy" ? copyGroupToVault : moveGroupToVault;
        await fn(moveToVaultGroup, moveToVaultTarget.path, targetId);
        switchToVaultAndGroup(moveToVaultTarget, targetId);
        const verb = moveToVaultAction === "copy" ? "Copied" : "Moved";
        if (targetId === moveToVaultGroup) {
          showToast(
            `${verb} "${getGroupName(moveToVaultGroup)}" into ${moveToVaultTarget.name}`,
          );
        } else {
          showToast(
            `${verb} "${getGroupName(moveToVaultGroup)}" to ${moveToVaultTarget.name} as "${getGroupName(targetId)}"`,
          );
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

  function isDescendant(groupId, potentialParentId, groups) {
    if (groupId === potentialParentId) return true;
    const group = groups.find((g) => g.id === groupId);
    if (!group || !group.parent_id) return false;
    return isDescendant(group.parent_id, potentialParentId, groups);
  }

  async function handleDropInto(source, target) {
    const sourceId = source.id;
    const targetId = target.id;

    // Prevent dropping into itself
    if (sourceId === targetId) return;

    // Prevent circular reference
    if (isDescendant(targetId, sourceId, $groups)) {
      showToast("Cannot drop a group into its own descendant");
      return;
    }

    try {
      await moveGroupToParent(sourceId, targetId);
    } catch (e) {
      console.error("Failed to move group to parent:", e);
      showToast("Failed to move group");
    }
  }
</script>

<svelte:window on:click={closeContextMenu} />

<div class="group-list">
  {#if trashMode}
    <TrashSidebar
      {trashGroups}
      {selectedTrashGroup}
      {hasUngroupedTrashEntries}
      {selectedGroup}
      groups={$groups}
      {onSelectTrashGroup}
      {onSelectGroup}
    />
  {:else}
    <div class="group-list-main">
      <GroupTitle
        title="Groups"
        showButton={true}
        onButtonClick={() => (showAdd = true)}
      />

      {#if $groups.length === 0}
        <p class="empty-state">No groups.</p>
      {:else}
        <Tree
          nodes={groupTree}
          selectedId={selectedGroup}
          onSelect={onSelectGroup}
          onContextMenu={(e, id) => openContextMenu(e, "group", id)}
          items={$groups}
          {onReorder}
          {onDropInto}
        />
      {/if}

      <TagSidebar
        tags={$tags}
        {selectedTags}
        {onSelectTag}
        onAddTag={() => (showAddTag = true)}
        onContextMenu={(e, tag) => openContextMenu(e, "tag", tag)}
      />
    </div>

    <div class="trash-row-container">
      <div class="group-row trash-row" class:selected={trashMode}>
        <div
          class="group-item"
          role="button"
          tabindex="0"
          on:click={onTrashClick}
          on:keydown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              onTrashClick();
            }
          }}
        >
          <span class="group-icon trash-icon">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><polyline points="3 6 5 6 21 6"></polyline><path
                d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
              ></path></svg
            >
          </span>
          <span class="group-name">Trash</span>
        </div>
      </div>
    </div>
  {/if}
</div>

{#if showAdd}
  <AddGroupDialog onAdd={handleAddGroup} onCancel={() => (showAdd = false)} />
{/if}

{#if showAddTag}
  <AddGroupDialog
    title="Add Tag"
    onAdd={handleAddTag}
    onCancel={() => (showAddTag = false)}
  />
{/if}

{#if deleteTarget}
  <DeleteGroupDialog
    group={deleteTarget}
    onDelete={handleDeleteGroup}
    onCancel={() => (deleteTarget = null)}
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
    on:moveToTrash={handleMoveToTrash}
  />
{/if}

{#if moveToVaultTarget}
  <GroupVaultMoveDialog
    groupId={moveToVaultGroup}
    groupName={getGroupName(moveToVaultGroup)}
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
    overflow: hidden;
    text-align: left;
  }

  .group-list-main {
    flex: 1;
    overflow-y: auto;
    min-height: 0;
  }

  .trash-row-container {
    flex-shrink: 0;
    border-top: 1px solid var(--border-color);
    background-color: var(--sidebar-bg);
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

  .group-row.selected {
    background-color: var(--selected-bg);
  }

  .group-row.selected .group-item {
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
    padding: 0.5rem;
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

  .group-icon {
    font-size: 1rem;
    opacity: 0.8;
  }

  .trash-icon {
    display: inline-flex;
    align-items: center;
    vertical-align: middle;
    transform: translateY(-2px);
    color: var(--muted-color);
    opacity: 0.9;
  }

  .group-row.selected .trash-icon {
    color: var(--selected-text);
  }

  .trash-row .group-item {
    gap: 0.35rem;
    padding: 1rem 0.5rem;
  }

  .trash-row .group-name {
    transform: translateY(1px);
  }

  .group-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .empty {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }
</style>
