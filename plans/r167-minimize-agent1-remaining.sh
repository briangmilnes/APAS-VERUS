#!/usr/bin/env bash
# R167 Agent1 remaining: chapters that failed on lock timeout
cd ~/projects/APAS-VERUS-agent1
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent1 \
    -l /home/milnes/projects/APAS-VERUS-agent1/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap18 --chapter Chap21 --chapter Chap23 \
    --chapter Chap26 --chapter Chap27 --chapter Chap28 \
    --chapter Chap35 --chapter Chap36 --chapter Chap37 --chapter Chap38
