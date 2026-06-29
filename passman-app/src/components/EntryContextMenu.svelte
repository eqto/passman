<script>
  import { createEventDispatcher, onMount, afterUpdate } from "svelte";
  import { groups, vaults, currentVault } from "../stores/vaults";
  import { CONTEXT_MENU_WIDTH, CONTEXT_MENU_PADDING } from "../lib/constants.js";
  import MoveCopySubmenu from "./MoveCopySubmenu.svelte";

  export let x = 0;
  export let y = 0;
  export let entry = null;

  const dispatch = createEventDispatcher();
  let menuEl;
  let mainWidth = 0;
  let left = x;
  let top = y;
  let showMove = false;
  let showCopy = false;

  const MENU_WIDTH = CONTEXT_MENU_WIDTH;

  $: currentGroupTags = (entry?.tags || []).filter((tag) => $groups.includes(tag));
  $: moveGroups = ($groups || []).filter((group) => !currentGroupTags.includes(group));
  $: moveVaults = ($vaults || []).filter((vault) => vault.path !== $currentVault?.path);

  $: submenuLeft = computeSubmenuLeft(left, mainWidth);

  function computeSubmenuLeft(baseLeft, width) {
    const w = width || MENU_WIDTH;
    let nextLeft = baseLeft + w;
    if (nextLeft + w > window.innerWidth - CONTEXT_MENU_PADDING) {
      nextLeft = baseLeft - w;
    }
    if (nextLeft < CONTEXT_MENU_PADDING) nextLeft = CONTEXT_MENU_PADDING;
    return nextLeft;
  }

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
    dispatch("moveToVault", { entry, vault: event.detail.vault, group: event.detail.group });
  }

  function handleCopyToGroup(event) {
    dispatch("copyToGroup", { entry, group: event.detail });
  }

  function handleCopyToVault(event) {
    dispatch("copyToVault", { entry, vault: event.detail.vault, group: event.detail.group });
  }

  function openMove() {
    showMove = true;
    showCopy = false;
  }

  function openCopy() {
    showCopy = true;
    showMove = false;
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  bind:this={menuEl}
  bind:clientWidth={mainWidth}
  class="context-menu"
  style="left: {left}px; top: {top}px;"
  on:click|stopPropagation
>
  <button
    class="context-menu-item"
    on:click={handleCopyPassword}
    disabled={!entry?.password}
  >
    Copy Password
  </button>
  <div class="context-menu-divider"></div>
  <div class="menu-item-wrapper" on:mouseenter={openMove}>
    <button class="context-menu-item has-submenu">
      <span>Move to</span>
      <span class="arrow">▶</span>
    </button>
  </div>
  <div class="menu-item-wrapper" on:mouseenter={openCopy}>
    <button class="context-menu-item has-submenu">
      <span>Copy to</span>
      <span class="arrow">▶</span>
    </button>
  </div>
</div>

{#if showMove}
  <MoveCopySubmenu
    groups={moveGroups}
    vaults={moveVaults}
    left={submenuLeft}
    {top}
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
    {top}
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

  .context-menu {
    position: fixed;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    z-index: 100;
    width: 10rem;
    padding: 0.25rem;
    overflow: visible;
  }

  .context-menu-item {
    width: 100%;
    text-align: left;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 0.375rem;
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .context-menu-item:hover {
    background-color: var(--hover-bg);
  }

  .context-menu-item:disabled {
    color: var(--muted-color);
    cursor: not-allowed;
  }

  .context-menu-item:disabled:hover {
    background-color: transparent;
  }

  .context-menu-divider {
    height: 1px;
    background-color: var(--border-color);
    margin: 0.25rem 0.5rem;
  }

  .arrow {
    font-size: 0.75rem;
    opacity: 0.7;
  }
</style>
