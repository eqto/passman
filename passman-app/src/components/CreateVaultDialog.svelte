<script>
  import { createEventDispatcher } from "svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { createVault } from "../stores/vaults";
  import { Dialog, DialogHeader, DialogBody, DialogFooter, DialogActions } from "./dialog";

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

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
  }
</script>

<Dialog on:keydown={handleKeydown}>
  <DialogHeader on:close={handleCancel}>Create Vault</DialogHeader>
  <DialogBody>
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
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" on:click={handleCancel}>
        Cancel
      </button>
      <button class="btn-primary" on:click={handleCreate}>
        Create
      </button>
    </DialogActions>
  </DialogFooter>
</Dialog>

<style>
  .path-row .modal-input {
    flex: 1;
  }
</style>
