#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(
  trap 'kill 0' SIGINT
  bash -c 'cd frontend; CARGO_TARGET_DIR=../target-trunk trunk serve --address 127.0.0.1 --port 8080'
)
