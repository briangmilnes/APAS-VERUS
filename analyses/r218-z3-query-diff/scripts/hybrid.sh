#!/bin/bash
# Usage: hybrid.sh CONTENT.smt2 ORDER_A.smt2 ORDER_B.smt2 K > OUT.smt2
# Emits the blocks of CONTENT.smt2 in this order: the first K blocks of
# ORDER_B's block order, then every remaining block in ORDER_A's order.
# K=0 gives ORDER_A's order; K=number of blocks gives ORDER_B's order.
# Block splitting and keys are as in reorder.sh.
set -euo pipefail
export LC_ALL=C
split() {
  awk 'BEGIN{b=""} /^;; |^\(push\)|^\(pop\)/{ if(b!="") print b; b=$0; next} {b=(NR==1 ? $0 : b "\x01" $0)} END{print b}' "$1"
}
key() {
  sed -E 's/anonymous_closure(%|__)[0-9]+/anonymous_closure\1X/g; s/(impl_closure&__FnOnce)[0-9]+/\1X/g; s/%%lambda%%[0-9]+/%%lambda%%L/g; s/%%global_location_label%%[0-9]+/%%GLL%%/g; s/[ \x01]+/ /g; s/\(#[0-9]+\)/(#N)/g; s/ \(get-info :reason-unknown\)//g'
}
tmp=$(mktemp -d)
split "$1" > "$tmp/c.raw"; key < "$tmp/c.raw" > "$tmp/c.key"
split "$2" | key > "$tmp/a.key"
split "$3" | key > "$tmp/b.key"
paste -d $'\002' "$tmp/c.key" "$tmp/c.raw" > "$tmp/c.pair"
# Order: first K keys of B, then A's keys minus those already taken.
awk -v K="$4" 'FNR==1{f++} f==1 && FNR<=K { print; took[$0]++; next } f==2 { if (took[$0] > 0) { took[$0]--; next } print }' "$tmp/b.key" "$tmp/a.key" > "$tmp/o.key"
awk -F'\002' 'NR==FNR { n[$1]++; raw[$1, n[$1]] = $2; next }
  { u[$0]++; if (!(($0, u[$0]) in raw)) { print "MISSING BLOCK: " substr($0,1,120) > "/dev/stderr"; exit 1 }
    print raw[$0, u[$0]] }' "$tmp/c.pair" "$tmp/o.key" | tr '\001' '\n'
rm -rf "$tmp"
