<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  export let group = null;
  export let onDelete;
  export let onCancel;
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
