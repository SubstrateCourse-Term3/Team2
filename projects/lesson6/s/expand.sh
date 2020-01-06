#!/bin/bash

set -x

cargo expand -p substrate-kitties > expanded-substrate-kitties.rs
cargo expand -p substrate-kitties-runtime > expanded-runtime.rs


