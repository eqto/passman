<script>
  import { createDragList } from "./drag";

  let {
    items = [],
    getKey = (x) => x,
    getTitle = null,
    selectedKey = null,
    axis = "horizontal",
    onReorder = null,
    onSelect = null,
    onKeydown = null,
    onContextMenu = null,
    itemTab, // snippet: (item) => tab content
    itemActions, // snippet: (item) => action buttons
  } = $props();

  const drag = createDragList({
    axis,
    getKey,
    onReorder: (reordered) => onReorder?.(reordered),
  });
  const { dragItem, dropTarget } = drag;

  function handleDragStart(e, item) {
    drag.dragStart(e, item);
  }

  function handleDragEnd() {
    drag.dragEnd();
  }

  function handleDragOver(e, item) {
    drag.handleDragOver(e, item);
  }

  function handleDragLeave() {
    drag.dragLeave();
  }

  function handleDrop(e, item) {
    drag.drop(e, items, item);
  }

  function handleClick(item) {
    onSelect?.(item);
  }

  function handleKeydown(e, item) {
    onKeydown?.(e, item);
  }

  function handleContextmenu(e, item) {
    e.preventDefault();
    onContextMenu?.(e, item);
  }
</script>

{#each items as item (getKey(item))}
  <div
    class="tab"
    class:selected={selectedKey === getKey(item)}
    class:dragging={$dragItem === item}
    class:drop-before={$dropTarget?.type === "before" &&
      getKey($dropTarget.item) === getKey(item)}
    class:drop-after={$dropTarget?.type === "after" &&
      getKey($dropTarget.item) === getKey(item)}
    role="button"
    tabindex="0"
    draggable="true"
    ondragstart={(e) => handleDragStart(e, item)}
    ondragend={handleDragEnd}
    ondragover={(e) => handleDragOver(e, item)}
    ondragleave={handleDragLeave}
    ondrop={(e) => handleDrop(e, item)}
    onclick={() => handleClick(item)}
    onkeydown={(e) => handleKeydown(e, item)}
    oncontextmenu={(e) => handleContextmenu(e, item)}
    title={getTitle ? getTitle(item) : undefined}
  >
    <span class="tab-name">
      {#if itemTab}
        {@render itemTab(item)}
      {:else}
        {item.label ?? item.name ?? String(item)}
      {/if}
    </span>
    {#if itemActions}
      <span class="tab-actions-inner" onclick={(e) => e.stopPropagation()}>
        {@render itemActions(item)}
      </span>
    {/if}
  </div>
{/each}

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

  .tab-actions-inner {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
</style>
