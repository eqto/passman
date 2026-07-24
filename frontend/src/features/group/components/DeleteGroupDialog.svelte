<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { group = null, onDelete, onCancel } = $props();
</script>

<Dialog>
  <DialogHeader onclick={onCancel}>Delete Group</DialogHeader>
  <DialogBody>
    <p>
      Delete group "{group?.name ?? group}"? Entries in this group will be moved
      to trash.
    </p>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" onclick={onCancel}> Cancel </button>
      <button class="btn-danger" onclick={() => onDelete(group)}>
        Delete
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
