<script>
  import { onMount, onDestroy } from "svelte";
  import * as vaultService from "../bindings/github.com/eqto/passman/internal/app/vaultservice.js";
  import { Vaults, vaults, initSaveListener } from "./features/vault/index.js";
  import AutoLock from "./components/AutoLock.svelte";
  import { Toast } from "./components/dialog";

  let saveUnlisten = null;
  let loadError = $state(null);

  onMount(async () => {
    try {
      const config = await vaultService.ListVaults();
      const list = Array.isArray(config?.vaults) ? config.vaults : [];
      vaults.set(list);
    } catch (e) {
      console.error("Failed to load vaults:", e);
      loadError = e.message || String(e);
      vaults.set([]);
    }
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

{#if loadError}
  <div class="vault-load-error" title={loadError}>⚠ {loadError}</div>
{/if}

<main>
  <Vaults />
</main>

<style>
  main {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    color: var(--text-color);
    position: relative;
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
