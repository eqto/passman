<script>
  import { onMount, onDestroy } from "svelte";
  import VaultList from "./features/vault/components/VaultList.svelte";
  import VaultView from "./features/vault/components/VaultView.svelte";
  import UnlockDialog from "./features/vault/components/UnlockDialog.svelte";
  import AutoLock from "./components/AutoLock.svelte";
  import Toast from "./components/Toast.svelte";
  import {
    loadVaults,
    currentVault,
    isUnlocked,
    lockVault,
    unlockVault,
    initSaveListener,
  } from "./features/vault/store.js";

  let saveUnlisten = null;

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
        lockVault();
      }
    }
  }
</script>

<svelte:window
  on:keydown={handleKeydown}
  on:contextmenu|preventDefault={() => {}}
/>

<AutoLock />
<Toast />

<main>
  <VaultList />
  <div class="content">
    {#if $currentVault && $isUnlocked}
      {#key $currentVault.path}
        <VaultView />
      {/key}
    {:else if $currentVault && !$isUnlocked}
      <div class="locked-state">
        <UnlockDialog
          path={$currentVault.path}
          name={$currentVault.name}
          onUnlock={handleUnlock}
          onCancel={handleCancelUnlock}
        />
      </div>
    {:else}
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
