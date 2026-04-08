#!/usr/bin/env bash
# R164 Agent6: veracity-minimize-proofs (asserts + proof blocks, --resume)
# 19 chapters, skipping fugly: 37 41 42 43 45 55

cd ~/projects/APAS-VERUS-agent6
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent6 \
    -l /home/milnes/projects/APAS-VERUS-agent6/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap38 --chapter Chap36 --chapter Chap27 --chapter Chap53 \
    --chapter Chap19 --chapter Chap23 --chapter Chap28 --chapter Chap44 \
    --chapter Chap57 --chapter Chap56 --chapter Chap66 --chapter Chap61 \
    --chapter Chap63 --chapter Chap49 --chapter Chap30 --chapter Chap11 \
    --chapter Chap12 --chapter Chap05 --chapter Chap02
