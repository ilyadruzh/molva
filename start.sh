#!/usr/bin/env bash

cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --key Alice \
  --name "ALICE" \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator