<script>
  import { createEventDispatcher } from "svelte";

  export let value = "";
  export let type = "text";
  export let placeholder = "";
  export let label = "";
  export let class_ = "";
  export let onFocus = null;

  const dispatch = createEventDispatcher();
</script>

<label class="form-field">
  {#if label}
    <span class="form-label">{label}</span>
  {/if}
  <input
    class="form-input {class_}"
    {type}
    {value}
    {placeholder}
    on:focus={onFocus}
    on:input={(e) => dispatch("input", e.target.value)}
  />
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
    border: 1px solid var(--input-border);
    border-radius: 0.5rem;
    background-color: var(--input-bg);
    color: var(--text-color);
  }

  .form-input:focus {
    outline: 2px solid var(--accent-color);
    outline-offset: 1px;
  }
</style>
