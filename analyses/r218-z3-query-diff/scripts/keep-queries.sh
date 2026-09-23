#!/bin/bash
# Usage: keep-queries.sh IN.smt2 "K1 K2 ..." > OUT.smt2
# Like keep-one-query.sh, but keeps the listed push ... pop query blocks
# (1-based, in file order) and drops the others.
awk -v L="$2" 'BEGIN { n = split(L, a, " "); for (i = 1; i <= n; i++) want[a[i]] = 1 }
  /^\(push\)/ { p++; inside = 1; keep = (p in want) }
  !inside || keep { print }
  /^\(pop\)/ { inside = 0; keep = 0 }' "$1"
