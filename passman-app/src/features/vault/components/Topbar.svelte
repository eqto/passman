<script>
  import { open } from "@tauri-apps/plugin-dialog";
  import ThemeToggle from "../../../components/ThemeToggle.svelte";
  import {
    OpenVaultMenu,
    CreateVaultDialog,
    ImportDialog,
    UnlockDialog,
    openVault,
    registerAndOpenVault,
  } from "../index.js";

  let showCreate = $state(false);
  let importFormat = $state(null);
  let showOpenDropdown = $state(false);
  let dropdownPosition = $state({ x: 0, y: 0 });
  let unlockTarget = $state(null);

  function handleWindowClick() {
    if (showOpenDropdown) showOpenDropdown = false;
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

  function handleButtercupImport() {
    importFormat = "buttercup";
  }

  function handleKeePassImport() {
    importFormat = "keepass";
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="topbar">
  <div class="topbar-left">
    <button class="btn-secondary" onclick={() => (showCreate = true)}>
      <span class="action-icon">+</span>
      <span class="action-text">New Vault</span>
    </button>
    <OpenVaultMenu
      bind:dropdownPosition
      bind:showDropdown={showOpenDropdown}
      onpickexisting={pickExistingVault}
      onbuttercupimport={handleButtercupImport}
      onkeepassimport={handleKeePassImport}
    />
  </div>
  <ThemeToggle />
</div>

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

{#if importFormat}
  <ImportDialog
    format={importFormat}
    onsuccess={() => (importFormat = null)}
    oncancel={() => (importFormat = null)}
  />
{/if}

<style>
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
    background-color: var(--bg-color);
    border-bottom: 1px solid var(--border-color);
  }

  .topbar-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .action-icon {
    margin-right: 0.25rem;
  }
</style>
