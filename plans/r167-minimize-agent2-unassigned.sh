#!/usr/bin/env bash
# R167 Agent2: remaining unqueued chapters (44,50,54)
# 44 and 50 not in any queue. 54 is in agent5's queue but agent5 is stuck on Chap52 for hours.
cd ~/projects/APAS-VERUS-agent2
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent2 \
    -l /home/milnes/projects/APAS-VERUS-agent2/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap44 --chapter Chap50 --chapter Chap54
