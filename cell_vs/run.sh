#!/bin/bash

export RUSTFLAGS="-Copt-level=$OPT"

CARGO_TARGET_DIR=target/cell cargo build --target wasm32-unknown-unknown --release --features cell
CARGO_TARGET_DIR=target/unsafe_cell cargo build --target wasm32-unknown-unknown --release --features unsafe_cell
CARGO_TARGET_DIR=target/ref_cell cargo build --target wasm32-unknown-unknown --release --features ref_cell

node run.js
