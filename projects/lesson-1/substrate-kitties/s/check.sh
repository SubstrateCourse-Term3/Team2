#!/bin/bash

set -x

export SKIP_WASM_BUILD=true
cargo check

