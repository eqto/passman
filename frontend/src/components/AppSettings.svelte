<script>
  import { Icon } from "./icons";
  import { Dialog, DialogHeader, DialogBody } from "./dialog";
  import { autoLockTimeoutMs } from "../stores/settings.js";

  let show = $state(false);

  const OPTIONS = [
    { label: "5 minutes", value: 5 * 60 * 1000 },
    { label: "15 minutes", value: 15 * 60 * 1000 },
    { label: "30 minutes", value: 30 * 60 * 1000 },
    { label: "1 hour", value: 60 * 60 * 1000 },
    { label: "6 hours", value: 6 * 60 * 60 * 1000 },
    { label: "1 day", value: 24 * 60 * 60 * 1000 },
  ];

  function handleKeydown(e) {
    if (e.key === "Escape") show = false;
  }
</script>

<button class="btn-copy-solid" onclick={() => (show = true)} title="Settings">
  <Icon name="settings" />
</button>

{#if show}
  <Dialog onkeydown={handleKeydown}>
    <DialogHeader onclick={() => (show = false)}>Settings</DialogHeader>
    <DialogBody>
      <div class="modal-form">
        <label for="autolock-timeout">Auto-lock vault after inactivity</label>
        <select
          id="autolock-timeout"
          class="modal-input"
          bind:value={$autoLockTimeoutMs}
        >
          {#each OPTIONS as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </DialogBody>
  </Dialog>
{/if}
