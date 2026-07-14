<script>
  import ThemeToggle from "../../../components/ThemeToggle.svelte";
  import OpenVaultMenu from "./OpenVaultMenu.svelte";

  let {
    dropdownPosition = $bindable({ x: 0, y: 0 }),
    showDropdown = $bindable(false),
    onCreate,
    onPickExisting,
    onButtercupImport,
  } = $props();
</script>

<div class="topbar">
  <button class="btn-secondary" onclick={() => onCreate?.()}>
    <span class="action-icon">+</span>
    <span class="action-text">New Vault</span>
  </button>
  <OpenVaultMenu
    bind:dropdownPosition
    bind:showDropdown
    onpickexisting={onPickExisting}
    onbuttercupimport={onButtercupImport}
  />
  <ThemeToggle />
</div>

<style>
  .topbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-shrink: 0;
    margin-left: auto;
  }

  .action-icon {
    font-size: 0.875rem;
    line-height: 1.25;
    margin-right: 0.25rem;
  }

  .action-text {
    position: relative;
    top: 0.0625rem;
  }
</style>
