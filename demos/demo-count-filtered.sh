#!/bin/bash
set -euo pipefail

# Demo: count-filtered
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/count-filtered.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/count-filtered.out 2>&1
