<script>
  import { getContext } from "svelte";

  let {
    tab,
    selected = false,
    onSelect,
    onKeydown,
    onContextMenu,
    onClose = null,
  } = $props();

  const { drag, getTabs } = getContext("tabs");
  const { dragItem, dropTarget } = drag;

  let el = $state(null);

  let dragging = $derived($dragItem === tab);
  let dropBefore = $derived(
    $dropTarget?.type === "before" && $dropTarget.item.id === tab.id,
  );
  let dropAfter = $derived(
    $dropTarget?.type === "after" && $dropTarget.item.id === tab.id,
  );

  $effect(() => {
    if (selected && el) {
      el.scrollIntoView({ behavior: "smooth", block: "nearest", inline: "nearest" });
    }
  });
</script>

{#if dropBefore}
  <div class="drop-indicator"></div>
{/if}
<div
  bind:this={el}
  class="tab"
  class:selected
  class:dragging
  role="button"
  tabindex="0"
  draggable="true"
  title={tab.title}
  ondragstart={(e) => drag.dragStart(e, tab)}
  ondragend={() => drag.dragEnd()}
  ondragover={(e) => drag.handleDragOver(e, tab, e.currentTarget)}
  ondragleave={() => drag.dragLeave()}
  ondrop={(e) => {
    drag.drop(e, getTabs(), tab, e.currentTarget);
  }}
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
{#if dropAfter}
  <div class="drop-indicator"></div>
{/if}

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

  .drop-indicator {
    width: 2px;
    height: 1.75rem;
    background-color: var(--accent-color);
    border-radius: 1px;
    flex-shrink: 0;
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
