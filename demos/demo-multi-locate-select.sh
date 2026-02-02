#!/bin/bash
set -euo pipefail

# Demo: multi-locate-select
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/multi-locate-select.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/multi-locate-select.out 2>&1
