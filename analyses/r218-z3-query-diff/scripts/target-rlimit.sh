#!/bin/bash
# Usage: target-rlimit.sh Z3_OUTPUT [N]
# Prints the result and rlimit consumed by the N-th check-sat (default 7,
# the body query of prefix_sums_dc_inner in the ScanDCMtPer module log).
# Verus brackets each check-sat with (get-info :all-statistics); the
# consumption is the first :rlimit-count after the result minus the last
# :rlimit-count before it.
N="${2:-7}"
awk -v N="$N" '/:rlimit-count/ { if (seen) { print res, $2 - before; exit } before = $2 }
  /^(unsat|sat|unknown)$/ { c++; if (c == N) { res = $1; seen = 1 } }' "$1"
