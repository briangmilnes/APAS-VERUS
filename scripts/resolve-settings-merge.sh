#!/bin/bash
# Resolve .claude/settings.local.json merge/rebase conflicts by unioning the allow lists.
# Extracts all unique permission strings from both sides and writes a merged file.
# Usage: resolve-settings-merge.sh [worktree-dir]
set -e
cd "${1:-.}"

SETTINGS=".claude/settings.local.json"

if ! git diff --name-only --diff-filter=U 2>/dev/null | grep -q "$SETTINGS"; then
    echo "No conflict in $SETTINGS"
    exit 0
fi

echo "Resolving $SETTINGS by unioning allow lists..."

# Extract all quoted permission strings from both sides of the conflict,
# strip conflict markers, deduplicate, and sort.
permissions=$(grep -E '^\s*"(Bash|Read|Write|Edit|Glob|Grep|WebFetch)\(' "$SETTINGS" \
    | grep -v '^[<>=]' \
    | sed 's/^[[:space:]]*//' \
    | sed 's/,$//' \
    | sort -u)

# Build the JSON
{
    echo '{'
    echo '  "permissions": {'
    echo '    "allow": ['
    # Output each permission, adding comma to all but last
    echo "$permissions" | awk '{lines[NR] = $0} END {for (i=1; i<NR; i++) print "      " lines[i] ","; print "      " lines[NR]}'
    echo '    ]'
    echo '  }'
    echo '}'
} > "$SETTINGS"

git add "$SETTINGS"
echo "Resolved $SETTINGS with $(echo "$permissions" | wc -l) unique permissions."
