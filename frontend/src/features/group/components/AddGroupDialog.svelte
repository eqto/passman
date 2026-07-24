<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { title = "Add Group", onAdd, onCancel } = $props();

  let groupName = $state("");
  let error = $state("");

  function handleAdd() {
    error = "";
    const trimmed = groupName.trim();
    if (!trimmed) {
      error = `${title.replace("Add ", "")} name cannot be empty.`;
      return;
    }
    onAdd(trimmed);
    groupName = "";
  }

  function handleKeydown(event) {
    if (event.key === "Enter") handleAdd();
    if (event.key === "Escape") onCancel();
  }

  function focus(node) {
    node.focus();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={onCancel}>{title}</DialogHeader>
  <DialogBody>
    <input
      class="modal-input"
      bind:value={groupName}
      placeholder={`${title.replace("Add ", "")} name`}
      use:focus
    />
    {#if error}
      <p class="modal-error">{error}</p>
    {/if}
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" onclick={onCancel}> Cancel </button>
      <button class="btn-primary" onclick={handleAdd}> Add </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
