#!/bin/bash
# Re-run every existing derive_* experiment under Verus 0.2026.09.13, one at a time.
cd /home/milnes/projects/APAS-VERUS
OUT=scratch/rerun-derives-results.txt
: > "$OUT"
for f in src/experiments/derive_*.rs; do
    n=$(basename "$f" .rs)
    log=$(scripts/validate-standard.sh --experiment "$n" 2>&1 | tail -40)
    verdict=$(echo "$log" | grep -E "^(verification results|error)" | tail -3 | tr '\n' ' ')
    logfile=$(echo "$log" | grep -oE "logs/validate-standard-$n\.[0-9-]+\.log" | tail -1)
    echo "$n | $verdict | $logfile" >> "$OUT"
done
echo DONE >> "$OUT"
