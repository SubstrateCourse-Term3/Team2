#!/bin/bash

set -x
export SKIP_WASM_BUILD=true
cargo test -p substrate-kitties -- "$1" --nocapture
cargo test -- "$1" --nocapture

