<script>
  import { onDestroy } from "svelte";
  import { lockVault, isUnlocked } from "../features/vault/store.js";
  import { AUTO_LOCK_TIMEOUT_MS } from "../lib/constants.js";

  const LOCK_TIMEOUT_MS = AUTO_LOCK_TIMEOUT_MS;
  let timer = null;

  $effect(() => {
    if ($isUnlocked) {
      resetTimer();
    } else {
      if (timer) clearTimeout(timer);
    }
  });

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

  onDestroy(() => {
    if (timer) clearTimeout(timer);
  });
</script>

<svelte:window
  onmousemove={handleActivity}
  onkeydown={handleActivity}
  onclick={handleActivity}
/>
