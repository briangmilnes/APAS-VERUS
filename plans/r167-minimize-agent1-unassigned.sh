#!/usr/bin/env bash
# R167 Agent1: unassigned chapters (02,11,12,26,30,61,63,64)
cd ~/projects/APAS-VERUS-agent1
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent1 \
    -l /home/milnes/projects/APAS-VERUS-agent1/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap02 --chapter Chap11 --chapter Chap12 \
    --chapter Chap26 --chapter Chap30 --chapter Chap61 \
    --chapter Chap63 --chapter Chap64
