<script>
  import { setContext, onMount } from "svelte";
  import { createDragList } from "./drag";
  import TabHeader from "./TabHeader.svelte";

  let {
    selectedKey = null,
    onSelect = null,
    onReorder = null,
    onKeydown = null,
    onContextMenu = null,
    onClose = null,
    children,
  } = $props();

  let tabs = $state([]);
  let tabsBarEl = $state(null);

  const drag = createDragList({
    axis: "horizontal",
    getKey: (tab) => tab.id,
    onReorder: (reordered) => {
      tabs = reordered;
      onReorder?.(reordered.map((tab) => tab.id));
    },
  });

  setContext("tabs", {
    registerTab: (tab) => {
      if (tab && tab.id != null) tabs.push(tab);
    },
    unregisterTab: (id) => {
      tabs = tabs.filter((tab) => tab?.id !== id);
    },
    drag,
    getTabs: () => tabs,
  });

  function handleWheel(event) {
    if (event.deltaY !== 0) {
      event.preventDefault();
      tabsBarEl.scrollLeft += event.deltaY;
    }
  }

  onMount(() => {
    if (tabsBarEl) {
      tabsBarEl.addEventListener("wheel", handleWheel, { passive: false });
    }
    return () => {
      if (tabsBarEl) {
        tabsBarEl.removeEventListener("wheel", handleWheel);
      }
    };
  });
</script>

{@render children?.()}

<div class="tabs-header-row">
  <div class="tabs-bar" bind:this={tabsBarEl}>
    {#each tabs as tab (tab.id)}
      <TabHeader
        {tab}
        selected={tab.id === selectedKey}
        onSelect={() => onSelect?.(tab.id)}
        onKeydown={(e) => onKeydown?.(e, tab.id)}
        onContextMenu={(e) => onContextMenu?.(e, tab.id)}
        onClose={onClose ? () => onClose(tab.id) : null}
      />
    {/each}
  </div>
</div>

<div class="tab-content">
  {#each tabs as tab (tab.id)}
    {#if tab.id === selectedKey && tab.content}
      {@render tab.content()}
    {/if}
  {/each}
</div>

<style>
  .tabs-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border-color);
    background-color: var(--sidebar-bg);
    width: 100%;
  }

  .tabs-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    flex: 1;
    min-width: 0;
  }

  .tab-content {
    flex: 1;
    overflow: hidden;
    display: flex;
  }
</style>
