#!/usr/bin/env bash

set pipefail -euo

EXAMPLE="pretty"

cargo build --target wasm32-wasi --example $EXAMPLE
wasm-tools component new target/wasm32-wasi/debug/examples/$EXAMPLE.wasm \
    --adapt resources/wasi_snapshot_preview1.command.wasm \
    -o target/$EXAMPLE.wasm
wasmtime run -S http target/$EXAMPLE.wasm
