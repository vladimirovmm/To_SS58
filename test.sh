#!/bin/bash
cargo fmt
#
cargo build
cargo clippy
cargo test -- --nocapture
