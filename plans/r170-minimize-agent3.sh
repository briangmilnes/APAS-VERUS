#!/usr/bin/env bash
# R170 Agent3: veracity-minimize-proofs --fresh (new thresholds)
cd ~/projects/APAS-VERUS-agent3
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent3 \
    -l /home/milnes/projects/APAS-VERUS-agent3/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap37 --chapter Chap38 --chapter Chap39
