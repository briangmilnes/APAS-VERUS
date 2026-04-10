#!/usr/bin/env bash
# R170 Agent2 remaining: Chap59,62,66
cd ~/projects/APAS-VERUS-agent2
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent2 \
    -l /home/milnes/projects/APAS-VERUS-agent2/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap59 --chapter Chap62 --chapter Chap66
