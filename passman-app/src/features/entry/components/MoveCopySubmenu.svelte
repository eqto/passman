<script>
  import { createEventDispatcher, onMount, afterUpdate } from "svelte";
  import { vaultData } from "../../vault/store.js";
  import { computeSubmenuLeft } from "../../../lib/menuPosition.js";

  export let groups = [];
  export let vaults = [];
  export let left = 0;
  export let top = 0;
  export let menuWidth = 160;

  const dispatch = createEventDispatcher();
  let activeVault = null;
  let vaultTop = 0;
  let menuEl;
  let adjustedTop = top;

  const MENU_WIDTH = 160;
  const PADDING = 8;

  $: vaultLeft = computeSubmenuLeft(left, menuWidth);

  function adjustPosition() {
    if (!menuEl) return;
    const rect = menuEl.getBoundingClientRect();
    const viewportHeight = window.innerHeight;

    if (rect.top + rect.height > viewportHeight - PADDING) {
      adjustedTop = viewportHeight - rect.height - PADDING;
      if (adjustedTop < PADDING) adjustedTop = PADDING;
    } else {
      adjustedTop = top;
    }
  }

  onMount(() => adjustPosition());
  afterUpdate(() => adjustPosition());

  function selectGroup(group) {
    dispatch("selectGroup", group.id);
  }

  function selectVaultGroup(vault, group) {
    dispatch("selectVaultGroup", { vault, groupId: group.id });
  }

  function handleVaultHover(vault, event) {
    activeVault = $vaultData[vault.path]?.unlocked ? vault : null;
    vaultTop = event?.currentTarget?.getBoundingClientRect().top ?? top;
  }

  $: targetVaultGroups = (vault) => $vaultData[vault.path]?.groups || [];
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  bind:this={menuEl}
  class="menu"
  style="left: {left}px; top: {adjustedTop}px"
  on:click|stopPropagation
>
  {#if groups.length === 0 && vaults.length === 0}
    <div class="submenu-empty">No other groups or vaults</div>
  {:else}
    {#if groups.length > 0}
      {#each groups as group}
        <div class="menu-item" on:click={() => selectGroup(group)}>
          {group.name}
        </div>
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
          <div
            class="menu-item has-submenu"
            style="cursor: {!$vaultData[vault.path]?.unlocked
              ? 'not-allowed'
              : 'default'}"
          >
            <span>{vault.name}</span>
            {#if !$vaultData[vault.path]?.unlocked}
              <span class="locked-badge">locked</span>
            {:else}
              <span class="context-menu-arrow">▶</span>
            {/if}
          </div>
          {#if activeVault?.id === vault.id}
            <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
            <div
              class="menu"
              style="left: {vaultLeft}px; top: {vaultTop}px"
              on:click|stopPropagation
            >
              {#if targetVaultGroups(activeVault).length > 0}
                {#each targetVaultGroups(activeVault) as group}
                  <div
                    class="menu-item"
                    on:click={() => selectVaultGroup(activeVault, group)}
                  >
                    {group.name}
                  </div>
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

  .menu-item.has-submenu {
    justify-content: space-between;
    gap: 0.5rem;
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
    border-radius: var(--shape-full);
  }
</style>
