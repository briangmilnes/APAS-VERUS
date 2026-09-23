#!/bin/bash
# Usage: reorder.sh CONTENT.smt2 ORDER.smt2 > OUT.smt2
# Emits the blocks of CONTENT.smt2 (raw text, its own names and numbering)
# in the block order of ORDER.smt2. A block starts at a line beginning with
# ";; ", "(push)" or "(pop)". Blocks are matched by a normalized key that
# erases names and numbers that depend only on emission order or on the
# crate-wide closure counter: anonymous_closure ids, impl_closure&__FnOnce ids in qids, %%lambda%% ids,
# %%global_location_label%% ids, span ids "(#N)", and the
# "(get-info :reason-unknown)" line Verus adds after an unknown result.
set -euo pipefail
export LC_ALL=C
split() { # file -> one block per line, fields separated by \x01
  awk 'BEGIN{b=""} /^;; |^\(push\)|^\(pop\)/{ if(b!="") print b; b=$0; next} {b=(NR==1 ? $0 : b "\x01" $0)} END{print b}' "$1"
}
key() {
  sed -E 's/anonymous_closure(%|__)[0-9]+/anonymous_closure\1X/g; s/(impl_closure&__FnOnce)[0-9]+/\1X/g; s/%%lambda%%[0-9]+/%%lambda%%L/g; s/%%global_location_label%%[0-9]+/%%GLL%%/g; s/[ \x01]+/ /g; s/\(#[0-9]+\)/(#N)/g; s/ \(get-info :reason-unknown\)//g'
}
tmp=$(mktemp -d)
split "$1" > "$tmp/c.raw"; key < "$tmp/c.raw" > "$tmp/c.key"
split "$2" > "$tmp/o.raw"; key < "$tmp/o.raw" > "$tmp/o.key"
paste -d $'\002' "$tmp/c.key" "$tmp/c.raw" > "$tmp/c.pair"
awk -F'\002' 'NR==FNR { n[$1]++; raw[$1, n[$1]] = $2; next }
  { u[$0]++; if (!(($0, u[$0]) in raw)) { print "MISSING BLOCK: " substr($0,1,120) > "/dev/stderr"; exit 1 }
    print raw[$0, u[$0]] }' "$tmp/c.pair" "$tmp/o.key" | tr '\001' '\n'
rm -rf "$tmp"
