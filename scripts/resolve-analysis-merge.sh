#!/bin/bash
# Resolve all analysis-only merge conflicts by taking --theirs (agent's version).
# Usage: resolve-analysis-merge.sh [worktree-dir]
set -e
cd "${1:-.}"
conflicts=$(git diff --name-only --diff-filter=U 2>/dev/null)
if [ -z "$conflicts" ]; then
    echo "No conflicts found."
    exit 0
fi
# Verify ALL conflicts are in analyses/
non_analysis=$(echo "$conflicts" | grep -v '/analyses/' || true)
if [ -n "$non_analysis" ]; then
    echo "ERROR: Non-analysis conflicts found:"
    echo "$non_analysis"
    exit 1
fi
echo "Resolving $(echo "$conflicts" | wc -l) analysis conflicts with --theirs..."
echo "$conflicts" | while read f; do
    git checkout --theirs "$f" && git add "$f"
done
echo "Done. Run: git add -A && GIT_EDITOR=true git merge --continue"
