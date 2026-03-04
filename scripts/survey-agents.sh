#!/usr/bin/env bash
# Survey all agent worktrees: commit, uncommitted changes, unpushed commits.
# Output as a table.
set -euo pipefail

MAIN=~/projects/APAS-VERUS
AGENTS=(
    "$MAIN:main:origin/main"
    "$MAIN-agent1:agent1/ready:origin/agent1/ready"
    "$MAIN-agent2:agent2/ready:origin/agent2/ready"
    "$MAIN-agent3:agent3/ready:origin/agent3/ready"
    "$MAIN-agent4:agent4/ready:origin/agent4/ready"
)

# Header
printf "%-8s  %-8s  %-10s  %-10s  %s\n" "Worktree" "Commit" "Uncommitted" "Unpushed" "Message"
printf "%-8s  %-8s  %-10s  %-10s  %s\n" "--------" "--------" "-----------" "--------" "-------"

for entry in "${AGENTS[@]}"; do
    IFS=: read -r dir branch remote <<< "$entry"
    name=$(basename "$dir")
    if [[ "$name" == "APAS-VERUS" ]]; then
        label="main"
    else
        label="${name#APAS-VERUS-}"
    fi

    if [[ ! -d "$dir/.git" && ! -f "$dir/.git" ]]; then
        printf "%-8s  %-8s  %-10s  %-10s  %s\n" "$label" "-" "-" "-" "NOT FOUND"
        continue
    fi

    hash=$(git -C "$dir" log --format='%h' -1)
    msg=$(git -C "$dir" log --format='%s' -1 | cut -c1-60)
    dirty=$(git -C "$dir" status --short)
    unpushed=$(git -C "$dir" log "$remote..HEAD" --oneline 2>/dev/null || true)

    if [[ -n "$dirty" ]]; then
        dirty_count="$(echo "$dirty" | wc -l | tr -d ' ') files"
    else
        dirty_count="clean"
    fi

    if [[ -n "$unpushed" ]]; then
        unpush_count="$(echo "$unpushed" | wc -l | tr -d ' ') commits"
    else
        unpush_count="pushed"
    fi

    printf "%-8s  %-8s  %-11s  %-10s  %s\n" "$label" "$hash" "$dirty_count" "$unpush_count" "$msg"
done
