<script>
  import { onDestroy } from "svelte";
  import { lockVault, isUnlocked } from "../stores/vaults";
  import { AUTO_LOCK_TIMEOUT_MS } from "../lib/constants.js";

  const LOCK_TIMEOUT_MS = AUTO_LOCK_TIMEOUT_MS;
  let timer = null;

  function resetTimer() {
    if (timer) clearTimeout(timer);
    if ($isUnlocked) {
      timer = setTimeout(() => {
        lockVault();
      }, LOCK_TIMEOUT_MS);
    }
  }

  function handleActivity() {
    resetTimer();
  }

  $: if ($isUnlocked) {
    resetTimer();
  } else {
    if (timer) clearTimeout(timer);
  }

  onDestroy(() => {
    if (timer) clearTimeout(timer);
  });
</script>

<svelte:window on:mousemove={handleActivity} on:keydown={handleActivity} on:click={handleActivity} />
