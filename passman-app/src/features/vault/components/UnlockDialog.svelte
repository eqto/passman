<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  let { path, name, onUnlock, onCancel } = $props();

  let password = $state("");
  let error = $state("");
  let isUnlocking = $state(false);

  async function handleUnlock() {
    if (isUnlocking) return;
    error = "";
    isUnlocking = true;
    try {
      await onUnlock(path, password);
      password = "";
    } catch (e) {
      error = e.toString();
    } finally {
      isUnlocking = false;
    }
  }

  function focus(node) {
    node.focus();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") onCancel();
  }
</script>

<Dialog onkeydown={handleKeydown}>
  <DialogHeader onclick={onCancel}>Unlock {name}</DialogHeader>
  <DialogBody>
    <p class="vault-path">File: {path}</p>
    <input
      class="modal-input"
      bind:value={password}
      type="password"
      placeholder="Vault password"
      disabled={isUnlocking}
      use:focus
      onkeydown={(e) => e.key === "Enter" && handleUnlock()}
    />
    {#if error}
      <p class="modal-error">{error}</p>
    {/if}
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button
        class="modal-cancel-btn"
        onclick={onCancel}
        disabled={isUnlocking}
      >
        Cancel
      </button>
      <button class="btn-primary" onclick={handleUnlock} disabled={isUnlocking}>
        Unlock
      </button>
    </DialogActions>
  </DialogFooter>
  {#if isUnlocking}
    <div class="progress-wrapper">
      <div class="progress-bar">
        <div class="progress-indeterminate"></div>
      </div>
    </div>
  {/if}
</Dialog>

<style>
  .vault-path {
    margin: 0;
    font-size: 0.875rem;
    color: var(--muted-color);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .progress-wrapper {
    margin: 0;
    padding-top: 1rem;
  }

  .progress-bar {
    width: 100%;
    height: 0.25rem;
    background-color: var(--input-border);
    border-radius: var(--shape-full);
    overflow: hidden;
  }

  .progress-indeterminate {
    width: 40%;
    height: 100%;
    background-color: var(--accent-color);
    border-radius: var(--shape-full);
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
