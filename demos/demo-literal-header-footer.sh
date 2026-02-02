#!/bin/bash
set -euo pipefail

# Demo: literal-header-footer
cd "$(dirname "$0")/.."

cargo run --bin pipe-run --release -- \
    specs/literal-header-footer.pipe \
    specs/input-fixed-80.data \
    -o work/sample-pipe-outputs/literal-header-footer.out 2>&1
