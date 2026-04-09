#!/usr/bin/env bash
# R167 Agent2: veracity-minimize-proofs (asserts + proof blocks, --resume)
# Chapters: Chap39 Chap40 Chap41

cd ~/projects/APAS-VERUS-agent2
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent2 \
    -l /home/milnes/projects/APAS-VERUS-agent2/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap39 --chapter Chap40 --chapter Chap41
