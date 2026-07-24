<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { vault, onRemove, onCancel } = $props();

  function handleRemove() {
    onRemove();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") onCancel();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={onCancel}>Remove Vault</DialogHeader>
  <DialogBody>
    <p>Remove "{vault.name}" from Passman?</p>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" onclick={onCancel}> Cancel </button>
      <button class="btn-danger" onclick={handleRemove}> Remove </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
