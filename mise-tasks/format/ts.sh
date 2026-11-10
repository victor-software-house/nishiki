#!/usr/bin/env bash
#MISE description="TypeScript formatting check"
#MISE tools = { "aqua:oxc-project/oxc/oxfmt" = "0.72.0" }
# oxfmt reads .oxfmtrc.json, which skips data files such as bench/corpus/ and oracle/inputs/.
set -euo pipefail
oxfmt --check '**/*.ts'
