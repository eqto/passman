<script>
  let {
    tab,
    selected = false,
    dragging = false,
    dropBefore = false,
    dropAfter = false,
    onSelect,
    onKeydown,
    onContextMenu,
    onClose = null,
    onDragStart,
    onDragEnd,
    onDragOver,
    onDragLeave,
    onDrop,
  } = $props();
</script>

<div
  class="tab"
  class:selected
  class:dragging
  class:drop-before={dropBefore}
  class:drop-after={dropAfter}
  role="button"
  tabindex="0"
  draggable="true"
  title={tab.title}
  ondragstart={onDragStart}
  ondragend={onDragEnd}
  ondragover={onDragOver}
  ondragleave={onDragLeave}
  ondrop={onDrop}
  onclick={onSelect}
  onkeydown={onKeydown}
  oncontextmenu={onContextMenu}
>
  <span class="tab-name">{tab.name}</span>
  {#if onClose}
    <span
      class="tab-actions"
      aria-hidden="true"
      onclick={(e) => e.stopPropagation()}
    >
      <button
        class="tab-action-btn delete-tab-btn"
        onclick={(e) => {
          e.stopPropagation();
          onClose(tab.id);
        }}
        title="Close"
      >
        ×
      </button>
    </span>
  {/if}
</div>

<style>
  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.5rem 0.5rem 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: var(--shape-sm);
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    white-space: nowrap;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .tab:hover {
    background-color: var(--hover-bg);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  }

  .tab.selected {
    background-color: var(--selected-bg);
    color: var(--selected-text);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
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
    line-height: 1.25;
  }

  .tab-actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .tab-action-btn {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border-radius: var(--shape-full);
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: inherit;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
  }

  .delete-tab-btn:hover {
    color: var(--on-danger-container);
    background-color: var(--danger-container);
  }
</style>
