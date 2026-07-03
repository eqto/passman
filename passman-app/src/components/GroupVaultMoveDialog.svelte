<script>
  import { Dialog, DialogHeader, DialogBody, DialogFooter, DialogActions } from "./dialog";

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

  function handleKeydown(event) {
    if (event.key === "Escape") onCancel();
  }
</script>

<Dialog on:keydown={handleKeydown}>
  {#if showNameInput}
    <DialogHeader on:close={backToWarning}>{action === "copy" ? "Copy" : "Move"} to {vaultName}</DialogHeader>
    <DialogBody>
      <p>Enter a new group name for the {action === "copy" ? "copy" : "move"}.</p>
      <input class="modal-input" bind:value={newName} placeholder="New group name" />
    </DialogBody>
    <DialogFooter>
      <DialogActions>
        <button class="modal-cancel-btn" on:click={backToWarning}>Back</button>
        <button class="btn-primary" on:click={handleCopy}>Copy as new</button>
      </DialogActions>
    </DialogFooter>
  {:else}
    <DialogHeader on:close={onCancel}>{action === "copy" ? "Copy" : "Move"} to {vaultName}</DialogHeader>
    <DialogBody>
      <p>Vault "{vaultName}" already has a group named "{group}".</p>
    </DialogBody>
    <DialogFooter>
      <DialogActions>
        <button class="modal-cancel-btn" on:click={onCancel}>Cancel</button>
        <button class="btn-primary" on:click={onMerge}>Merge</button>
        <button class="btn-primary" on:click={startCopyAsNew}>Copy as new</button>
      </DialogActions>
    </DialogFooter>
  {/if}
</Dialog>
