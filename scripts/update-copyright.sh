#!/bin/bash
# Update copyright headers across all APAS-VERUS source files.
# Replaces the old single-line copyright with SPDX + new copyright.
#
# Old:  //[! ] Copyright (C) 2025 Acar, Blelloch and Milnes from '...'
# New:  // SPDX-License-Identifier: MIT
#       // Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

set -e
cd "$(dirname "$0")/.."

NEW1='// SPDX-License-Identifier: MIT'
NEW2='// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes'

DIRS=(src tests rust_verify_test benches)

updated=0
skipped=0

while IFS= read -r -d '' f; do
    if grep -q 'Copyright.*2025.*Acar' "$f"; then
        # Replace the copyright line (//! or // variant) with two new lines.
        # Use awk to do a clean line-by-line replacement without regex backreference issues.
        awk -v new1="$NEW1" -v new2="$NEW2" '
            /^\/\/[! ] Copyright.*2025.*Acar/ {
                print new1
                print new2
                next
            }
            { print }
        ' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
        updated=$((updated + 1))
    else
        skipped=$((skipped + 1))
    fi
done < <(find "${DIRS[@]}" -name "*.rs" -print0 2>/dev/null)

echo "Updated: $updated files"
echo "Skipped: $skipped files (no old header)"
