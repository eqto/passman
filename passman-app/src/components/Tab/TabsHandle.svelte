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
  <span class="tab-name">
    {#if tab.content}
      {@render tab.content()}
    {:else}
      {tab.name}
    {/if}
  </span>
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
    line-height: 1.25;
  }
</style>
