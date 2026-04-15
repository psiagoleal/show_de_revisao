// Caminho relativo: vite.config.ts
/// \file vite.config.ts
/// \brief Vite configuration for multi-window Tauri app
/// \author Iago Souza

import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],

    // Force Svelte to use the browser/client runtime instead of server
    resolve: {
        conditions: ["browser"],
    },

    build: {
        // Tauri controla o WebView, então podemos usar esnext com segurança
        // Isso habilita top-level await que é usado em main.ts e display.ts
        target: "esnext",
        rollupOptions: {
            input: {
                main: resolve(__dirname, "index.html"),
                display: resolve(__dirname, "display.html"),
            },
        },
    },

    // Prevent vite from obscuring Rust errors
    clearScreen: false,

    // Tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
});
