#!/bin/bash
set -euo pipefail

# Demo: reverse-text (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/reverse-text.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/reverse-text.out 2>&1
