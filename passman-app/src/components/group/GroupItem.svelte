<script>
  export let group;
  export let selectedGroup = "";
  export let depth = 0;
  export let onSelectGroup;
  export let onContextMenu;
  export let onDelete;
  export let dragItem;
  export let dragOver;
  export let insertBefore;
  export let dragStart;
  export let dragEnd;
  export let handleDragOver;
  export let dragLeave;
  export let drop;
  export let flatGroups = [];
  export let hasChildren = false;
  export let isCollapsed = false;
  export let toggleGroup = () => {};
  export let hasAnyChildren = false;
</script>

<div
  class="group-row"
  class:has-tree-line={depth > 0}
  class:selected={selectedGroup === group.id}
  class:dragging={$dragItem === group.id}
  class:drop-before={$dragOver === group.id && $insertBefore === true}
  class:drop-after={$dragOver === group.id && $insertBefore === false}
  role="listitem"
  aria-grabbed={$dragItem === group.id}
  draggable={true}
  on:dragstart={(e) => dragStart(e, group)}
  on:dragend={dragEnd}
  on:dragover={(e) => handleDragOver(e, group)}
  on:dragleave={dragLeave}
  on:drop={(e) => drop(e, flatGroups, group)}
  style="padding-left: {depth * 1.5}rem"
>
  <div
    class="group-item"
    role="button"
    tabindex="0"
    on:click={() => onSelectGroup(group.id)}
    on:keydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        onSelectGroup(group.id);
      }
    }}
    on:contextmenu={(e) => onContextMenu(e, group.id)}
  >
    {#if hasAnyChildren}
      {#if hasChildren}
        <span
          class="expand-icon"
          class:collapsed={isCollapsed}
          role="button"
          tabindex="0"
          aria-label={isCollapsed ? "Expand group" : "Collapse group"}
          on:click|stopPropagation={(e) => toggleGroup(group.id)}
          on:keydown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              e.stopPropagation();
              toggleGroup(group.id);
            }
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            ><polyline points="9 18 15 12 9 6"></polyline></svg
          >
        </span>
      {:else}
        <span class="expand-icon-spacer"></span>
      {/if}
    {/if}
    <div class="group-icon folder-icon">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><path
          d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
        /></svg
      >
    </div>
    <div class="group-name">{group.name}</div>
  </div>
  <button
    class="btn-icon-danger"
    title="Delete group"
    on:click={() => onDelete(group)}
  >
    ×
  </button>
</div>

<style>
  .group-row {
    display: flex;
    align-items: center;
    cursor: grab;
  }

  .group-row:hover {
    background-color: var(--hover-bg);
  }

  .group-row.has-tree-line {
    padding-left: 1.5rem;
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

  .group-row.selected {
    background-color: var(--selected-bg);
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
    gap: 0.35rem;
    padding: 0.5rem;
    cursor: pointer;
  }

  .btn-icon-danger {
    display: none;
  }

  .group-row:hover .btn-icon-danger {
    display: inline-flex;
  }

  .expand-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    color: var(--muted-color);
    cursor: pointer;
    transition: transform 0.2s ease;
    transform: rotate(90deg);
  }

  .expand-icon-spacer {
    display: inline-flex;
    width: 0.75rem;
    height: 1rem;
  }

  .expand-icon.collapsed {
    transform: rotate(0deg);
  }

  .expand-icon:hover {
    color: var(--text-color);
  }

  .group-row.selected .expand-icon {
    color: var(--selected-text);
  }

  .folder-icon {
    display: flex;
    align-items: center;
    color: var(--muted-color);
  }

  .group-row.selected .folder-icon {
    color: var(--selected-text);
  }

  .group-name {
    padding-top: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
