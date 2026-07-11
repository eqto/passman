<script>
  import { createEventDispatcher } from "svelte";
  import { renameVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  export let vault = null;

  let settingsName = vault ? vault.name : "";
  const dispatch = createEventDispatcher();

  async function handleRename() {
    if (!vault || !settingsName.trim()) return;
    await renameVault(vault.id, settingsName.trim());
    dispatch("renamed");
  }

  function handleCancel() {
    dispatch("cancel");
  }

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
  }
</script>

<Dialog on:keydown={handleKeydown}>
  <DialogHeader on:close={handleCancel}>Vault Settings</DialogHeader>
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
      <button class="modal-cancel-btn" on:click={handleCancel}> Cancel </button>
      <button class="btn-primary" on:click={handleRename}> Save </button>
    </DialogActions>
  </DialogFooter>
</Dialog>
