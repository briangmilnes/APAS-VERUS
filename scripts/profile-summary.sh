#!/bin/bash
# Summarize Z3 profile traces: count instantiations per quantifier.
# Usage: scripts/profile-summary.sh <profile-dir-or-file>
#        scripts/profile-summary.sh logs/profile/Chap65/20260329-153241/
#        scripts/profile-summary.sh .verus-solver-log/Chap03__InsertionSortStEph__InsertionSortStEph.profile

set -uo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <profile-dir-or-file>"
    exit 1
fi

TARGET="$1"

if [ -d "$TARGET" ]; then
    FILES=("$TARGET"/*.profile)
else
    FILES=("$TARGET")
fi

for pf in "${FILES[@]}"; do
    [ -f "$pf" ] || continue
    fname=$(basename "$pf" .profile)

    # Count [new-match] per quantifier.
    # [new-match] format: [new-match] <fingerprint> <quant-id> <bind1> ... ; <result>
    # [mk-quant] format: [mk-quant] <id> <name> <arity> <body> <pattern>
    # Map quant id -> name from [mk-quant], count matches per quant from [new-match].
    result=$(awk '
    /^\[mk-quant\]/ {
        qname[$2] = $3
    }
    /^\[new-match\]/ {
        qid = $3
        count[qid]++
        total++
    }
    END {
        printf "TOTAL %d\n", total
        for (qid in count) {
            name = (qid in qname) ? qname[qid] : qid
            printf "%8d  %s  (%s)\n", count[qid], name, qid
        }
    }
    ' "$pf" | sort -rn)

    total=$(echo "$result" | head -1 | awk '{print $2}')
    echo ""
    echo "=== $fname: $total instantiations ==="
    echo "$result" | tail -n +2 | head -30
done

# Grand totals.
echo ""
echo "=== Per-module totals ==="
for pf in "${FILES[@]}"; do
    [ -f "$pf" ] || continue
    fname=$(basename "$pf" .profile)
    total=$(awk '/^\[new-match\]/ { n++ } END { printf "%d", n+0 }' "$pf")
    printf "%8d  %s\n" "$total" "$fname"
done | sort -rn

echo ""
echo "=== Grand top quantifiers (top 40) ==="
for pf in "${FILES[@]}"; do
    [ -f "$pf" ] || continue
    awk '
    /^\[mk-quant\]/ { qname[$2] = $3 }
    /^\[new-match\]/ { qid = $3; count[qid]++ }
    END {
        for (qid in count) {
            name = (qid in qname) ? qname[qid] : qid
            printf "%8d  %s\n", count[qid], name
        }
    }
    ' "$pf"
done | awk '{ counts[$2] += $1 } END { for (n in counts) printf "%8d  %s\n", counts[n], n }' \
    | sort -rn | head -40
