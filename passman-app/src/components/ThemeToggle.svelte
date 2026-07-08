<script>
  import { theme } from "../stores/theme.js";

  function cycleTheme() {
    const themes = ["light", "dark", "auto"];
    const currentIndex = themes.indexOf($theme);
    const nextIndex = (currentIndex + 1) % themes.length;
    theme.set(themes[nextIndex]);
  }

  function getThemeIcon() {
    const isDark = document.documentElement.classList.contains("dark");
    const strokeColor = isDark ? "#ffffff" : "#111827";

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
  on:click={cycleTheme}
  title={$theme.charAt(0).toUpperCase() + $theme.slice(1)}
>
  {@html getThemeIcon()}
</button>

<style>
  .theme-toggle-btn {
    width: 2.25rem;
    height: 2.25rem;
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--hover-bg);
    border: none;
    border-radius: 0.5rem;
    color: var(--text-color);
    cursor: pointer;
  }

  .theme-toggle-btn:hover {
    outline: 1px solid var(--accent-color);
  }
</style>
