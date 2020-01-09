#!/bin/bash

# http://telemetry.polkadot.io
set -x
target/debug/substrate-kitties purge-chain --base-path=/tmp/alice --chain=local -y
target/debug/substrate-kitties \
  --base-path=/tmp/alice \
  --chain=local \
  --alice \
  --node-key=0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url=ws://telemetry.polkadot.io:1024 \
  --validator \
  --rpc-external

