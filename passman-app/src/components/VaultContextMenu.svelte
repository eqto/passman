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

<div class="menu" style="left: {x}px; top: {y}px">
  {#if canRename}
    <div class="menu-item" on:click={handleSettings}>
      Settings
    </div>
  {:else}
    <div class="menu-item" style="opacity: 0.6; cursor: not-allowed">
      Settings (vault locked)
    </div>
  {/if}
  <div class="menu-item danger" on:click={handleRemove}>
    Remove Vault
  </div>
</div>

<style>
  .menu {
    min-width: 8rem;
  }

  .menu-item.danger {
    color: var(--danger-color);
  }

  .menu-item.danger:hover {
    background-color: rgba(239, 68, 68, 0.1);
  }
</style>
