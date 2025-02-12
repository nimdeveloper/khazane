import tailwindcss from "@tailwindcss/vite";
import { defineNuxtConfig } from "nuxt/config";
import * as path from "path";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2024-11-01",
    // (optional) Enable the Nuxt devtools
    devtools: {
        enabled: true,

        timeline: {
            enabled: true,
        },
    },
    // Enable SSG
    ssr: false,
    // Enables the development server to be discoverable by other devices when running on iOS physical devices
    devServer: { host: process.env.TAURI_DEV_HOST || "localhost" },
    css: ["~/assets/css/main.css"],
    vite: {
        css: {
            preprocessorOptions: {
                scss: {
                    api: "modern-compiler",
                },
            },
        },
        // Better support for Tauri CLI output
        clearScreen: false,
        // Enable environment variables
        // Additional environment variables can be found at
        // https://v2.tauri.app/reference/environment-variables/
        envPrefix: ["VITE_", "TAURI_"],
        server: {
            // Tauri requires a consistent port
            strictPort: true,
        },
        plugins: [tailwindcss()],
    },
    rootDir: "src",
    typescript: {
        typeCheck: true,
    },
    modules: ["@vueuse/nuxt", "@pinia/nuxt"],
    pinia: {
        storesDirs: ["./src/stores/**"],
    },
});
