<script>
  import { groups } from "../stores/vaults";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  export let entry;
  export let onEdit;
  export let onRestore;
  export let onDelete;
  export let trashMode = false;

  let passwordVisible = false;

  $: visibleTags = (entry?.tags || []).filter((tag) => !$groups.includes(tag));

  async function copy(text) {
    if (!text) return;
    await writeText(text);
  }

  async function handleDelete() {
    await onDelete(entry);
  }

  async function handleRestore() {
    await onRestore(entry);
  }

</script>

{#if entry}
  <div class="entry-details">
    <div class="details-header">
      <div class="entry-title-area">
        <h2 class="entry-title">{entry.title}</h2>
        <p class="entry-url">{entry.url || ""}</p>
      </div>
    </div>

    <div class="details-body">
      <div class="field">
        <span class="label">Username</span>
        <div class="field-row">
          <input type="text" value={entry.username} readonly />
          {#if entry.username}
            <button class="copy-btn" aria-label="Copy username" on:click={() => copy(entry.username)}>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
            </button>
          {/if}
        </div>
      </div>

      <div class="field">
        <span class="label">Password</span>
        <div class="field-row">
          <input type={passwordVisible ? "text" : "password"} value={entry.password} readonly />
          {#if entry.password}
            <button class="copy-btn" aria-label={passwordVisible ? "Hide password" : "Reveal password"} on:click={() => (passwordVisible = !passwordVisible)}>
              {#if passwordVisible}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"></path><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"></path><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"></path><line x1="2" x2="22" y1="2" y2="22"></line></svg>
              {:else}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path><circle cx="12" cy="12" r="3"></circle></svg>
              {/if}
            </button>
            <button class="copy-btn" aria-label="Copy password" on:click={() => copy(entry.password)}>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
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

      {#if visibleTags.length > 0}
        <div class="field">
          <span class="label">Tags</span>
          <div class="tags">
            {#each visibleTags as tag}
              <span class="tag-chip">{tag}</span>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <div class="details-footer">
      {#if trashMode}
        <button class="edit-btn" on:click={handleRestore}>
          Restore
        </button>
        <button class="delete-btn" on:click={handleDelete}>
          Delete Permanently
        </button>
      {:else}
        <button class="edit-btn" on:click={() => onEdit(entry)}>
          Edit
        </button>
        <button class="delete-btn" on:click={handleDelete}>
          Delete
        </button>
      {/if}
    </div>
  </div>
{:else}
  <div class="empty-details">
    Select an entry to view details.
  </div>
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
    flex-direction: column;
    gap: 0.25rem;
  }

  .field .label {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--muted-color);
    letter-spacing: 0.05em;
  }

  .field-row {
    display: flex;
    gap: 0.5rem;
  }

  .field-row input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    background-color: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: 0.375rem;
    color: var(--text-color);
  }

  .copy-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
    background-color: var(--hover-bg);
    border: none;
    border-radius: 0.375rem;
    color: var(--text-color);
    cursor: pointer;
  }

  .copy-btn:hover {
    background-color: var(--border-color);
  }

  .copy-btn svg {
    display: block;
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
    border-radius: 0.375rem;
    color: var(--text-color);
    font-size: 0.875rem;
    white-space: pre-wrap;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag-chip {
    padding: 0.25rem 0.75rem;
    background-color: var(--hover-bg);
    border: 1px solid var(--border-color);
    border-radius: 9999px;
    color: var(--text-color);
    font-size: 0.875rem;
  }

  .details-footer {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

  .edit-btn,
  .delete-btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    cursor: pointer;
  }

  .edit-btn {
    background-color: var(--accent-color);
    color: var(--selected-text);
  }

  .edit-btn:hover {
    background-color: var(--accent-hover);
  }

  .delete-btn {
    background-color: var(--hover-bg);
    color: var(--danger-color);
  }

  .delete-btn:hover {
    background-color: rgba(239, 68, 68, 0.15);
  }

  .empty-details {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--muted-color);
    font-size: 0.875rem;
  }
</style>
