<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { debounce } from "../../../lib/debounce.js";
  import { showToast } from "../../../stores/toast.js";
  import { createContextMenuState } from "../../../lib/createContextMenu.svelte.js";
  import EntryContextMenu from "./EntryContextMenu.svelte";
  import EntryRow from "./EntryRow.svelte";

  const SEARCH_DEBOUNCE_MS = 150;

  let {
    entries,
    selectedEntry = null,
    selectedTags = [],
    trashMode = false,
    hideNewButton = false,
    onSelect,
    onNew,
    onToggleTag = (tag) => {},
    onClearTags = () => {},
    onMoveToGroup = () => {},
    onMoveToVault = () => {},
    onCopyToGroup = () => {},
    onCopyToVault = () => {},
  } = $props();

  let search = $state("");
  let filterSearch = $state("");
  const {
    state: contextMenu,
    open: openContextMenuState,
    close: closeContextMenu,
  } = createContextMenuState({ entry: null });

  const setFilterSearch = debounce((value) => {
    filterSearch = value;
  }, SEARCH_DEBOUNCE_MS);

  function onSearchInput(event) {
    search = event.target.value;
    setFilterSearch(search);
  }

  function openContextMenu(event, entry) {
    if (trashMode) return;
    openContextMenuState(event, { entry });
  }

  let filtered = $derived(
    entries.filter((e) => {
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
    }),
  );

  async function handleMenuCopyPassword() {
    if (contextMenu.entry?.password) {
      await writeText(contextMenu.entry.password);
      showToast("Password copied to clipboard");
    }
    closeContextMenu();
  }

  function handleMenuAction(handler, detail) {
    if (detail.vault !== undefined) {
      handler(detail.entry, detail.vault, detail.groupId);
    } else {
      handler(detail.entry, detail.group);
    }
    closeContextMenu();
  }
</script>

<svelte:window onclick={closeContextMenu} />

<div class="entry-list">
  <div class="list-header section-header">
    <span class="count">{filtered.length} entries</span>
  </div>

  <input
    value={search}
    oninput={onSearchInput}
    placeholder="Search entries..."
    class="modal-input search-input"
  />

  {#if selectedTags.length > 0}
    <div class="tag-filter-bar">
      {#each selectedTags as tag}
        <button class="tag-filter-chip" onclick={() => onToggleTag(tag)}>
          {tag} <span class="remove">×</span>
        </button>
      {/each}
      <button class="btn-ghost" onclick={onClearTags}>Clear</button>
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
    <button class="btn-secondary new-entry-btn" onclick={onNew}>
      + New Entry
    </button>
  {/if}
</div>

{#if contextMenu.show}
  <EntryContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    entry={contextMenu.entry}
    oncopyPassword={handleMenuCopyPassword}
    onmoveToGroup={(detail) => handleMenuAction(onMoveToGroup, detail)}
    onmoveToVault={(detail) => handleMenuAction(onMoveToVault, detail)}
    oncopyToGroup={(detail) => handleMenuAction(onCopyToGroup, detail)}
    oncopyToVault={(detail) => handleMenuAction(onCopyToVault, detail)}
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
    background-color: var(--chip-active-bg);
    color: var(--chip-active-text);
    border: 1px solid var(--chip-active-border);
    border-radius: var(--shape-sm);
    font-size: 0.75rem;
    cursor: pointer;
  }
</style>
