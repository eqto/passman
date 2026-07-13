<script>
  import { createEventDispatcher } from "svelte";
  import { CopyIcon, EyeIcon, EyeOffIcon } from "../../../components/icons";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { showToast } from "../../../stores/toast.js";
  import Input from "../../../components/form/Input.svelte";

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
    <Input
      {value}
      {type}
      label={label || "<empty>"}
      placeholder="<empty>"
      readonly={true}
      transparent={true}
      {revealable}
      {copyable}
      {multiline}
      copyLabel="Copy"
      class_:empty={!value}
      on:copy={() => copy()}
    />
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

  .entry-input > .form-field {
    flex: 1;
    min-width: 0;
  }

  .field-input {
    min-width: 0;
    padding: 0.5rem 0.75rem;
    line-height: 1.5;
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

  .empty {
    color: var(--muted-color);
    font-style: italic;
  }
</style>
