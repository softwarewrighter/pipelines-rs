#!/bin/bash
set -euo pipefail

# Demo: reverse-text
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/reverse-text.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/reverse-text.out 2>&1
