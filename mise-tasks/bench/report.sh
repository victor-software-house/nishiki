#!/usr/bin/env bash
#MISE description="Render benchmark records: render, compute, summary or check"
#MISE tools={bun="1.4.4"}
# bench/report.ts says what each command does; `check` runs in check:local.
set -euo pipefail
bun bench/report.ts "$@"
