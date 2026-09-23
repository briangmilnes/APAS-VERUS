#!/bin/bash
# Usage: keep-one-query.sh IN.smt2 K > OUT.smt2
# Keeps every command outside push/pop (the module prelude and the
# per-function declarations Verus emits between queries) and only the K-th
# push ... pop query block; all other query blocks are dropped. This removes
# the prior check-sat calls from the Z3 session while keeping the context.
awk -v K="$2" '/^\(push\)/ { p++; inside = 1; keep = (p == K) }
  !inside || keep { print }
  /^\(pop\)/ { inside = 0; keep = 0 }' "$1"
