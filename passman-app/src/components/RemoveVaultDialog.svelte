<script>
  export let vault;
  export let onRemove;
  export let onCancel;

  function handleRemove() {
    onRemove();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") onCancel();
  }
</script>

<div class="modal-overlay">
  <div
    class="modal"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    on:keydown={handleKeydown}
  >
    <h2>Remove Vault</h2>
    <p class="message">
      Remove "{vault.name}" from Passman?
    </p>
    <div class="modal-actions">
      <button class="modal-cancel-btn" on:click={onCancel}>
        Cancel
      </button>
      <button class="modal-danger-btn" on:click={handleRemove}>
        Remove
      </button>
    </div>
  </div>
</div>

<style>
  .message {
    margin: 0 0 0.5rem;
    font-size: 0.875rem;
    color: var(--text-color);
  }
</style>
