#!/bin/bash
set -euo pipefail

# Demo: locate-errors
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/locate-errors.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/locate-errors.out 2>&1
