#!/usr/bin/env bash
#MISE description="Format the TypeScript files in place"
#MISE tools = { "aqua:oxc-project/oxc/oxfmt" = "0.72.0" }
set -euo pipefail
oxfmt --write '**/*.ts'
