#!/bin/bash
# r213: record a chapter that needed no edit, then commit and push.
# Usage: r213-record-clean.sh NN VERIFIED VLOGSTAMP RTT_TARGETS RTT_PASS RTTSTAMP "PTT text" "PTT cell"
set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"
NN=$1; V=$2; VL=$3; RT=$4; RP=$5; RL=$6; PTXT=$7; PCELL=$8
DOC=docs/ChaptersInOrder.md
cat >> $DOC <<EOT

### Chap$NN

- First run on 09.13: $V verified, 0 errors, 0 warnings, 0 trigger notes
  (\`logs/validate.$VL.log\`). No edit.
- RTT: $RT targets, $RP tests pass (\`logs/rtt.$RL.log\`).
- PTT: $PTXT
EOT
# Summary row: after the last row of the summary table.
last=$(grep -n '^| [0-9]* | [0-9][0-9] | ' $DOC | tail -1)
ln=${last%%:*}
idx=$(echo "$last" | awk -F'|' '{gsub(/ /,"",$2); print $2+1}')
awk -v ln="$ln" -v row="| $idx | $NN | $V | 0 | 0 | $RP pass | $PCELL | r213 Chap$NN |" \
    '{print} NR==ln {print row}' $DOC > $DOC.tmp && mv $DOC.tmp $DOC
# Status row in ChapterVerusification.md.
CV=docs/ChapterVerusification.md
awk -v nn="$NN" -v st="r213: $V verified, 0 err, 0 warn; RTT $RP pass" -v ev="\`validate.$VL.log\`" \
    'BEGIN{FS=OFS="|"} $3==" " nn " " && NF==8 {$6=" " st " "; $7=" " ev " "} {print}' $CV > $CV.tmp && mv $CV.tmp $CV
git add -A
git commit -q -m "r213 Chap$NN: $V verified, 0 errors, 0 warnings; RTT $RP pass

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>"
git push -q 2>&1 | tail -2
git log --oneline -1
grep "^| [0-9]* | $NN |" $CV
