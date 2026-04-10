#!/usr/bin/env bash
# R170 Agent4 remaining: Chap61,63,64
cd ~/projects/APAS-VERUS-agent4
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent4 \
    -l /home/milnes/projects/APAS-VERUS-agent4/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap61 --chapter Chap63 --chapter Chap64
