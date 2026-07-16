<script>
  let {
    title = "",
    showButton = false,
    onButtonClick = null,
    isTrash = false,
  } = $props();
</script>

<div class="section-header" class:trash-header={isTrash}>
  <span>{title}</span>
  {#if showButton && onButtonClick}
    <button class="btn-icon" title="New" onclick={onButtonClick}> + </button>
  {/if}
</div>

<style>
  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    margin-bottom: 0;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--muted-color);
    letter-spacing: 0.05em;
  }

  .section-header:not(:first-child) {
    margin-top: 1rem;
  }

  .trash-header {
    justify-content: flex-start;
    gap: 0.5rem;
  }
</style>
