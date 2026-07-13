<script>
  import { createEventDispatcher } from "svelte";

  export let value = "";
  export let type = "text";
  export let placeholder = "";
  export let class_ = "";
  export let onFocus = null;
  export let readonly = false;
  export let multiline = false;

  const dispatch = createEventDispatcher();
</script>

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
    {type}
    {value}
    {placeholder}
    {readonly}
    on:focus={onFocus}
    on:input={(e) => dispatch("input", e.target.value)}
  />
{/if}

<style>
  .form-input {
    flex: 1;
    min-width: 0;
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
</style>
