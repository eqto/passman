<script>
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";
  import { CopyIcon, EyeIcon, EyeOffIcon } from "../../../components/icons";

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
              <EyeOffIcon size={16} />
            {:else}
              <EyeIcon size={16} />
            {/if}
          </button>
          <button
            class="btn-copy-solid"
            aria-label="Copy password"
            on:click={() => copy(field.value, field.label || "Password")}
          >
            <CopyIcon size={16} />
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
            <CopyIcon size={16} />
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
