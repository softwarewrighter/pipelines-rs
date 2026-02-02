#!/bin/bash
set -euo pipefail

# Demo: engineers-only
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/engineers-only.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/engineers-only.out 2>&1
