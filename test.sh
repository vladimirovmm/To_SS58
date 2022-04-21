#!/bin/bash
#cargo fmt
#
#cargo build
#cargo clippy --no-deps
#cargo test -- --nocapture

cargo build --lib --target wasm32-unknown-unknown --no-default-features
cargo clippy --target wasm32-unknown-unknown --no-deps -- --allow=clippy::unused_unit
wasm-pack test --firefox --headless