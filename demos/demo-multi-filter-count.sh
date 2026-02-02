#!/bin/bash
set -euo pipefail

# Demo: multi-filter-count
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/multi-filter-count.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/multi-filter-count.out 2>&1
