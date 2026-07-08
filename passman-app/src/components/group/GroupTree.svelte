<script>
  import Tree from "../Tree.svelte";
  import GroupTreeItem from "./GroupTreeItem.svelte";

  export let nodes = [];
  export let selectedGroup = "";
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

  function checkAnyChildren(nodes) {
    for (const node of nodes) {
      if (node.children.length > 0 || checkAnyChildren(node.children)) {
        return true;
      }
    }
    return false;
  }

  $: hasAnyChildren = checkAnyChildren(nodes);

  $: itemProps = {
    onSelectGroup,
    onContextMenu,
    onDelete,
    dragItem,
    dragOver,
    insertBefore,
    dragStart,
    dragEnd,
    handleDragOver,
    dragLeave,
    drop,
    flatGroups,
    hasAnyChildren,
  };
</script>

<Tree
  {nodes}
  selectedId={selectedGroup}
  itemComponent={GroupTreeItem}
  {itemProps}
/>
