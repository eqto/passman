import { writable } from "svelte/store";
import { Store } from "@tauri-apps/plugin-store";

const STORE_KEY = "theme";
const store = new Store(".settings.json");

export const theme = writable("auto");

export async function loadTheme() {
  try {
    const savedTheme = await store.get(STORE_KEY);
    if (savedTheme && (savedTheme === "light" || savedTheme === "dark" || savedTheme === "auto")) {
      theme.set(savedTheme);
    }
  } catch (e) {
    console.error("Failed to load theme from store:", e);
  }
}

export async function saveTheme(value) {
  try {
    await store.set(STORE_KEY, value);
    await store.save();
  } catch (e) {
    console.error("Failed to save theme to store:", e);
  }
}

export function applyTheme(value) {
  const root = document.documentElement;
  if (value === "dark") {
    root.classList.add("dark");
  } else if (value === "light") {
    root.classList.remove("dark");
  } else {
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    root.classList.toggle("dark", prefersDark);
  }
}

// Subscribe to theme changes and apply them
theme.subscribe((value) => {
  applyTheme(value);
  saveTheme(value);
});

if (typeof window !== "undefined") {
  const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
  mediaQuery.addEventListener("change", () => {
    // Re-apply theme when system preference changes
    let currentTheme = "auto";
    const unsubscribe = theme.subscribe((value) => {
      currentTheme = value;
      unsubscribe();
    });
    applyTheme(currentTheme);
  });

  // Load theme on startup
  loadTheme();
}
