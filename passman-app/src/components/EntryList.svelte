<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { currentVault, vaultData, setVaultViewState, groups } from "../stores/vaults";
  import { freeTags } from "../lib/tags.js";
  import { debounce } from "../lib/debounce.js";
  import { showToast } from "../stores/toast.js";
  import { closeAllContextMenus } from "../stores/contextMenu.js";
  import EntryContextMenu from "./EntryContextMenu.svelte";
  import { onMount } from "svelte";

  const SEARCH_DEBOUNCE_MS = 150;

  export let entries;
  export let selectedEntry = null;
  export let selectedTags = [];
  export let trashMode = false;
  export let onSelect;
  export let onNew;
  export let onToggleTag = (tag) => {};
  export let onClearTags = () => {};
  export let onMoveToGroup = () => {};
  export let onMoveToVault = () => {};
  export let onCopyToGroup = () => {};
  export let onCopyToVault = () => {};

  let search = "";
  let filterSearch = "";
  let contextMenu = { show: false, x: 0, y: 0, entry: null };

  const setFilterSearch = debounce((value) => {
    filterSearch = value;
  }, SEARCH_DEBOUNCE_MS);

  onMount(() => {
    window.addEventListener('close-all-context-menus', closeContextMenu);
    return () => {
      window.removeEventListener('close-all-context-menus', closeContextMenu);
    };
  });

  $: if ($currentVault) {
    const viewState = $vaultData[$currentVault.path]?.viewState || {};
    search = viewState.search || "";
    filterSearch = search;
  }

  function saveState() {
    if ($currentVault) {
      setVaultViewState($currentVault.path, { search });
    }
  }

  function toggleTag(tag) {
    onToggleTag(tag);
  }

  function clearTagFilter() {
    onClearTags();
  }

  function onSearchInput(event) {
    search = event.target.value;
    setFilterSearch(search);
    saveState();
  }

  $: filtered = entries.filter((e) => {
    const q = filterSearch.toLowerCase();
    const matchesSearch =
      e.title.toLowerCase().includes(q) ||
      e.username.toLowerCase().includes(q) ||
      e.url.toLowerCase().includes(q) ||
      e.notes.toLowerCase().includes(q) ||
      (e.tags || []).some((tag) => tag.toLowerCase().includes(q));
    if (!matchesSearch) return false;
    if (selectedTags.length > 0) {
      return selectedTags.every((tag) => (e.tags || []).includes(tag));
    }
    return true;
  });

  async function copyPassword(event, password) {
    event.stopPropagation();
    if (!password) return;
    await writeText(password);
    showToast("Password copied to clipboard");
  }

  function openContextMenu(event, entry) {
    if (trashMode) return;
    event.preventDefault();
    closeAllContextMenus();
    contextMenu = { show: true, x: event.clientX, y: event.clientY, entry };
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, entry: null };
  }

  async function handleMenuCopyPassword() {
    if (contextMenu.entry?.password) {
      await writeText(contextMenu.entry.password);
      showToast("Password copied to clipboard");
    }
    closeContextMenu();
  }

  function handleMenuMoveToGroup(event) {
    onMoveToGroup(event.detail.entry, event.detail.group);
    closeContextMenu();
  }

  function handleMenuMoveToVault(event) {
    onMoveToVault(event.detail.entry, event.detail.vault, event.detail.group);
    closeContextMenu();
  }

  function handleMenuCopyToGroup(event) {
    onCopyToGroup(event.detail.entry, event.detail.group);
    closeContextMenu();
  }

  function handleMenuCopyToVault(event) {
    onCopyToVault(event.detail.entry, event.detail.vault, event.detail.group);
    closeContextMenu();
  }

  function handleEntryKeydown(event, entry) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onSelect(entry);
    }
  }
</script>


<svelte:window on:click={closeContextMenu} />

