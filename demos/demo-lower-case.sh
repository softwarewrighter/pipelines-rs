#!/bin/bash
set -euo pipefail

# Demo: lower-case
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/lower-case.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/lower-case.out 2>&1
