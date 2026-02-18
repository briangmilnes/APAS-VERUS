#!/bin/bash
# cargo check --lib

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"
cargo check --lib 2>&1 | sed 's/\x1b\[[0-9;]*m//g'
