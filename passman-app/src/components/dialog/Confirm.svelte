<script>
  import Dialog from "./Dialog.svelte";
  import DialogHeader from "./DialogHeader.svelte";
  import DialogBody from "./DialogBody.svelte";
  import DialogFooter from "./DialogFooter.svelte";
  import DialogActions from "./DialogActions.svelte";

  let {
    title = "Confirm",
    message = "",
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    onconfirm = null,
    oncancel = null,
  } = $props();
</script>

<Dialog>
  <DialogHeader onclick={oncancel}>{title}</DialogHeader>
  <DialogBody>
    <p>{message}</p>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" onclick={oncancel}>
        {cancelLabel}
      </button>
      <button class="btn-primary" onclick={onconfirm}>
        {confirmLabel}
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
