<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";
  import { updateEntry } from "../store.js";
  import TagManager from "./TagManager.svelte";
  import EntryInput from "./EntryInput.svelte";
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
        <p class="entry-url">{entry.url || ""}</p>
      </div>
    </div>

    <div class="details-body">
      <div class="field">
        <span class="label">Username</span>
        <div class="field-row">
          <span class="field-value" class:empty={!entry.username}
            >{entry.username || "<empty>"}</span
          >
          {#if entry.username}
            <button
              class="btn-copy-solid"
              aria-label="Copy username"
              on:click={() => copy(entry.username, "Username")}
            >
              <CopyIcon size={16} />
            </button>
          {:else}
            <button class="btn-copy-solid" aria-label="Copy username" disabled>
              <CopyIcon size={16} />
            </button>
          {/if}
        </div>
      </div>

      <div class="field">
        <span class="label">Password</span>
        <div class="field-row">
          <span
            class="field-value"
            class:masked={entry.password && !passwordVisible}
            class:empty={!entry.password}>{entry.password || "<empty>"}</span
          >
          <button
            class="btn-copy-solid"
            aria-label={passwordVisible ? "Hide password" : "Reveal password"}
            on:click={() => (passwordVisible = !passwordVisible)}
          >
            {#if passwordVisible}
              <EyeOffIcon size={16} />
            {:else}
              <EyeIcon size={16} />
            {/if}
          </button>
          {#if entry.password}
            <button
              class="btn-copy-solid"
              aria-label="Copy password"
              on:click={() => copy(entry.password, "Password")}
            >
              <CopyIcon size={16} />
            </button>
          {:else}
            <button class="btn-copy-solid" aria-label="Copy password" disabled>
              <CopyIcon size={16} />
            </button>
          {/if}
        </div>
      </div>

      {#if entry.url}
        <div class="field">
          <span class="label">URL</span>
          <a class="url-link" href={entry.url} target="_blank" rel="noopener">
            {entry.url}
          </a>
        </div>
      {/if}

      {#if entry.notes}
        <div class="field">
          <span class="label">Notes</span>
          <div class="notes">{entry.notes}</div>
        </div>
      {/if}

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

  .entry-url {
    margin: 0.25rem 0 0;
    font-size: 0.875rem;
    color: var(--muted-color);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .details-body {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .field {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
  }

  .field .label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--muted-color);
    white-space: nowrap;
    flex-shrink: 0;
    min-width: 4rem;
  }

  .field-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    padding-right: 0.5rem;
  }

  .field-value {
    flex: 1;
    min-width: 0;
    padding: 0.5rem 0.75rem;
    line-height: 1.5;
    color: var(--text-color);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .field-value.masked {
    -webkit-text-security: disc;
    text-security: disc;
    letter-spacing: 0.1em;
  }

  .field-value.empty {
    color: var(--muted-color);
    font-style: italic;
  }

  .url-link {
    color: var(--accent-color);
    text-decoration: none;
    font-size: 0.875rem;
    word-break: break-all;
  }

  .url-link:hover {
    text-decoration: underline;
  }

  .notes {
    padding: 0.75rem;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    color: var(--text-color);
    font-size: 0.875rem;
    white-space: pre-wrap;
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
