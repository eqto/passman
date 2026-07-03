<script>
  import { addEntry, updateEntry, generatePassword } from "../stores/entries";
  import { groups } from "../stores/vaults";
  import { DEFAULT_PASSWORD_LENGTH } from "../lib/constants.js";
  import { freeTags } from "../lib/tags.js";
  import CustomFieldEditor from "./CustomFieldEditor.svelte";
  import Chip from "./form/Chip.svelte";
  import TagContextMenu from "./TagContextMenu.svelte";
  import Confirm from "./dialog/Confirm.svelte";

  export let entry;
  export let selectedGroup = "";
  export let onClose;

  let form = { ...entry, tags: entry.tags || [], fields: entry.fields || [] };
  $: displayTags = freeTags(form.tags, $groups);
  let error = "";
  let tagInput = "";
  let showTagInput = false;
  let tagInputEl;
  let tagContextMenu = { show: false, x: 0, y: 0, tag: null };
  let confirmDeleteTag = null;
  let passwordLength = DEFAULT_PASSWORD_LENGTH;
  let passwordOptions = {
    uppercase: true,
    lowercase: true,
    digits: true,
    symbols: true,
  };

  function addTag() {
    const raw = tagInput.split(",").map((t) => t.trim()).filter((t) => t);
    let next = form.tags;
    for (const tag of raw) {
      if (!next.includes(tag)) {
        next = [...next, tag];
      }
    }
    form = { ...form, tags: next };
    tagInput = "";
    showTagInput = false;
  }

  function openTagContextMenu(event, tag) {
    event.preventDefault();
    tagContextMenu = { show: true, x: event.clientX, y: event.clientY, tag };
  }

  function handleTagDelete(tag) {
    tagContextMenu = { ...tagContextMenu, show: false };
    confirmDeleteTag = tag;
  }

  function confirmTagDelete() {
    if (!confirmDeleteTag) return;
    form = { ...form, tags: form.tags.filter((t) => t !== confirmDeleteTag) };
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

  $: if (showTagInput && tagInputEl) tagInputEl.focus();

  async function handleSave() {
    error = "";
    try {
      if (selectedGroup && !form.tags.includes(selectedGroup)) {
        form = { ...form, tags: [...form.tags, selectedGroup] };
      }
      const now = new Date().toISOString();
      const updated = { ...form, updated_at: now };
      if (entry.title) {
        await updateEntry(updated);
      } else {
        updated.created_at = now;
        await addEntry(updated);
      }
      onClose();
    } catch (e) {
      error = e.toString();
    }
  }

  async function handleGenerate() {
    form.password = await generatePassword(passwordLength, passwordOptions);
  }
</script>

<div class="entry-editor">
  <h2>{entry.title ? "Edit Entry" : "New Entry"}</h2>
  <div class="form">
    <input bind:value={form.title} placeholder="Title" />
    <input bind:value={form.username} placeholder="Username" />
    <div class="password-row">
      <input bind:value={form.password} type="password" placeholder="Password" />
      <button class="btn-secondary generate-btn" on:click={handleGenerate}>
        Generate
      </button>
    </div>
    {#if entry.title && form.url}
      <input bind:value={form.url} placeholder="URL" />
    {/if}
    {#if entry.title && form.notes}
      <textarea bind:value={form.notes} placeholder="Notes" rows="6"></textarea>
    {/if}
    <div class="tags-section">
      <div class="tags-list">
        {#each displayTags as tag}
          <Chip
            size="medium"
            as="span"
            title="Right-click to delete"
            role="button"
            tabindex="0"
            on:contextmenu={(event) => openTagContextMenu(event, tag)}
          >
            {tag}
          </Chip>
        {/each}
        {#if !showTagInput}
          <button
            class="add-tag-chip"
            type="button"
            on:click={() => showTagInput = true}
          >
            + add tag
          </button>
        {/if}
      </div>
      {#if showTagInput}
        <div class="tag-input-row">
          <input
            class="tag-input"
            bind:this={tagInputEl}
            bind:value={tagInput}
            placeholder="Add tag"
            maxlength="20"
            on:keydown={handleTagKeydown}
          />
          <button class="btn-secondary" type="button" on:click={addTag}>
            Save
          </button>
          <button
            class="btn-icon"
            type="button"
            aria-label="Cancel"
            on:click={() => {
              tagInput = "";
              showTagInput = false;
            }}
          >
            ×
          </button>
        </div>
      {/if}
    </div>
    <CustomFieldEditor
      customFields={form.fields}
      onChange={(fields) => form = { ...form, fields }}
    />
  </div>
  {#if error}
    <p class="error">{error}</p>
  {/if}
  <div class="actions">
    <button class="modal-cancel-btn" on:click={onClose}>
      Cancel
    </button>
    <button class="btn-primary" on:click={handleSave}>
      Save
    </button>
  </div>
</div>

{#if tagContextMenu.show}
  <TagContextMenu
    x={tagContextMenu.x}
    y={tagContextMenu.y}
    on:delete={() => handleTagDelete(tagContextMenu.tag)}
    on:close={() => tagContextMenu = { ...tagContextMenu, show: false }}
  />
{/if}

{#if confirmDeleteTag}
  <Confirm
    title="Delete Tag"
    message={`Delete tag "${confirmDeleteTag}"?`}
    confirmLabel="Delete"
    on:confirm={confirmTagDelete}
    on:cancel={() => confirmDeleteTag = null}
  />
{/if}

<style>
  .entry-editor {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    padding: 1.5rem;
  }

  h2 {
    margin: 0 0 1.5rem;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-color);
  }

  .form {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 0.25rem;
  }

  input,
  textarea {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--input-border);
    border-radius: 0.5rem;
    background-color: var(--input-bg);
    color: var(--text-color);
    resize: vertical;
  }

  input:focus,
  textarea:focus {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }

  .password-row input {
    flex: 1;
  }

  .password-row {
    display: flex;
    gap: 0.5rem;
  }

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
    border-radius: 0.5rem;
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
    border-radius: 0.5rem;
    color: var(--text-color);
    font-size: 0.875rem;
  }

  .error {
    margin: 0.5rem 0 0;
    font-size: 0.875rem;
    color: var(--danger-color);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

</style>
