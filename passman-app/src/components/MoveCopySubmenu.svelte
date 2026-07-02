<script>
  import { createEventDispatcher } from "svelte";
  import { vaultData } from "../stores/vaults";

  export let groups = [];
  export let vaults = [];
  export let left = 0;
  export let top = 0;
  export let menuWidth = 160;

  const dispatch = createEventDispatcher();
  let activeVault = null;
  let vaultTop = 0;

  const MENU_WIDTH = 160;

  $: vaultLeft = computeSubmenuLeft(left, menuWidth);

  function computeSubmenuLeft(baseLeft, width) {
    const w = width || MENU_WIDTH;
    let nextLeft = baseLeft + w;
    if (nextLeft + w > window.innerWidth - 8) {
      nextLeft = baseLeft - w;
    }
    if (nextLeft < 8) nextLeft = 8;
    return nextLeft;
  }

  function selectGroup(group) {
    dispatch("selectGroup", group);
  }

  function selectVaultGroup(vault, group) {
    dispatch("selectVaultGroup", { vault, group });
  }

  function handleVaultHover(vault, event) {
    activeVault = $vaultData[vault.path]?.unlocked ? vault : null;
    vaultTop = event?.currentTarget?.getBoundingClientRect().top ?? top;
  }

  $: targetVaultGroups = (vault) => $vaultData[vault.path]?.groups || [];
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  class="context-menu submenu"
  style="left: {left}px; top: {top}px; height: calc(100vh - {top}px - 8px);"
  on:click|stopPropagation
>
  {#if groups.length === 0 && vaults.length === 0}
    <div class="submenu-empty">No other groups or vaults</div>
  {:else}
    {#if groups.length > 0}
      {#each groups as group}
        <button class="submenu-item" on:click={() => selectGroup(group)}>
          {group}
        </button>
      {/each}
    {/if}
    {#if groups.length > 0 && vaults.length > 0}
      <div class="context-menu-divider"></div>
    {/if}
    {#if vaults.length > 0}
      {#each vaults as vault}
        <div
          class="menu-item-wrapper"
          on:mouseenter={(e) => handleVaultHover(vault, e)}
        >
          <button
            class="submenu-item has-submenu"
            disabled={!$vaultData[vault.path]?.unlocked}
          >
            <span>{vault.name}</span>
            {#if !$vaultData[vault.path]?.unlocked}
              <span class="locked-badge">locked</span>
            {:else}
              <span class="context-menu-arrow">▶</span>
            {/if}
          </button>
          {#if activeVault?.id === vault.id}
            <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
            <div
              class="context-menu submenu"
              style="left: {vaultLeft}px; top: {vaultTop}px; height: calc(100vh - {vaultTop}px - 8px);"
              on:click|stopPropagation
            >
              {#if targetVaultGroups(activeVault).length > 0}
                {#each targetVaultGroups(activeVault) as group}
                  <button class="submenu-item" on:click={() => selectVaultGroup(activeVault, group)}>
                    {group}
                  </button>
                {/each}
              {:else}
                <div class="submenu-empty">No groups</div>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  {/if}
</div>

<style>
  .menu-item-wrapper {
    position: relative;
    display: block;
  }

  .context-menu {
    width: 10rem;
  }

  .context-menu.submenu {
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: thin;
  }

  .submenu-item {
    justify-content: space-between;
    gap: 0.5rem;
  }

  .submenu-item:disabled {
    cursor: not-allowed;
  }

  .submenu-item:disabled:hover {
    background-color: transparent;
  }

  .submenu-empty {
    padding: 0.375rem 0.75rem;
    font-size: 0.875rem;
    color: var(--muted-color);
  }

  .locked-badge {
    font-size: 0.7rem;
    padding: 0.125rem 0.375rem;
    background-color: var(--hover-bg);
    border: 1px solid var(--border-color);
    border-radius: 9999px;
  }
</style>
