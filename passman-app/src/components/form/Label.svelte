<script>
  export let text = "";
  export let label = "";
  export let width = "5rem";

  $: displayText = text || label;
</script>

<span class="label" style="width: {width}">
  {displayText}
  <slot />
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
