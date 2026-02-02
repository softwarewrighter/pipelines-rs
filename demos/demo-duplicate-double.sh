#!/bin/bash
set -euo pipefail

# Demo: duplicate-double
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/duplicate-double.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/duplicate-double.out 2>&1
