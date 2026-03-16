#!/bin/bash
# Analyze proof holes across all chapters and produce a prioritized summary.
# Outputs to /tmp/hole-priorities.txt (and stdout).
#
# Usage: scripts/hole-priorities.sh [--no-info]
#   --no-info  Exclude accept() and assume_eq_clone_workaround from output

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT=/tmp/hole-priorities.txt
SHOW_INFO=true

if [[ "${1:-}" == "--no-info" ]]; then
    SHOW_INFO=true  # We filter below instead
    SKIP_INFO=true
else
    SKIP_INFO=false
fi

cd "$PROJECT_ROOT"

echo "Scanning chapters..." >&2

# Build raw data: chapter|file|lineno|category
TMP_RAW=/tmp/hole-raw.txt
> "$TMP_RAW"

for dir in src/Chap*/; do
    chap=$(basename "$dir")
    log="$dir/analyses/veracity-review-verus-proof-holes.log"
    if [[ ! -f "$log" ]]; then
        continue
    fi
    grep -E "^/\S+\.rs:[0-9]+: (error|warning):" "$log" | while IFS= read -r line; do
        # Format: /full/path/to/File.rs:123: error: category_name - description
        # Extract file: everything up to .rs, then just the basename.
        file=$(echo "$line" | sed -E 's,(^[^:]+\.rs):.*,\1,; s,.*/,,'  )
        lineno=$(echo "$line" | sed -E 's,^[^:]+:([0-9]+):.*,\1,')
        category=$(echo "$line" | sed -E 's,.*(error|warning): ,,; s, -.*,,; s, .*,,; s,[()],,g')
        echo "${chap}|${file}|${lineno}|${category}"
    done >> "$TMP_RAW"
done

total_raw=$(wc -l < "$TMP_RAW")
echo "Found $total_raw total entries across all chapters." >&2

{
echo "============================================================"
echo "HOLE PRIORITIES — $(date '+%Y-%m-%d %H:%M')"
echo "============================================================"
echo ""

# Section 1: Summary by category.
echo "1. Summary by Category"
echo "------------------------------"
printf "  %-35s %5s\n" "Category" "Count"
printf "  %-35s %5s\n" "---" "-----"

for cat in fn_missing_requires fn_missing_requires_ensures requires_true \
           assume external_body assume_specification \
           accept assume_eq_clone_workaround unsafe_impl external; do
    count=$(grep -c "|${cat}$" "$TMP_RAW" || true)
    if [[ "$count" -gt 0 ]]; then
        if [[ "$SKIP_INFO" == "true" && ("$cat" == "accept" || "$cat" == "assume_eq_clone_workaround") ]]; then
            continue
        fi
        printf "  %-35s %5d\n" "$cat" "$count"
    fi
done
echo ""

actionable=$(grep -cE "\|(fn_missing_requires|fn_missing_requires_ensures|requires_true|assume|external_body|assume_specification)$" "$TMP_RAW" || true)
echo "  Total actionable: $actionable"
echo ""

# Section 2: Actionable by chapter.
echo "2. Actionable Holes by Chapter (sorted by count)"
echo "-------------------------------------------------"
printf "  %-10s %5s %5s %5s %5s\n" "Chapter" "Total" "FnMR" "Assum" "ExtBd"
printf "  %-10s %5s %5s %5s %5s\n" "----------" "-----" "-----" "-----" "-----"

# Build chapter counts, sort by total descending.
TMP_CHAP=/tmp/hole-chap.txt
> "$TMP_CHAP"
for chap in $(cut -d'|' -f1 "$TMP_RAW" | sort -u); do
    fmr=$(grep "^${chap}|" "$TMP_RAW" | grep -cE "\|(fn_missing_requires|fn_missing_requires_ensures|requires_true)$" || true)
    assumes=$(grep "^${chap}|" "$TMP_RAW" | grep -c "|assume$" || true)
    ext=$(grep "^${chap}|" "$TMP_RAW" | grep -c "|external_body$" || true)
    act=$((fmr + assumes + ext))
    if [[ "$act" -gt 0 ]]; then
        echo "$act|$chap|$fmr|$assumes|$ext" >> "$TMP_CHAP"
    fi
done
sort -t'|' -k1 -rn "$TMP_CHAP" | while IFS='|' read -r act chap fmr assumes ext; do
    printf "  %-10s %5d %5d %5d %5d\n" "$chap" "$act" "$fmr" "$assumes" "$ext"
done
rm -f "$TMP_CHAP"
echo ""

# Section 3: Mechanical fixes by file.
echo "3. Mechanical Fixes (fn_missing_requires/ensures/requires_true) by File"
echo "-----------------------------------------------------------------------"
printf "  %-10s %-40s %5s\n" "Chapter" "File" "Count"
printf "  %-10s %-40s %5s\n" "----------" "----------------------------------------" "-----"

grep -E "\|(fn_missing_requires|fn_missing_requires_ensures|requires_true)$" "$TMP_RAW" | \
    awk -F'|' '{print $1"|"$2}' | sort | uniq -c | sort -rn | \
    while read -r count chapfile; do
        chap=$(echo "$chapfile" | cut -d'|' -f1)
        file=$(echo "$chapfile" | cut -d'|' -f2)
        printf "  %-10s %-40s %5d\n" "$chap" "$file" "$count"
    done
echo ""

# Section 4: Real proof work by file.
echo "4. Real Proof Work (assume + external_body) by File"
echo "---------------------------------------------------"
printf "  %-10s %-40s %5s %5s\n" "Chapter" "File" "Assum" "ExtBd"
printf "  %-10s %-40s %5s %5s\n" "----------" "----------------------------------------" "-----" "-----"

grep -E "\|(assume|external_body)$" "$TMP_RAW" | \
    awk -F'|' '{print $1"|"$2}' | sort -u | \
    while IFS='|' read -r chap file; do
        a=$(grep "^${chap}|${file}|" "$TMP_RAW" | grep -c "|assume$" || true)
        e=$(grep "^${chap}|${file}|" "$TMP_RAW" | grep -c "|external_body$" || true)
        total=$((a + e))
        printf "  %03d|%-10s %-40s %5d %5d\n" "$total" "$chap" "$file" "$a" "$e"
    done | sort -rn | sed 's/^ *[0-9]*|/  /'
echo ""

echo "Generated: $(date)"

} > "$OUT"

cat "$OUT"
rm -f "$TMP_RAW"
