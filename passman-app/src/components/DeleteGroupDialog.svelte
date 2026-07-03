<script>
  import { Dialog, DialogHeader, DialogBody, DialogFooter, DialogActions } from "./dialog";

  export let group = "";
  export let onDelete;
  export let onCancel;
</script>

<Dialog>
  <DialogHeader on:close={onCancel}>Delete Group</DialogHeader>
  <DialogBody>
    <p>Delete group "{group}"? Entries in this group will keep their other tags.</p>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" on:click={onCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={() => onDelete(group)}>
        Delete
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
