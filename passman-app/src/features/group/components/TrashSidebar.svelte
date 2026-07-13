<script>
  import Tree from "../../../components/Tree.svelte";
  import { buildTree } from "../groupTree.js";

  export let trashGroups = [];
  export let selectedTrashGroup = "";
  export let hasUngroupedTrashEntries = false;
  export let selectedGroup = "";
  export let groups = [];
  export let onSelectTrashGroup;
  export let onSelectGroup;

  $: trashGroupTree = buildTree(trashGroups);
</script>

<div class="trash-header">
  <button
    class="btn-icon"
    title="Back to groups"
    onclick={() => onSelectGroup(selectedGroup || (groups[0]?.id ?? ""))}
  >
    ←
  </button>
  <span>Trash</span>
</div>

{#if trashGroups.length === 0 && !hasUngroupedTrashEntries}
  <p class="empty-state">No deleted items.</p>
{:else}
  <Tree
    nodes={trashGroupTree}
    selectedId={selectedTrashGroup}
    onSelect={onSelectTrashGroup}
  />
  {#if hasUngroupedTrashEntries}
    <div
      class="group-row"
      class:selected={selectedTrashGroup === "__ungrouped__"}
      role="listitem"
    >
      <div
        class="group-item"
        role="button"
        tabindex="0"
        onclick={() => onSelectTrashGroup("__ungrouped__")}
        onkeydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            onSelectTrashGroup("__ungrouped__");
          }
        }}
      >
        <span class="group-icon">📄</span>
        <span class="group-name">Ungrouped</span>
      </div>
    </div>
  {/if}
{/if}

<style>
  .trash-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
    margin-bottom: 0.5rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--muted-color);
    letter-spacing: 0.05em;
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

  .group-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
