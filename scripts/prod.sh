#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
trunk build --release --public-url /assets/
popd

pushd backend
cargo run --release -- --port 8080
popd
