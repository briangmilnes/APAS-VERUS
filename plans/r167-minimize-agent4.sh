#!/usr/bin/env bash
# R167 Agent4: veracity-minimize-proofs (asserts + proof blocks, --resume)
# Chapters: Chap43 Chap44 Chap45 Chap47 Chap49 Chap50 Chap51

cd ~/projects/APAS-VERUS-agent4
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent4 \
    -l /home/milnes/projects/APAS-VERUS-agent4/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap43 --chapter Chap44 --chapter Chap45 --chapter Chap47 --chapter Chap49 --chapter Chap50 --chapter Chap51
