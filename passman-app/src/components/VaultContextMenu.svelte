<script>
  import { createEventDispatcher } from "svelte";

  export let x = 0;
  export let y = 0;
  export let canRename = false;

  const dispatch = createEventDispatcher();

  function handleSettings() {
    if (canRename) {
      dispatch("settings");
    }
  }

  function handleRemove() {
    dispatch("remove");
  }
</script>

<div class="context-menu" style="left: {x}px; top: {y}px">
  {#if canRename}
    <button class="context-menu-item" on:click={handleSettings}>
      Settings
    </button>
  {:else}
    <button class="context-menu-item disabled" disabled>
      Settings (vault locked)
    </button>
  {/if}
  <button class="context-menu-item danger" on:click={handleRemove}>
    Remove Vault
  </button>
</div>

<style>
  .context-menu {
    position: fixed;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    z-index: 100;
    min-width: 8rem;
    padding: 0.25rem;
  }

  .context-menu-item {
    width: 100%;
    text-align: left;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 0.375rem;
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.875rem;
  }

  .context-menu-item:hover {
    background-color: var(--hover-bg);
  }

  .context-menu-item.disabled {
    color: var(--muted-color);
    cursor: not-allowed;
  }

  .context-menu-item.disabled:hover {
    background-color: transparent;
  }

  .context-menu-item.danger {
    color: var(--danger-color);
  }

  .context-menu-item.danger:hover {
    background-color: rgba(239, 68, 68, 0.1);
  }
</style>
