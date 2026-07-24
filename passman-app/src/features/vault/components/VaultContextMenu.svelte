<script>
  let {
    x = 0,
    y = 0,
    canRename = false,
    canLock = false,
    onlock = null,
    onsettings = null,
    onremove = null,
  } = $props();

  function handleLock() {
    if (canLock) {
      onlock?.();
    }
  }

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
  {#if canLock}
    <div
      class="menu-item"
      role="menuitem"
      tabindex="0"
      onclick={handleLock}
      onkeydown={(e) => (e.key === "Enter" || e.key === " ") && handleLock()}
    >
      Lock Vault
    </div>
    <div class="context-menu-divider"></div>
  {/if}
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
    background-color: var(--danger-container);
    color: var(--on-danger-container);
  }
</style>
