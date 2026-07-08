<script>
  import { addEntry, updateEntry, generatePassword } from "../stores/entries";
  import { DEFAULT_PASSWORD_LENGTH } from "../lib/constants.js";
  import CustomFieldEditor from "./CustomFieldEditor.svelte";
  import TagManager from "./TagManager.svelte";
  import Confirm from "./dialog/Confirm.svelte";

  export let entry;
  export let selectedGroup = "";
  export let onClose;
  export let onDelete = null;

  let form = { ...entry, tags: entry.tags || [], fields: entry.fields || [] };
  $: displayTags = form.tags || [];
  let error = "";
  let confirmDeleteEntry = false;
  let passwordLength = DEFAULT_PASSWORD_LENGTH;
  let passwordOptions = {
    uppercase: true,
    lowercase: true,
    digits: true,
    symbols: true,
  };

  function addTags(raw) {
    let next = form.tags;
    for (const tag of raw) {
      if (!next.includes(tag)) {
        next = [...next, tag];
      }
    }
    form = { ...form, tags: next };
  }

  function removeTag(tag) {
    form = { ...form, tags: form.tags.filter((t) => t !== tag) };
  }

  async function handleSave() {
    error = "";
    try {
      const now = new Date().toISOString();
      const updated = {
        ...form,
        group_id: selectedGroup || form.group_id || null,
        updated_at: now,
      };
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

  function handleConfirmDelete() {
    confirmDeleteEntry = false;
    if (onDelete) {
      onDelete(form);
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
      <input
        bind:value={form.password}
        type="password"
        placeholder="Password"
      />
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
    <TagManager tags={displayTags} onAddTag={addTags} onRemoveTag={removeTag} />
    <CustomFieldEditor
      customFields={form.fields}
      onChange={(fields) => (form = { ...form, fields })}
    />
  </div>
  {#if error}
    <p class="error">{error}</p>
  {/if}
  <div class="actions">
    {#if entry.title && onDelete}
      <button
        class="btn-danger delete-action"
        on:click={() => (confirmDeleteEntry = true)}
      >
        Delete
      </button>
    {/if}
    <button class="modal-cancel-btn" on:click={onClose}> Cancel </button>
    <button class="btn-primary" on:click={handleSave}> Save </button>
  </div>
</div>

{#if confirmDeleteEntry}
  <Confirm
    title="Delete Entry"
    message={`Delete entry "${form.title}"?`}
    confirmLabel="Delete"
    on:confirm={handleConfirmDelete}
    on:cancel={() => (confirmDeleteEntry = false)}
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

  .delete-action {
    margin-right: auto;
  }
</style>
