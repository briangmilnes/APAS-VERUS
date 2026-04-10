#!/usr/bin/env bash
# R170 Agent5: veracity-minimize-proofs --fresh (new thresholds)
cd ~/projects/APAS-VERUS-agent5
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent5 \
    -l /home/milnes/projects/APAS-VERUS-agent5/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap42 --chapter Chap43
