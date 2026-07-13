<script>
  import { onMount, onDestroy } from "svelte";
  import VaultList from "./features/vault/components/VaultList.svelte";
  import VaultView from "./features/vault/components/VaultView.svelte";
  import UnlockDialog from "./features/vault/components/UnlockDialog.svelte";
  import AutoLock from "./components/AutoLock.svelte";
  import { Confirm, Toast } from "./components/dialog";
  import {
    loadVaults,
    vaults,
    currentVault,
    vaultData,
    isUnlocked,
    lockVault,
    unlockVault,
    initSaveListener,
  } from "./features/vault/store.js";

  let saveUnlisten = null;
  let showLockConfirm = false;

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

  async function handleUnlock(path, password) {
    await unlockVault(password);
  }

  function handleCancelUnlock() {
    currentVault.set(null);
  }

  function handleKeydown(event) {
    if ((event.ctrlKey || event.metaKey) && event.key === "l") {
      event.preventDefault();
      if ($currentVault && $isUnlocked) {
        showLockConfirm = true;
      }
    }
  }

  $: unlockedVaults = ($vaults || []).filter(
    (v) => $vaultData[v.path]?.unlocked,
  );
  $: currentVaultUnlocked = $currentVault && $isUnlocked;

  async function handleLockConfirmed() {
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

{#if showLockConfirm}
  <Confirm
    title="Lock Vault"
    message={`Lock "${$currentVault?.name}"? You will need to re-enter the password to access it again.`}
    confirmLabel="Lock"
    onconfirm={handleLockConfirmed}
    oncancel={() => (showLockConfirm = false)}
  />
{/if}

<main>
  <VaultList />
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
          onUnlock={handleUnlock}
          onCancel={handleCancelUnlock}
        />
      </div>
    {/if}
    {#if !$currentVault}
      <div class="empty-state">Select or create a vault to get started.</div>
    {/if}
  </div>
</main>

<style>
  main {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    color: var(--text-color);
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
