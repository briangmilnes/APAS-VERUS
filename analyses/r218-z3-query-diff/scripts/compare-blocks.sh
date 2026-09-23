#!/bin/bash
# Usage: compare-blocks.sh A.smt2 B.smt2
# Splits each Verus SMT log into blocks (a block starts at ";; ", "(push)"
# or "(pop)"), normalizes names that depend only on emission order or on the
# crate-wide closure counter (see reorder.sh), and reports:
#   - block, declare-sort, declare-fun, declare-const, assert, forall,
#     push and check-sat counts for each file;
#   - blocks present in only one file (as a multiset);
#   - how many block headers are in a different position.
set -euo pipefail
export LC_ALL=C
split() {
  awk 'BEGIN{b=""} /^;; |^\(push\)|^\(pop\)/{ if(b!="") print b; b=$0; next} {b=(NR==1 ? $0 : b "\x01" $0)} END{print b}' "$1"
}
key() {
  sed -E 's/anonymous_closure(%|__)[0-9]+/anonymous_closure\1X/g; s/(impl_closure&__FnOnce)[0-9]+/\1X/g; s/%%lambda%%[0-9]+/%%lambda%%L/g; s/%%global_location_label%%[0-9]+/%%GLL%%/g; s/[ \x01]+/ /g; s/\(#[0-9]+\)/(#N)/g; s/ \(get-info :reason-unknown\)//g'
}
tmp=$(mktemp -d)
for x in A B; do
  f=$1; [ $x = B ] && f=$2
  split "$f" > "$tmp/$x.raw"; key < "$tmp/$x.raw" > "$tmp/$x.key"
  sort "$tmp/$x.key" > "$tmp/$x.sorted"
  printf '%s %s: blocks=%s declare-sort=%s declare-fun=%s declare-const=%s assert=%s forall=%s push=%s check-sat=%s\n' "$x" "$f" \
    "$(wc -l < "$tmp/$x.key")" "$(grep -c '^(declare-sort' "$f")" "$(grep -c '^ *(declare-fun' "$f")" \
    "$(grep -c '^ *(declare-const' "$f")" "$(grep -c '^ *(assert' "$f")" "$(grep -c 'forall' "$f")" \
    "$(grep -c '^(push)' "$f")" "$(grep -c '(check-sat)' "$f")"
done
echo "blocks only in A: $(awk -v MODE=Aonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A.sorted" "$tmp/B.sorted" | wc -l)"
awk -v MODE=Aonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A.sorted" "$tmp/B.sorted" | cut -c1-160 | sed 's/^/  A-only: /'
echo "blocks only in B: $(awk -v MODE=Bonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A.sorted" "$tmp/B.sorted" | wc -l)"
awk -v MODE=Bonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A.sorted" "$tmp/B.sorted" | cut -c1-160 | sed 's/^/  B-only: /'
cut -d$'\001' -f1 "$tmp/A.raw" | key > "$tmp/A.h"; cut -d$'\001' -f1 "$tmp/B.raw" | key > "$tmp/B.h"
echo "header-order diff lines (diff of first-line sequences): $(diff "$tmp/A.h" "$tmp/B.h" | grep -c '^[<>]' || true)"
rm -rf "$tmp"
