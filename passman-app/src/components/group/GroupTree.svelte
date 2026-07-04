<script>
  import GroupItem from './GroupItem.svelte';

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
  />
  {#if children.length > 0}
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
    />
  {/if}
{/each}
