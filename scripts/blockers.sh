#!/bin/bash
# blockers.sh — Generate plans/blockers.md for every src/ChapNN/ directory.
# Each file lists which downstream chapters import from this chapter.
#
# Usage: scripts/blockers.sh

set -euo pipefail
cd "$(dirname "$0")/../src"

# Build the inverse dependency map: for each chapter, find who imports it.
declare -A BLOCKS

for dir in Chap*/; do
    chap="${dir%/}"
    # Find all chapters that import from this one.
    downstream=$(grep -rl "use crate::${chap}::" Chap*/*.rs 2>/dev/null \
        | sed 's|/.*||' | sort -u | grep -v "^${chap}$" || true)
    BLOCKS["$chap"]="$downstream"
done

# Write blockers.md for each chapter.
for dir in Chap*/; do
    chap="${dir%/}"
    mkdir -p "${chap}/plans"
    file="${chap}/plans/blockers.md"
    downstream="${BLOCKS[$chap]:-}"

    if [ -z "$downstream" ]; then
        cat > "$file" << EOF
# ${chap} Blockers

No downstream chapters import from ${chap}.
EOF
    else
        {
            echo "# ${chap} Blockers"
            echo ""
            echo "Proving ${chap} blocks these downstream chapters:"
            echo ""
            for dep in $downstream; do
                echo "- ${dep}"
            done
        } > "$file"
    fi
done

echo "blockers.md written for $(ls -d Chap*/plans/blockers.md | wc -l) chapters."
