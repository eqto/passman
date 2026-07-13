<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { freeTags } from "../../../lib/tags.js";
  import { showToast } from "../../../stores/toast.js";
  import Chip from "../../../components/form/Chip.svelte";
  import { CopyIcon } from "../../../components/icons";

  export let entry;
  export let selected = false;
  export let selectedTags = [];
  export let onSelect;
  export let onToggleTag;
  export let onContextMenu;

  async function copyPassword(event, password) {
    event.stopPropagation();
    if (!password) return;
    await writeText(password);
    showToast("Password copied to clipboard");
  }
</script>

<div
  class="entry-row"
  class:selected
  role="button"
  tabindex="0"
  onclick={() => onSelect(entry)}
  ondblclick={(e) => copyPassword(e, entry.password)}
  onkeydown={(e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onSelect(entry);
    }
  }}
  oncontextmenu={(e) => onContextMenu(e, entry)}
>
  <div class="entry-info">
    <div class="entry-title-row">
      <div class="entry-title">{entry.title}</div>
      {#if freeTags(entry.tags).length > 0}
        <div class="entry-tags">
          {#each freeTags(entry.tags) as tag}
            <Chip
              size="small"
              active={selectedTags.includes(tag)}
              onclick={(event) => {
                event.stopPropagation();
                onToggleTag(tag);
              }}
              text={tag}
            />
          {/each}
        </div>
      {/if}
    </div>
    <div class="entry-subtitle">
      {entry.username ||
        (entry.fields || []).find((f) => f.label === "URL")?.value ||
        "No details"}
    </div>
  </div>
  {#if entry.password}
    <button
      class="btn-copy"
      title="Copy password"
      aria-label="Copy password"
      onclick={(e) => copyPassword(e, entry.password)}
    >
      <CopyIcon size={16} />
    </button>
  {/if}
</div>

<style>
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

  :global(.entry-row.selected .chip) {
    color: var(--selected-text);
    border-color: rgba(255, 255, 255, 0.3);
    background-color: rgba(255, 255, 255, 0.15);
  }

  :global(.entry-row.selected .chip.active) {
    background-color: var(--selected-text);
    color: var(--selected-bg);
    border-color: var(--selected-text);
  }
</style>
