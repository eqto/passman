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
  } from "../stores/vaults";
  import UnlockDialog from "./UnlockDialog.svelte";
  import CreateVaultDialog from "./CreateVaultDialog.svelte";
  import VaultSettingsDialog from "./VaultSettingsDialog.svelte";
  import VaultContextMenu from "./VaultContextMenu.svelte";
  import RemoveVaultDialog from "./RemoveVaultDialog.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { createDragList } from "../lib/dragList.js";

  let showCreate = false;
  let unlockTarget = null;

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
            class="tab-action-btn lock-tab-btn"
            on:click|stopPropagation={() => handleLock(vault)}
            title="Lock vault"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 -960 960 960" fill="currentColor"><path d="M534.5-302.03Q557-324.06 557-355q0-30-22.67-54.5t-54.5-24.5q-31.83 0-54.33 24.5t-22.5 55q0 30.5 22.67 52.5t54.5 22q31.83 0 54.33-22.03ZM220-80q-24.75 0-42.37-17.63Q160-115.25 160-140v-434q0-24.75 17.63-42.38Q195.25-634 220-634h330v-96q0-78.85 55.61-134.42Q661.21-920 740.11-920q78.89 0 134.39 55.58Q930-808.85 930-730h-60q0-54-37.88-92t-92-38Q686-860 648-822.08q-38 37.91-38 92.08v96h130q24.75 0 42.38 17.62Q800-598.75 800-574v434q0 24.75-17.62 42.37Q764.75-80 740-80H220Z"/></svg>
          </button>
        {:else}
          <button
            class="tab-action-btn delete-tab-btn"
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
    <button class="new-vault-btn" on:click={() => showCreate = true}>
      + New Vault
    </button>
    <button class="open-vault-btn" on:click={pickExistingVault}>
      Open Vault File
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
    gap: 0.25rem;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 0.5rem;
    background: transparent;
    color: var(--text-color);
    cursor: grab;
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
    margin-left: 0.25rem;
    background: transparent;
    border: none;
    color: var(--muted-color);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 50%;
  }

  .tab-action-btn:hover {
    background-color: var(--hover-bg);
  }

  .tab.selected .lock-tab-btn {
    color: #ffffff;
    background-color: var(--accent-color);
  }

  .tab.selected .lock-tab-btn:hover {
    color: #ffffff;
    background-color: var(--accent-hover);
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
  }

  .new-vault-btn,
  .open-vault-btn {
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
  }

  .new-vault-btn {
    background-color: var(--accent-color);
    color: white;
  }

  .new-vault-btn:hover {
    background-color: var(--accent-hover);
  }

  .open-vault-btn {
    background-color: var(--hover-bg);
    color: var(--text-color);
  }

  .open-vault-btn:hover {
    filter: brightness(0.95);
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
