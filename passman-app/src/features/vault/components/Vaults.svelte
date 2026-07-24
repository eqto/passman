<script>
  import Tabs from "../../../components/Tab/Tabs.svelte";
  import Tab from "../../../components/Tab/Tab.svelte";
  import { Icon } from "../../../components/icons";
  import { Confirm } from "../../../components/dialog";
  import { createContextMenuState } from "../../../lib/createContextMenu.svelte.js";
  import {
    VaultContextMenu,
    VaultSettingsDialog,
    RemoveVaultDialog,
    VaultView,
    UnlockDialog,
    Topbar,
    CreateVaultDialog,
    registerAndOpenVault,
    vaults,
    currentVault,
    vaultData,
    isUnlocked,
    reorderVaults,
    lockVaultByPath,
    deleteVault,
    unlockVault,
    lockVault,
  } from "../index.js";

  const {
    state: contextMenu,
    open: openContextMenu,
    close: closeContextMenu,
  } = createContextMenuState({ vault: null });
  let showSettings = $state(false);
  let settingsVault = $state(null);
  let removeVault = $state(null);
  let showLockConfirm = $state(false);
  let showUnlockDialog = $state(false);
  let unlockTargetVault = $state(null);
  let showCreate = $state(false);

  let canLockVault = $derived(
    !!contextMenu.vault && !!$vaultData[contextMenu.vault.path],
  );

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
      openContextMenu(event, { vault });
    }
  }

  function handleWindowClick() {
    if (contextMenu.show) closeContextMenu();
  }

  function handleLockVault() {
    if (contextMenu.vault && $vaultData[contextMenu.vault.path]) {
      lockVaultByPath(contextMenu.vault.path);
    }
    closeContextMenu();
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

  function handleUnlockClick(vault) {
    unlockTargetVault = { ...vault, registered: true };
    showUnlockDialog = true;
  }

  async function handleUnlockCurrent(path, password) {
    if (unlockTargetVault?.registered) {
      await unlockVault(password);
    } else {
      const id = crypto.randomUUID();
      await registerAndOpenVault(id, path, password);
    }
    showUnlockDialog = false;
    unlockTargetVault = null;
  }

  function handleCancelUnlock() {
    showUnlockDialog = false;
    unlockTargetVault = null;
  }

  function handleCloseTab(id) {
    const vault = $vaults.find((v) => v.id === id);
    if (vault) handleRemove(vault);
  }

  function handleKeydown(event) {
    if ((event.ctrlKey || event.metaKey) && event.key === "l") {
      event.preventDefault();
      if ($currentVault && $isUnlocked) {
        showLockConfirm = true;
      }
    }
  }

  async function handleGlobalLockConfirmed() {
    showLockConfirm = false;
    await lockVault();
  }
</script>

<svelte:window
  oncontextmenu={(e) => e.preventDefault()}
  onclick={handleWindowClick}
  onkeydown={handleKeydown}
/>

<div class="vault-tabs">
  <Topbar />
  <Tabs
    selectedKey={$currentVault ? $currentVault.id : null}
    onSelect={selectVault}
    onReorder={reorderVaults}
    onKeydown={handleTabKeydown}
    onContextMenu={handleContextMenu}
    onClose={handleCloseTab}
  >
    {#each $vaults as vault (vault.id)}
      <Tab name={vault.id} label={vault.name} title={vault.path}>
        {#if $vaultData[vault.path]?.unlocked}
          <VaultView {vault} />
        {:else}
          <div class="locked-state">
            <div class="locked-content">
              <Icon name="lock" size={48} />
              <h2>{vault.name} is locked</h2>
              <p>Enter your password to access this vault.</p>
              <button
                class="btn-primary"
                onclick={() => handleUnlockClick(vault)}
              >
                Unlock
              </button>
            </div>
          </div>
        {/if}
      </Tab>
    {/each}
  </Tabs>
  {#if !$currentVault}
    <div class="empty-state">
      <div class="empty-state-content">
        <Icon name="key" size={64} />
        <h2>Welcome to Passman</h2>
        <p>
          Select an open vault tab, or choose an action below to get started.
        </p>
        <div class="empty-state-actions">
          <button class="btn-primary" onclick={() => (showCreate = true)}>
            Create New Vault
          </button>
        </div>
        <p class="empty-state-hint">
          or open an existing vault from the toolbar
        </p>
      </div>
    </div>
  {/if}
</div>

{#if contextMenu.show}
  <VaultContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    canLock={canLockVault}
    canRename={contextMenu.vault &&
      $currentVault &&
      $currentVault.id === contextMenu.vault.id &&
      $isUnlocked}
    onlock={handleLockVault}
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

{#if showUnlockDialog && unlockTargetVault}
  <UnlockDialog
    path={unlockTargetVault.path}
    name={unlockTargetVault.name}
    onUnlock={handleUnlockCurrent}
    onCancel={handleCancelUnlock}
  />
{/if}

{#if removeVault}
  <RemoveVaultDialog
    vault={removeVault}
    onRemove={handleRemoveConfirmed}
    onCancel={() => (removeVault = null)}
  />
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

{#if showCreate}
  <CreateVaultDialog
    oncreated={() => (showCreate = false)}
    oncancel={() => (showCreate = false)}
  />
{/if}

<style>
  .vault-tabs {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }

  .locked-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--muted-color);
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 2rem;
    color: var(--muted-color);
    pointer-events: none;
  }

  .empty-state-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.25rem;
    max-width: 24rem;
    pointer-events: auto;
  }

  .empty-state-content h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-color);
  }

  .empty-state-content p {
    margin: 0;
    font-size: 0.95rem;
    color: var(--muted-color);
  }

  .empty-state-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .locked-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    text-align: center;
  }

  .locked-content h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .locked-content p {
    margin: 0;
    color: var(--muted-color);
  }

  .empty-state-hint {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--muted-color);
  }
</style>
