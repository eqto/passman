import { onMount } from "svelte";

export function useContextMenu(closeFn) {
  onMount(() => {
    window.addEventListener("close-all-context-menus", closeFn);
    return () => {
      window.removeEventListener("close-all-context-menus", closeFn);
    };
  });
}
