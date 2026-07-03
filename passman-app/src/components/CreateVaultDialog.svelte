<script>
  import { createEventDispatcher } from "svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { createVault } from "../stores/vaults";

  let newName = "";
  let newPath = "";
  let newPassword = "";

  const dispatch = createEventDispatcher();

  async function pickFile() {
    const selected = await save({
      filters: [{ name: "Passman Vault", extensions: ["pmv"] }],
    });
    if (selected) {
      newPath = selected.endsWith(".pmv") ? selected : `${selected}.pmv`;
    }
  }

  async function handleCreate() {
    if (!newName || !newPath || !newPassword) return;
    const id = crypto.randomUUID();
    await createVault(id, newName, newPath, newPassword);
    reset();
    dispatch("created");
  }

  function reset() {
    newName = "";
    newPath = "";
    newPassword = "";
  }

  function handleCancel() {
    reset();
    dispatch("cancel");
  }
</script>

<div class="modal-overlay">
  <div class="modal">
    <h2>Create Vault</h2>
    <div class="modal-form">
      <input class="modal-input" bind:value={newName} placeholder="Vault name" />
      <div class="path-row">
        <input class="modal-input" bind:value={newPath} placeholder="File path" />
        <button class="btn-secondary browse-btn" on:click={pickFile}>
          Browse
        </button>
      </div>
      <input class="modal-input" bind:value={newPassword} type="password" placeholder="Vault password" />
    </div>
    <div class="modal-actions">
      <button class="modal-cancel-btn" on:click={handleCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={handleCreate}>
        Create
      </button>
    </div>
  </div>
</div>

<style>
  .path-row .modal-input {
    flex: 1;
  }
</style>
