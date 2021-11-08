import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import copy from "rollup-plugin-copy";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    copy({
      targets: [
        {
          src: "node_modules/og_image_writer/wasm_bg.wasm",
          dest: "dist",
        },
      ],
      verbose: true,
      hook: "writeBundle",
    }),
  ],
});
