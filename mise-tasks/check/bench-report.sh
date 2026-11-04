#!/usr/bin/env bash
#MISE description="Every benchmark record's Markdown and the bench README summary match their JSON"
#MISE tools={bun="1.4.4"}
set -euo pipefail
bun bench/report.ts check
