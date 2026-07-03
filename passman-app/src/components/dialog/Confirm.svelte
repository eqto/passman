<script>
  import { createEventDispatcher } from "svelte";
  import Dialog from "./Dialog.svelte";
  import DialogHeader from "./DialogHeader.svelte";
  import DialogBody from "./DialogBody.svelte";
  import DialogFooter from "./DialogFooter.svelte";
  import DialogActions from "./DialogActions.svelte";

  export let title = "Confirm";
  export let message = "";
  export let confirmLabel = "Confirm";
  export let cancelLabel = "Cancel";

  const dispatch = createEventDispatcher();
</script>

<Dialog>
  <DialogHeader on:close={() => dispatch("cancel")}>{title}</DialogHeader>
  <DialogBody>
    <p>{message}</p>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" on:click={() => dispatch("cancel")}>
        {cancelLabel}
      </button>
      <button class="btn-primary" on:click={() => dispatch("confirm")}>
        {confirmLabel}
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
