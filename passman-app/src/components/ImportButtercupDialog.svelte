<script>
  import { createEventDispatcher } from "svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { convertButtercupVault } from "../stores/vaults";

  let step = 1;
  let bcupPath = "";
  let password = "";
  let outputPath = "";
  let vaultName = "";
  let error = "";
  let loading = false;

  const dispatch = createEventDispatcher();

  async function pickBcupFile() {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [{ name: "Buttercup Vault", extensions: ["bcup"] }],
    });
    if (selected) {
      bcupPath = selected;
    }
  }

  async function handleDecrypt() {
    if (!bcupPath || !password) return;
    
    loading = true;
    error = "";
    
    try {
      // First, we'll decrypt the buttercup file to get the vault name
      // For now, we'll derive the name from the file path
      const fileName = bcupPath.split(/[/\\]/).pop().replace(/\.bcup$/i, "");
      vaultName = fileName || "Imported Buttercup Vault";
      
      // Move to step 2
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
      dispatch("success");
      reset();
    } catch (e) {
      error = e.message || "Failed to import buttercup vault";
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    reset();
    dispatch("cancel");
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
</script>

<div class="modal-overlay">
  <div class="modal">
    <h2>
      {#if step === 1}
        Import Buttercup Vault
      {:else}
        Save Passman Vault
      {/if}
    </h2>
    
    {#if error}
      <div class="error-message">{error}</div>
    {/if}
    
    <div class="modal-form">
      {#if step === 1}
        <div class="form-group">
          <label for="bcup-path">Buttercup file</label>
          <div class="path-row">
            <input id="bcup-path" class="modal-input" bind:value={bcupPath} placeholder="Select .bcup file" readonly />
            <button class="browse-btn" on:click={pickBcupFile} disabled={loading}>
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
          />
        </div>
      {:else}
        <div class="success-message">
          ✓ Successfully decrypted buttercup vault
        </div>
        <div class="form-group">
          <label for="vault-name">Vault name</label>
          <input id="vault-name" class="modal-input" bind:value={vaultName} placeholder="Vault name" disabled />
        </div>
        <div class="form-group">
          <label for="output-path">Save as</label>
          <div class="path-row">
            <input id="output-path" class="modal-input" bind:value={outputPath} placeholder="Select save location" readonly />
            <button class="browse-btn" on:click={pickOutputFile} disabled={loading}>
              Browse
            </button>
          </div>
        </div>
      {/if}
    </div>
    
    <div class="modal-actions">
      {#if step === 1}
        <button class="modal-cancel-btn" on:click={handleCancel} disabled={loading}>
          Cancel
        </button>
        <button 
          class="modal-primary-btn" 
          on:click={handleDecrypt} 
          disabled={!bcupPath || !password || loading}
        >
          {loading ? "Decrypting..." : "Next"}
        </button>
      {:else}
        <button class="modal-cancel-btn" on:click={handleBack} disabled={loading}>
          Back
        </button>
        <button 
          class="modal-primary-btn" 
          on:click={handleImport} 
          disabled={!outputPath || loading}
        >
          {loading ? "Importing..." : "Import"}
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .form-group {
    margin-bottom: 1rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-color);
  }

  .path-row {
    display: flex;
    gap: 0.5rem;
  }

  .path-row .modal-input {
    flex: 1;
  }

  .browse-btn {
    padding: 0.5rem 0.75rem;
    background-color: var(--hover-bg);
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
    color: var(--text-color);
  }

  .browse-btn:hover:not(:disabled) {
    filter: brightness(0.95);
  }

  .browse-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .success-message {
    padding: 0.75rem;
    background-color: rgba(34, 197, 94, 0.1);
    color: #22c55e;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .error-message {
    padding: 0.75rem;
    background-color: rgba(239, 68, 68, 0.1);
    color: var(--danger-color);
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }
</style>
