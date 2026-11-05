#!/usr/bin/env bash
#MISE description="Each stream-benchmark trace in bench/traces matches its source"
#MISE tools={bun="1.4.4"}
# A trace means something only if it replays the bytes its source holds
# (bench/traces/README.md).
set -euo pipefail
bun test bench/traces.test.ts
bun bench/traces.ts check
