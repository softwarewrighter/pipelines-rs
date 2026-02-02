#!/bin/bash
set -euo pipefail

# Demo: change-strip-prefix
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/change-strip-prefix.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/change-strip-prefix.out 2>&1
