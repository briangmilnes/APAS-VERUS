#!/bin/bash
# proof-velocity.sh — Show proof state and weekly velocity
# Usage: scripts/proof-velocity.sh [days]  (default: 7)

set -uo pipefail
cd "$(git rev-parse --show-toplevel)"

DAYS=${1:-7}
TODAY=$(date +%Y-%m-%d)

echo "=== Proof State ==="
echo ""

# Current state from chapter-cleanliness-status.log
STATUS="analyses/chapter-cleanliness-status.log"
if [[ -f "$STATUS" ]]; then
    head -1 "$STATUS"
    echo ""
fi

# Current verified count from last validate
VERIFIED=$(git log --format="%s" -50 | grep -oP '\d{4} verified' | head -1 | sed 's/ verified//' || true)
echo "Verified: ${VERIFIED:-unknown}"

# RTT/PTT from recent commits
RTT=$(git log --format="%s" -50 | grep -oP '\d{4} (RTT|rtt)' | head -1 | grep -oP '\d{4}' || true)
PTT=$(git log --format="%s" -100 | grep -oP '\d+ (PTT|ptt|proof time)' | head -1 | grep -oP '^\d+' || true)
echo "RTT: ${RTT:-unknown}  PTT: ${PTT:-unknown}"
echo ""

echo "=== Remaining Holes by Chapter ==="
echo ""
if [[ -f "$STATUS" ]]; then
    printf "| %-3s | %-8s | %5s | %-9s | %-40s |\n" "#" "Chapter" "Holes" "ClnDeps?" "Blocked by"
    printf "| %-3s | %-8s | %5s | %-9s | %-40s |\n" "---" "--------" "-----" "---------" "----------------------------------------"
    IDX=1
    while read -r chap holes files clndeps rest; do
        blocked=$(echo "$rest" | sed 's/^ *//')
        printf "| %-3d | %-8s | %5d | %-9s | %-40s |\n" "$IDX" "$chap" "$holes" "${clndeps:-}" "${blocked:--}"
        IDX=$((IDX + 1))
    done < <(grep "^  Chap" "$STATUS" | awk 'NF >= 3 && $2 ~ /^[0-9]+$/' | sort -k2 -rn)
    echo ""
    # Use the authoritative header for totals (excludes Example files)
    HEADER_HOLES=$(head -1 "$STATUS" | grep -oP '\d+ holes' | grep -oP '\d+' || true)
    HOLED=$(head -1 "$STATUS" | grep -oP '\d+ holed' | grep -oP '\d+' || true)
    SUM_HOLES=$(grep "^  Chap" "$STATUS" | awk 'NF >= 3 && $2 ~ /^[0-9]+$/ {sum+=$2} END {print sum+0}')
    echo "Total: ${HEADER_HOLES:-$SUM_HOLES} holes (global) across ${HOLED:-?} chapters"
fi
echo ""

echo "=== Velocity (last $DAYS days) ==="
echo ""
printf "| %-10s | %8s | %6s |\n" "Date" "Verified" "Delta"
printf "| %-10s | %8s | %6s |\n" "----------" "--------" "------"

PREV=""
for i in $(seq "$DAYS" -1 0); do
    DAY=$(date -d "$TODAY - $i days" +%Y-%m-%d 2>/dev/null || date -v-${i}d +%Y-%m-%d 2>/dev/null)
    COUNT=$(git log --before="${DAY}T23:59:59" --format="%s" | grep -oP '\d{4} verified' | sed 's/ verified//' | sort -n | tail -1 || true)
    if [[ -n "$COUNT" ]]; then
        if [[ -n "$PREV" ]]; then
            DELTA=$((COUNT - PREV))
            printf "| %-10s | %8d | %+6d |\n" "$DAY" "$COUNT" "$DELTA"
        else
            printf "| %-10s | %8d | %6s |\n" "$DAY" "$COUNT" "--"
        fi
        PREV=$COUNT
    fi
done

if [[ -n "$PREV" ]]; then
    BASELINE_DATE=$(date -d "$TODAY - $DAYS days" +%Y-%m-%d 2>/dev/null || date -v-${DAYS}d +%Y-%m-%d 2>/dev/null)
    FIRST=$(git log --before="${BASELINE_DATE}T23:59:59" --format="%s" | grep -oP '\d{4} verified' | sed 's/ verified//' | sort -n | tail -1 || true)
    if [[ -n "$FIRST" ]]; then
        NET=$((PREV - FIRST))
        RATE=$((NET / DAYS))
        echo ""
        echo "Net: +$NET verified over $DAYS days (~$RATE/day)"
    fi
fi
