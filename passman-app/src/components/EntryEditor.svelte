<script>
  import { addEntry, updateEntry, generatePassword } from "../stores/entries";
  import { groups } from "../stores/vaults";
  import { DEFAULT_PASSWORD_LENGTH } from "../lib/constants.js";
  import { freeTags } from "../lib/tags.js";

  export let entry;
  export let selectedGroup = "";
  export let onClose;

  let form = { ...entry, tags: entry.tags || [] };
  $: displayTags = freeTags(form.tags, $groups);
  let error = "";
  let tagInput = "";
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
  }

  function removeTag(tag) {
    form = { ...form, tags: form.tags.filter((t) => t !== tag) };
  }

  function handleTagKeydown(event) {
    if (event.key === "Enter" || event.key === ",") {
      event.preventDefault();
      addTag();
    }
  }

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
      <button class="generate-btn" on:click={handleGenerate}>
        Generate
      </button>
    </div>
    {#if entry.title}
      <input bind:value={form.url} placeholder="URL" />
      <textarea bind:value={form.notes} placeholder="Notes" rows="6"></textarea>
    {/if}
    <div class="tags-section">
      <div class="tags-list">
        {#each displayTags as tag}
          <span class="tag-chip">
            {tag}
            <button type="button" on:click={() => removeTag(tag)}>×</button>
          </span>
        {/each}
      </div>
      <div class="tag-input-row">
        <input
          bind:value={tagInput}
          placeholder="Add tag"
          on:keydown={handleTagKeydown}
        />
        <button class="add-tag-btn" type="button" on:click={addTag}>
          +
        </button>
      </div>
    </div>
  </div>
  {#if error}
    <p class="error">{error}</p>
  {/if}
  <div class="actions">
    <button class="cancel-btn" on:click={onClose}>
      Cancel
    </button>
    <button class="save-btn" on:click={handleSave}>
      Save
    </button>
  </div>
</div>

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
  }

  input,
  textarea {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--input-border);
    border-radius: 0.375rem;
    background-color: var(--input-bg);
    color: var(--text-color);
    resize: vertical;
  }

  input:focus,
  textarea:focus {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }

  .password-row {
    display: flex;
    gap: 0.5rem;
  }

  .password-row input {
    flex: 1;
  }

  .generate-btn {
    padding: 0.5rem 0.75rem;
    background-color: var(--hover-bg);
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    cursor: pointer;
    color: var(--text-color);
  }

  .generate-btn:hover {
    background-color: var(--border-color);
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

  .tag-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background-color: var(--hover-bg);
    border: 1px solid var(--border-color);
    border-radius: 9999px;
    color: var(--text-color);
    font-size: 0.875rem;
  }

  .tag-chip button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    padding: 0;
    background: transparent;
    border: none;
    color: var(--muted-color);
    cursor: pointer;
    font-size: 0.875rem;
    line-height: 1;
  }

  .tag-chip button:hover {
    color: var(--danger-color);
  }

  .tag-input-row {
    display: flex;
    gap: 0.5rem;
  }

  .tag-input-row input {
    flex: 1;
  }

  .add-tag-btn {
    padding: 0.5rem 0.75rem;
    background-color: var(--hover-bg);
    border: none;
    border-radius: 0.375rem;
    font-size: 1rem;
    line-height: 1;
    cursor: pointer;
    color: var(--text-color);
  }

  .add-tag-btn:hover {
    background-color: var(--border-color);
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

  .cancel-btn {
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    color: var(--muted-color);
    cursor: pointer;
    border-radius: 0.375rem;
  }

  .cancel-btn:hover {
    color: var(--text-color);
  }

  .save-btn {
    padding: 0.5rem 1rem;
    background-color: var(--accent-color);
    color: var(--selected-text);
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
  }

  .save-btn:hover {
    background-color: var(--accent-hover);
  }
</style>
