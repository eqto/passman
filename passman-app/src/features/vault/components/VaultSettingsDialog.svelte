<script>
  import { renameVault, changeSecurityLevel } from "../store.js";
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";
  import SecurityLevelSlider from "./SecurityLevelSlider.svelte";

  let { vault = null, onrenamed = null, oncancel = null } = $props();

  let settingsName = $state(vault ? vault.name : "");
  let securityLevel = $state("medium");
  let securityPassword = $state("");
  let showSecurityChange = $state(false);
  let securityError = $state("");
  let securityLoading = $state(false);

  async function handleRename() {
    if (!vault || !settingsName.trim()) return;
    await renameVault(vault.id, settingsName.trim());
    onrenamed?.();
  }

  async function handleChangeSecurity() {
    if (!vault || !securityPassword || !securityLevel) return;
    securityLoading = true;
    securityError = "";
    try {
      await changeSecurityLevel(vault.path, securityPassword, securityLevel);
      showSecurityChange = false;
      securityPassword = "";
      securityError = "";
    } catch (e) {
      securityError = e.message || e.toString();
    } finally {
      securityLoading = false;
    }
  }

  function handleCancelSecurity() {
    showSecurityChange = false;
    securityPassword = "";
    securityError = "";
  }

  function handleCancel() {
    oncancel?.();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") handleCancel();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={handleCancel}>Vault Settings</DialogHeader>
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

      <div class="form-group security-section">
        <label>Security Level</label>
        {#if showSecurityChange}
          <SecurityLevelSlider
            bind:value={securityLevel}
            disabled={securityLoading}
          />
          <input
            class="modal-input"
            bind:value={securityPassword}
            type="password"
            placeholder="Enter vault password to confirm"
            disabled={securityLoading}
          />
          {#if securityError}
            <p class="modal-error">{securityError}</p>
          {/if}
          <div class="security-actions">
            <button
              class="modal-cancel-btn"
              onclick={handleCancelSecurity}
              disabled={securityLoading}
            >
              Cancel
            </button>
            <button
              class="btn-primary"
              onclick={handleChangeSecurity}
              disabled={!securityPassword || securityLoading}
            >
              {securityLoading ? "Applying..." : "Apply"}
            </button>
          </div>
        {:else}
          <button
            class="btn-secondary change-security-btn"
            onclick={() => (showSecurityChange = true)}
          >
            Change Security Level
          </button>
        {/if}
      </div>
    </div>
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button class="modal-cancel-btn" onclick={handleCancel}> Cancel </button>
      <button class="btn-primary" onclick={handleRename}> Save </button>
    </DialogActions>
  </DialogFooter>
</Dialog>

<style>
  .security-section {
    margin-top: var(--space-3, 0.75rem);
  }

  .security-section label {
    display: block;
    font-size: var(--font-size-sm, 0.875rem);
    color: var(--muted-color);
    margin-bottom: var(--space-2, 0.5rem);
  }

  .change-security-btn {
    width: 100%;
  }

  .security-actions {
    display: flex;
    gap: var(--space-2, 0.5rem);
    margin-top: var(--space-2, 0.5rem);
  }

  .security-actions button {
    flex: 1;
  }

  .modal-error {
    color: var(--danger-color);
    font-size: var(--font-size-sm, 0.875rem);
    margin: var(--space-1, 0.25rem) 0;
  }
</style>
