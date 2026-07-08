<script>
  import { open } from "@tauri-apps/plugin-dialog";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let showDropdown = false;
  export let dropdownPosition = { x: 0, y: 0 };

  function pickExistingVault() {
    dispatch("pickExisting");
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
    dispatch("buttercupImport");
  }
</script>

<div class="btn-secondary dropdown-button">
  <button class="dropdown-main-btn" on:click={pickExistingVault}>Open Vault</button>
  <div class="dropdown-separator"></div>
  <button class="btn-icon dropdown-toggle" on:click={toggleOpenDropdown} title="Open options">
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 -960 960 960" fill="currentColor"><path d="M480-345 240-585l56-56 184 184 184-184 56 56-240 240Z"/></svg>
  </button>
</div>
{#if showDropdown}
  <div class="dropdown-menu" style="left: {dropdownPosition.x}px; top: {dropdownPosition.y}px;">
    <button class="dropdown-item" on:click={handleButtercupImport}>
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
