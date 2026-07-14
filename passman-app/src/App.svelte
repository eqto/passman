<script>
  import { onMount, onDestroy } from "svelte";
  import VaultTabs from "./features/vault/components/VaultTabs.svelte";
  import Topbar from "./features/vault/components/Topbar.svelte";
  import VaultView from "./features/vault/components/VaultView.svelte";
  import UnlockDialog from "./features/vault/components/UnlockDialog.svelte";
  import CreateVaultDialog from "./features/vault/components/CreateVaultDialog.svelte";
  import VaultSettingsDialog from "./features/vault/components/VaultSettingsDialog.svelte";
  import VaultContextMenu from "./features/vault/components/VaultContextMenu.svelte";
  import RemoveVaultDialog from "./features/vault/components/RemoveVaultDialog.svelte";
  import ImportButtercupDialog from "./features/vault/components/ImportButtercupDialog.svelte";
  import AutoLock from "./components/AutoLock.svelte";
  import { Confirm, Toast } from "./components/dialog";
  import { closeAllContextMenus } from "./stores/contextMenu.js";
  import { useContextMenu } from "./lib/createContextMenu.js";
  import { open } from "@tauri-apps/plugin-dialog";
  import {
    loadVaults,
    vaults,
    currentVault,
    vaultData,
    isUnlocked,
    loadError,
    openVault,
    registerAndOpenVault,
    lockVault,
    lockVaultByPath,
    unlockVault,
    deleteVault,
    reorderVaults,
    initSaveListener,
  } from "./features/vault/store.js";

  let saveUnlisten = null;
  let showLockConfirm = $state(false);

  let showCreate = $state(false);
  let unlockTarget = $state(null);
  let showOpenDropdown = $state(false);
  let showButtercupImport = $state(false);
  let dropdownPosition = $state({ x: 0, y: 0 });

  let contextMenu = $state({ show: false, x: 0, y: 0, vault: null });
  let showSettings = $state(false);
  let settingsVault = $state(null);
  let removeVault = $state(null);
  let lockTarget = $state(null);

  onMount(async () => {
    await loadVaults();
    try {
      saveUnlisten = await initSaveListener();
    } catch (e) {
      console.error("Failed to init save listener:", e);
    }
  });

  onDestroy(() => {
    if (saveUnlisten) saveUnlisten();
  });

  useContextMenu(handleWindowClick);

  async function handleUnlockCurrent(path, password) {
    await unlockVault(password);
  }

  function handleCancelUnlock() {
    currentVault.set(null);
  }

  async function handleOpenExisting(path, password) {
    if (!unlockTarget) return;
    if (unlockTarget.registered) {
      await openVault(path, password);
    } else {
      const id = crypto.randomUUID();
      await registerAndOpenVault(id, path, password);
    }
    unlockTarget = null;
  }

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

  function handleButtercupImport() {
    showButtercupImport = true;
  }

  function handleLock(vault) {
    lockTarget = vault;
  }

  function handleDelete(vault) {
    removeVault = vault;
  }

  async function handleLockConfirmed() {
    if (!lockTarget) return;
    await lockVaultByPath(lockTarget.path);
    lockTarget = null;
  }

  async function handleRemoveConfirmed() {
    if (!removeVault) return;
    const vault = removeVault;
    removeVault = null;
    const isUnlocked = $vaultData[vault.path]?.unlocked;
    if (isUnlocked) {
      await lockVaultByPath(vault.path);
    }
    try {
      await deleteVault(vault.id, vault.path);
    } catch (e) {
      console.error("Failed to remove vault:", e);
    }
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
    if (contextMenu.show) closeContextMenu();
    if (showOpenDropdown) showOpenDropdown = false;
  }

  function handleKeydown(event) {
    if ((event.ctrlKey || event.metaKey) && event.key === "l") {
      event.preventDefault();
      if ($currentVault && $isUnlocked) {
        showLockConfirm = true;
      }
    }
  }

  let unlockedVaults = $derived(
    ($vaults || []).filter((v) => $vaultData[v.path]?.unlocked),
  );
  let currentVaultUnlocked = $derived($currentVault && $isUnlocked);

  async function handleGlobalLockConfirmed() {
    showLockConfirm = false;
    await lockVault();
  }
</script>

<svelte:window
  onkeydown={handleKeydown}
  oncontextmenu={(e) => e.preventDefault()}
/>

<AutoLock />
<Toast />

{#if $loadError}
  <div class="vault-load-error" title={$loadError}>⚠ {$loadError}</div>
{/if}

{#if showLockConfirm}
  <Confirm
    title="Lock Vault"
    message={`Lock "${$currentVault?.name}"? You will need to re-enter the password to access it again.`}
    confirmLabel="Lock"
    onconfirm={handleGlobalLockConfirmed}
    oncancel={() => (showLockConfirm = false)}
  />
{/if}

<main>
  <header class="app-header">
    <VaultTabs
      onContextMenu={handleContextMenu}
      onLock={handleLock}
      onRemove={handleDelete}
    />
    <Topbar
      bind:dropdownPosition
      bind:showDropdown={showOpenDropdown}
      onCreate={() => (showCreate = true)}
      onPickExisting={pickExistingVault}
      onButtercupImport={handleButtercupImport}
    />
  </header>

  <div class="content">
    {#each unlockedVaults as vault (vault.path)}
      <div
        class="vault-view-wrapper"
        class:hidden={!($currentVault && $currentVault.path === vault.path)}
      >
        <VaultView {vault} />
      </div>
    {/each}
    {#if $currentVault && !currentVaultUnlocked}
      <div class="locked-state">
        <UnlockDialog
          path={$currentVault.path}
          name={$currentVault.name}
          onUnlock={handleUnlockCurrent}
          onCancel={handleCancelUnlock}
        />
      </div>
    {/if}
    {#if !$currentVault}
      <div class="empty-state">Select or create a vault to get started.</div>
    {/if}
  </div>
</main>

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
    onUnlock={handleOpenExisting}
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
  main {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    color: var(--text-color);
  }

  .app-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 1rem;
    background-color: var(--sidebar-bg);
    gap: 1rem;
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

  .content {
    flex: 1;
    overflow: hidden;
    display: flex;
    border-top: 1px solid var(--border-color);
  }

  .vault-view-wrapper {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  .vault-view-wrapper.hidden {
    display: none;
  }

  .empty-state,
  .locked-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--muted-color);
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
  }
</style>
