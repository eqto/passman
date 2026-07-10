<script>
  import { createEventDispatcher } from "svelte";

  export let nodes = [];
  export let selectedId = "";
  export let depth = 0;
  export let expanded = new Set();
  export let itemComponent;
  export let itemProps = {};

  // Optional drag-and-drop handlers to pass down to tree items
  export let dragItem = null;
  export let dragOver = null;
  export let insertBefore = null;
  export let dropInto = null;
  export let flatGroups = [];
  export let dragStart = null;
  export let dragEnd = null;
  export let handleDragOver = null;
  export let dragLeave = null;
  export let drop = null;

  const dispatch = createEventDispatcher();

  function getId(node) {
    return node.group?.id ?? node.id;
  }

  function getChildren(node) {
    return node.children ?? [];
  }

  function toggle(id) {
    expanded = new Set(expanded);
    if (expanded.has(id)) {
      expanded.delete(id);
    } else {
      expanded.add(id);
    }
    dispatch("toggle", { id, expanded });
  }

  $: treeItemProps = {
    ...itemProps,
    dragItem,
    dragOver,
    insertBefore,
    dropInto,
    flatGroups,
    dragStart,
    dragEnd,
    handleDragOver,
    dragLeave,
    drop,
  };
</script>

{#each nodes as node (getId(node))}
  {@const id = getId(node)}
  {@const children = getChildren(node)}
  {@const hasChildren = children.length > 0}
  {@const isCollapsed = !expanded.has(id)}
  <svelte:component
    this={itemComponent}
    {node}
    {id}
    {depth}
    {hasChildren}
    {isCollapsed}
    selected={selectedId === id}
    toggle={() => toggle(id)}
    {...treeItemProps}
  />
  {#if hasChildren && !isCollapsed}
    <svelte:self
      nodes={children}
      {selectedId}
      depth={depth + 1}
      {expanded}
      {itemComponent}
      itemProps={treeItemProps}
    />
  {/if}
{/each}
