<script>
  export let title = "Add Group";
  export let onAdd;
  export let onCancel;

  let groupName = "";
  let error = "";

  function handleAdd() {
    error = "";
    const trimmed = groupName.trim();
    if (!trimmed) {
      error = `${title.replace("Add ", "")} name cannot be empty.`;
      return;
    }
    onAdd(trimmed);
    groupName = "";
  }

  function handleKeydown(event) {
    if (event.key === "Enter") handleAdd();
    if (event.key === "Escape") onCancel();
  }

  function focus(node) {
    node.focus();
  }
</script>

<div class="modal-overlay">
  <div class="modal">
    <h2>{title}</h2>
    <input
      class="modal-input"
      bind:value={groupName}
      placeholder={`${title.replace("Add ", "")} name`}
      on:keydown={handleKeydown}
      use:focus
    />
    {#if error}
      <p class="modal-error">{error}</p>
    {/if}
    <div class="modal-actions">
      <button class="modal-cancel-btn" on:click={onCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={handleAdd}>
        Add
      </button>
    </div>
  </div>
</div>
