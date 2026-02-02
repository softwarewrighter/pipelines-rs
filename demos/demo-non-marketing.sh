#!/bin/bash
set -euo pipefail

# Demo: non-marketing
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/non-marketing.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/non-marketing.out 2>&1
