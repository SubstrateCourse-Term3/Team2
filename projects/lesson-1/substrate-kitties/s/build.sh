#!/bin/bash

set -x
target/debug/substrate-kitties purge-chain --dev -y -d target/substrate
WASM_BUILD_TYPE=release cargo run -- --dev -d target/substrate --execution=NativeElseWasm
