<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";

  export let fields = [];

  let visibleCustomFieldIds = new Set();

  function toggleCustomFieldVisibility(id) {
    const next = new Set(visibleCustomFieldIds);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    visibleCustomFieldIds = next;
  }

  async function copy(text, type = "item") {
    if (!text) return;
    await writeText(text);
    showToast(`${type} copied to clipboard`);
  }
</script>

{#each fields as field (field.id)}
  <div class="field">
    <span class="label">{field.label || "Custom field"}</span>
    {#if field.type === "note"}
      <div class="notes">{field.value}</div>
    {:else if field.type === "password" || field.type === "otp"}
      <div class="field-row">
        <input
          type={visibleCustomFieldIds.has(field.id) ? "text" : "password"}
          value={field.value}
          readonly
        />
        {#if field.value}
          <button
            class="btn-copy-solid"
            aria-label={visibleCustomFieldIds.has(field.id)
              ? "Hide password"
              : "Reveal password"}
            on:click={() => toggleCustomFieldVisibility(field.id)}
          >
            {#if visibleCustomFieldIds.has(field.id)}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                ><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"></path><path
                  d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"
                ></path><path
                  d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"
                ></path><line x1="2" x2="22" y1="2" y2="22"></line></svg
              >
            {:else}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                ><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"
                ></path><circle cx="12" cy="12" r="3"></circle></svg
              >
            {/if}
          </button>
          <button
            class="btn-copy-solid"
            aria-label="Copy password"
            on:click={() => copy(field.value, field.label || "Password")}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><rect x="9" y="9" width="13" height="13" rx="2" ry="2"
              ></rect><path
                d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
              ></path></svg
            >
          </button>
        {/if}
      </div>
    {:else}
      <div class="field-row">
        <input type="text" value={field.value} readonly />
        {#if field.value}
          <button
            class="btn-copy-solid"
            aria-label="Copy value"
            on:click={() => copy(field.value, field.label || "Value")}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><rect x="9" y="9" width="13" height="13" rx="2" ry="2"
              ></rect><path
                d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
              ></path></svg
            >
          </button>
        {/if}
      </div>
    {/if}
  </div>
{/each}

<style>
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
    padding-right: 0.5rem;
  }

  .field-row input {
    flex: 1;
    min-width: 0;
    padding: 0.5rem 0.75rem;
    background-color: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: 0.5rem;
    color: var(--text-color);
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
</style>
