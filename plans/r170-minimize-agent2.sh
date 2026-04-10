#!/usr/bin/env bash
# R170 Agent2: veracity-minimize-proofs --fresh (new thresholds)
cd ~/projects/APAS-VERUS-agent2
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent2 \
    -l /home/milnes/projects/APAS-VERUS-agent2/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap03 --chapter Chap06 --chapter Chap11 --chapter Chap17 --chapter Chap21 --chapter Chap23 --chapter Chap26 --chapter Chap27 --chapter Chap28 --chapter Chap30 --chapter Chap35 --chapter Chap36
