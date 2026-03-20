#!/usr/bin/env bash
# Watch agent worktrees for new commits. Poll every 30 minutes.
# Detects commits made AFTER the watcher starts (compares HEAD snapshots).
# Exits when any agent's HEAD changes.
# Usage: scripts/watch-agents.sh

set -euo pipefail

AGENTS=(agent1 agent2 agent3 agent4)
MAIN_DIR="/home/milnes/projects/APAS-VERUS"
INTERVAL=1800  # 30 minutes

main_head() {
    git -C "$MAIN_DIR" rev-parse origin/main 2>/dev/null
}

agent_head() {
    local n=$1
    local dir="/home/milnes/projects/APAS-VERUS-${n}"
    [[ -d "$dir" ]] && git -C "$dir" rev-parse HEAD 2>/dev/null || echo "missing"
}

agent_dirty() {
    local n=$1
    local dir="/home/milnes/projects/APAS-VERUS-${n}"
    [[ -d "$dir" ]] && git -C "$dir" status --porcelain 2>/dev/null | head -5 || echo "missing"
}

agent_branch() {
    local n=$1
    local dir="/home/milnes/projects/APAS-VERUS-${n}"
    [[ -d "$dir" ]] && git -C "$dir" branch --show-current 2>/dev/null || echo "missing"
}

agent_log() {
    local n=$1
    local base=$2
    local dir="/home/milnes/projects/APAS-VERUS-${n}"
    [[ -d "$dir" ]] && git -C "$dir" log --oneline "${base}..HEAD" 2>/dev/null || echo "  (no new commits)"
}

# Snapshot initial state
MAIN=$(main_head)
echo "=== Agent Watcher started at $(date) ==="
echo "Main HEAD: ${MAIN:0:11}"
echo ""

declare -A INITIAL_HEADS
echo "Initial snapshots:"
for a in "${AGENTS[@]}"; do
    INITIAL_HEADS[$a]=$(agent_head "$a")
    branch=$(agent_branch "$a")
    dirty=$(agent_dirty "$a")
    short=${INITIAL_HEADS[$a]:0:11}
    if [[ "${INITIAL_HEADS[$a]}" == "missing" ]]; then
        echo "  $a: worktree missing"
    elif [[ -n "$dirty" ]]; then
        n_dirty=$(echo "$dirty" | wc -l)
        echo "  $a ($branch): HEAD=$short, $n_dirty dirty files (working)"
    else
        echo "  $a ($branch): HEAD=$short, clean"
    fi
done
echo ""
echo "Polling every ${INTERVAL}s. Will exit when any HEAD changes."
echo ""

while true; do
    sleep "$INTERVAL"
    echo "--- Poll at $(date) ---"
    DONE=()

    for a in "${AGENTS[@]}"; do
        head=$(agent_head "$a")
        branch=$(agent_branch "$a")
        dirty=$(agent_dirty "$a")
        short=${head:0:11}
        init_short=${INITIAL_HEADS[$a]:0:11}

        if [[ "$head" == "missing" ]]; then
            echo "  $a: worktree missing"
            continue
        fi

        if [[ "$head" != "${INITIAL_HEADS[$a]}" ]]; then
            # HEAD changed — new commit(s) since watcher started
            new_commits=$(agent_log "$a" "${INITIAL_HEADS[$a]}")
            echo "  $a ($branch): NEW COMMIT at $short (was $init_short)"
            if [[ -n "$new_commits" ]]; then
                echo "$new_commits" | sed 's/^/    /'
            fi
            DONE+=("$a")
        elif [[ -n "$dirty" ]]; then
            n_dirty=$(echo "$dirty" | wc -l)
            echo "  $a ($branch): working ($n_dirty dirty files), HEAD=$short"
        else
            echo "  $a ($branch): idle, HEAD=$short"
        fi
    done

    if [[ ${#DONE[@]} -gt 0 ]]; then
        echo ""
        echo "=== AGENTS WITH NEW COMMITS: ${DONE[*]} ==="
        echo "=== Exiting at $(date) ==="
        exit 0
    fi

    echo "  No new commits yet."
    echo ""
done
