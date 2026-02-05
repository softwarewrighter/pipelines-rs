#!/bin/bash
set -euo pipefail

# Demo: literal-header-footer (record-at-a-time)
cd "$(dirname "$0")/.."

cargo run -p naive-pipe --bin pipe-run-rat --release -- \
    ../specs/literal-header-footer.pipe \
    ../specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/literal-header-footer.out 2>&1
