#!/bin/bash

export RUSTFLAGS="-Copt-level=$OPT"

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/externref_bench.wasm --out-dir pkg --nodejs

node run.js
