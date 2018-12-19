#!/usr/bin/env bash

sudo apt-get update && apt-get install -y curl gcc clang
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

cd dev/back/
cargo build
cargo run