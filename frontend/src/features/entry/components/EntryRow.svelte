<script>
  import { Clipboard } from "@wailsio/runtime";
  import { freeTags } from "../../../lib/tags.js";
  import { showToast } from "../../../stores/toast.js";
  import Chip from "../../../components/form/Chip.svelte";
  import { Icon } from "../../../components/icons";

  let {
    entry,
    selected = false,
    selectedTags = [],
    onSelect,
    onToggleTag,
    onContextMenu,
  } = $props();

  async function copyPassword(event, password) {
    event.stopPropagation();
    if (!password) return;
    await Clipboard.SetText(password);
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
    <div class="entry-title">{entry.title}</div>
    <div class="entry-subtitle">
      {entry.username ||
        (entry.fields || []).find((f) => f.label === "URL")?.value ||
        "No details"}
    </div>
  </div>
  <div class="entry-right">
    {#if freeTags(entry.tags).length > 0}
      <div class="entry-tags">
        {#each freeTags(entry.tags) as tag}
          <Chip size="small" active={selectedTags.includes(tag)} text={tag} />
        {/each}
      </div>
    {/if}
    {#if entry.password}
      <button
        class="btn-copy"
        title="Copy password"
        aria-label="Copy password"
        onclick={(e) => copyPassword(e, entry.password)}
      >
        <Icon name="copy" size={16} />
      </button>
    {/if}
  </div>
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
    border-radius: var(--shape-sm);
    color: var(--text-color);
    cursor: pointer;
    text-align: left;
    user-select: none;
  }

  .entry-row:hover {
    background-color: var(--hover-bg);
  }

  .entry-row:not(.selected) .entry-info {
    color: var(--muted-color);
  }

  .entry-row:not(.selected):hover .entry-info {
    color: var(--text-color);
  }

  .entry-row.selected {
    background-color: var(--selected-bg);
    color: var(--selected-text);
  }

  .entry-row.selected .entry-subtitle {
    color: var(--selected-text);
    opacity: 0.7;
  }

  .entry-info {
    min-width: 0;
    flex: 1;
  }

  .entry-right {
    position: relative;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }

  .entry-row:hover .btn-copy {
    opacity: 1;
  }

  .btn-copy {
    position: absolute;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    z-index: 1;
  }

  .entry-row.selected .btn-copy:hover {
    background-color: var(--hover-bg);
    color: var(--selected-text);
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
    justify-content: flex-end;
    max-width: 40%;
  }
</style>
