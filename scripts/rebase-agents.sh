#!/usr/bin/env bash
# Rebase all agent worktrees onto origin/main and force-push.
# Run from main worktree after main is committed and pushed.
set -euo pipefail

MAIN=~/projects/APAS-VERUS
AGENTS=(
    "$MAIN-agent1:agent1/ready"
    "$MAIN-agent2:agent2/ready"
    "$MAIN-agent3:agent3/ready"
    "$MAIN-agent4:agent4/ready"
)

# Verify main is clean and pushed.
main_dirty=$(git -C "$MAIN" status --short)
main_unpushed=$(git -C "$MAIN" log origin/main..HEAD --oneline 2>/dev/null || true)
if [[ -n "$main_dirty" ]]; then
    echo "ERROR: main has uncommitted changes. Commit first."
    exit 1
fi
if [[ -n "$main_unpushed" ]]; then
    echo "ERROR: main has unpushed commits. Push first."
    exit 1
fi

failed=0
for entry in "${AGENTS[@]}"; do
    IFS=: read -r dir branch <<< "$entry"
    label="${branch%%/*}"

    if [[ ! -d "$dir/.git" && ! -f "$dir/.git" ]]; then
        echo "$label: NOT FOUND — skipping"
        continue
    fi

    echo "$label: fetching..."
    git -C "$dir" fetch origin

    echo "$label: rebasing onto origin/main..."
    if ! git -C "$dir" rebase origin/main; then
        echo "ERROR: $label rebase failed. Fix conflicts in $dir, then rerun."
        failed=1
        break
    fi

    echo "$label: force-pushing..."
    git -C "$dir" push origin "$branch" --force

    echo "$label: done"
    echo ""
done

if [[ $failed -eq 0 ]]; then
    echo "All agents rebased and pushed."
    ~/projects/APAS-VERUS/scripts/survey-agents.sh
fi
