#!/bin/bash
set -euo pipefail

# Demo: duplicate-triple
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/duplicate-triple.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/duplicate-triple.out 2>&1
