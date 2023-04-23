import { resolve } from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import {
  createStyleImportPlugin,
  // AndDesignVueResolve,
  // VantResolve,
  // ElementPlusResolve,
  // NutuiResolve,
  AntdResolve,
} from "vite-plugin-style-import";

const mobile =
  process.env.TAURI_PLATFORM === "android" ||
  process.env.TAURI_PLATFORM === "ios";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    createStyleImportPlugin({
      resolves: [
        // AndDesignVueResolve(),
        // VantResolve(),
        // ElementPlusResolve(),
        // NutuiResolve(),
        AntdResolve(),
      ],
      libs: [
        // If you don’t have the resolve you need, you can write it directly in the lib, or you can provide us with PR
        {
          libraryName: "antd",
          esModule: true,
          resolveStyle: (name) => {
            return `antd/es/${name}/style/index`;
          },
        },
      ],
    }),
  ],
  css: {
    preprocessorOptions: {
      less: {
        javascriptEnabled: true,
      },
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    build: {
      rollupOptions: {
        input: {
          main: resolve(__dirname, "about.html"),
          settings: resolve(__dirname, "settings.html"),
          input: resolve(__dirname, "input_window.html"),
        },
      },
    },
  },
}));