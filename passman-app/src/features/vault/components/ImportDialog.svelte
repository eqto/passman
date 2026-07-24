<script>
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { convertButtercupVault, convertKeepassVault } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { format = "buttercup", onsuccess = null, oncancel = null } = $props();

  const config = {
    buttercup: {
      title: "Import Buttercup Vault",
      fileLabel: "Buttercup file",
      filePlaceholder: "Select .bcup file",
      fileFilter: { name: "Buttercup Vault", extensions: ["bcup"] },
      passwordPlaceholder: "Buttercup master password",
      infoText: "Enter the Buttercup master password to import the vault.",
      importFn: convertButtercupVault,
      errorText: "Failed to import buttercup vault",
    },
    keepass: {
      title: "Import KeePass Database",
      fileLabel: "KeePass file",
      filePlaceholder: "Select .kdbx file",
      fileFilter: { name: "KeePass Database", extensions: ["kdbx"] },
      passwordPlaceholder: "KeePass master password",
      infoText: "Enter the KeePass master password to import the database.",
      importFn: convertKeepassVault,
      errorText: "Failed to import KeePass database",
    },
  };

  let cfg = $derived(config[format] || config.buttercup);

  let step = $state(1);
  let sourcePath = $state("");
  let password = $state("");
  let outputPath = $state("");
  let error = $state("");
  let loading = $state(false);
  let passwordInput;

  async function pickSourceFile() {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [cfg.fileFilter],
    });
    if (selected) {
      sourcePath = selected;
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

  function handleNext() {
    if (!sourcePath || !outputPath) return;
    error = "";
    step = 2;
    setTimeout(() => passwordInput?.focus(), 0);
  }

  async function handleImport() {
    if (!sourcePath || !outputPath || !password) return;

    loading = true;
    error = "";

    try {
      await cfg.importFn(sourcePath, password, outputPath);
      onsuccess?.();
      reset();
    } catch (e) {
      error = e.message || cfg.errorText;
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
    sourcePath = "";
    password = "";
    outputPath = "";
    error = "";
    loading = false;
  }

  function handleBack() {
    step = 1;
    error = "";
  }

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
    if (event.key === "Enter" && step === 1 && sourcePath && outputPath) {
      handleNext();
    }
    if (
      event.key === "Enter" &&
      step === 2 &&
      sourcePath &&
      outputPath &&
      password
    ) {
      handleImport();
    }
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={handleCancel}>
    {#if step === 1}
      {cfg.title}
    {:else}
      Enter Password
    {/if}
  </DialogHeader>
  <DialogBody>
    {#if error}
      <div class="error-message">{error}</div>
    {/if}
    <div class="modal-form">
      {#if step === 1}
        <div class="form-group">
          <label for="source-path">{cfg.fileLabel}</label>
          <div class="path-row">
            <input
              id="source-path"
              class="modal-input"
              bind:value={sourcePath}
              placeholder={cfg.filePlaceholder}
              readonly
            />
            <button
              class="btn-secondary browse-btn"
              onclick={pickSourceFile}
              disabled={loading}
            >
              Browse
            </button>
          </div>
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
      {:else}
        <div class="info-message">
          {cfg.infoText}
        </div>
        <div class="form-group">
          <label for="password">Password</label>
          <input
            id="password"
            class="modal-input"
            bind:value={password}
            type="password"
            placeholder={cfg.passwordPlaceholder}
            disabled={loading}
            bind:this={passwordInput}
          />
        </div>
        {#if loading}
          <div class="progress-bar" aria-label="Importing vault">
            <div class="progress-bar-fill"></div>
          </div>
        {/if}
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
          onclick={handleNext}
          disabled={!sourcePath || !outputPath || loading}
        >
          Next
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
          disabled={!password || loading}
        >
          {loading ? "Importing..." : "Import"}
        </button>
      {/if}
    </DialogActions>
  </DialogFooter>
</Dialog>

<style>
  :global(.modal) {
    --modal-width: 35rem;
  }

  .modal-form :global(.modal-input[readonly]) {
    cursor: text;
  }

  .info-message {
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

  .progress-bar {
    height: 4px;
    background-color: var(--hover-bg);
    border-radius: var(--shape-full);
    overflow: hidden;
    margin-top: var(--space-2);
  }

  .progress-bar-fill {
    height: 100%;
    width: 40%;
    background-color: var(--accent-color);
    border-radius: var(--shape-full);
    animation: progress-indeterminate 1.2s ease-in-out infinite;
  }

  @keyframes progress-indeterminate {
    0% {
      margin-left: -40%;
    }
    100% {
      margin-left: 100%;
    }
  }
</style>
