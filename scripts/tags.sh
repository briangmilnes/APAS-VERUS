#!/usr/bin/env bash
set -euo pipefail

# Generate Emacs TAGS covering both src/ and tests/ using universal-ctags

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
TAGS=~/projects/verus-etags/target/release/verus-etags
TAGS_FILE="${ROOT_DIR}/TAGS"

if ! command -v ctags >/dev/null 2>&1; then
  echo "Error: ctags not found. Install universal-ctags (e.g., sudo apt install universal-ctags)." >&2
  exit 1
fi

$TAGS -R ${ROOT_DIR}/src ~/projects/verus-lang/source/builtin ~/projects/verus-lang/source/vstd
echo "Wrote tags: ${TAGS_FILE}"


