<script>
  let {
    text = "",
    size = "medium",
    active = false,
    as = "button",
    type = "button",
    title = "",
    role = undefined,
    tabindex = undefined,
    onclick = null,
    oncontextmenu = null,
    onkeydown = null,
  } = $props();

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
  class:disabled={!onclick}
  {type}
  {title}
  {role}
  {tabindex}
  {onclick}
  {oncontextmenu}
  {onkeydown}
>
  {text}
</svelte:element>

<style>
  .chip {
    display: inline-flex;
    align-items: center;
    background-color: var(--chip-bg);
    border: 1px solid var(--chip-border);
    border-radius: 9999px;
    color: var(--chip-text);
    cursor: pointer;
    line-height: 1;
  }

  .chip:hover:not(.disabled) {
    background-color: var(--chip-hover-bg);
    border-color: var(--chip-hover-border);
  }

  .chip.disabled {
    background-color: var(--chip-bg);
    border-color: var(--chip-border);
    color: var(--chip-text);
    cursor: default;
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
    background-color: var(--chip-active-bg);
    color: var(--chip-active-text);
    border-color: var(--chip-active-border);
  }
</style>
