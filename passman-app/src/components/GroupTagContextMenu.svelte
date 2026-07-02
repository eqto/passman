<script>
  import { createEventDispatcher } from "svelte";
  import { vaultData } from "../stores/vaults";

  export let x = 0;
  export let y = 0;
  export let type = "tag";
  export let item = "";
  export let groups = [];
  export let vaults = [];

  const dispatch = createEventDispatcher();
  let activeMenu = null;
  let mainWidth = 0;

  const MENU_WIDTH = 160;

  $: mergeTargets = type === "group" ? groups.filter((g) => g !== item) : [];
  $: moveToGroupTargets = type === "tag" ? groups : [];
  $: unlockedVaults = vaults.filter((v) => $vaultData[v.path]?.unlocked);
  $: submenuLeft = computeSubmenuLeft(x, mainWidth);

  function computeSubmenuLeft(baseLeft, width) {
    const w = width || MENU_WIDTH;
    let nextLeft = baseLeft + w;
    if (nextLeft + w > window.innerWidth - 8) {
      nextLeft = baseLeft - w;
    }
    if (nextLeft < 8) nextLeft = 8;
    return nextLeft;
  }

  function openMenu(menu) {
    activeMenu = menu;
  }

  function handleMerge(target) {
    dispatch("mergeGroup", { source: item, target });
  }

  function handleMoveToGroup(target) {
    dispatch("moveToGroup", { item, target });
  }

  function handleMoveToVault(vault) {
    dispatch("moveToVault", { source: item, targetPath: vault.path });
  }

  function handleCopyToVault(vault) {
    dispatch("copyToVault", { source: item, targetPath: vault.path });
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  bind:clientWidth={mainWidth}
  class="context-menu"
  style="left: {x}px; top: {y}px;"
  on:click|stopPropagation
>
  {#if type === "group"}
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("merge")}>
      <button class="context-menu-item has-submenu">
        <span>Merge to group</span>
        <span class="context-menu-arrow">▶</span>
      </button>
    </div>
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("moveToVault")}>
      <button class="context-menu-item has-submenu">
        <span>Move to</span>
        <span class="context-menu-arrow">▶</span>
      </button>
    </div>
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("copyToVault")}>
      <button class="context-menu-item has-submenu">
        <span>Copy to</span>
        <span class="context-menu-arrow">▶</span>
      </button>
    </div>
  {/if}

  {#if type === "tag"}
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("moveToGroup")}>
      <button class="context-menu-item has-submenu">
        <span>Move to group</span>
        <span class="context-menu-arrow">▶</span>
      </button>
    </div>
  {/if}
</div>

{#if activeMenu}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div
    class="context-menu submenu"
    style="left: {submenuLeft}px; top: {y}px; max-height: calc(100vh - {y}px - 8px);"
    on:click|stopPropagation
  >
    {#if activeMenu === "merge"}
      {#if mergeTargets.length > 0}
        {#each mergeTargets as target}
          <button class="submenu-item" on:click={() => handleMerge(target)}>
            {target}
          </button>
        {/each}
      {:else}
        <div class="submenu-empty">No other groups</div>
      {/if}
    {:else if activeMenu === "moveToGroup"}
      {#if moveToGroupTargets.length > 0}
        {#each moveToGroupTargets as target}
          {#if target !== item}
            <button class="submenu-item" on:click={() => handleMoveToGroup(target)}>
              {target}
            </button>
          {/if}
        {/each}
      {:else}
        <div class="submenu-empty">No groups</div>
      {/if}
    {:else if activeMenu === "moveToVault" || activeMenu === "copyToVault"}
      {#if unlockedVaults.length > 0}
        {#each unlockedVaults as vault}
          <button
            class="submenu-item"
            on:click={() =>
              activeMenu === "moveToVault" ? handleMoveToVault(vault) : handleCopyToVault(vault)}
          >
            {vault.name}
          </button>
        {/each}
      {:else}
        <div class="submenu-empty">No unlocked vaults</div>
      {/if}
    {/if}
  </div>
{/if}

<style>
  .menu-item-wrapper {
    position: relative;
    display: block;
  }

  .context-menu {
    min-width: 10rem;
  }

  .context-menu.submenu {
    overflow-y: auto;
  }

  .context-menu-item,
  .submenu-item {
    justify-content: space-between;
    gap: 0.5rem;
  }

  .submenu-empty {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    color: var(--muted-color);
  }
</style>
