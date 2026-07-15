<script>
  import { onMount, onDestroy } from "svelte";
  import { Vaults, Topbar } from "./features/vault/index.js";
  import AutoLock from "./components/AutoLock.svelte";
  import { Toast } from "./components/dialog";
  import {
    loadVaults,
    loadError,
    initSaveListener,
  } from "./features/vault/index.js";

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
</script>

<AutoLock />
<Toast />

{#if $loadError}
  <div class="vault-load-error" title={$loadError}>⚠ {$loadError}</div>
{/if}

<main>
  <header class="app-header">
    <Vaults />
    <Topbar />
  </header>
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
</style>
