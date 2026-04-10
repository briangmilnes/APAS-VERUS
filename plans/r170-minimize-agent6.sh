#!/usr/bin/env bash
# R170 Agent6: veracity-minimize-proofs --fresh (new thresholds)
cd ~/projects/APAS-VERUS-agent6
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent6 \
    -l /home/milnes/projects/APAS-VERUS-agent6/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap44 --chapter Chap45 --chapter Chap47 --chapter Chap49 --chapter Chap50 --chapter Chap51 --chapter Chap52
