#!/bin/bash

set -x
target/debug/substrate-kitties purge-chain --dev -y -d target/substrate
target/debug/substrate-kitties --dev -d target/substrate --execution=Native
