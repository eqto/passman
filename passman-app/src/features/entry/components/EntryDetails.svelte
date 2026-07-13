<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";
  import { updateEntry } from "../store.js";
  import TagManager from "./TagManager.svelte";
  import EntryInput from "./EntryInput.svelte";
  import Input from "../../../components/form/Input.svelte";
  import { CopyIcon, EyeIcon, EyeOffIcon } from "../../../components/icons";

  export let entry;
  export let onEdit;
  export let onRestore;
  export let onDelete;
  export let trashMode = false;

  let passwordVisible = false;

  $: visibleTags = entry?.tags || [];

  async function updateTags(nextTags) {
    await updateEntry({
      ...entry,
      tags: nextTags,
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
          on:click={() => !trashMode && onEdit(entry)}
        >
          {entry.title}
        </h2>
      </div>
    </div>

    <div class="details-body">
      <div class="field">
        <Input
          value={entry.username}
          label="Username"
          placeholder="<empty>"
          readonly={true}
          transparent={true}
          copyable={true}
          copyLabel="Copy username"
          class_:empty={!entry.username}
          on:copy={(e) => copy(e.detail, "Username")}
        />
      </div>

      <div class="field">
        <Input
          value={entry.password}
          type="password"
          label="Password"
          placeholder="<empty>"
          readonly={true}
          transparent={true}
          copyable={true}
          copyLabel="Copy password"
          revealable={true}
          class_:empty={!entry.password}
          on:copy={(e) => copy(e.detail, "Password")}
        />
      </div>

      {#if visibleTags.length > 0 || !trashMode}
        <div class="field">
          <span class="label">Tags</span>
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
        <button class="btn-secondary" on:click={() => onRestore(entry)}>
          Restore
        </button>
        <button class="btn-danger" on:click={() => onDelete(entry)}>
          Delete Permanently
        </button>
      {:else}
        <button class="btn-secondary" on:click={() => onEdit(entry)}>
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

  .field > :global(.form-field) {
    flex: 1;
    min-width: 0;
  }

  .empty {
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
