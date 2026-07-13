<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { debounce } from "../../../lib/debounce.js";
  import { showToast } from "../../../stores/toast.js";
  import { closeAllContextMenus } from "../../../stores/contextMenu.js";
  import { useContextMenu } from "../../../lib/createContextMenu.js";
  import EntryContextMenu from "./EntryContextMenu.svelte";
  import EntryRow from "./EntryRow.svelte";

  const SEARCH_DEBOUNCE_MS = 150;

  export let entries;
  export let selectedEntry = null;
  export let selectedTags = [];
  export let trashMode = false;
  export let hideNewButton = false;
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

  useContextMenu(closeContextMenu);

  function onSearchInput(event) {
    search = event.target.value;
    setFilterSearch(search);
  }

  $: filtered = entries.filter((e) => {
    const q = filterSearch.toLowerCase();
    const matchesSearch =
      e.title.toLowerCase().includes(q) ||
      e.username.toLowerCase().includes(q) ||
      (e.fields || []).some((f) => f.value.toLowerCase().includes(q)) ||
      (e.tags || []).some((tag) => tag.toLowerCase().includes(q));
    if (!matchesSearch) return false;
    if (selectedTags.length > 0) {
      return selectedTags.every((tag) => (e.tags || []).includes(tag));
    }
    return true;
  });

  async function handleMenuCopyPassword() {
    if (contextMenu.entry?.password) {
      await writeText(contextMenu.entry.password);
      showToast("Password copied to clipboard");
    }
    closeContextMenu();
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

  function handleMenuAction(handler, event) {
    if (event.detail.vault !== undefined) {
      handler(event.detail.entry, event.detail.vault, event.detail.groupId);
    } else {
      handler(event.detail.entry, event.detail.group);
    }
    closeContextMenu();
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
        <button class="tag-filter-chip" on:click={() => onToggleTag(tag)}>
          {tag} <span class="remove">×</span>
        </button>
      {/each}
      <button class="btn-ghost" on:click={onClearTags}>Clear</button>
    </div>
  {/if}

  {#if filtered.length === 0}
    <p class="empty-state">No entries found.</p>
  {:else}
    <div class="entries">
      {#each filtered as entry (entry.id)}
        <EntryRow
          {entry}
          selected={selectedEntry && selectedEntry.id === entry.id}
          {selectedTags}
          {onSelect}
          {onToggleTag}
          onContextMenu={openContextMenu}
        />
      {/each}
    </div>
  {/if}

  {#if !trashMode && !hideNewButton}
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
    on:moveToGroup={(e) => handleMenuAction(onMoveToGroup, e)}
    on:moveToVault={(e) => handleMenuAction(onMoveToVault, e)}
    on:copyToGroup={(e) => handleMenuAction(onCopyToGroup, e)}
    on:copyToVault={(e) => handleMenuAction(onCopyToVault, e)}
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
    border-radius: var(--shape-full);
    font-size: 0.75rem;
    cursor: pointer;
  }
</style>
