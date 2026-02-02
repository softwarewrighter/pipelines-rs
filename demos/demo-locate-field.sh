#!/bin/bash
set -euo pipefail

# Demo: locate-field
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/locate-field.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/locate-field.out 2>&1
