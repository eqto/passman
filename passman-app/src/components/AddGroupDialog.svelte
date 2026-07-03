<script>
  import Dialog from "./dialog/Dialog.svelte";
  import DialogHeader from "./dialog/DialogHeader.svelte";
  import DialogBody from "./dialog/DialogBody.svelte";
  import DialogFooter from "./dialog/DialogFooter.svelte";
  import DialogActions from "./dialog/DialogActions.svelte";

  export let title = "Add Group";
  export let onAdd;
  export let onCancel;

  let groupName = "";
  let error = "";

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

<Dialog on:keydown={handleKeydown}>
  <DialogHeader on:close={onCancel}>{title}</DialogHeader>
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
      <button class="modal-cancel-btn" on:click={onCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={handleAdd}>
        Add
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
