#!/bin/bash
set -euo pipefail

# Demo: upper-case
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/upper-case.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/upper-case.out 2>&1
