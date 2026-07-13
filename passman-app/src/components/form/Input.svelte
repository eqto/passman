<script>
  import { createEventDispatcher } from "svelte";
  import { EyeIcon, EyeOffIcon, CopyIcon } from "../icons";

  export let value = "";
  export let type = "text";
  export let placeholder = "";
  export let label = "";
  export let class_ = "";
  export let onFocus = null;
  export let revealable = false;
  export let readonly = false;
  export let transparent = false;
  export let copyable = false;
  export let copyLabel = "Copy";
  export let multiline = false;

  const dispatch = createEventDispatcher();
  let revealed = false;

  function copy() {
    dispatch("copy", value);
  }
</script>

<label class="form-field" class:transparent>
  {#if label}
    <span class="form-label">{label}</span>
  {/if}
  {#if multiline}
    <textarea
      class="form-input {class_}"
      rows="3"
      {value}
      {placeholder}
      {readonly}
      on:focus={onFocus}
      on:input={(e) => dispatch("input", e.target.value)}
    ></textarea>
  {:else}
    <input
      class="form-input {class_}"
      type={!value ? "text" : revealable && revealed ? "text" : type}
      {value}
      {placeholder}
      {readonly}
      on:focus={onFocus}
      on:input={(e) => dispatch("input", e.target.value)}
    />
  {/if}
  {#if copyable}
    <button
      class="btn-copy-solid"
      type="button"
      aria-label={copyLabel}
      disabled={!value}
      on:click={copy}
    >
      <CopyIcon size={16} />
    </button>
  {/if}
  {#if revealable}
    <button
      class="btn-copy-solid"
      type="button"
      aria-label={revealed ? "Hide" : "Reveal"}
      disabled={!value}
      on:click={() => (revealed = !revealed)}
    >
      {#if revealed}
        <EyeOffIcon size={16} />
      {:else}
        <EyeIcon size={16} />
      {/if}
    </button>
  {/if}
</label>

<style>
  .form-field {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
  }

  .form-label {
    font-size: 0.8rem;
    color: var(--muted-color);
    font-weight: 500;
    white-space: nowrap;
    flex-shrink: 0;
    min-width: 4rem;
  }

  .form-input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    line-height: 1.5;
    border: 1px solid var(--input-border);
    border-radius: 0.5rem;
    background-color: var(--input-bg);
    color: var(--text-color);
    resize: vertical;
  }

  .form-input:focus {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }

  .transparent .form-input {
    background-color: transparent;
    border-color: transparent;
    cursor: text;
  }

  .transparent .form-input:hover {
    border-color: var(--input-border);
    border-style: dashed;
  }

  .transparent .form-input:focus {
    outline: none;
    border-color: var(--accent-color);
    border-style: dashed;
  }
</style>
