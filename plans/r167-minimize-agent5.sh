#!/usr/bin/env bash
# R167 Agent5: veracity-minimize-proofs (asserts + proof blocks, --resume)
# Chapters: Chap52 Chap53 Chap54

cd ~/projects/APAS-VERUS-agent5
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent5 \
    -l /home/milnes/projects/APAS-VERUS-agent5/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap52 --chapter Chap53 --chapter Chap54
