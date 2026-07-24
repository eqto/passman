<script>
  import { vaultData } from "../../vault/index.js";
  import { computeSubmenuLeft } from "../../../lib/menuPosition.js";

  let {
    x = 0,
    y = 0,
    type = "tag",
    item = "",
    groups = [],
    vaults = [],
    onmovetogroup = null,
    onmergegroup = null,
    onmovetovault = null,
    oncopytovault = null,
    onmovetotrash = null,
  } = $props();

  let activeMenu = $state(null);
  let mainWidth = $state(0);
  let mergeItemEl;
  let moveToVaultItemEl;
  let copyToVaultItemEl;
  let moveToGroupItemEl;
  let submenuTop = $state(y);

  const MENU_WIDTH = 160;

  const mergeTargets = $derived(
    type === "group" ? groups.filter((g) => g.id !== item) : [],
  );
  const moveToGroupTargets = $derived(type === "tag" ? groups : []);
  const unlockedVaults = $derived(
    vaults.filter((v) => $vaultData[v.path]?.unlocked),
  );
  const submenuLeft = $derived(computeSubmenuLeft(x, mainWidth));

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
    onmergegroup?.({ sourceId: item, targetId: target.id });
  }

  function handleMoveToGroup(target) {
    onmovetogroup?.({ item, target: target.id });
  }

  function handleMoveToVault(vault) {
    onmovetovault?.({ sourceId: item, targetPath: vault.path });
  }

  function handleCopyToVault(vault) {
    oncopytovault?.({ sourceId: item, targetPath: vault.path });
  }

  function handleMoveToTrash() {
    onmovetotrash?.({ groupId: item });
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  bind:clientWidth={mainWidth}
  class="menu"
  style="left: {x}px; top: {y}px;"
  onclick={(e) => e.stopPropagation()}
>
  {#if type === "group"}
    <div class="menu-item-wrapper" onmouseenter={() => openMenu("merge")}>
      <div bind:this={mergeItemEl} class="menu-item has-submenu">
        <span>Merge to group</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
    <div class="menu-item-wrapper" onmouseenter={() => openMenu("moveToVault")}>
      <div bind:this={moveToVaultItemEl} class="menu-item has-submenu">
        <span>Move to</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
    <div class="menu-item-wrapper" onmouseenter={() => openMenu("copyToVault")}>
      <div bind:this={copyToVaultItemEl} class="menu-item has-submenu">
        <span>Copy to</span>
        <span class="context-menu-arrow">▶</span>
      </div>
    </div>
    <div class="context-menu-divider"></div>
    <div class="menu-item-wrapper" onmouseenter={() => openMenu(null)}>
      <div class="menu-item danger" onclick={handleMoveToTrash}>
        <span>Move to trash</span>
      </div>
    </div>
  {/if}

  {#if type === "tag"}
    <div class="menu-item-wrapper" onmouseenter={() => openMenu("moveToGroup")}>
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
    onclick={(e) => e.stopPropagation()}
  >
    {#if activeMenu === "merge"}
      {#if mergeTargets.length > 0}
        {#each mergeTargets as target}
          <div class="menu-item" onclick={() => handleMerge(target)}>
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
            <div class="menu-item" onclick={() => handleMoveToGroup(target)}>
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
            onclick={() =>
              activeMenu === "moveToVault"
                ? handleMoveToVault(vault)
                : handleCopyToVault(vault)}
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

  .menu-item.danger {
    color: var(--danger-color);
  }

  .menu-item.danger:hover {
    background-color: var(--danger-container);
    color: var(--on-danger-container);
  }
</style>
