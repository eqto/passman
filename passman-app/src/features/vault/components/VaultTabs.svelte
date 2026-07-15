<script>
  import Tabs from "../../../components/Tab/Tabs.svelte";
  import Tab from "../../../components/Tab/Tab.svelte";
  import { Icon } from "../../../components/icons";
  import { vaults, currentVault, vaultData, reorderVaults } from "../store.js";

  let { onContextMenu, onLock, onRemove } = $props();

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
      onContextMenu?.(event, vault);
    }
  }
</script>

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
              onclick={() => onLock?.(vault)}
              title="Lock vault"
            >
              <Icon name="lock" size={18} />
            </button>
          {:else}
            <button
              class="btn-icon tab-action-btn delete-tab-btn"
              onclick={() => onRemove?.(vault)}
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
