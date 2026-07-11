<script>
  export let title = "";
  export let showButton = false;
  export let onButtonClick = null;
  export let isTrash = false;
</script>

<div class="section-header" class:trash-header={isTrash}>
  <span>{title}</span>
  {#if showButton && onButtonClick}
    <button class="btn-icon" title="New" on:click={onButtonClick}>
      +
    </button>
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
    line-height: 1.75;
  }

  .section-header:not(:first-child) {
    margin-top: 1rem;
  }

  .trash-header {
    justify-content: flex-start;
    gap: 0.5rem;
  }
</style>
