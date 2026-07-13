<script>
  import { get } from "svelte/store";
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
  } from "../store.js";
  import { closeAllContextMenus } from "../../../stores/contextMenu.js";
  import { useContextMenu } from "../../../lib/createContextMenu.js";
  import UnlockDialog from "./UnlockDialog.svelte";
  import CreateVaultDialog from "./CreateVaultDialog.svelte";
  import VaultSettingsDialog from "./VaultSettingsDialog.svelte";
  import VaultContextMenu from "./VaultContextMenu.svelte";
  import RemoveVaultDialog from "./RemoveVaultDialog.svelte";
  import ImportButtercupDialog from "./ImportButtercupDialog.svelte";
  import ThemeToggle from "../../../components/ThemeToggle.svelte";
  import OpenVaultMenu from "./OpenVaultMenu.svelte";
  import Tabs from "../../../components/Tabs.svelte";
  import { LockIcon } from "../../../components/icons";
  import { Confirm } from "../../../components/dialog";
  import { open } from "@tauri-apps/plugin-dialog";

  let showCreate = false;
  let unlockTarget = null;
  let showOpenDropdown = false;
  let showButtercupImport = false;
  let dropdownPosition = { x: 0, y: 0 };

  let contextMenu = { show: false, x: 0, y: 0, vault: null };
  let showSettings = false;
  let settingsVault = null;
  let removeVault = null;
  let lockTarget = null;

  function handleCloseAllContextMenus() {
    closeContextMenu();
    showOpenDropdown = false;
  }

  useContextMenu(handleCloseAllContextMenus);

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

  function handleLock(vault) {
    lockTarget = vault;
  }

  async function handleLockConfirmed() {
    if (!lockTarget) return;
    await lockVaultByPath(lockTarget.path);
    lockTarget = null;
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

  // handleCancelRemove inlined — removeVault = null is set directly

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
    if (contextMenu.show) closeContextMenu();
    if (showOpenDropdown) showOpenDropdown = false;
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

  function handleButtercupImport() {
    showButtercupImport = true;
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="vault-tabs">
  <div class="tabs">
    <Tabs
      items={$vaults}
      getKey={(v) => v.id}
      getTitle={(v) => v.path}
      selectedKey={$currentVault ? $currentVault.id : null}
      onReorder={async (items) => reorderVaults(items.map((v) => v.id))}
      onSelect={selectVault}
      onKeydown={handleTabKeydown}
      onContextMenu={handleContextMenu}
    >
      {#snippet itemTab(vault)}
        {vault.name}
      {/snippet}
      {#snippet itemActions(vault)}
        {#if $vaultData[vault.path]?.unlocked}
          <button
            class="btn-icon tab-action-btn lock-tab-btn"
            onclick={() => handleLock(vault)}
            title="Lock vault"
          >
            <LockIcon size={18} />
          </button>
        {:else}
          <button
            class="btn-icon tab-action-btn delete-tab-btn"
            onclick={() => handleDelete(vault)}
            title="Remove vault"
          >
            ×
          </button>
        {/if}
      {/snippet}
    </Tabs>
  </div>

  <div class="tab-actions">
    <button class="btn-secondary" onclick={() => (showCreate = true)}>
      <span class="action-icon">+</span>
      <span>New Vault</span>
    </button>
    <OpenVaultMenu
      bind:dropdownPosition
      bind:showDropdown={showOpenDropdown}
      onpickexisting={pickExistingVault}
      onbuttercupimport={handleButtercupImport}
    />
    <ThemeToggle />
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

{#if showCreate}
  <CreateVaultDialog
    oncreated={() => (showCreate = false)}
    oncancel={() => (showCreate = false)}
  />
{/if}

{#if unlockTarget}
  <UnlockDialog
    path={unlockTarget.path}
    name={unlockTarget.name || unlockTarget.path}
    onUnlock={handleUnlock}
    onCancel={() => (unlockTarget = null)}
  />
{/if}

{#if showButtercupImport}
  <ImportButtercupDialog
    onsuccess={() => (showButtercupImport = false)}
    oncancel={() => (showButtercupImport = false)}
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

  .vault-load-error {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    color: var(--on-danger-container);
    background-color: var(--danger-container);
    border-bottom: 1px solid var(--border-color);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
