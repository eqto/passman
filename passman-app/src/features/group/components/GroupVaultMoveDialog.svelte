<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  export const groupId = "";
  export let groupName = "";
  export let vaultName = "";
  export let action = "move";
  export let onMerge;
  export let onCopy;
  export let onCancel;

  let showNameInput = false;
  let newName = "";

  function startCopyAsNew() {
    newName = `${groupName} (copy)`;
    showNameInput = true;
  }

  function backToWarning() {
    showNameInput = false;
  }

  function handleCopy() {
    const trimmed = newName.trim();
    if (!trimmed) return;
    onCopy(trimmed);
  }

  function handleKeydown(event) {
    if (event.key === "Escape") onCancel();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  {#if showNameInput}
    <DialogHeader onclick={backToWarning}
      >{action === "copy" ? "Copy" : "Move"} to {vaultName}</DialogHeader
    >
    <DialogBody>
      <p>
        Enter a new group name for the {action === "copy" ? "copy" : "move"}.
      </p>
      <input
        class="modal-input"
        bind:value={newName}
        placeholder="New group name"
      />
    </DialogBody>
    <DialogFooter>
      <DialogActions>
        <button class="modal-cancel-btn" onclick={backToWarning}>Back</button>
        <button class="btn-primary" onclick={handleCopy}>Copy as new</button>
      </DialogActions>
    </DialogFooter>
  {:else}
    <DialogHeader onclick={onCancel}
      >{action === "copy" ? "Copy" : "Move"} to {vaultName}</DialogHeader
    >
    <DialogBody>
      <p>Vault "{vaultName}" already has a group named "{groupName}".</p>
    </DialogBody>
    <DialogFooter>
      <DialogActions>
        <button class="modal-cancel-btn" onclick={onCancel}>Cancel</button>
        <button class="btn-primary" onclick={onMerge}>Merge</button>
        <button class="btn-primary" onclick={startCopyAsNew}>Copy as new</button
        >
      </DialogActions>
    </DialogFooter>
  {/if}
</Dialog>
