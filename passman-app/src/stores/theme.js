import { writable } from "svelte/store";

const STORE_KEY = "passman.theme";

function loadTheme() {
  if (typeof window === "undefined") return null;
  try {
    const saved = localStorage.getItem(STORE_KEY);
    if (saved === "light" || saved === "dark" || saved === "auto") {
      return saved;
    }
  } catch (e) {
    console.error("Failed to load theme from localStorage:", e);
  }
  return null;
}

function saveTheme(value) {
  if (typeof window === "undefined") return;
  try {
    localStorage.setItem(STORE_KEY, value);
  } catch (e) {
    console.error("Failed to save theme to localStorage:", e);
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

const initialTheme = loadTheme() || "auto";
export const theme = writable(initialTheme);

// Apply and persist theme changes
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
}
