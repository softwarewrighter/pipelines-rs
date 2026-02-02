#!/bin/bash
set -euo pipefail

# Demo: literal-footer
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/literal-footer.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/literal-footer.out 2>&1
