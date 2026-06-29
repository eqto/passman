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

if (typeof window !== "undefined") {
  const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
  mediaQuery.addEventListener("change", () => applyTheme("system"));
}
