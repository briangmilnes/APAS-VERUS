#!/usr/bin/env bash
# R167 Agent6: veracity-minimize-proofs (asserts + proof blocks, --resume)
# Chapters: Chap55 Chap56 Chap57 Chap58 Chap59 Chap62 Chap65 Chap66

cd ~/projects/APAS-VERUS-agent6
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent6 \
    -l /home/milnes/projects/APAS-VERUS-agent6/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap55 --chapter Chap56 --chapter Chap57 --chapter Chap58 --chapter Chap59 --chapter Chap62 --chapter Chap65 --chapter Chap66
