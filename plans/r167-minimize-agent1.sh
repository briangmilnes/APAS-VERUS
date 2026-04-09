#!/usr/bin/env bash
# R167 Agent1: veracity-minimize-proofs (asserts + proof blocks, --resume)
# Chapters: Chap02 Chap03 Chap05 Chap06 Chap11 Chap17 Chap18 Chap19 Chap21 Chap23 Chap26 Chap27 Chap28 Chap35 Chap36 Chap37 Chap38

cd ~/projects/APAS-VERUS-agent1
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent1 \
    -l /home/milnes/projects/APAS-VERUS-agent1/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap02 --chapter Chap03 --chapter Chap05 --chapter Chap06 --chapter Chap11 --chapter Chap17 --chapter Chap18 --chapter Chap19 --chapter Chap21 --chapter Chap23 --chapter Chap26 --chapter Chap27 --chapter Chap28 --chapter Chap35 --chapter Chap36 --chapter Chap37 --chapter Chap38
