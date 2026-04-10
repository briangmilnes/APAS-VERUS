#!/usr/bin/env bash
# R170 Agent7: veracity-minimize-proofs --fresh (new thresholds)
cd ~/projects/APAS-VERUS-agent7
git status --porcelain | wc -l  # must be 0

~/projects/veracity/target/release/veracity-minimize-proofs \
    -c /home/milnes/projects/APAS-VERUS-agent7 \
    -l /home/milnes/projects/APAS-VERUS-agent7/src/vstdplus \
    --project APAS --no-lib-min -a -p --danger --fresh \
    --chapter Chap53 --chapter Chap54 --chapter Chap55 --chapter Chap56 --chapter Chap57 --chapter Chap58 --chapter Chap59 --chapter Chap61 --chapter Chap62 --chapter Chap63 --chapter Chap64 --chapter Chap66
