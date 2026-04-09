#!/usr/bin/env bash
# R167 Agent3: veracity-minimize-proofs (asserts + proof blocks, --resume)
# Chapters: Chap42

cd ~/projects/APAS-VERUS-agent3
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent3 \
    -l /home/milnes/projects/APAS-VERUS-agent3/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap42
