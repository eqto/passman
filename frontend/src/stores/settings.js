import { writable } from "svelte/store";
import { AUTO_LOCK_TIMEOUT_MS } from "../lib/constants.js";

const KEY = "passman.autoLockTimeoutMs";

function load() {
  const raw = localStorage.getItem(KEY);
  const v = raw === null ? NaN : Number(raw);
  return Number.isFinite(v) && v > 0 ? v : AUTO_LOCK_TIMEOUT_MS;
}

export const autoLockTimeoutMs = writable(load());

autoLockTimeoutMs.subscribe((v) => {
  try {
    localStorage.setItem(KEY, String(v));
  } catch (e) {
    console.error("Failed to save auto-lock timeout:", e);
  }
});
