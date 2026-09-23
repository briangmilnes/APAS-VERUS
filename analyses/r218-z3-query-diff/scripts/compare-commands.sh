#!/bin/bash
# Usage: compare-commands.sh A.smt2 B.smt2 [OUTDIR]
# Compares the module prelude (everything before the first "(push)") of two
# Verus SMT logs at the granularity of single top-level SMT commands
# (a command starts at a line beginning with "(").  Comment lines are
# dropped. Names that depend only on emission order or on the crate-wide
# closure counter are normalized as in reorder.sh. Prints command counts and
# the commands present in only one prelude (multiset difference); with
# OUTDIR, writes them to OUTDIR/{A,B}-only.smt2.
set -euo pipefail
export LC_ALL=C
cmds() {
  awk '/^\(push\)/{exit} /^;/{next} /^\(/{ if(c!="") print c; c=$0; next} { c=c "\x01" $0 } END{ if(c!="") print c }' "$1" |
  sed -E 's/anonymous_closure(%|__)[0-9]+/anonymous_closure\1X/g; s/(impl_closure&__FnOnce)[0-9]+/\1X/g; s/%%lambda%%[0-9]+/%%lambda%%L/g; s/%%global_location_label%%[0-9]+/%%GLL%%/g; s/[ \x01]+/ /g'
}
tmp=$(mktemp -d)
cmds "$1" | sort > "$tmp/A"; cmds "$2" | sort > "$tmp/B"
echo "prelude commands: A=$(wc -l < "$tmp/A") B=$(wc -l < "$tmp/B")"
echo "A-only: $(awk -v MODE=Aonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A" "$tmp/B" | wc -l)  B-only: $(awk -v MODE=Bonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A" "$tmp/B" | wc -l)"
if [ $# -ge 3 ]; then
  mkdir -p "$3"
  awk -v MODE=Aonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A" "$tmp/B" | tr '\001' '\n' > "$3/A-only.smt2"
  awk -v MODE=Bonly -f "$(dirname "$0")/msdiff.awk" "$tmp/A" "$tmp/B" | tr '\001' '\n' > "$3/B-only.smt2"
fi
rm -rf "$tmp"
