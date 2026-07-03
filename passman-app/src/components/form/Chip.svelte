<script>
  export let size = "medium";
  export let active = false;
  export let as = "button";
  export let type = "button";
  export let title = "";
  export let role = undefined;
  export let tabindex = undefined;

  const sizeClasses = {
    small: "chip-small",
    medium: "chip-medium",
    large: "chip-large",
  };
</script>

<svelte:element
  this={as}
  class="chip {sizeClasses[size]}"
  class:active
  {type}
  {title}
  {role}
  {tabindex}
  on:click
  on:contextmenu
  on:keydown
>
  <slot />
</svelte:element>

<style>
  .chip {
    display: inline-flex;
    align-items: center;
    background-color: var(--hover-bg);
    border: 1px solid var(--border-color);
    border-radius: 9999px;
    color: var(--text-color);
    cursor: pointer;
    line-height: 1;
  }

  .chip:hover {
    border-color: var(--accent-color);
  }

  .chip-small {
    padding: 0.125rem 0.375rem;
    font-size: 0.7rem;
  }

  .chip-medium {
    padding: 0.3rem 0.75rem 0.2rem;
    font-size: 0.875rem;
  }

  .chip-large {
    padding: 0.5rem 1rem;
    font-size: 1rem;
  }

  .chip.active {
    background-color: var(--selected-bg);
    color: var(--selected-text);
    border-color: var(--selected-bg);
  }
</style>
