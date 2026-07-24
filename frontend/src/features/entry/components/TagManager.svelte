<script>
  import Chip from "../../../components/form/Chip.svelte";
  import TagContextMenu from "../../../components/TagContextMenu.svelte";
  import Confirm from "../../../components/dialog/Confirm.svelte";

  let {
    tags = [],
    readOnly = false,
    onAddTag = null,
    onRemoveTag = null,
  } = $props();

  let tagInput = $state("");
  let showTagInput = $state(false);
  let tagInputEl;
  let tagContextMenu = $state({ show: false, x: 0, y: 0, tag: null });
  let confirmDeleteTag = $state(null);

  $effect(() => {
    if (showTagInput && tagInputEl) tagInputEl.focus();
  });

  function addTag() {
    const raw = tagInput
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t);
    if (raw.length === 0) return;
    if (onAddTag) {
      onAddTag(raw);
    }
    tagInput = "";
    showTagInput = false;
  }

  function openTagContextMenu(event, tag) {
    if (readOnly) return;
    event.preventDefault();
    tagContextMenu = { show: true, x: event.clientX, y: event.clientY, tag };
  }

  function handleTagDelete(tag) {
    tagContextMenu = { ...tagContextMenu, show: false };
    confirmDeleteTag = tag;
  }

  function confirmTagDelete() {
    if (!confirmDeleteTag) return;
    if (onRemoveTag) {
      onRemoveTag(confirmDeleteTag);
    }
    confirmDeleteTag = null;
  }

  function handleTagKeydown(event) {
    if (event.key === "Enter" || event.key === ",") {
      event.preventDefault();
      addTag();
    } else if (event.key === "Escape") {
      tagInput = "";
      showTagInput = false;
    }
  }
</script>

<div class="tags-section">
  <div class="tags-list">
    {#each tags as tag}
      <Chip
        size="medium"
        as="span"
        title={readOnly ? "" : "Right-click to delete"}
        role="button"
        tabindex="0"
        oncontextmenu={(event) => openTagContextMenu(event, tag)}
        text={tag}
      />
    {/each}
    {#if !readOnly && !showTagInput}
      <button
        class="add-tag-chip"
        type="button"
        onclick={() => (showTagInput = true)}
      >
        + add tag
      </button>
    {/if}
  </div>
  {#if showTagInput}
    <div class="tag-input-row">
      <input
        class="tag-input"
        type="text"
        placeholder="Add tag"
        maxlength="20"
        bind:this={tagInputEl}
        bind:value={tagInput}
        onkeydown={handleTagKeydown}
      />
      <button class="btn-secondary" type="button" onclick={addTag}>
        Save
      </button>
      <button
        class="btn-icon"
        type="button"
        aria-label="Cancel"
        onclick={() => {
          tagInput = "";
          showTagInput = false;
        }}
      >
        ×
      </button>
    </div>
  {/if}
</div>

{#if tagContextMenu.show}
  <TagContextMenu
    x={tagContextMenu.x}
    y={tagContextMenu.y}
    ondelete={() => handleTagDelete(tagContextMenu.tag)}
    onclose={() => (tagContextMenu = { ...tagContextMenu, show: false })}
  />
{/if}

{#if confirmDeleteTag}
  <Confirm
    title="Delete Tag"
    message={`Delete tag "${confirmDeleteTag}"?`}
    confirmLabel="Delete"
    onconfirm={confirmTagDelete}
    oncancel={() => (confirmDeleteTag = null)}
  />
{/if}

<style>
  .tags-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .add-tag-chip {
    padding: 0.25rem 0.75rem;
    background-color: transparent;
    border: 1px dashed var(--border-color);
    border-radius: var(--shape-sm);
    color: var(--muted-color);
    font-size: 0.875rem;
    cursor: pointer;
  }

  .add-tag-chip:hover {
    color: var(--text-color);
    border-color: var(--accent-color);
  }

  .tag-input-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin-left: 0.5rem;
  }

  .tag-input {
    width: 8rem;
    padding: 0.25rem 0.5rem;
    background-color: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: var(--shape-sm);
    color: var(--text-color);
    font-size: 0.875rem;
  }

  .tag-input-row .btn-secondary {
    height: auto;
    padding: 0.25rem 0.75rem;
    font-size: 0.875rem;
  }
</style>
