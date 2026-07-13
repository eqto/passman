<script>
  let {
    value = $bindable(""),
    type = "text",
    placeholder = "",
    class_ = "",
    readonly = false,
    multiline = false,
    autofocus = false,
    onfocus = null,
    oninput = null,
  } = $props();

  let el = $state();
  $effect(() => {
    if (el && autofocus) el.focus();
  });
</script>

{#if multiline}
  <textarea
    bind:this={el}
    class="form-input {class_}"
    rows="3"
    {value}
    {placeholder}
    {readonly}
    {onfocus}
    oninput={(e) => {
      value = e.target.value;
      oninput?.(e.target.value);
    }}
  ></textarea>
{:else}
  <input
    bind:this={el}
    class="form-input {class_}"
    {type}
    {value}
    {placeholder}
    {readonly}
    {onfocus}
    oninput={(e) => {
      value = e.target.value;
      oninput?.(e.target.value);
    }}
  />
{/if}

<style>
  .form-input {
    flex: 1;
    font-size: 1rem;
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
