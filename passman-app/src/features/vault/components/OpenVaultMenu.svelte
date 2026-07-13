<script>
  import { ChevronIcon } from "../../../components/icons";

  let {
    showDropdown = false,
    dropdownPosition = { x: 0, y: 0 },
    onpickexisting = null,
    onbuttercupimport = null,
  } = $props();

  function pickExistingVault() {
    onpickexisting?.();
  }

  function toggleOpenDropdown(event) {
    event.stopPropagation();
    const rect = event.currentTarget.getBoundingClientRect();
    const menuWidth = 180;
    const windowWidth = window.innerWidth;

    let x = rect.left;
    if (x + menuWidth > windowWidth) {
      x = windowWidth - menuWidth - 16;
    }

    dropdownPosition = {
      x,
      y: rect.bottom + 4,
    };
    showDropdown = !showDropdown;
  }

  function handleButtercupImport() {
    showDropdown = false;
    onbuttercupimport?.();
  }
</script>

<div class="btn-secondary dropdown-button">
  <button class="dropdown-main-btn" onclick={pickExistingVault}
    >Open Vault</button
  >
  <div class="dropdown-separator"></div>
  <button
    class="btn-icon dropdown-toggle"
    onclick={toggleOpenDropdown}
    title="Open options"
  >
    <ChevronIcon size={16} />
  </button>
</div>
{#if showDropdown}
  <div
    class="dropdown-menu"
    style="left: {dropdownPosition.x}px; top: {dropdownPosition.y}px;"
  >
    <button class="dropdown-item" onclick={handleButtercupImport}>
      Open Buttercup format
    </button>
  </div>
{/if}

<style>
  .dropdown-button {
    position: relative;
    display: flex;
    align-items: center;
    padding: 0.5rem 0.75rem 0.25rem;
    background-color: var(--hover-bg);
    color: var(--text-color);
    border-radius: 0.5rem;
    gap: 0.375rem;
    height: 2.25rem;
    line-height: 1.5rem;
  }

  .dropdown-button:hover {
    outline: 1px solid var(--accent-color);
  }

  .dropdown-main-btn {
    background: transparent;
    border: none;
    padding: 0;
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    line-height: 1;
  }

  .dropdown-separator {
    width: 1px;
    height: 1.25rem;
    background-color: var(--border-color);
  }

  .dropdown-menu {
    position: fixed;
    background-color: var(--sidebar-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 9999;
    min-width: 180px;
  }

  .dropdown-item {
    width: 100%;
    padding: 0.5rem 0.75rem;
    text-align: left;
    background: transparent;
    border: none;
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .dropdown-item:hover {
    background-color: var(--hover-bg);
  }
</style>
