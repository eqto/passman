import { onMount } from "svelte";
import { closeAllContextMenus } from "../stores/contextMenu.js";

export function useContextMenu(closeFn) {
  onMount(() => {
    window.addEventListener("close-all-context-menus", closeFn);
    return () => {
      window.removeEventListener("close-all-context-menus", closeFn);
    };
  });
}

export function createContextMenuState(defaults = {}) {
  const base = { show: false, x: 0, y: 0, ...defaults };
  let state = $state({ ...base });

  useContextMenu(() => { Object.assign(state, base); });

  function open(event, extra = {}) {
    event.preventDefault();
    closeAllContextMenus();
    Object.assign(state, { show: true, x: event.clientX, y: event.clientY, ...extra });
  }

  function close() {
    Object.assign(state, base);
  }

  return { state, open, close };
}
