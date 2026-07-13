<script>
  import { createEventDispatcher } from "svelte";
  import { CopyIcon, EyeIcon, EyeOffIcon } from "../../../components/icons";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";

  export let label = "";
  export let value = "";
  export let type = "text";
  export let editing = false;
  export let labelPlaceholder = "";
  export let valuePlaceholder = "";
  export let copyable = true;
  export let revealable = false;
  export let multiline = false;
  export let onFocus = null;

  const dispatch = createEventDispatcher();
  let revealed = false;

  async function copy() {
    if (!value) return;
    await writeText(value);
    showToast(`${label || "Value"} copied to clipboard`);
  }
</script>

<div class="entry-input">
  {#if editing}
    <input
      class="field-input label-input"
      type="text"
      value={label}
      placeholder={labelPlaceholder}
      on:focus={onFocus}
      on:input={(e) => dispatch("labelchange", e.target.value)}
    />
    {#if multiline}
      <textarea
        class="field-input value-input"
        rows="3"
        {value}
        placeholder={valuePlaceholder}
        on:input={(e) => dispatch("input", e.target.value)}
      ></textarea>
    {:else}
      <input
        class="field-input value-input"
        {type}
        {value}
        placeholder={valuePlaceholder}
        on:input={(e) => dispatch("input", e.target.value)}
      />
    {/if}
  {:else}
    <span class="field-label">{label || "—"}</span>
    <div class="field-row">
      {#if multiline}
        <div class="notes">{value || "—"}</div>
      {:else if revealable}
        <input type={revealed ? "text" : "password"} {value} readonly />
      {:else}
        <input type="text" {value} readonly />
      {/if}
      {#if !multiline && value && revealable}
        <button
          class="btn-copy-solid"
          aria-label={revealed ? "Hide" : "Reveal"}
          on:click={() => (revealed = !revealed)}
        >
          {#if revealed}
            <EyeOffIcon size={16} />
          {:else}
            <EyeIcon size={16} />
          {/if}
        </button>
      {/if}
      {#if !multiline && value && copyable}
        <button class="btn-copy-solid" aria-label="Copy" on:click={copy}>
          <CopyIcon size={16} />
        </button>
      {/if}
    </div>
  {/if}
</div>

<style>
  .entry-input {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .field-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--muted-color);
    white-space: nowrap;
    flex-shrink: 0;
    min-width: 4rem;
  }

  .field-input {
    min-width: 0;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--input-border);
    border-radius: 0.5rem;
    background-color: var(--input-bg);
    color: var(--text-color);
    resize: vertical;
  }

  .field-input:focus {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }

  .label-input {
    width: 8rem;
    flex-shrink: 0;
  }

  .value-input {
    flex: 1;
    width: auto;
  }

  .field-row {
    display: flex;
    gap: 0.5rem;
    flex: 1;
    align-items: center;
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
    flex: 1;
    padding: 0.75rem;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    color: var(--text-color);
    font-size: 0.875rem;
    white-space: pre-wrap;
  }
</style>
