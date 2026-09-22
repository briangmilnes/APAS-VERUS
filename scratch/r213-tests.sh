#!/bin/bash
# r213: run one chapter's RTT (scratch/r212-rtt.sh) and, when the chapter has
# registered proof-time tests, its PTT (scratch/r212-ptt.sh), one after the
# other; print each log name and the pass/fail totals. A chapter with no
# [[test]] target is skipped (r212-rtt.sh would run the whole suite).
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"
c="$1"
grep -rn "\.finite()" src/$c/*.rs | head -3
if grep -q "path = \"tests/$c/" Cargo.toml; then
    scratch/r212-rtt.sh "$c" > /dev/null 2>&1
    f=$(ls -t logs/rtt.*.log | head -1)
    echo "RTT $f"
    grep -E "rc:|FAILED|panicked|^error" "$f" | head -30
    grep "test result" "$f" | awk '{s+=$4; x+=$6; n++} END {print "RTT " n " targets " s " passed " x " failed"}'
else
    echo "RTT none registered"
fi
if grep -q "path = \"tests/$c/" rust_verify_test/Cargo.toml; then
    scratch/r212-ptt.sh "$c" > /dev/null 2>&1
    f=$(ls -t logs/ptt-$c.*.log | head -1)
    echo "PTT $f"
    grep -E "rc:|^error" "$f" | sort | uniq -c | head -30
    grep "test result" "$f" | awk '{s+=$4; x+=$6; n++} END {print "PTT " n " targets " s " passed " x " failed"}'
else
    echo "PTT none registered"
fi
