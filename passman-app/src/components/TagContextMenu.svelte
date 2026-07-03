<script>
  import { createEventDispatcher } from "svelte";

  export let x = 0;
  export let y = 0;

  const dispatch = createEventDispatcher();

  function handleDelete() {
    dispatch("delete");
  }

  function handleClickOutside() {
    dispatch("close");
  }
</script>

<svelte:window on:click={handleClickOutside} />

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div class="menu" style="left: {x}px; top: {y}px;" on:click|stopPropagation>
  <div class="menu-item danger" on:click={handleDelete}>
    Delete
  </div>
</div>

<style>
  .menu {
    position: fixed;
    z-index: 100;
    min-width: 8rem;
    background-color: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    padding: 0.25rem 0;
  }

  .menu-item {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    cursor: pointer;
    color: var(--text-color);
  }

  .menu-item:hover {
    background-color: var(--hover-bg);
  }

  .menu-item.danger {
    color: var(--danger-color);
  }
</style>
