<script>
  import { renameVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { vault = null, onrenamed = null, oncancel = null } = $props();

  let settingsName = $state(vault ? vault.name : "");

  async function handleRename() {
    if (!vault || !settingsName.trim()) return;
    await renameVault(vault.id, settingsName.trim());
    onrenamed?.();
  }

  function handleCancel() {
    oncancel?.();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={handleCancel}>Vault Settings</DialogHeader>
  <DialogBody>
    <div class="modal-form">
      <input
        class="modal-input"
        bind:value={settingsName}
        placeholder="Vault name"
      />
      <div class="path-field form-group">
        <label for="vault-path">File location</label>
        <input
          id="vault-path"
          class="modal-input"
          type="text"
          value={vault?.path || ""}
          readonly
        />
      </div>
    </div>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" onclick={handleCancel}> Cancel </button>
      <button class="btn-primary" onclick={handleRename}> Save </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
