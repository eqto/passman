<script>
  import { createEventDispatcher } from "svelte";

  export let nodes = [];
  export let selectedId = "";
  export let depth = 0;
  export let expanded = new Set();
  export let itemComponent;
  export let itemProps = {};

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
    {...itemProps}
  />
  {#if hasChildren && !isCollapsed}
    <svelte:self
      nodes={children}
      {selectedId}
      depth={depth + 1}
      {expanded}
      {itemComponent}
      {itemProps}
    />
  {/if}
{/each}
