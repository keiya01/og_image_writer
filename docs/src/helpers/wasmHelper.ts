import init from "og_image_writer";

const initOnlyOnceCarry = () => {
  let isInitialized = false;
  return async () => {
    if (isInitialized) {
      return;
    }
    const wasmPath =
      process.env.NODE_ENV === "production"
        ? "/wasm_bg.wasm"
        : // see: https://vitejs.dev/guide/features.html#npm-dependency-resolving-and-pre-bundling
          "/node_modules/og_image_writer/wasm_bg.wasm";
    await init(wasmPath);
    isInitialized = true;
  };
};
export const initOnlyOnce = initOnlyOnceCarry();
