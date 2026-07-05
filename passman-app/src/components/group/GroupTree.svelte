<script>
  import GroupItem from "./GroupItem.svelte";

  export let nodes = [];
  export let selectedGroup = "";
  export let depth = 0;
  export let onSelectGroup;
  export let onContextMenu;
  export let onDelete;
  export let dragItem;
  export let dragOver;
  export let insertBefore;
  export let dragStart;
  export let dragEnd;
  export let handleDragOver;
  export let dragLeave;
  export let drop;
  export let flatGroups = [];
  export let collapsed = new Set();
  export let toggleGroup = (groupId) => {
    collapsed = new Set(collapsed);
    if (collapsed.has(groupId)) {
      collapsed.delete(groupId);
    } else {
      collapsed.add(groupId);
    }
  };
  export let hasAnyChildren = false;

  function checkAnyChildren(nodes) {
    for (const node of nodes) {
      if (node.children.length > 0 || checkAnyChildren(node.children)) {
        return true;
      }
    }
    return false;
  }

  $: hasAnyChildren = checkAnyChildren(nodes);
</script>

{#each nodes as { group, children } (group.id)}
  <GroupItem
    {group}
    {selectedGroup}
    {depth}
    {onSelectGroup}
    {onContextMenu}
    {onDelete}
    {dragItem}
    {dragOver}
    {insertBefore}
    {dragStart}
    {dragEnd}
    {handleDragOver}
    {dragLeave}
    {drop}
    {flatGroups}
    hasChildren={children.length > 0}
    isCollapsed={collapsed.has(group.id)}
    {toggleGroup}
    {hasAnyChildren}
  />
  {#if children.length > 0 && !collapsed.has(group.id)}
    <svelte:self
      nodes={children}
      {selectedGroup}
      depth={depth + 1}
      {onSelectGroup}
      {onContextMenu}
      {onDelete}
      {dragItem}
      {dragOver}
      {insertBefore}
      {dragStart}
      {dragEnd}
      {handleDragOver}
      {dragLeave}
      {drop}
      {flatGroups}
      {collapsed}
      {toggleGroup}
      {hasAnyChildren}
    />
  {/if}
{/each}
