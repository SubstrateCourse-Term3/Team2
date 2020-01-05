#!/bin/bash
set -x
target/debug/substrate-kitties purge-chain --dev -y -d target/substrate

set -e
BUILD_DUMMY_WASM_BINARY=true cargo check
WASM_BUILD_TYPE=release cargo run -- --dev -d target/substrate --execution=Native
