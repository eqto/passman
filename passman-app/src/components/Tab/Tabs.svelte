<script>
  import { setContext } from "svelte";
  import { createDragList } from "./drag";
  import TabsHandle from "./TabsHandle.svelte";

  let {
    selectedKey = null,
    onSelect = null,
    onReorder = null,
    onKeydown = null,
    onContextMenu = null,
  } = $props();

  let tabs = $state([]);

  const drag = createDragList({
    axis: "horizontal",
    getKey: (tab) => tab.id,
    onReorder: (reordered) => onReorder?.(reordered.map((tab) => tab.id)),
  });
  const { dragItem, dropTarget } = drag;

  setContext("tabs", {
    registerTab: (tab) => {
      if (tab && tab.id != null) tabs.push(tab);
    },
    unregisterTab: (id) => {
      tabs = tabs.filter((tab) => tab?.id !== id);
    },
  });

  function handleDragStart(e, tab) {
    drag.dragStart(e, tab);
  }

  function handleDragEnd() {
    drag.dragEnd();
  }

  function handleDragOver(e, tab) {
    drag.handleDragOver(e, tab);
  }

  function handleDragLeave() {
    drag.dragLeave();
  }

  function handleDrop(e, tab) {
    drag.drop(e, tabs, tab);
  }
</script>

<!-- svelte-ignore slot_element_deprecated -->
<slot />

{#each tabs as tab (tab.id)}
  <TabsHandle
    {tab}
    selected={tab.id === selectedKey}
    dragging={$dragItem === tab}
    dropBefore={$dropTarget?.type === "before" &&
      $dropTarget.item.id === tab.id}
    dropAfter={$dropTarget?.type === "after" && $dropTarget.item.id === tab.id}
    onSelect={() => onSelect?.(tab.id)}
    onKeydown={(e) => onKeydown?.(e, tab.id)}
    onContextMenu={(e) => onContextMenu?.(e, tab.id)}
    onDragStart={(e) => handleDragStart(e, tab)}
    onDragEnd={handleDragEnd}
    onDragOver={(e) => handleDragOver(e, tab)}
    onDragLeave={handleDragLeave}
    onDrop={(e) => handleDrop(e, tab)}
  />
{/each}
