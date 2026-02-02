#!/bin/bash
set -euo pipefail

# Demo: multi-transform
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/multi-transform.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/multi-transform.out 2>&1
