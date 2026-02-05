#!/bin/bash
set -euo pipefail

# Demo: literal-footer (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/literal-footer.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/literal-footer.out 2>&1
