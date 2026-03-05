#!/bin/bash
# Resolve all analysis-only rebase conflicts by taking --ours (main's version).
# Usage: resolve-analysis-rebase.sh [worktree-dir]
set -e
cd "${1:-.}"
while true; do
    conflicts=$(git diff --name-only --diff-filter=U 2>/dev/null)
    if [ -z "$conflicts" ]; then
        echo "No conflicts remain. Rebase complete or not in progress."
        break
    fi
    # Verify ALL conflicts are in analyses/
    non_analysis=$(echo "$conflicts" | grep -v '/analyses/' || true)
    if [ -n "$non_analysis" ]; then
        echo "ERROR: Non-analysis conflicts found:"
        echo "$non_analysis"
        exit 1
    fi
    echo "Resolving $(echo "$conflicts" | wc -l) analysis conflicts with --ours..."
    echo "$conflicts" | while read f; do
        git checkout --ours "$f" && git add "$f"
    done
    GIT_EDITOR=true git rebase --continue 2>&1 || true
done
