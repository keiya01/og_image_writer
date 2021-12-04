import { defineConfig } from "vite";
import path from "path";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import autoPreprocess from "svelte-preprocess";

const IS_PROD = process.env.NODE_ENV === "production";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte({
      preprocess: autoPreprocess(),
    }),
  ],
  resolve: {
    alias: !IS_PROD
      ? {
          og_image_writer: path.resolve("../wasm/pkg"),
        }
      : undefined,
  },
});
