<script>
  import { setContext } from "svelte";
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
</script>

{@render children?.()}

<div class="tabs-bar">
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

<div class="tab-content">
  {#each tabs as tab (tab.id)}
    {#if tab.id === selectedKey && tab.content}
      {@render tab.content()}
    {/if}
  {/each}
</div>

<style>
  .tabs-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
    padding: 0.5rem;
  }

  .tab-content {
    border-top: 1px solid var(--border-color);
    flex: 1;
    overflow: hidden;
    display: flex;
  }
</style>
