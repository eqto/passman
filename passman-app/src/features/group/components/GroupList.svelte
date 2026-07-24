<script>
  import { vaults, vaultData } from "../../vault/index.js";
  import { createContextMenuState } from "../../../lib/createContextMenu.svelte.js";
  import { createGroupVaultMove } from "../groupVaultMove.svelte.js";
  import { addGroup, deleteGroup, mergeGroups } from "../store.js";
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

  let {
    vault,
    selectedGroup = "",
    selectedTags = [],
    selectedTrashGroup = "",
    trashMode = false,
    trashGroups = [],
    hasUngroupedTrashEntries = false,
    onSelectGroup,
    onSelectTag = (tag) => {},
    onSelectTrashGroup,
    onTrashClick,
  } = $props();

  const vaultPath = vault.path;

  let vaultGroups = $derived($vaultData[vaultPath]?.groups || []);
  let vaultTags = $derived.by(() => {
    const allEntries = $vaultData[vaultPath]?.entries || [];
    const filtered = selectedGroup
      ? allEntries.filter((e) => e.group_id === selectedGroup)
      : allEntries;
    const set = new Set();
    for (const entry of filtered) {
      for (const tag of entry.tags || []) {
        set.add(tag);
      }
    }
    return Array.from(set).sort((a, b) => a.localeCompare(b));
  });

  let showAdd = $state(false);
  let deleteTarget = $state(null);
  const {
    state: contextMenu,
    open: openContextMenu,
    close: closeContextMenu,
  } = createContextMenuState({ type: "tag", item: "" });
  const vaultMove = createGroupVaultMove(vaultPath, getGroupName);

  let moveVaults = $derived(
    ($vaults || []).filter((v) => v.path !== vaultPath),
  );

  async function handleAddGroup(name) {
    await addGroup({ id: crypto.randomUUID(), name, parent_id: null });
    showAdd = false;
  }

  async function handleDeleteGroup(group) {
    await deleteGroup(group.id);
    if (selectedGroup === group.id) {
      onSelectGroup("");
    }
    deleteTarget = null;
  }

  function openContextMenuEvent(event, type, item) {
    openContextMenu(event, { type, item });
  }

  function getGroupName(groupId) {
    const group = vaultGroups.find((g) => g.id === groupId);
    return group ? group.name : groupId;
  }

  let groupTree = $derived(buildTree(vaultGroups));

  async function handleMoveToGroup(detail) {
    const { item, target } = detail;
    await moveEntriesWithTagToGroup(item, target);
    closeContextMenu();
  }

  async function handleMergeGroup(detail) {
    const { sourceId, targetId } = detail;
    await mergeGroups(sourceId, targetId);
    if (selectedGroup === sourceId) {
      onSelectGroup(targetId);
    }
    closeContextMenu();
  }

  function handleMoveToTrash(detail) {
    const group = vaultGroups.find((g) => g.id === detail.groupId);
    if (group) {
      deleteTarget = group;
    }
    closeContextMenu();
  }

  async function handleVaultAction(detail, action) {
    await vaultMove.handleVaultAction(detail, action, closeContextMenu);
  }

  async function handleVaultResolve(targetId) {
    await vaultMove.handleResolve(targetId, closeContextMenu);
  }

  function cancelMoveToVault() {
    vaultMove.reset();
  }
</script>

<svelte:window onclick={closeContextMenu} />

<div class="group-list">
  {#if trashMode}
    <TrashSidebar
      {trashGroups}
      {selectedTrashGroup}
      {hasUngroupedTrashEntries}
      {selectedGroup}
      groups={vaultGroups}
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

      {#if vaultGroups.length === 0}
        <p class="empty-state">No groups.</p>
      {:else}
        <Tree
          nodes={groupTree}
          selectedId={selectedGroup}
          highlightedId={contextMenu.show && contextMenu.type === "group"
            ? contextMenu.item
            : ""}
          onSelect={onSelectGroup}
          onContextMenu={(e, id) => openContextMenuEvent(e, "group", id)}
          items={vaultGroups}
          onReorder={onReorderGroups}
          onDropInto={({ source, target }) =>
            handleDropInto(source, target, vaultGroups)}
        />
      {/if}
    </div>

    <div class="tag-sidebar-container">
      <TagSidebar
        tags={vaultTags}
        {selectedTags}
        {onSelectTag}
        onContextMenu={(e, tag) => openContextMenuEvent(e, "tag", tag)}
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
    groups={vaultGroups}
    vaults={moveVaults}
    onmovetogroup={handleMoveToGroup}
    onmergegroup={handleMergeGroup}
    onmovetovault={(e) => handleVaultAction(e, "move")}
    oncopytovault={(e) => handleVaultAction(e, "copy")}
    onmovetotrash={handleMoveToTrash}
  />
{/if}

{#if vaultMove.moveToVaultTarget}
  <GroupVaultMoveDialog
    groupId={vaultMove.moveToVaultGroup}
    groupName={getGroupName(vaultMove.moveToVaultGroup)}
    vaultName={vaultMove.moveToVaultTarget.name}
    action={vaultMove.moveToVaultAction}
    onMerge={() => handleVaultResolve(vaultMove.moveToVaultGroup)}
    onCopy={(targetId) => handleVaultResolve(targetId)}
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

  .tag-sidebar-container {
    flex-shrink: 0;
    border-top: 1px solid var(--border-color);
    background-color: var(--sidebar-bg);
  }
</style>
