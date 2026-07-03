<script>
  import { get } from "svelte/store";
  import { onMount } from "svelte";
  import {
    vaults,
    currentVault,
    vaultData,
    isUnlocked,
    loadError,
    openVault,
    registerAndOpenVault,
    lockVaultByPath,
    deleteVault,
    reorderVaults,
  } from "../stores/vaults";
  import { closeAllContextMenus } from "../stores/contextMenu.js";
  import { theme } from "../stores/theme.js";
  import UnlockDialog from "./UnlockDialog.svelte";
  import CreateVaultDialog from "./CreateVaultDialog.svelte";
  import VaultSettingsDialog from "./VaultSettingsDialog.svelte";
  import VaultContextMenu from "./VaultContextMenu.svelte";
  import RemoveVaultDialog from "./RemoveVaultDialog.svelte";
  import ImportButtercupDialog from "./ImportButtercupDialog.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { createDragList } from "../lib/dragList.js";

  let showCreate = false;
  let unlockTarget = null;
  let showOpenDropdown = false;
  let showButtercupImport = false;
  let dropdownPosition = { x: 0, y: 0 };

  let contextMenu = { show: false, x: 0, y: 0, vault: null };
  let showSettings = false;
  let settingsVault = null;
  let removeVault = null;

  const drag = createDragList({
    axis: "horizontal",
    getKey: (v) => v.id,
    onReorder: async (items) => reorderVaults(items.map((v) => v.id)),
  });
  const { dragItem, dragOver, insertBefore } = drag;

  onMount(() => {
    window.addEventListener('close-all-context-menus', closeContextMenu);
    window.addEventListener('close-all-context-menus', () => {
      showOpenDropdown = false;
    });
    return () => {
      window.removeEventListener('close-all-context-menus', closeContextMenu);
      window.removeEventListener('close-all-context-menus', () => {
        showOpenDropdown = false;
      });
    };
  });

  async function pickExistingVault() {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [{ name: "Passman Vault", extensions: ["pmv"] }],
    });
    if (selected) {
      unlockTarget = { path: selected, registered: false };
    }
  }

  function selectVault(vault) {
    if ($currentVault && $currentVault.path === vault.path) {
      return;
    }
    currentVault.set(vault);
  }

  async function handleUnlock(path, password) {
    if (unlockTarget.registered) {
      await openVault(path, password);
    } else {
      const id = crypto.randomUUID();
      await registerAndOpenVault(id, path, password);
    }
    unlockTarget = null;
  }

  async function handleLock(vault) {
    await lockVaultByPath(vault.path);
  }

  function handleDelete(vault) {
    removeVault = vault;
  }

  async function handleRemoveConfirmed() {
    if (!removeVault) return;
    const vault = removeVault;
    removeVault = null;
    const isUnlocked = get(vaultData)[vault.path]?.unlocked;
    if (isUnlocked) {
      await lockVaultByPath(vault.path);
    }
    try {
      await deleteVault(vault.id, vault.path);
    } catch (e) {
      console.error("Failed to remove vault:", e);
    }
  }

  function handleCancelRemove() {
    removeVault = null;
  }

  function handleContextMenu(event, vault) {
    event.preventDefault();
    closeAllContextMenus();
    contextMenu = { show: true, x: event.clientX, y: event.clientY, vault };
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, vault: null };
  }

  function openSettings() {
    if (
      !contextMenu.vault ||
      !$isUnlocked ||
      !$currentVault ||
      $currentVault.id !== contextMenu.vault.id
    ) {
      closeContextMenu();
      return;
    }
    settingsVault = contextMenu.vault;
    showSettings = true;
    closeContextMenu();
  }

  function handleWindowClick() {
    if (contextMenu.show) {
      closeContextMenu();
    }
    if (showOpenDropdown) {
      showOpenDropdown = false;
    }
  }

  function closeSettings() {
    showSettings = false;
    settingsVault = null;
  }

  function handleTabKeydown(event, vault) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      selectVault(vault);
    }
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
      y: rect.bottom + 4
    };
    showOpenDropdown = !showOpenDropdown;
  }

  function cycleTheme() {
    const themes = ["light", "dark", "auto"];
    const currentIndex = themes.indexOf($theme);
    const nextIndex = (currentIndex + 1) % themes.length;
    theme.set(themes[nextIndex]);
  }

  function getThemeIcon() {
    const isDark = document.documentElement.classList.contains("dark");
    const strokeColor = isDark ? "#ffffff" : "#111827";
    
    if ($theme === "light") {
      return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="${strokeColor}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/></svg>`;
    } else if ($theme === "dark") {
      return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="${strokeColor}" stroke="${strokeColor}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>`;
    } else {
      return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="${strokeColor}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><defs><clipPath id="moon-clip"><rect x="0" y="0" width="24" height="12"/></clipPath><clipPath id="sun-clip"><rect x="0" y="12" width="24" height="12"/></clipPath></defs><g clip-path="url(#moon-clip)"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" fill="${strokeColor}" stroke="${strokeColor}"/></g><g clip-path="url(#sun-clip)"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/></g></svg>`;
    }
  }

  function handleButtercupImport() {
    showOpenDropdown = false;
    showButtercupImport = true;
  }

</script>

<svelte:window on:click={handleWindowClick} />

<div class="vault-tabs">
  <div class="tabs">
    {#each $vaults as vault (vault.id)}
      <div
        class="tab"
        class:selected={$currentVault && $currentVault.path === vault.path}
        class:dragging={$dragItem === vault}
        class:drop-before={$dragOver === vault && $insertBefore === true}
        class:drop-after={$dragOver === vault && $insertBefore === false}
        role="button"
        tabindex="0"
        draggable={true}
        on:dragstart={(e) => drag.dragStart(e, vault)}
        on:dragend={drag.dragEnd}
        on:dragover={(e) => drag.handleDragOver(e, vault)}
        on:dragleave={drag.dragLeave}
        on:drop={(e) => drag.drop(e, $vaults, vault)}
        on:click={() => selectVault(vault)}
        on:keydown={(e) => handleTabKeydown(e, vault)}
        on:contextmenu|preventDefault={(e) => handleContextMenu(e, vault)}
        title={vault.path}
      >
        <span class="tab-name">{vault.name}</span>
        {#if $vaultData[vault.path]?.unlocked}
          <button
            class="btn-icon tab-action-btn lock-tab-btn"
            on:click|stopPropagation={() => handleLock(vault)}
            title="Lock vault"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 -960 960 960" fill="currentColor"><path d="M534.5-302.03Q557-324.06 557-355q0-30-22.67-54.5t-54.5-24.5q-31.83 0-54.33 24.5t-22.5 55q0 30.5 22.67 52.5t54.5 22q31.83 0 54.33-22.03ZM220-80q-24.75 0-42.37-17.63Q160-115.25 160-140v-434q0-24.75 17.63-42.38Q195.25-634 220-634h330v-96q0-78.85 55.61-134.42Q661.21-920 740.11-920q78.89 0 134.39 55.58Q930-808.85 930-730h-60q0-54-37.88-92t-92-38Q686-860 648-822.08q-38 37.91-38 92.08v96h130q24.75 0 42.38 17.62Q800-598.75 800-574v434q0 24.75-17.62 42.37Q764.75-80 740-80H220Z"/></svg>
          </button>
        {:else}
          <button
            class="btn-icon tab-action-btn delete-tab-btn"
            on:click|stopPropagation={() => handleDelete(vault)}
            title="Remove vault"
          >
            ×
          </button>
        {/if}
      </div>
    {/each}
  </div>

  <div class="tab-actions">
    <button class="btn-secondary" on:click={() => showCreate = true}>
      <span class="action-icon">+</span>
      <span>New Vault</span>
    </button>
    <div class="btn-secondary dropdown-button">
      <button class="dropdown-main-btn" on:click={pickExistingVault}>Open Vault</button>
      <div class="dropdown-separator"></div>
      <button class="btn-icon dropdown-toggle" on:click={toggleOpenDropdown} title="Open options">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 -960 960 960" fill="currentColor"><path d="M480-345 240-585l56-56 184 184 184-184 56 56-240 240Z"/></svg>
      </button>
    </div>
    {#if showOpenDropdown}
      <div class="dropdown-menu" style="left: {dropdownPosition.x}px; top: {dropdownPosition.y}px;">
        <button class="dropdown-item" on:click={handleButtercupImport}>
          Open Buttercup format
        </button>
      </div>
    {/if}
    <button class="btn-icon theme-toggle-btn" on:click={cycleTheme} title={$theme.charAt(0).toUpperCase() + $theme.slice(1)}>
      {@html getThemeIcon()}
    </button>
  </div>
</div>

{#if $loadError}
  <div class="vault-load-error" title={$loadError}>
    ⚠ {$loadError}
  </div>
{/if}

{#if contextMenu.show}
  <VaultContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    canRename={contextMenu.vault &&
      $currentVault &&
      $currentVault.id === contextMenu.vault.id &&
      $isUnlocked}
    on:settings={openSettings}
    on:remove={() => {
      if (contextMenu.vault) {
        removeVault = contextMenu.vault;
      }
      closeContextMenu();
    }}
  />
{/if}

{#if showSettings}
  <VaultSettingsDialog
    vault={settingsVault}
    on:renamed={closeSettings}
    on:cancel={closeSettings}
  />
{/if}

{#if removeVault}
  <RemoveVaultDialog
    vault={removeVault}
    onRemove={handleRemoveConfirmed}
    onCancel={handleCancelRemove}
  />
{/if}

{#if showCreate}
  <CreateVaultDialog
    on:created={() => showCreate = false}
    on:cancel={() => showCreate = false}
  />
{/if}

{#if unlockTarget}
  <UnlockDialog
    path={unlockTarget.path}
    name={unlockTarget.name || unlockTarget.path}
    onUnlock={handleUnlock}
    onCancel={() => unlockTarget = null}
  />
{/if}

{#if showButtercupImport}
  <ImportButtercupDialog
    on:success={() => showButtercupImport = false}
    on:cancel={() => showButtercupImport = false}
  />
{/if}

<style>
  .vault-tabs {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 1rem;
    background-color: var(--sidebar-bg);
    gap: 1rem;
  }

  .tabs {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    overflow-x: auto;
    min-width: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.5rem 0.5rem 0.75rem;
    border: none;
    border-radius: 0.5rem;
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    white-space: nowrap;
  }

  .tab:hover {
    background-color: var(--hover-bg);
  }

  .tab.selected {
    background-color: var(--selected-bg);
    color: var(--selected-text);
  }

  .tab.dragging {
    cursor: grabbing;
    opacity: 0.6;
  }

  .tab.drop-before {
    border-left: 2px solid var(--selected-bg);
  }

  .tab.drop-after {
    border-right: 2px solid var(--selected-bg);
  }

  .tab-name {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 12rem;
  }

  .tab-action-btn {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border-radius: 50%;
  }

  .lock-tab-btn {
    padding: 0.25rem;
  }

  .tab.selected .lock-tab-btn {
    color: var(--selected-text);
    background-color: transparent;
  }

  .tab.selected .lock-tab-btn:hover {
    color: var(--selected-text);
    background-color: rgba(128, 128, 128, 0.2);
  }

  .delete-tab-btn:hover {
    color: var(--danger-color);
    background-color: rgba(239, 68, 68, 0.1);
  }

  .tab-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-shrink: 0;
    position: relative;
  }

  .action-icon {
    font-size: 0.875rem;
    line-height: 1;
  }

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

  .theme-toggle-btn {
    width: 2.25rem;
    height: 2.25rem;
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--hover-bg);
    border: none;
    border-radius: 0.5rem;
    color: var(--text-color);
    cursor: pointer;
  }

  .theme-toggle-btn:hover {
    outline: 1px solid var(--accent-color);
  }

  .vault-load-error {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    color: var(--danger-color);
    background-color: rgba(239, 68, 68, 0.1);
    border-bottom: 1px solid var(--border-color);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

</style>
