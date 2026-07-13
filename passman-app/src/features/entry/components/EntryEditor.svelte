<script>
  import { addEntry, updateEntry } from "../store.js";
  import { Input, Label } from "../../../components/form";
  import EntryInput from "./EntryInput.svelte";
  import PasswordGenerator from "../../../components/PasswordGenerator.svelte";
  import { SettingsIcon, EyeIcon, EyeOffIcon } from "../../../components/icons";
  import TagManager from "./TagManager.svelte";
  import Confirm from "../../../components/dialog/Confirm.svelte";

  export let entry;
  export let selectedGroup = "";
  export let onClose;
  export let onDelete = null;

  let form = { ...entry, tags: entry.tags || [], fields: entry.fields || [] };
  $: displayTags = form.tags || [];
  let error = "";
  let confirmDeleteEntry = false;

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
        fields: form.fields.filter((f) => f.label.trim() || f.value.trim()),
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

  const FIELD_TYPES = [
    { value: "text", label: "Text" },
    { value: "note", label: "Note" },
    { value: "password", label: "Password" },
    { value: "otp", label: "OTP" },
  ];

  let openMenuId = null;
  let pendingLabelIds = new Set();
  let passwordVisible = false;

  function addField() {
    const id = crypto.randomUUID();
    pendingLabelIds = new Set([...pendingLabelIds, id]);
    form = {
      ...form,
      fields: [...form.fields, { id, label: "", type: "text", value: "" }],
    };
  }

  function clearPendingLabel(id) {
    if (pendingLabelIds.has(id)) {
      pendingLabelIds = new Set([...pendingLabelIds].filter((x) => x !== id));
      updateField(id, { label: "" });
    }
  }

  function updateField(id, patch) {
    form = {
      ...form,
      fields: form.fields.map((f) => (f.id === id ? { ...f, ...patch } : f)),
    };
  }

  function removeField(id) {
    form = { ...form, fields: form.fields.filter((f) => f.id !== id) };
  }

  function setFieldType(id, type) {
    updateField(id, { type });
    openMenuId = null;
  }

  function toggleMenu(id) {
    openMenuId = openMenuId === id ? null : id;
  }

  function handleClickOutside(event) {
    if (!event.target.closest(".custom-field-menu")) {
      openMenuId = null;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="entry-editor">
  <h2>{entry.title ? "Edit Entry" : "New Entry"}</h2>
  <div class="form">
    <Label text="Title" />
    <Input bind:value={form.title} placeholder="Title" />
    <Label text="Username" />
    <Input bind:value={form.username} placeholder="Username" />
    <div class="password-row">
      <Label text="Password" />
      <Input
        bind:value={form.password}
        type={passwordVisible ? "text" : "password"}
        placeholder="Password"
      />
      <button
        class="btn-copy-solid"
        type="button"
        aria-label={passwordVisible ? "Hide" : "Reveal"}
        disabled={!form.password}
        on:click={() => (passwordVisible = !passwordVisible)}
      >
        {#if passwordVisible}
          <EyeOffIcon size={16} />
        {:else}
          <EyeIcon size={16} />
        {/if}
      </button>
      <PasswordGenerator on:use={(e) => (form.password = e.detail)} />
    </div>
    <TagManager tags={displayTags} onAddTag={addTags} onRemoveTag={removeTag} />
    {#each form.fields as field (field.id)}
      <div class="custom-field">
        <EntryInput
          label={field.label}
          value={field.value}
          type={field.type === "password" || field.type === "otp"
            ? "password"
            : "text"}
          labelPlaceholder={field.label ? "" : "Field name"}
          valuePlaceholder="Field value"
          editing={true}
          copyable={false}
          revealable={field.type === "password" || field.type === "otp"}
          multiline={field.type === "note"}
          onFocus={() => clearPendingLabel(field.id)}
          on:labelchange={(e) => updateField(field.id, { label: e.detail })}
          on:input={(e) => updateField(field.id, { value: e.detail })}
        />
        <div class="custom-field-menu">
          <button
            class="btn-icon gear-btn"
            type="button"
            aria-label="Field options"
            on:click|stopPropagation={() => toggleMenu(field.id)}
          >
            <SettingsIcon size={16} />
          </button>
          {#if openMenuId === field.id}
            <div class="menu-dropdown">
              {#each FIELD_TYPES as type}
                <button
                  type="button"
                  class="menu-item"
                  class:active={field.type === type.value}
                  on:click|stopPropagation={() =>
                    setFieldType(field.id, type.value)}
                >
                  {type.label}
                </button>
              {/each}
              <div class="menu-divider"></div>
              <button
                type="button"
                class="menu-item danger"
                on:click|stopPropagation={() => removeField(field.id)}
              >
                Remove
              </button>
            </div>
          {/if}
        </div>
      </div>
    {/each}
    <button class="add-field-btn" type="button" on:click={addField}>
      + Add custom field
    </button>
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

  .form > label input,
  .form > .password-row input,
  .form > input,
  .form > textarea {
    width: 100%;
    padding: 0.5rem 0.75rem;
    line-height: 1.5;
    border: 1px solid var(--input-border);
    border-radius: 0.5rem;
    background-color: var(--input-bg);
    color: var(--text-color);
    resize: vertical;
  }

  .form > label input:focus,
  .form > .password-row input:focus,
  .form > input:focus,
  .form > textarea:focus {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }

  .password-row > :global(.form-input) {
    flex: 1;
    min-width: 0;
  }

  .password-row {
    display: flex;
    align-items: center;
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

  .custom-field {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .custom-field > :global(.entry-input) {
    flex: 1;
    min-width: 0;
  }

  .custom-field-menu {
    position: relative;
    flex-shrink: 0;
  }

  .gear-btn {
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
  }

  .menu-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 0.25rem;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
    padding: 0.25rem;
    z-index: 10;
    min-width: 8rem;
  }

  .menu-item {
    width: 100%;
    padding: 0.5rem 0.75rem;
    text-align: left;
    background: transparent;
    border: none;
    border-radius: 0.25rem;
    color: var(--text-color);
    font-size: 0.875rem;
    cursor: pointer;
  }

  .menu-item:hover,
  .menu-item.active {
    background-color: var(--hover-bg);
  }

  .menu-item.danger {
    color: var(--danger-color);
  }

  .menu-divider {
    height: 1px;
    background-color: var(--border-color);
    margin: 0.25rem 0.5rem;
  }

  .add-field-btn {
    align-self: stretch;
    padding: 0.5rem 0.75rem;
    background-color: transparent;
    border: 1px dashed var(--input-border);
    border-radius: 0.5rem;
    color: var(--muted-color);
    font-size: 0.875rem;
    cursor: pointer;
    transition:
      border-color 0.15s ease,
      color 0.15s ease;
  }

  .add-field-btn:hover {
    border-color: var(--accent-color);
    color: var(--accent-color);
  }
</style>
