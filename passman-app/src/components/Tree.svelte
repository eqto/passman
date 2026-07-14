<script>
  import { getContext, setContext } from "svelte";
  import { createDragList } from "./Tab/drag";
  import TreeItem from "./TreeItem.svelte";
  import Self from "./Tree.svelte";

  let {
    nodes = [],
    selectedId = "",
    highlightedId = "",
    depth = 0,
    expanded = $bindable(new Set()),
    onSelect,
    onContextMenu = null,
    items = [],
    onReorder = null,
    onDropInto = null,
    getKey = (x) => x.id,
    axis = "vertical",
    ontoggle = null,
  } = $props();

  // Share a single drag state across recursive Tree instances via context
  const parentDrag = getContext("drag");
  const drag =
    parentDrag ||
    (onReorder || onDropInto
      ? createDragList({ axis, getKey, onReorder, onDropInto })
      : null);
  setContext("drag", drag);

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
    ontoggle?.({ id, expanded });
  }

  // Destructure stores from drag so we can use $ auto-subscribe in template
  const dragItem = drag?.dragItem;
  const dropTarget = drag?.dropTarget;

  let dragProps = $derived(
    drag
      ? {
          dragItem: drag.dragItem,
          dropTarget: drag.dropTarget,
          dragStart: drag.dragStart,
          dragEnd: drag.dragEnd,
          handleDragOver: drag.handleDragOver,
          dragLeave: drag.dragLeave,
          drop: drag.drop,
          items,
        }
      : {},
  );
</script>

{#if drag && nodes.length > 0}
  <div
    class="drop-zone-top"
    role="none"
    ondragover={(e) => drag.handleDragOverFirst(e, nodes[0].group ?? nodes[0])}
    ondrop={(e) => drag.dropFirst(e, items, nodes[0].group ?? nodes[0])}
  ></div>
{/if}

{#each nodes as node (getId(node))}
  {@const id = getId(node)}
  {@const children = getChildren(node)}
  {@const hasChildren = children.length > 0}
  {@const isCollapsed = !expanded.has(id)}
  {#if drag && $dropTarget?.type === "before" && $dropTarget.item.id === id}
    <TreeItem dropPlaceholder={true} placeholderDepth={depth} />
  {/if}
  <TreeItem
    {node}
    {id}
    {depth}
    {hasChildren}
    {isCollapsed}
    selected={selectedId === id}
    highlighted={highlightedId === id}
    toggle={() => toggle(id)}
    {onSelect}
    {onContextMenu}
    {...dragProps}
  />
  {#if hasChildren && !isCollapsed}
    <Self
      nodes={children}
      {selectedId}
      {highlightedId}
      depth={depth + 1}
      {expanded}
      {onSelect}
      {onContextMenu}
      {items}
      {getKey}
      {axis}
      {onReorder}
      {onDropInto}
    />
  {/if}
  {#if drag && $dropTarget?.type === "after" && $dropTarget.item.id === id}
    <TreeItem dropPlaceholder={true} placeholderDepth={depth} />
  {/if}
{/each}

<style>
  .drop-zone-top {
    height: 8px;
    margin-left: 0;
  }
</style>
