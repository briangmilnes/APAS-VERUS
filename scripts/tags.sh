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

# Find all .rs files excluding attic directories
FILES=$(find ${ROOT_DIR}/src ~/projects/verus/source/builtin ~/projects/verus/source/vstd \
    -name '*.rs' -not -path '*/attic/*')

$TAGS $FILES
echo "Wrote tags: ${TAGS_FILE}"


