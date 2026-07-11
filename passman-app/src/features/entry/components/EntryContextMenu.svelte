<script>
  import { createEventDispatcher, onMount, afterUpdate } from "svelte";
  import { groups, vaults, currentVault } from "../../vault/store.js";
  import {
    CONTEXT_MENU_WIDTH,
    CONTEXT_MENU_PADDING,
  } from "../../../lib/constants.js";
  import { computeSubmenuLeft } from "../../../lib/menuPosition.js";
  import MoveCopySubmenu from "./MoveCopySubmenu.svelte";

  export let x = 0;
  export let y = 0;
  export let entry = null;

  const dispatch = createEventDispatcher();
  let menuEl;
  let moveItemEl;
  let copyItemEl;
  let mainWidth = 0;
  let left = x;
  let top = y;
  let showMove = false;
  let showCopy = false;
  let moveItemTop = y;
  let copyItemTop = y;

  const MENU_WIDTH = CONTEXT_MENU_WIDTH;

  $: currentGroupId = entry?.group_id || null;
  $: moveGroups = ($groups || []).filter(
    (group) => group.id !== currentGroupId,
  );
  $: moveVaults = ($vaults || []).filter(
    (vault) => vault.path !== $currentVault?.path,
  );

  $: submenuLeft = computeSubmenuLeft(left, mainWidth);

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
  afterUpdate(() => adjustPosition());

  function handleCopyPassword() {
    if (entry?.password) {
      dispatch("copyPassword", entry);
    }
  }

  function handleMoveToGroup(event) {
    dispatch("moveToGroup", { entry, group: event.detail });
  }

  function handleMoveToVault(event) {
    dispatch("moveToVault", {
      entry,
      vault: event.detail.vault,
      group: event.detail.group,
    });
  }

  function handleCopyToGroup(event) {
    dispatch("copyToGroup", { entry, group: event.detail });
  }

  function handleCopyToVault(event) {
    dispatch("copyToVault", {
      entry,
      vault: event.detail.vault,
      group: event.detail.group,
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
  on:click|stopPropagation
>
  <div
    class="menu-item"
    on:click={handleCopyPassword}
    style="cursor: {!entry?.password
      ? 'not-allowed'
      : 'pointer'}; opacity: {!entry?.password ? '0.6' : '1'}"
  >
    Copy Password
  </div>
  <div class="context-menu-divider"></div>
  <div class="menu-item-wrapper" on:mouseenter={openMove}>
    <div bind:this={moveItemEl} class="menu-item has-submenu">
      <span>Move to</span>
      <span class="context-menu-arrow">▶</span>
    </div>
  </div>
  <div class="menu-item-wrapper" on:mouseenter={openCopy}>
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
    on:selectGroup={handleMoveToGroup}
    on:selectVaultGroup={handleMoveToVault}
  />
{/if}

{#if showCopy}
  <MoveCopySubmenu
    groups={moveGroups}
    vaults={moveVaults}
    left={submenuLeft}
    top={copyItemTop}
    menuWidth={mainWidth}
    on:selectGroup={handleCopyToGroup}
    on:selectVaultGroup={handleCopyToVault}
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
