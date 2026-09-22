#!/bin/bash
# r213: run scripts/validate.sh isolate on each chapter given, one at a time,
# and print the log name with its verification-results line and any warning
# or error count. Usage: scratch/r213-regress.sh Chap02 Chap03 ...
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"
for c in "$@"; do
    scripts/validate.sh isolate "$c" > /dev/null 2>&1
    f=$(ls -t logs/validate.*.log | head -1)
    r=$(grep "verification results" "$f")
    w=$(grep -c "^warning" "$f")
    e=$(grep -c "^error" "$f")
    t=$(grep -c "automatically chose triggers" "$f")
    echo "$c $f | $r | warnings $w | error lines $e | trigger notes $t"
done
