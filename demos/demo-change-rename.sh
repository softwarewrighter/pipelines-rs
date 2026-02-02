#!/bin/bash
set -euo pipefail

# Demo: change-rename
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/change-rename.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/change-rename.out 2>&1
