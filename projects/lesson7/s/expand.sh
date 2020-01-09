#!/bin/bash

set -x
export BUILD_DUMMY_WASM_BINARY=1
cargo expand -p substrate-kitties > expanded-substrate-kitties.rs
cargo expand -p substrate-kitties-runtime > expanded-runtime.rs


