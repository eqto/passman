<script>
  let { x = 0, y = 0, ondelete = null, onclose = null } = $props();

  function handleDelete() {
    ondelete?.();
  }

  function handleClickOutside() {
    onclose?.();
  }
</script>

<svelte:window onclick={handleClickOutside} />

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  class="menu"
  style="left: {x}px; top: {y}px;"
  onclick={(e) => e.stopPropagation()}
>
  <div class="menu-item danger" onclick={handleDelete}>Delete</div>
</div>

<style>
  .menu {
    position: fixed;
    z-index: 100;
    min-width: 8rem;
    background-color: var(--bg-color);
    border: none;
    border-radius: var(--shape-sm);
    box-shadow:
      0 0 1px 0 rgba(0, 0, 0, 0.3),
      0 1px 2px 0 rgba(0, 0, 0, 0.3),
      0 2px 6px 2px rgba(0, 0, 0, 0.15);
    padding: var(--space-1) 0;
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
