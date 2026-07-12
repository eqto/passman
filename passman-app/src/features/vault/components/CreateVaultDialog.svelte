<script>
  import { createEventDispatcher } from "svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { createVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let newName = "";
  let newPath = "";
  let newPassword = "";
  let error = "";
  let isCreating = false;

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
    if (isCreating) return;
    if (!newName || !newPath || !newPassword) return;
    error = "";
    isCreating = true;
    try {
      const id = crypto.randomUUID();
      await createVault(id, newName, newPath, newPassword);
      reset();
      dispatch("created");
    } catch (e) {
      error = e.toString();
    } finally {
      isCreating = false;
    }
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
      <input
        class="modal-input"
        bind:value={newName}
        placeholder="Vault name"
        disabled={isCreating}
      />
      <div class="path-row">
        <input
          class="modal-input"
          bind:value={newPath}
          placeholder="File path"
          disabled={isCreating}
        />
        <button class="btn-secondary browse-btn" on:click={pickFile}>
          Browse
        </button>
      </div>
      <input
        class="modal-input"
        bind:value={newPassword}
        type="password"
        placeholder="Vault password"
        disabled={isCreating}
      />
      {#if error}
        <p class="modal-error">{error}</p>
      {/if}
    </div>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button
        class="modal-cancel-btn"
        on:click={handleCancel}
        disabled={isCreating}
      >
        Cancel
      </button>
      <button class="btn-primary" on:click={handleCreate} disabled={isCreating}>
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
