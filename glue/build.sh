#!/bin/bash

set -e

cd "$(dirname "$0")"

cargo build --target wasm32-unknown-unknown --profile release
wasm-bindgen --out-dir ../web/pkg --typescript --target web --reference-types \
    "../target/wasm32-unknown-unknown/release/glue.wasm"
