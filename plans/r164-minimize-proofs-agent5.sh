#!/usr/bin/env bash
# R164 Agent5: veracity-minimize-proofs (asserts + proof blocks, --resume)
# 19 chapters, skipping fugly: 37 41 42 43 45 55

cd ~/projects/APAS-VERUS-agent5
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent5 \
    -l /home/milnes/projects/APAS-VERUS-agent5/src/vstdplus \
    --project APAS -a -p --no-lib-min \
    --chapter Chap52 --chapter Chap65 --chapter Chap39 --chapter Chap51 \
    --chapter Chap62 --chapter Chap26 --chapter Chap35 --chapter Chap47 \
    --chapter Chap18 --chapter Chap21 --chapter Chap06 --chapter Chap40 \
    --chapter Chap54 --chapter Chap59 --chapter Chap17 --chapter Chap50 \
    --chapter Chap58 --chapter Chap64 --chapter Chap03
