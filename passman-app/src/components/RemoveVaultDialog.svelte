<script>
  import Dialog from "./dialog/Dialog.svelte";
  import DialogHeader from "./dialog/DialogHeader.svelte";
  import DialogBody from "./dialog/DialogBody.svelte";
  import DialogFooter from "./dialog/DialogFooter.svelte";
  import DialogActions from "./dialog/DialogActions.svelte";

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

<Dialog on:keydown={handleKeydown}>
  <DialogHeader on:close={onCancel}>Remove Vault</DialogHeader>
  <DialogBody>
    <p>Remove "{vault.name}" from Passman?</p>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" on:click={onCancel}>
        Cancel
      </button>
      <button class="btn-danger" on:click={handleRemove}>
        Remove
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
