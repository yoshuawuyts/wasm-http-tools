#!/usr/bin/env bash

set pipefail -euo

cargo build --target wasm32-wasi --example main
wasm-tools component new target/wasm32-wasi/debug/examples/main.wasm \
    --adapt resources/wasi_snapshot_preview1.command.wasm \
    -o target/main.wasm
wasmtime run -S http target/main.wasm
