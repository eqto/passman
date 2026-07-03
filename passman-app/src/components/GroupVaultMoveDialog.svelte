<script>
  export let group = "";
  export let vaultName = "";
  export let action = "move";
  export let onMerge;
  export let onCopy;
  export let onCancel;

  let showNameInput = false;
  let newName = "";

  function startCopyAsNew() {
    newName = `${group} (copy)`;
    showNameInput = true;
  }

  function backToWarning() {
    showNameInput = false;
  }

  function handleCopy() {
    const trimmed = newName.trim();
    if (!trimmed) return;
    onCopy(trimmed);
  }
</script>

<div class="modal-overlay">
  <div class="modal">
    {#if showNameInput}
      <h2>{action === "copy" ? "Copy" : "Move"} to {vaultName}</h2>
      <p>Enter a new group name for the {action === "copy" ? "copy" : "move"}.</p>
      <input class="modal-input" bind:value={newName} placeholder="New group name" />
      <div class="modal-actions">
        <button class="modal-cancel-btn" on:click={backToWarning}>Back</button>
        <button class="btn-primary" on:click={handleCopy}>Copy as new</button>
      </div>
    {:else}
      <h2>{action === "copy" ? "Copy" : "Move"} to {vaultName}</h2>
      <p>Vault "{vaultName}" already has a group named "{group}".</p>
      <div class="modal-actions">
        <button class="modal-cancel-btn" on:click={onCancel}>Cancel</button>
        <button class="btn-primary" on:click={onMerge}>Merge</button>
        <button class="btn-primary" on:click={startCopyAsNew}>Copy as new</button>
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
  }
</style>
