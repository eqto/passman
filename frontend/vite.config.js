import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wails from "@wailsio/runtime/plugins/vite";

export default defineConfig(async () => ({
  plugins: [svelte(), wails("./bindings")],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "WAILS_"],
  build: {
    target: "es2021",
    minify: "oxc",
    sourcemap: true,
    outDir: "dist",
  },
}));
