<script>
  import { Clipboard } from "@wailsio/runtime";
  import { showToast } from "../../../stores/toast.js";
  import { Input, Label } from "../../../components/form";
  import { Icon } from "../../../components/icons";
  import PasswordGenerator from "../../../components/PasswordGenerator.svelte";

  let {
    label = "",
    value = "",
    type = "text",
    editing = false,
    labelPlaceholder = "",
    valuePlaceholder = "",
    copyable = true,
    revealable = false,
    multiline = false,
    onFocus = null,
    onlabelchange = null,
    oninput = null,
  } = $props();

  let revealed = $state(false);

  async function copy() {
    if (!value) return;
    await Clipboard.SetText(value);
    showToast(`${label || "Value"} copied to clipboard`);
  }
</script>

<div class="entry-input">
  {#if editing}
    <Input
      value={label}
      placeholder={labelPlaceholder}
      onfocus={onFocus}
      oninput={(v) => onlabelchange?.(v)}
      class_="label-input"
    />
    <div class="value-wrapper">
      <Input
        {type}
        {value}
        placeholder={valuePlaceholder}
        {multiline}
        oninput={(v) => oninput?.(v)}
      />
      {#if revealable}
        <PasswordGenerator onuse={(pw) => oninput?.(pw)} />
      {/if}
    </div>
  {:else}
    <Label text={label || "<empty>"} />
    <Input
      {value}
      type={revealable && revealed ? "text" : type}
      placeholder="<empty>"
      readonly={true}
      class_="transparent {!value ? 'empty' : ''}"
      {multiline}
    />
    {#if copyable}
      <button
        class="btn-copy-solid"
        type="button"
        aria-label="Copy"
        disabled={!value}
        onclick={copy}
      >
        <Icon name="copy" size={16} />
      </button>
    {/if}
    {#if revealable}
      <button
        class="btn-copy-solid"
        type="button"
        aria-label={revealed ? "Hide" : "Reveal"}
        disabled={!value}
        onclick={() => (revealed = !revealed)}
      >
        {#if revealed}
          <Icon name="eye-off" size={16} />
        {:else}
          <Icon name="eye" size={16} />
        {/if}
      </button>
    {/if}
  {/if}
</div>

<style>
  .entry-input {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    gap: 0.5rem;
    min-width: 0;
  }

  .entry-input > :global(.form-input) {
    flex: 1;
    min-width: 0;
  }

  .entry-input > :global(.label-input) {
    width: 5rem;
    flex: 0 0 5rem;
  }

  .value-wrapper {
    position: relative;
    flex: 1;
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    min-width: 0;
  }

  .entry-input > :global(.transparent) {
    background-color: transparent;
    border-color: transparent;
    cursor: text;
  }

  .entry-input > :global(.transparent:hover) {
    border-color: var(--input-border);
    border-style: dashed;
  }

  .entry-input > :global(.transparent:focus) {
    outline: none;
  }

  .entry-input > :global(.empty) {
    color: var(--muted-color);
    font-style: italic;
  }
</style>
