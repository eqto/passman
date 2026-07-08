<script>
  export let node;
  export let id;
  export let depth;
  export let hasChildren;
  export let isCollapsed;
  export let selected;
  export let toggle;
  export let onSelectTrashGroup;
</script>

<div
  class="group-row"
  class:selected
  class:has-tree-line={depth > 0}
  role="listitem"
  style="padding-left: {depth * 1.5}rem"
>
  <div
    class="group-item"
    role="button"
    tabindex="0"
    on:click={() => onSelectTrashGroup(id)}
    on:keydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        onSelectTrashGroup(id);
      }
    }}
  >
    {#if hasChildren}
      <span
        class="expand-icon"
        class:collapsed={isCollapsed}
        role="button"
        tabindex="0"
        aria-label={isCollapsed ? "Expand group" : "Collapse group"}
        on:click|stopPropagation={(e) => toggle()}
        on:keydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            e.stopPropagation();
            toggle();
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
    <span class="group-icon folder-icon">
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
    </span>
    <span class="group-name">{node.group.name}</span>
  </div>
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

  .group-row.selected {
    background-color: var(--selected-bg);
  }

  .group-row.selected .group-item {
    color: var(--selected-text);
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
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
