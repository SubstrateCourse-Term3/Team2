#!/bin/bash

target/debug/substrate-kitties purge-chain -y --dev --base-path=/tmp/substrate-kitties
target/debug/substrate-kitties \
  --dev \
  --name=substrate-kitties \
  --base-path=/tmp/substrate-kitties \
  --telemetry-url=ws://telemetry.polkadot.io:1024


