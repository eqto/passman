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
    on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); onSelectGroup(group.id); } }}
    on:contextmenu={(e) => onContextMenu(e, group.id)}
  >
    <span class="group-name">{group.name}</span>
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
    position: relative;
    padding-left: 1.5rem;
  }

  .group-row.has-tree-line::before {
    content: '';
    position: absolute;
    left: 0.75rem;
    top: 0;
    bottom: 0;
    width: 1px;
    border-left: 2px solid var(--border-color);
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
</style>
