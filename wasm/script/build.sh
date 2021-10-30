#!/bin/sh

wasm-pack build
sed -i "" -e 's/"name": "wasm"/"name": "og_image_writer"/' pkg/package.json
