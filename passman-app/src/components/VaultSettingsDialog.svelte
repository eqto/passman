<script>
  import { createEventDispatcher } from "svelte";
  import { renameVault } from "../stores/vaults";

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
</script>

<div class="modal-overlay">
  <div class="modal">
    <h2>Vault Settings</h2>
    <div class="modal-form">
      <input class="modal-input" bind:value={settingsName} placeholder="Vault name" />
      <div class="path-field form-group">
        <label for="vault-path">File location</label>
        <input id="vault-path" class="modal-input" type="text" value={vault?.path || ""} readonly />
      </div>
    </div>
    <div class="modal-actions">
      <button class="modal-cancel-btn" on:click={handleCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={handleRename}>
        Save
      </button>
    </div>
  </div>
</div>

