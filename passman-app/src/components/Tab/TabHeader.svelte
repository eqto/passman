<script>
  import { getContext } from "svelte";
  import { Icon } from "../icons";

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
      el.scrollIntoView({
        behavior: "smooth",
        block: "nearest",
        inline: "nearest",
      });
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
  <!-- {#if onClose}
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
        aria-label="Close"
      >
        <Icon name="close" size={16} />
      </button>
    </span>
  {/if} -->
</div>
{#if dropAfter}
  <div class="drop-indicator"></div>
{/if}

<style>
  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: var(--shape-sm);
    background-color: var(--card-bg);
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
    font-size: var(--font-size-sm);
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 12rem;
    white-space: nowrap;
    padding: 0.5rem 1rem;
  }

  .tab-actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .tab-action-btn {
    width: 2rem;
    height: 2rem;
    padding: 0;
    border-radius: var(--shape-full);
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: inherit;
    cursor: pointer;
  }

  .tab-action-btn:focus-visible {
    box-shadow: 0 0 0 2px var(--accent-color);
  }

  .delete-tab-btn:hover {
    color: var(--on-danger-container);
    background-color: var(--danger-container);
  }
</style>
