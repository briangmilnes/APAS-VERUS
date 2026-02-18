#!/bin/bash
# Reset the current agent branch to origin/main and force-push.
# Run in an agent worktree AFTER main has merged and pushed.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

CURRENT_BRANCH=$(git branch --show-current)

if [ "$CURRENT_BRANCH" = "main" ]; then
    echo "ERROR: on main. This script is for agent worktrees only."
    exit 1
fi

git fetch origin
git reset --hard origin/main
git push origin "$CURRENT_BRANCH" --force
