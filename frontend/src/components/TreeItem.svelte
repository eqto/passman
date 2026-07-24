<script>
  import { Icon } from "./icons";

  let {
    node,
    id,
    depth,
    hasChildren,
    isCollapsed,
    selected,
    highlighted = false,
    toggle,
    onSelect,
    onContextMenu = null,
    dragItem = null,
    dropTarget = null,
    dragStart = null,
    dragEnd = null,
    handleDragOver = null,
    dragLeave = null,
    drop = null,
    items = [],
    dropPlaceholder = false,
    placeholderDepth = 0,
  } = $props();

  let isDragging = $derived($dragItem?.id === id);
  let isDropInto = $derived(
    $dropTarget?.type === "into" && $dropTarget.item.id === id,
  );

  function onDragStartHandler(event) {
    dragStart(event, node.group ?? node);
  }

  function onDragEndHandler() {
    dragEnd();
  }
</script>

{#if dropPlaceholder}
  <div
    class="drop-placeholder"
    style="margin-left: {placeholderDepth * 1.5}rem"
  ></div>
{:else}
  <div
    class="tree-row"
    class:selected
    class:highlighted
    class:has-tree-line={depth > 0}
    class:dragging={isDragging}
    class:drop-into={isDropInto}
    role="listitem"
    aria-grabbed={isDragging}
    draggable={!!dragStart}
    ondragstart={dragStart ? onDragStartHandler : null}
    ondragend={dragEnd ? onDragEndHandler : null}
    ondragover={handleDragOver
      ? (e) => handleDragOver(e, node.group ?? node)
      : null}
    ondragleave={dragLeave ?? null}
    ondrop={drop ? (e) => drop(e, items, node.group ?? node) : null}
    style="padding-left: {depth * 1.5}rem"
  >
    <div
      class="tree-item"
      role="button"
      tabindex="0"
      onclick={() => onSelect(id)}
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onSelect(id);
        }
      }}
      oncontextmenu={onContextMenu ? (e) => onContextMenu(e, id) : null}
    >
      {#if hasChildren}
        <span
          class="expand-icon"
          class:collapsed={isCollapsed}
          role="button"
          tabindex="0"
          aria-label={isCollapsed ? "Expand" : "Collapse"}
          onclick={(e) => {
            e.stopPropagation();
            toggle();
          }}
          onkeydown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              e.stopPropagation();
              toggle();
            }
          }}
        >
          <Icon name="chevron" size={12} />
        </span>
      {:else}
        <span class="expand-icon-spacer"></span>
      {/if}
      <span class="folder-icon">
        <Icon name="folder" size={16} />
      </span>
      <span class="tree-name">{node.group?.name ?? node.name}</span>
    </div>
  </div>
{/if}

<style>
  .drop-placeholder {
    height: 2px;
    background-color: var(--accent-color);
    border-radius: 1px;
    margin: 0;
  }

  .tree-row {
    display: flex;
    align-items: center;
    cursor: grab;
    position: relative;
  }

  .tree-row:hover {
    background-color: var(--hover-bg);
  }

  .tree-row.has-tree-line {
    padding-left: 1.5rem;
  }

  .tree-row.selected {
    background-color: var(--selected-bg);
  }

  .tree-row.highlighted {
    background-color: var(--selected-bg);
  }

  .tree-row.selected .tree-item {
    color: var(--selected-text);
  }

  .tree-row:not(.selected) .tree-item {
    color: var(--muted-color);
  }

  .tree-row:not(.selected):hover .tree-item {
    color: var(--text-color);
  }

  .tree-row.dragging {
    position: relative;
  }

  .tree-row.dragging::after {
    content: "";
    position: absolute;
    inset: 0;
    background-color: var(--accent-color);
    opacity: 0.3;
    pointer-events: none;
    z-index: 0;
  }

  .tree-row.drop-into {
    background-color: var(--accent-color) !important;
    opacity: 0.7;
  }

  .tree-item {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.375rem 0.5rem;
    cursor: pointer;
    font-size: 0.875rem;
    color: var(--text-color);
  }

  .expand-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    color: var(--muted-color);
    cursor: pointer;
    transition: transform var(--motion-duration-medium-1)
      var(--motion-easing-standard);
    transform: rotate(90deg);
  }

  .expand-icon-spacer {
    display: inline-flex;
    width: 1rem;
    height: 1rem;
  }

  .expand-icon.collapsed {
    transform: rotate(0deg);
  }

  .expand-icon:hover {
    color: var(--text-color);
  }

  .tree-row.selected .expand-icon {
    color: var(--selected-text);
  }

  .folder-icon {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    color: var(--muted-color);
  }

  .tree-row.selected .folder-icon {
    color: var(--selected-text);
  }

  .tree-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tree-item :global(svg) {
    display: block;
  }
</style>
