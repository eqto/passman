<script>
  import { onMount } from "svelte";
  import { groups, vaults, currentVault } from "../../vault/index.js";
  import {
    CONTEXT_MENU_WIDTH,
    CONTEXT_MENU_PADDING,
  } from "../../../lib/constants.js";
  import { computeSubmenuLeft } from "../../../lib/menuPosition.js";
  import MoveCopySubmenu from "./MoveCopySubmenu.svelte";

  let {
    x = 0,
    y = 0,
    entry = null,
    oncopyPassword = null,
    onmoveToGroup = null,
    onmoveToVault = null,
    oncopyToGroup = null,
    oncopyToVault = null,
  } = $props();

  let menuEl = $state();
  let moveItemEl = $state();
  let copyItemEl = $state();
  let mainWidth = $state(0);
  let left = $state(x);
  let top = $state(y);
  let showMove = $state(false);
  let showCopy = $state(false);
  let moveItemTop = $state(y);
  let copyItemTop = $state(y);

  const MENU_WIDTH = CONTEXT_MENU_WIDTH;

  let currentGroupId = $derived(entry?.group_id || null);
  let moveGroups = $derived(
    ($groups || []).filter((group) => group.id !== currentGroupId),
  );
  let moveVaults = $derived(
    ($vaults || []).filter((vault) => vault.path !== $currentVault?.path),
  );

  let submenuLeft = $derived(computeSubmenuLeft(left, mainWidth));

  function adjustPosition() {
    if (!menuEl) return;
    const rect = menuEl.getBoundingClientRect();
    let nextLeft = x;
    let nextTop = y;
    if (nextLeft + rect.width > window.innerWidth - CONTEXT_MENU_PADDING) {
      nextLeft = window.innerWidth - rect.width - CONTEXT_MENU_PADDING;
    }
    if (nextLeft < CONTEXT_MENU_PADDING) nextLeft = CONTEXT_MENU_PADDING;
    if (nextTop + rect.height > window.innerHeight - CONTEXT_MENU_PADDING) {
      nextTop = window.innerHeight - rect.height - CONTEXT_MENU_PADDING;
    }
    if (nextTop < CONTEXT_MENU_PADDING) nextTop = CONTEXT_MENU_PADDING;
    if (window.innerHeight - nextTop - CONTEXT_MENU_PADDING < 250) {
      nextTop = window.innerHeight - 250 - CONTEXT_MENU_PADDING;
      if (nextTop < CONTEXT_MENU_PADDING) nextTop = CONTEXT_MENU_PADDING;
    }
    if (left !== nextLeft) left = nextLeft;
    if (top !== nextTop) top = nextTop;
  }

  onMount(() => adjustPosition());

  $effect(() => {
    // Re-run adjustPosition after DOM updates
    adjustPosition();
  });

  function handleCopyPassword() {
    if (entry?.password) {
      oncopyPassword?.(entry);
    }
  }

  function handleMoveToGroup(detail) {
    onmoveToGroup?.({ entry, group: detail });
  }

  function handleMoveToVault(detail) {
    onmoveToVault?.({
      entry,
      vault: detail.vault,
      groupId: detail.groupId,
    });
  }

  function handleCopyToGroup(detail) {
    oncopyToGroup?.({ entry, group: detail });
  }

  function handleCopyToVault(detail) {
    oncopyToVault?.({
      entry,
      vault: detail.vault,
      groupId: detail.groupId,
    });
  }

  function openMove() {
    showMove = true;
    showCopy = false;
    if (moveItemEl) {
      moveItemTop = moveItemEl.getBoundingClientRect().top;
    }
  }

  function openCopy() {
    showCopy = true;
    showMove = false;
    if (copyItemEl) {
      copyItemTop = copyItemEl.getBoundingClientRect().top;
    }
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  bind:this={menuEl}
  bind:clientWidth={mainWidth}
  class="menu"
  style="left: {left}px; top: {top}px;"
  onclick={(e) => e.stopPropagation()}
>
  <div
    class="menu-item"
    onclick={handleCopyPassword}
    style="cursor: {!entry?.password
      ? 'not-allowed'
      : 'pointer'}; opacity: {!entry?.password ? '0.6' : '1'}"
  >
    Copy Password
  </div>
  <div class="context-menu-divider"></div>
  <div class="menu-item-wrapper" onmouseenter={openMove}>
    <div bind:this={moveItemEl} class="menu-item has-submenu">
      <span>Move to</span>
      <span class="context-menu-arrow">▶</span>
    </div>
  </div>
  <div class="menu-item-wrapper" onmouseenter={openCopy}>
    <div bind:this={copyItemEl} class="menu-item has-submenu">
      <span>Copy to</span>
      <span class="context-menu-arrow">▶</span>
    </div>
  </div>
</div>

{#if showMove}
  <MoveCopySubmenu
    groups={moveGroups}
    vaults={moveVaults}
    left={submenuLeft}
    top={moveItemTop}
    menuWidth={mainWidth}
    onselectGroup={handleMoveToGroup}
    onselectVaultGroup={handleMoveToVault}
  />
{/if}

{#if showCopy}
  <MoveCopySubmenu
    groups={moveGroups}
    vaults={moveVaults}
    left={submenuLeft}
    top={copyItemTop}
    menuWidth={mainWidth}
    onselectGroup={handleCopyToGroup}
    onselectVaultGroup={handleCopyToVault}
  />
{/if}

<style>
  .menu-item-wrapper {
    position: relative;
    display: block;
  }

  .menu {
    width: 10rem;
  }

  .menu-item.has-submenu {
    cursor: default;
  }
</style>
