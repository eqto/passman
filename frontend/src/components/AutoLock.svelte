<script>
  import { onDestroy } from "svelte";
  import { lockVault, isUnlocked } from "../features/vault/index.js";
  import { autoLockTimeoutMs } from "../stores/settings.js";

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
      }, $autoLockTimeoutMs);
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
