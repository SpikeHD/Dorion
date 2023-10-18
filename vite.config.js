import { defineConfig } from "vite";
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  root: 'src',
  // Vite optons tailored for Tauri development and only applied in `tauri dev` or `tauri build`
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
    target: ["es2021", "chrome100", "safari13"],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'src/index.html'),
        bar: resolve(__dirname, 'src/top.html'),
        extra: resolve(__dirname, 'src/extra.css'),
      },
      output: {
        dir: 'src-tauri/html',
        assetFileNames: '[name].[ext]'
      }
    },
    outDir: 'src-tauri/html'
  },
});
