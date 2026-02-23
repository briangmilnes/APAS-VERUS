#!/bin/bash
# Regenerate path/ from src. Run from fixture root (tests/fixtures/APAS-VERUS).
#
# Strategy: copy src/X_path.rs -> path/X.rs for round-trip outputs.
# For modules without _path (e.g. ArraySeqMtEph, BFS*), copy src/X.rs -> path/X.rs.
#
# Usage: scripts/regenerate-path.sh

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$PROJECT_ROOT"
rm -rf path
mkdir -p path

# 1. Copy all _path.rs -> path/X.rs
find src -name '*_path.rs' | while read -r f; do
  base="${f#src/}"
  base="${base%_path.rs}"
  mkdir -p "path/$(dirname "$base")"
  cp "$f" "path/${base}.rs"
done

# 2. Copy modules referenced in lib.rs that don't have _path
for m in Types Concurrency ParaPairs; do
  [ -f "src/${m}.rs" ] && cp "src/${m}.rs" "path/${m}.rs"
done
# vstdplus: non-_path files (e.g. arithmetic subdir) - only if not already in path
find src/vstdplus -name '*.rs' ! -name '*_path*' | while read -r f; do
  base="${f#src/}"
  dst="path/$base"
  [ -f "$dst" ] && continue
  mkdir -p "path/$(dirname "$base")"
  cp "$f" "$dst"
done
# Chap modules without _path (lib.rs references but no paths-write output)
for base in Chap18/ArraySeqMtEph Chap19/ArraySeqMtEph Chap54/BFSStEph Chap54/BFSStPer Chap54/BFSMtEph Chap54/BFSMtPer; do
  [ -f "src/${base}.rs" ] && [ ! -f "path/${base}.rs" ] && { mkdir -p "path/$(dirname "$base")"; cp "src/${base}.rs" "path/${base}.rs"; }
done

# 3. Copy lib.rs
cp src/lib.rs path/lib.rs

echo "Regenerated path/ ($(find path -name '*.rs' | wc -l) files)"
