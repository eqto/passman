<script>
  let {
    x = 0,
    y = 0,
    canRename = false,
    onsettings = null,
    onremove = null,
  } = $props();

  function handleSettings() {
    if (canRename) {
      onsettings?.();
    }
  }

  function handleRemove() {
    onremove?.();
  }
</script>

<div class="menu" style="left: {x}px; top: {y}px" role="menu">
  {#if canRename}
    <div
      class="menu-item"
      role="menuitem"
      tabindex="0"
      onclick={handleSettings}
      onkeydown={(e) =>
        (e.key === "Enter" || e.key === " ") && handleSettings()}
    >
      Settings
    </div>
  {:else}
    <div
      class="menu-item"
      style="opacity: 0.6; cursor: not-allowed"
      role="menuitem"
    >
      Settings (vault locked)
    </div>
  {/if}
  <div
    class="menu-item danger"
    role="menuitem"
    tabindex="0"
    onclick={handleRemove}
    onkeydown={(e) => (e.key === "Enter" || e.key === " ") && handleRemove()}
  >
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