<div class="entry-list">
  <div class="list-header section-header">
    <span class="count">{filtered.length} entries</span>
  </div>

  <input
    value={search}
    on:input={onSearchInput}
    placeholder="Search entries..."
    class="modal-input search-input"
  />

  {#if selectedTags.length > 0}
    <div class="tag-filter-bar">
      {#each selectedTags as tag}
        <button class="tag-filter-chip" on:click={() => toggleTag(tag)}>
          {tag} <span class="remove">×</span>
        </button>
      {/each}
      <button class="btn-ghost" on:click={clearTagFilter}>Clear</button>
    </div>
  {/if}

  {#if filtered.length === 0}
    <p class="empty-state">No entries found.</p>
  {:else}
    <div class="entries">
      {#each filtered as entry (entry.id)}
        <div
          class="entry-row"
          class:selected={selectedEntry && selectedEntry.id === entry.id}
          role="button"
          tabindex="0"
          on:click={() => onSelect(entry)}
          on:dblclick={(e) => copyPassword(e, entry.password)}
          on:keydown={(e) => handleEntryKeydown(e, entry)}
          on:contextmenu={(e) => openContextMenu(e, entry)}
        >
          <div class="entry-info">
            <div class="entry-title-row">
              <div class="entry-title">{entry.title}</div>
              {#if freeTags(entry.tags, $groups).length > 0}
                <div class="entry-tags">
                  {#each freeTags(entry.tags, $groups) as tag}
                    <button
                      class="entry-tag"
                      class:active={selectedTags.includes(tag)}
                      on:click|stopPropagation={() => toggleTag(tag)}
                    >
                      {tag}
                    </button>
                  {/each}
                </div>
              {/if}
            </div>
            <div class="entry-subtitle">{entry.username || entry.url || "No details"}</div>
          </div>
          {#if entry.password}
            <button
              class="btn-copy"
              title="Copy password"
              aria-label="Copy password"
              on:click={(e) => copyPassword(e, entry.password)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
            </button>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  {#if !trashMode}
    <button class="btn-secondary new-entry-btn" on:click={onNew}>
      + New Entry
    </button>
  {/if}
</div>

{#if contextMenu.show}
  <EntryContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    entry={contextMenu.entry}
    on:copyPassword={handleMenuCopyPassword}
    on:moveToGroup={handleMenuMoveToGroup}
    on:moveToVault={handleMenuMoveToVault}
    on:copyToGroup={handleMenuCopyToGroup}
    on:copyToVault={handleMenuCopyToVault}
  />
{/if}

<style>
  .entry-list {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    padding: 1rem;
    border-right: 1px solid var(--border-color);
  }

  .list-header {
    margin-bottom: 0.75rem;
  }

  .search-input {
    margin-bottom: 0.75rem;
  }

  .entries {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .entry-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem;
    background: transparent;
    border: none;
    border-radius: 0.5rem;
    color: var(--text-color);
    cursor: pointer;
    text-align: left;
    user-select: none;
  }

  .entry-row:hover {
    background-color: var(--hover-bg);
  }

  .entry-row:not(.selected) .entry-info {
    opacity: 0.85;
  }

  .entry-row:not(.selected):hover .entry-info {
    opacity: 0.8;
  }

  .entry-row.selected {
    background-color: var(--selected-bg);
    color: var(--selected-text);
  }

  .entry-row.selected .entry-subtitle {
    color: var(--selected-text);
    opacity: 0.8;
  }

  .entry-info {
    min-width: 0;
    flex: 1;
  }

  .entry-row:hover .btn-copy,
  .entry-row.selected .btn-copy {
    opacity: 1;
  }

  .entry-row.selected .btn-copy:hover {
    background-color: rgba(255, 255, 255, 0.15);
    color: var(--selected-text);
  }

  .entry-title-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .entry-title {
    font-weight: 400;
    font-size: 0.875rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1 1 auto;
    min-width: 0;
  }

  .entry-subtitle {
    font-size: 0.75rem;
    color: var(--muted-color);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .entry-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    flex-shrink: 0;
    max-width: 50%;
  }

  .entry-tag {
    display: inline-flex;
    align-items: center;
    padding: 0.125rem 0.375rem;
    background-color: var(--hover-bg);
    border: 1px solid var(--border-color);
    border-radius: 9999px;
    color: var(--muted-color);
    font-size: 0.7rem;
    line-height: 1;
    cursor: pointer;
  }

  .entry-tag.active {
    background-color: var(--selected-bg);
    color: var(--selected-text);
    border-color: var(--selected-bg);
  }

  .entry-row.selected .entry-tag {
    color: var(--selected-text);
    border-color: rgba(255, 255, 255, 0.3);
    background-color: rgba(255, 255, 255, 0.15);
  }

  .entry-row.selected .entry-tag.active {
    background-color: var(--selected-text);
    color: var(--selected-bg);
    border-color: var(--selected-text);
  }

  .tag-filter-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.25rem;
    margin-bottom: 0.75rem;
  }

  .tag-filter-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background-color: var(--selected-bg);
    color: var(--selected-text);
    border: 1px solid var(--selected-bg);
    border-radius: 9999px;
    font-size: 0.75rem;
    cursor: pointer;
  }


</style>
