#!/usr/bin/env bash

sudo apt-get update && apt-get install -y curl gcc clang cmake pkg-config libssl-dev git clang libclang-dev
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable