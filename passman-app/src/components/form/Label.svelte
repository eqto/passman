<script>
  let { text = "", label = "", width = "5rem" } = $props();

  let displayText = $derived(text || label);
</script>

<span class="label" style="width: {width}">
  {displayText}
</span>

<style>
  .label {
    font-size: 1rem;
    color: var(--muted-color);
    font-weight: 500;
    white-space: normal;
    word-break: break-word;
    flex-shrink: 0;
    padding-top: 0.5rem;
  }
</style>
