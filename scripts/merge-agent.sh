#!/bin/bash
# Merge an agent branch into the current worktree (should be main).
# Usage: merge-agent.sh <branch>
# Example: merge-agent.sh agent1/ready

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

BRANCH="${1:-}"
if [ -z "$BRANCH" ]; then
    echo "Usage: merge-agent.sh <branch>"
    exit 1
fi

cd "$PROJECT_ROOT"

git fetch origin
git merge "$BRANCH" --no-edit || {
    echo "CONFLICTS. Resolve, then run: $SCRIPT_DIR/validate-check-rtt-ptt.sh"
    git diff --name-only --diff-filter=U
    exit 1
}

"$SCRIPT_DIR/validate-check-rtt-ptt.sh"
