#!/bin/sh

wasm-pack build --release --target web
sed -i "" -e 's/"name": "wasm"/"name": "og_image_writer"/' pkg/package.json
wasm-strip pkg/wasm_bg.wasm
