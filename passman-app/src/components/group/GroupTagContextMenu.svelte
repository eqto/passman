<script>
  import { createEventDispatcher } from "svelte";
  import { vaultData } from "../../stores/vaults";
  import { computeSubmenuLeft } from "../../lib/menuPosition.js";

  export let x = 0;
  export let y = 0;
  export let type = "tag";
  export let item = "";
  export let groups = [];
  export let vaults = [];

  const dispatch = createEventDispatcher();
  let activeMenu = null;
  let mainWidth = 0;
  let mergeItemEl;
  let moveToVaultItemEl;
  let copyToVaultItemEl;
  let moveToGroupItemEl;
  let submenuTop = y;

  const MENU_WIDTH = 160;

  $: mergeTargets = type === "group" ? groups.filter((g) => g.id !== item) : [];
  $: moveToGroupTargets = type === "tag" ? groups : [];
  $: unlockedVaults = vaults.filter((v) => $vaultData[v.path]?.unlocked);
  $: submenuLeft = computeSubmenuLeft(x, mainWidth);

  function getGroupName(groupId) {
    const group = groups.find((g) => g.id === groupId);
    return group ? group.name : groupId;
  }

  function openMenu(menu) {
    activeMenu = menu;
    let itemEl;
    if (menu === "merge") itemEl = mergeItemEl;
    else if (menu === "moveToVault") itemEl = moveToVaultItemEl;
    else if (menu === "copyToVault") itemEl = copyToVaultItemEl;
    else if (menu === "moveToGroup") itemEl = moveToGroupItemEl;
    
    if (itemEl) {
      submenuTop = itemEl.getBoundingClientRect().top;
    }
  }

  function handleMerge(target) {
    dispatch("mergeGroup", { sourceId: item, targetId: target.id });
  }

  function handleMoveToGroup(target) {
    dispatch("moveToGroup", { item, target: target.id });
  }

  function handleMoveToVault(vault) {
    dispatch("moveToVault", { sourceId: item, targetPath: vault.path });
  }

  function handleCopyToVault(vault) {
    dispatch("copyToVault", { sourceId: item, targetPath: vault.path });
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  bind:clientWidth={mainWidth}
  class="menu"
  style="left: {x}px; top: {y}px;"
  on:click|stopPropagation
>
  {#if type === "group"}
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("merge")}>
      <div bind:this={mergeItemEl} class="menu-item has-submenu">
        <span>Merge to group</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("moveToVault")}>
      <div bind:this={moveToVaultItemEl} class="menu-item has-submenu">
        <span>Move to</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("copyToVault")}>
      <div bind:this={copyToVaultItemEl} class="menu-item has-submenu">
        <span>Copy to</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
  {/if}

  {#if type === "tag"}
    <div class="menu-item-wrapper" on:mouseenter={() => openMenu("moveToGroup")}>
      <div bind:this={moveToGroupItemEl} class="menu-item has-submenu">
        <span>Move to group</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
  {/if}
</div>

{#if activeMenu}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div
    class="menu"
    style="left: {submenuLeft}px; top: {submenuTop}px"
    on:click|stopPropagation
  >
    {#if activeMenu === "merge"}
      {#if mergeTargets.length > 0}
        {#each mergeTargets as target}
          <div class="menu-item" on:click={() => handleMerge(target)}>
            {target.name}
          </div>
        {/each}
      {:else}
        <div class="submenu-empty">No other groups</div>
      {/if}
    {:else if activeMenu === "moveToGroup"}
      {#if moveToGroupTargets.length > 0}
        {#each moveToGroupTargets as target}
          {#if target.id !== item}
            <div class="menu-item" on:click={() => handleMoveToGroup(target)}>
              {target.name}
            </div>
          {/if}
        {/each}
      {:else}
        <div class="submenu-empty">No groups</div>
      {/if}
    {:else if activeMenu === "moveToVault" || activeMenu === "copyToVault"}
      {#if unlockedVaults.length > 0}
        {#each unlockedVaults as vault}
          <div
            class="menu-item"
            on:click={() =>
              activeMenu === "moveToVault" ? handleMoveToVault(vault) : handleCopyToVault(vault)}
          >
            {vault.name}
          </div>
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

  .menu {
    min-width: 10rem;
  }

  .menu-item.has-submenu {
    justify-content: space-between;
    gap: 0.5rem;
  }

  .submenu-empty {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    color: var(--muted-color);
  }
</style>
