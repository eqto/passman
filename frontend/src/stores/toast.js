import { writable } from "svelte/store";

export const toast = writable({ message: "", visible: false });

let timeout = null;

export function showToast(message, duration = 3000) {
  if (timeout) clearTimeout(timeout);
  toast.set({ message, visible: true });
  timeout = setTimeout(() => {
    toast.set({ message: "", visible: false });
  }, duration);
}
