<script>
  import {
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    DialogActions,
  } from "../../../components/dialog";

  export let path;
  export let name;
  export let onUnlock;
  export let onCancel;

  let password = "";
  let error = "";
  let isUnlocking = false;

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

<Dialog on:keydown={handleKeydown}>
  <DialogHeader on:close={onCancel}>Unlock {name}</DialogHeader>
  <DialogBody>
    <p class="vault-path">File: {path}</p>
    <input
      class="modal-input"
      bind:value={password}
      type="password"
      placeholder="Vault password"
      disabled={isUnlocking}
      use:focus
      on:keydown={(e) => e.key === "Enter" && handleUnlock()}
    />
    {#if error}
      <p class="modal-error">{error}</p>
    {/if}
  </DialogBody>
  <DialogFooter>
    <DialogActions>
      <button
        class="modal-cancel-btn"
        on:click={onCancel}
        disabled={isUnlocking}
      >
        Cancel
      </button>
      <button
        class="btn-primary"
        on:click={handleUnlock}
        disabled={isUnlocking}
      >
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
