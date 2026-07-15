<script>
  import Tabs from "../../../components/Tab/Tabs.svelte";
  import Tab from "../../../components/Tab/Tab.svelte";
  import { Icon } from "../../../components/icons";
  import { Confirm } from "../../../components/dialog";
  import { closeAllContextMenus } from "../../../stores/contextMenu.js";
  import { useContextMenu } from "../../../lib/createContextMenu.js";
  import VaultContextMenu from "./VaultContextMenu.svelte";
  import VaultSettingsDialog from "./VaultSettingsDialog.svelte";
  import RemoveVaultDialog from "./RemoveVaultDialog.svelte";
  import {
    vaults,
    currentVault,
    vaultData,
    isUnlocked,
    reorderVaults,
    lockVaultByPath,
    deleteVault,
  } from "../store.js";

  let contextMenu = $state({ show: false, x: 0, y: 0, vault: null });
  let showSettings = $state(false);
  let settingsVault = $state(null);
  let removeVault = $state(null);
  let lockTarget = $state(null);

  useContextMenu(handleWindowClick);

  function selectVault(id) {
    const vault = $vaults.find((v) => v.id === id);
    if (!vault || ($currentVault && $currentVault.path === vault.path)) {
      return;
    }
    currentVault.set(vault);
  }

  function handleTabKeydown(event, id) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      selectVault(id);
    }
  }

  function handleContextMenu(event, id) {
    const vault = $vaults.find((v) => v.id === id);
    if (vault) {
      event.preventDefault();
      closeAllContextMenus();
      contextMenu = { show: true, x: event.clientX, y: event.clientY, vault };
    }
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, vault: null };
  }

  function handleWindowClick() {
    if (contextMenu.show) closeContextMenu();
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

  function closeSettings() {
    showSettings = false;
    settingsVault = null;
  }

  function handleLock(vault) {
    lockTarget = vault;
  }

  async function handleLockConfirmed() {
    if (!lockTarget) return;
    await lockVaultByPath(lockTarget.path);
    lockTarget = null;
  }

  function handleRemove(vault) {
    removeVault = vault;
  }

  async function handleRemoveConfirmed() {
    if (!removeVault) return;
    const vault = removeVault;
    removeVault = null;
    const unlocked = $vaultData[vault.path]?.unlocked;
    if (unlocked) {
      await lockVaultByPath(vault.path);
    }
    try {
      await deleteVault(vault.id, vault.path);
    } catch (e) {
      console.error("Failed to remove vault:", e);
    }
  }
</script>

<svelte:window
  oncontextmenu={(e) => e.preventDefault()}
  onclick={handleWindowClick}
/>

<div class="tabs">
  <Tabs
    selectedKey={$currentVault ? $currentVault.id : null}
    onSelect={selectVault}
    onReorder={reorderVaults}
    onKeydown={handleTabKeydown}
    onContextMenu={handleContextMenu}
  >
    {#each $vaults as vault (vault.id)}
      <Tab name={vault.id} title={vault.path}>
        <span class="tab-name">{vault.name}</span>
        <span
          class="tab-actions-inner"
          aria-hidden="true"
          onclick={(e) => e.stopPropagation()}
        >
          {#if $vaultData[vault.path]?.unlocked}
            <button
              class="btn-icon tab-action-btn lock-tab-btn"
              onclick={() => handleLock(vault)}
              title="Lock vault"
            >
              <Icon name="lock" size={18} />
            </button>
          {:else}
            <button
              class="btn-icon tab-action-btn delete-tab-btn"
              onclick={() => handleRemove(vault)}
              title="Remove vault"
            >
              ×
            </button>
          {/if}
        </span>
      </Tab>
    {/each}
  </Tabs>
</div>

{#if contextMenu.show}
  <VaultContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    canRename={contextMenu.vault &&
      $currentVault &&
      $currentVault.id === contextMenu.vault.id &&
      $isUnlocked}
    onsettings={openSettings}
    onremove={() => {
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
    onrenamed={closeSettings}
    oncancel={closeSettings}
  />
{/if}

{#if lockTarget}
  <Confirm
    title="Lock Vault"
    message={`Lock "${lockTarget.name}"? You will need to re-enter the password to access it again.`}
    confirmLabel="Lock"
    onconfirm={handleLockConfirmed}
    oncancel={() => (lockTarget = null)}
  />
{/if}

{#if removeVault}
  <RemoveVaultDialog
    vault={removeVault}
    onRemove={handleRemoveConfirmed}
    onCancel={() => (removeVault = null)}
  />
{/if}

<style>
  .tabs {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    overflow-x: auto;
    min-width: 0;
  }

  .tabs :global(.tab-name) {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 12rem;
    line-height: 1.25;
  }

  .tabs :global(.tab-actions-inner) {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .tabs :global(.tab-action-btn) {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border-radius: var(--shape-full);
  }

  .tabs :global(.lock-tab-btn) {
    padding: 0.25rem;
  }

  .tabs :global(.tab.selected .lock-tab-btn) {
    color: var(--selected-text);
    background-color: transparent;
  }

  .tabs :global(.tab.selected .lock-tab-btn:hover) {
    color: var(--selected-text);
    background-color: var(--hover-bg);
  }

  .tabs :global(.delete-tab-btn:hover) {
    color: var(--on-danger-container);
    background-color: var(--danger-container);
  }
</style>
