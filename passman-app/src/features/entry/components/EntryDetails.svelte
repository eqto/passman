<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";
  import { updateEntry } from "../store.js";
  import TagManager from "./TagManager.svelte";
  import EntryInput from "./EntryInput.svelte";
  import { Input, Label } from "../../../components/form";
  import { Icon } from "../../../components/icons";

  let { entry, onEdit, onRestore, onDelete, trashMode = false } = $props();

  let passwordVisible = $state(false);

  let visibleTags = $derived(
    (entry?.tags || []).slice().sort((a, b) => a.localeCompare(b)),
  );

  async function updateTags(nextTags) {
    await updateEntry({
      ...entry,
      tags: nextTags.slice().sort((a, b) => a.localeCompare(b)),
      updated_at: new Date().toISOString(),
    });
  }

  async function addTags(raw) {
    await updateTags([...new Set([...(entry.tags || []), ...raw])]);
  }

  async function removeTag(tag) {
    await updateTags((entry.tags || []).filter((t) => t !== tag));
  }

  async function copy(text, type = "item") {
    if (!text) return;
    await writeText(text);
    showToast(`${type} copied to clipboard`);
  }

  // handleDelete and handleRestore are inlined in template
</script>

{#if entry}
  <div class="entry-details">
    <div class="details-header">
      <div class="entry-title-area">
        <h2
          class="entry-title"
          class:editable={!trashMode}
          title={trashMode ? entry.title : "Click to edit"}
          onclick={() => !trashMode && onEdit(entry)}
        >
          {entry.title}
        </h2>
      </div>
    </div>

    <div class="details-body">
      <div class="field">
        <Label text="Username" />
        <Input
          value={entry.username}
          placeholder="<empty>"
          readonly={true}
          class_="transparent {!entry.username ? 'empty' : ''}"
        />
        <button
          class="btn-copy-solid"
          type="button"
          aria-label="Copy username"
          disabled={!entry.username}
          onclick={() => copy(entry.username, "Username")}
        >
          <Icon name="copy" size={16} />
        </button>
      </div>

      <div class="field">
        <Label text="Password" />
        <Input
          value={entry.password}
          type={passwordVisible ? "text" : "password"}
          placeholder="<empty>"
          readonly={true}
          class_="transparent {!entry.password ? 'empty' : ''}"
        />
        <button
          class="btn-copy-solid"
          type="button"
          aria-label="Copy password"
          disabled={!entry.password}
          onclick={() => copy(entry.password, "Password")}
        >
          <Icon name="copy" size={16} />
        </button>
        <button
          class="btn-copy-solid"
          type="button"
          aria-label={passwordVisible ? "Hide" : "Reveal"}
          disabled={!entry.password}
          onclick={() => (passwordVisible = !passwordVisible)}
        >
          {#if passwordVisible}
            <Icon name="eye-off" size={16} />
          {:else}
            <Icon name="eye" size={16} />
          {/if}
        </button>
      </div>

      {#if visibleTags.length > 0 || !trashMode}
        <div class="field">
          <Label text="Tags" />
          <TagManager
            tags={visibleTags}
            readOnly={trashMode}
            onAddTag={addTags}
            onRemoveTag={removeTag}
          />
        </div>
      {/if}

      {#each entry?.fields || [] as field (field.id)}
        <EntryInput
          label={field.label}
          value={field.value}
          type={field.type === "password" || field.type === "otp"
            ? "password"
            : "text"}
          revealable={field.type === "password" || field.type === "otp"}
          multiline={field.type === "note"}
        />
      {/each}
    </div>

    <div class="details-footer" class:justify-end={!trashMode}>
      {#if trashMode}
        <button class="btn-secondary" onclick={() => onRestore(entry)}>
          Restore
        </button>
        <button class="btn-danger" onclick={() => onDelete(entry)}>
          Delete Permanently
        </button>
      {:else}
        <button class="btn-secondary" onclick={() => onEdit(entry)}>
          Edit
        </button>
      {/if}
    </div>
  </div>
{:else}
  <div class="empty-details">Select an entry to view details.</div>
{/if}

<style>
  .entry-details {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-color);
    padding: 1.5rem;
  }

  .details-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .entry-title-area {
    min-width: 0;
  }

  .entry-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-color);
  }

  .entry-title.editable {
    cursor: pointer;
  }

  .details-body {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding-right: 0.25rem;
  }

  .field {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .field > :global(.form-input) {
    flex: 1;
    min-width: 0;
  }

  .field > :global(.label) {
    flex-shrink: 0;
  }

  .field > :global(.transparent) {
    background-color: transparent;
    border-color: transparent;
    cursor: text;
  }

  .field > :global(.transparent:hover) {
    border-color: var(--input-border);
    border-style: dashed;
  }

  .field > :global(.transparent:focus) {
    outline: none;
  }

  .field > :global(.empty) {
    color: var(--muted-color);
    font-style: italic;
  }

  .details-footer {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

  .details-footer.justify-end {
    justify-content: flex-end;
  }

  .empty-details {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
  }
</style>
