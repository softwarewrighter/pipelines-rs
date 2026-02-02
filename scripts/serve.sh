#!/bin/bash
set -euo pipefail

# Serve the WASM UI on port 9952 (from docs/ for GitHub Pages parity)

cd "$(dirname "$0")/../docs"

echo "Serving pipelines-rs UI at http://localhost:9952"
echo "Press Ctrl+C to stop"
basic-http-server -a 0.0.0.0:9952
