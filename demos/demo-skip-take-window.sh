#!/bin/bash
set -euo pipefail

# Demo: skip-take-window
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/skip-take-window.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/skip-take-window.out 2>&1
