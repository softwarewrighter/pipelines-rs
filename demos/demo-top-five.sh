#!/bin/bash
set -euo pipefail

# Demo: top-five
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/top-five.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/top-five.out 2>&1
