#!/bin/bash
# Regenerate path/ from src. Run from fixture root (tests/fixtures/APAS-VERUS).
#
# Strategy:
# 1. paths-write: for each .vp, output directly to path/X.rs (no _path in src).
# 2. Copy modules without .vp (vstdplus non-vp, Chap fallbacks).
# 3. Copy lib.rs.
#
# Usage: scripts/regenerate-path.sh
# Requires: veracity-paths-read has been run (src/analyses/**/*.vp exist).

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERACITY="${VERACITY:-$PROJECT_ROOT/../veracity/target/release}"

cd "$PROJECT_ROOT"
rm -rf path
mkdir -p path

# 1. paths-write: each .vp -> path/X.rs (sibling of src, no _path clutter)
find src/analyses -name '*.vp' 2>/dev/null | while read -r vp; do
  base="${vp#src/analyses/}"
  base="${base%.vp}"
  src="src/${base}.rs"
  out="path/${base}.rs"
  if [ -f "$src" ]; then
    mkdir -p "path/$(dirname "$base")"
    "$VERACITY/veracity-paths-write" -v "$vp" -s "$src" -o "$out"
  fi
done

# 2. Copy modules without .vp (paths-read skips or no verus! block)
# vstdplus: non-.vp files
find src/vstdplus -name '*.rs' 2>/dev/null | while read -r f; do
  base="${f#src/}"
  dst="path/$base"
  [ -f "$dst" ] && continue
  mkdir -p "path/$(dirname "$base")"
  cp "$f" "$dst"
done
# Chap modules that may lack .vp
for base in Chap18/ArraySeqMtEph Chap19/ArraySeqMtEph Chap54/BFSStEph Chap54/BFSStPer Chap54/BFSMtEph Chap54/BFSMtPer; do
  [ -f "src/${base}.rs" ] && [ ! -f "path/${base}.rs" ] && { mkdir -p "path/$(dirname "$base")"; cp "src/${base}.rs" "path/${base}.rs"; }
done

# 3. Copy lib.rs
cp src/lib.rs path/lib.rs

echo "Regenerated path/ ($(find path -name '*.rs' 2>/dev/null | wc -l) files)"
