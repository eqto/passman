<script>
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { convertKeepassVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { onsuccess = null, oncancel = null } = $props();

  let step = $state(1);
  let kdbxPath = $state("");
  let password = $state("");
  let outputPath = $state("");
  let vaultName = $state("");
  let error = $state("");
  let loading = $state(false);
  let passwordInput;

  async function pickKdbxFile() {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [{ name: "KeePass Database", extensions: ["kdbx"] }],
    });
    if (selected) {
      kdbxPath = selected;
      passwordInput.focus();
    }
  }

  async function handleDecrypt() {
    if (!kdbxPath || !password) return;

    loading = true;
    error = "";

    try {
      const fileName = kdbxPath
        .split(/[/\\]/)
        .pop()
        .replace(/\.kdbx$/i, "");
      vaultName = fileName || "Imported KeePass Database";

      step = 2;
    } catch (e) {
      error = e.message || "Failed to decrypt KeePass database";
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
      await convertKeepassVault(kdbxPath, password, outputPath);
      onsuccess?.();
      reset();
    } catch (e) {
      error = e.message || "Failed to import KeePass database";
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
    kdbxPath = "";
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
    if (event.key === "Enter" && step === 1 && kdbxPath && password) {
      handleDecrypt();
    }
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={handleCancel}>
    {#if step === 1}
      Import KeePass Database
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
          <label for="kdbx-path">KeePass file</label>
          <div class="path-row">
            <input
              id="kdbx-path"
              class="modal-input"
              bind:value={kdbxPath}
              placeholder="Select .kdbx file"
              readonly
            />
            <button
              class="btn-secondary browse-btn"
              onclick={pickKdbxFile}
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
            placeholder="KeePass master password"
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
          disabled={!kdbxPath || !password || loading}
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
    padding: var(--space-3);
    background-color: var(--hover-bg);
    color: var(--text-color);
    border-radius: var(--shape-sm);
    font-size: var(--font-size-sm);
  }

  .error-message {
    padding: var(--space-3);
    background-color: var(--danger-container);
    color: var(--on-danger-container);
    border-radius: var(--shape-sm);
    font-size: var(--font-size-sm);
  }
</style>
