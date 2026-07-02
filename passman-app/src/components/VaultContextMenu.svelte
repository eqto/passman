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
    min-width: 8rem;
  }

  .context-menu-item.danger {
    color: var(--danger-color);
  }

  .context-menu-item.danger:hover {
    background-color: rgba(239, 68, 68, 0.1);
  }
</style>
