#!/bin/bash
cd ~/projects/APAS-VERUS-agent4
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c . -l src/vstdplus --project APAS --chapter Chap53 \
  -F src/Chap53/PQMinStPer.rs \
  -a -p --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
