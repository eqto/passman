<script>
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { convertButtercupVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { onsuccess = null, oncancel = null } = $props();

  let step = $state(1);
  let bcupPath = $state("");
  let password = $state("");
  let outputPath = $state("");
  let vaultName = $state("");
  let error = $state("");
  let loading = $state(false);
  let passwordInput;

  async function pickBcupFile() {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [{ name: "Buttercup Vault", extensions: ["bcup"] }],
    });
    if (selected) {
      bcupPath = selected;
      passwordInput.focus();
    }
  }

  async function handleDecrypt() {
    if (!bcupPath || !password) return;

    loading = true;
    error = "";

    try {
      const fileName = bcupPath
        .split(/[/\\]/)
        .pop()
        .replace(/\.bcup$/i, "");
      vaultName = fileName || "Imported Buttercup Vault";

      step = 2;
    } catch (e) {
      error = e.message || "Failed to decrypt buttercup file";
    } finally {
      loading = false;
    }
  }

  async function pickOutputFile() {
    const selected = await save({
      filters: [{ name: "Passman Vault", extensions: ["pmv"] }],
    });
    if (selected) {
      outputPath = selected.endsWith(".pmv") ? selected : `${selected}.pmv`;
    }
  }

  async function handleImport() {
    if (!outputPath) return;

    loading = true;
    error = "";

    try {
      await convertButtercupVault(bcupPath, password, outputPath);
      onsuccess?.();
      reset();
    } catch (e) {
      error = e.message || "Failed to import buttercup vault";
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    reset();
    oncancel?.();
  }

  function reset() {
    step = 1;
    bcupPath = "";
    password = "";
    outputPath = "";
    vaultName = "";
    error = "";
    loading = false;
  }

  function handleBack() {
    step = 1;
    error = "";
  }

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
    if (event.key === "Enter" && step === 1 && bcupPath && password) {
      handleDecrypt();
    }
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={handleCancel}>
    {#if step === 1}
      Import Buttercup Vault
    {:else}
      Save Passman Vault
    {/if}
  </DialogHeader>
  <DialogBody>
    {#if error}
      <div class="error-message">{error}</div>
    {/if}
    <div class="modal-form">
      {#if step === 1}
        <div class="form-group">
          <label for="bcup-path">Buttercup file</label>
          <div class="path-row">
            <input
              id="bcup-path"
              class="modal-input"
              bind:value={bcupPath}
              placeholder="Select .bcup file"
              readonly
            />
            <button
              class="btn-secondary browse-btn"
              onclick={pickBcupFile}
              disabled={loading}
            >
              Browse
            </button>
          </div>
        </div>
        <div class="form-group">
          <label for="password">Password</label>
          <input
            id="password"
            class="modal-input"
            bind:value={password}
            type="password"
            placeholder="Buttercup master password"
            disabled={loading}
            bind:this={passwordInput}
          />
        </div>
      {:else}
        <div class="success-message">
          Successfully opening {vaultName}. Select where to save the new file
        </div>
        <div class="form-group">
          <label for="output-path">Save as</label>
          <div class="path-row">
            <input
              id="output-path"
              class="modal-input"
              bind:value={outputPath}
              placeholder="Select save location"
              readonly
            />
            <button
              class="btn-secondary browse-btn"
              onclick={pickOutputFile}
              disabled={loading}
            >
              Browse
            </button>
          </div>
        </div>
      {/if}
    </div>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      {#if step === 1}
        <button
          class="modal-cancel-btn"
          onclick={handleCancel}
          disabled={loading}
        >
          Cancel
        </button>
        <button
          class="btn-primary"
          onclick={handleDecrypt}
          disabled={!bcupPath || !password || loading}
        >
          {loading ? "Decrypting..." : "Next"}
        </button>
      {:else}
        <button
          class="modal-cancel-btn"
          onclick={handleBack}
          disabled={loading}
        >
          Back
        </button>
        <button
          class="btn-primary"
          onclick={handleImport}
          disabled={!outputPath || loading}
        >
          {loading ? "Saving..." : "Save"}
        </button>
      {/if}
    </DialogActions>
  </DialogFooter>
</Dialog>

<style>
  .success-message {
    padding: 0.75rem;
    background-color: rgba(34, 197, 94, 0.1);
    color: #22c55e;
    border-radius: 0.5rem;
    font-size: 0.875rem;
  }

  .error-message {
    padding: 0.75rem;
    background-color: rgba(239, 68, 68, 0.1);
    color: var(--danger-color);
    border-radius: 0.5rem;
    font-size: 0.875rem;
  }
</style>
