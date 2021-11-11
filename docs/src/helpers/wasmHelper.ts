import init from "og_image_writer";
import wasm from "og_image_writer/wasm_bg.wasm?url";

const initOnlyOnceCarry = () => {
  let isInitialized = false;
  return async () => {
    if (isInitialized) {
      return;
    }
    await init(wasm);
    isInitialized = true;
  };
};
export const initOnlyOnce = initOnlyOnceCarry();
