import { defineConfig } from "vite";

export default defineConfig({
  base: process.env.PAGES_BASE ?? "/bevy-tutorial/",
  root: "site",
  publicDir: false,
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    assetsDir: "assets",
  },
});
