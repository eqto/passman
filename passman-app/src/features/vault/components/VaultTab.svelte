<script>
  import { LockIcon } from "../../../components/icons";

  export let vault;
  export let selected = false;
  export let dragging = false;
  export let dropBefore = false;
  export let dropAfter = false;
  export let unlocked = false;
  export let onSelect;
  export let onLock;
  export let onRemove;
  export let onContextMenu;
  export let onDragStart;
  export let onDragEnd;
  export let onDragOver;
  export let onDragLeave;
  export let onDrop;
  export let onKeydown;
</script>

<div
  class="tab"
  class:selected
  class:dragging
  class:drop-before={dropBefore}
  class:drop-after={dropAfter}
  role="button"
  tabindex="0"
  draggable={true}
  on:dragstart={onDragStart}
  on:dragend={onDragEnd}
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
  on:click={() => onSelect(vault)}
  on:keydown={(e) => onKeydown(e, vault)}
  on:contextmenu|preventDefault={(e) => onContextMenu(e, vault)}
  title={vault.path}
>
  <span class="tab-name">{vault.name}</span>
  {#if unlocked}
    <button
      class="btn-icon tab-action-btn lock-tab-btn"
      on:click|stopPropagation={() => onLock(vault)}
      title="Lock vault"
    >
      <LockIcon size={18} />
    </button>
  {:else}
    <button
      class="btn-icon tab-action-btn delete-tab-btn"
      on:click|stopPropagation={() => onRemove(vault)}
      title="Remove vault"
    >
      ×
    </button>
  {/if}
</div>

<style>
  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.5rem 0.5rem 0.75rem;
    border: none;
    border-radius: var(--shape-sm);
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    white-space: nowrap;
  }

  .tab:hover {
    background-color: var(--hover-bg);
  }

  .tab.selected {
    background-color: var(--selected-bg);
    color: var(--selected-text);
  }

  .tab.dragging {
    cursor: grabbing;
    opacity: 0.6;
  }

  .tab.drop-before {
    border-left: 2px solid var(--accent-color);
  }

  .tab.drop-after {
    border-right: 2px solid var(--accent-color);
  }

  .tab-name {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 12rem;
  }

  .tab-action-btn {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border-radius: var(--shape-full);
  }

  .lock-tab-btn {
    padding: 0.25rem;
  }

  .tab.selected .lock-tab-btn {
    color: var(--selected-text);
    background-color: transparent;
  }

  .tab.selected .lock-tab-btn:hover {
    color: var(--selected-text);
    background-color: var(--hover-bg);
  }

  .delete-tab-btn:hover {
    color: var(--on-danger-container);
    background-color: var(--danger-container);
  }
</style>
