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
  import { useContextMenu } from "../../../lib/createContextMenu.js";
  import {
    addGroup,
    addTag,
    deleteGroup,
    mergeGroups,
    moveGroupToVault,
    copyGroupToVault,
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
  import TrashRow from "./TrashRow.svelte";
  import { buildTree } from "../groupTree.js";
  import { onReorderGroups, handleDropInto } from "../groupActions.js";

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

  useContextMenu(closeContextMenu);

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

  function resetMoveToVault() {
    moveToVaultTarget = null;
    moveToVaultGroup = "";
    moveToVaultAction = "move";
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
    resetMoveToVault();
    closeContextMenu();
  }

  function cancelMoveToVault() {
    resetMoveToVault();
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
          highlightedId={contextMenu.show && contextMenu.type === "group"
            ? contextMenu.item
            : ""}
          onSelect={onSelectGroup}
          onContextMenu={(e, id) => openContextMenu(e, "group", id)}
          items={$groups}
          onReorder={onReorderGroups}
          onDropInto={({ source, target }) =>
            handleDropInto(source, target, $groups)}
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
      <TrashRow {trashMode} {onTrashClick} />
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
    on:moveToVault={(e) => handleVaultAction(e, "move")}
    on:copyToVault={(e) => handleVaultAction(e, "copy")}
    on:moveToTrash={handleMoveToTrash}
  />
{/if}

{#if moveToVaultTarget}
  <GroupVaultMoveDialog
    groupId={moveToVaultGroup}
    groupName={getGroupName(moveToVaultGroup)}
    vaultName={moveToVaultTarget.name}
    action={moveToVaultAction}
    onMerge={() => handleVaultResolve(moveToVaultGroup)}
    onCopy={(e) => handleVaultResolve(e.detail)}
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
</style>
