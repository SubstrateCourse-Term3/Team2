#!/bin/bash

# http://telemetry.polkadot.io
set -x
target/debug/substrate-kitties purge-chain --base-path=/tmp/bob --chain=local -y
target/debug/substrate-kitties \
  --base-path=/tmp/bob \
  --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port=30334 \
  --telemetry-url=ws://telemetry.polkadot.io:1024 \
  --validator \
  --rpc-port=9945 \
  --ws-port=9946 \
  --grafana-port=9956 \
  --rpc-external

