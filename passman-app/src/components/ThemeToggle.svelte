<script>
  import { writable } from "svelte/store";
  import { onMount, onDestroy } from "svelte";

  const STORE_KEY = "passman.theme";

  function loadTheme() {
    try {
      const saved = localStorage.getItem(STORE_KEY);
      if (saved === "light" || saved === "dark" || saved === "auto") {
        return saved;
      }
    } catch (e) {
      console.error("Failed to load theme from localStorage:", e);
    }
    return "auto";
  }

  function saveTheme(value) {
    try {
      localStorage.setItem(STORE_KEY, value);
    } catch (e) {
      console.error("Failed to save theme to localStorage:", e);
    }
  }

  function applyTheme(value) {
    const root = document.documentElement;
    if (value === "dark") {
      root.classList.add("dark");
    } else if (value === "light") {
      root.classList.remove("dark");
    } else {
      const prefersDark = window.matchMedia(
        "(prefers-color-scheme: dark)",
      ).matches;
      root.classList.toggle("dark", prefersDark);
    }
  }

  const theme = writable(loadTheme());

  let mediaQuery;
  let mediaHandler;

  onMount(() => {
    theme.subscribe((value) => {
      applyTheme(value);
      saveTheme(value);
    });

    mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaHandler = () => {
      let currentTheme = "auto";
      const unsubscribe = theme.subscribe((value) => {
        currentTheme = value;
        unsubscribe();
      });
      if (currentTheme === "auto") applyTheme("auto");
    };
    mediaQuery.addEventListener("change", mediaHandler);
  });

  onDestroy(() => {
    if (mediaQuery && mediaHandler) {
      mediaQuery.removeEventListener("change", mediaHandler);
    }
  });

  function cycleTheme() {
    const themes = ["light", "dark", "auto"];
    const currentIndex = themes.indexOf($theme);
    const nextIndex = (currentIndex + 1) % themes.length;
    theme.set(themes[nextIndex]);
  }

  function getThemeIcon() {
    const isDark = document.documentElement.classList.contains("dark");
    const strokeColor = isDark ? "#e2e2e9" : "#191c20";

    if ($theme === "light") {
      return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="${strokeColor}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/></svg>`;
    } else if ($theme === "dark") {
      return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="${strokeColor}" stroke="${strokeColor}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>`;
    } else {
      return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="${strokeColor}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><defs><clipPath id="moon-clip"><rect x="0" y="0" width="24" height="12"/></clipPath><clipPath id="sun-clip"><rect x="0" y="12" width="24" height="12"/></clipPath></defs><g clip-path="url(#moon-clip)"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" fill="${strokeColor}" stroke="${strokeColor}"/></g><g clip-path="url(#sun-clip)"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/></g></svg>`;
    }
  }
</script>

<button
  class="btn-icon theme-toggle-btn"
  onclick={cycleTheme}
  title={$theme.charAt(0).toUpperCase() + $theme.slice(1)}
>
  {@html getThemeIcon()}
</button>

<style>
  .theme-toggle-btn {
    width: var(--btn-height);
    height: var(--btn-height);
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--hover-bg);
    border: none;
    border-radius: var(--shape-full);
    color: var(--text-color);
    cursor: pointer;
  }

  .theme-toggle-btn:hover {
    background-color: var(--hover-bg);
  }
</style>
