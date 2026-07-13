<script>
  import { save } from "@tauri-apps/plugin-dialog";
  import { createVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { oncreated = null, oncancel = null } = $props();

  let newName = $state("");
  let newPath = $state("");
  let newPassword = $state("");
  let error = $state("");
  let isCreating = $state(false);

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
      oncreated?.();
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
    oncancel?.();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={handleCancel}>Create Vault</DialogHeader>
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
        <button class="btn-secondary browse-btn" onclick={pickFile}>
          Browse
        </button>
      </div>
      <input
        class="modal-input"
        bind:value={newPassword}
        type="password"
        placeholder="Vault password"
        disabled={isCreating}
        onkeydown={(e) => e.key === "Enter" && handleCreate()}
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
        onclick={handleCancel}
        disabled={isCreating}
      >
        Cancel
      </button>
      <button class="btn-primary" onclick={handleCreate} disabled={isCreating}>
        Create
      </button>
    </DialogActions>
  </DialogFooter>
  {#if isCreating}
    <div class="progress-wrapper">
      <div class="progress-bar">
        <div class="progress-indeterminate"></div>
      </div>
    </div>
  {/if}
</Dialog>

<style>
  .path-row .modal-input {
    flex: 1;
  }

  .progress-wrapper {
    padding-top: 1rem;
  }

  .progress-bar {
    width: 100%;
    height: 0.625rem;
    background-color: var(--input-border);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .progress-indeterminate {
    width: 40%;
    height: 100%;
    background: linear-gradient(
      90deg,
      var(--accent-color),
      var(--accent-hover)
    );
    border-radius: 0.5rem;
    animation: indeterminate 1.5s ease-in-out infinite;
  }

  @keyframes indeterminate {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(250%);
    }
  }
</style>
