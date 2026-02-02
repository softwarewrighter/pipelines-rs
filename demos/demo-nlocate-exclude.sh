#!/bin/bash
set -euo pipefail

# Demo: nlocate-exclude
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/nlocate-exclude.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/nlocate-exclude.out 2>&1
