#!/usr/bin/env bash
set -euo pipefail

# Generate Emacs TAGS covering src/ plus the vstd and builtin sources.
# The prebuilt verus release has no source tree, so vstd/builtin come from
# cargo's git checkout of the verus repo (fetched by `cargo fetch`).

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
TAGS=~/projects/verus-etags/target/release/verus-etags
TAGS_FILE="${ROOT_DIR}/TAGS"

if ! command -v ctags >/dev/null 2>&1; then
  echo "Error: ctags not found. Install universal-ctags (e.g., sudo apt install universal-ctags)." >&2
  exit 1
fi

VERUS_SRC=$(ls -d ~/.cargo/git/checkouts/verus-*/*/source 2>/dev/null | head -1 || true)
if [ -z "$VERUS_SRC" ]; then
  echo "Warning: no verus git checkout under ~/.cargo/git/checkouts; run 'cargo fetch' first. Tagging src/ only." >&2
  VSTD_DIRS=""
else
  VSTD_DIRS="$VERUS_SRC/builtin $VERUS_SRC/vstd"
fi

# Find all .rs files excluding attic directories
FILES=$(find ${ROOT_DIR}/src $VSTD_DIRS -name '*.rs' -not -path '*/attic/*')

$TAGS $FILES
echo "Wrote tags: ${TAGS_FILE}"
