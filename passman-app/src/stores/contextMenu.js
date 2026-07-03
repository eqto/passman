export function closeAllContextMenus() {
  window.dispatchEvent(new CustomEvent('close-all-context-menus'));
}
