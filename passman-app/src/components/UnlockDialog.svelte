<script>
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
</script>

<div class="modal-overlay">
  <div class="modal">
    <h2>Unlock Vault</h2>
    <p class="vault-name">{name}</p>
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
    <div class="modal-actions">
      <button class="modal-cancel-btn" on:click={onCancel} disabled={isUnlocking}>
        Cancel
      </button>
      <button class="modal-primary-btn" on:click={handleUnlock} disabled={isUnlocking}>
        Unlock
      </button>
    </div>
    {#if isUnlocking}
      <div class="progress-wrapper">
        <div class="progress-bar">
          <div class="progress-indeterminate"></div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .vault-name {
    margin: 0 0 1rem;
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
    border-radius: 0.375rem;
    overflow: hidden;
  }

  .progress-indeterminate {
    width: 40%;
    height: 100%;
    background: linear-gradient(90deg, var(--accent-color), var(--accent-hover));
    border-radius: 0.375rem;
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
