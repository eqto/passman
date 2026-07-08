<script>
  import GroupItem from "./GroupItem.svelte";

  export let node;
  export let id;
  export let depth;
  export let hasChildren;
  export let isCollapsed;
  export let selected;
  export let toggle;

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
  export let hasAnyChildren = false;
</script>

<GroupItem
  group={node.group}
  selectedGroup={selected ? id : ""}
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
  {hasChildren}
  {isCollapsed}
  toggleGroup={() => toggle()}
  {hasAnyChildren}
/>
