#!/bin/bash
set -euo pipefail

# Demo: count-records
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/count-records.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/count-records.out 2>&1
