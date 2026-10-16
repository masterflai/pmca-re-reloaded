import { defineConfig } from "vite";

/**
 * Front-end build and dev-server configuration.
 *
 * The only thing that has to be pinned here is the dev-server port. Tauri
 * loads the front-end from `devUrl` in `src-tauri/tauri.conf.json`, which is a
 * fixed `http://localhost:1420`; a dev server on any other port leaves the
 * application window blank. `strictPort` makes a busy port an error instead of
 * a silent move to the next free one, which would reproduce exactly that
 * failure while looking like a successful start.
 */
export default defineConfig({
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // `src-tauri` is Cargo's territory. Watching it makes the dev server
      // follow `target/`, where a freshly linked, still-open `.dll` aborts the
      // watcher with `EBUSY` and takes the whole `tauri dev` run down with it.
      ignored: ["**/src-tauri/**"],
    },
  },
});
