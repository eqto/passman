<script>
  import Dialog from "./dialog/Dialog.svelte";
  import DialogHeader from "./dialog/DialogHeader.svelte";
  import DialogBody from "./dialog/DialogBody.svelte";
  import DialogFooter from "./dialog/DialogFooter.svelte";
  import DialogActions from "./dialog/DialogActions.svelte";

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
