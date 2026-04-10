#!/bin/bash
cd ~/projects/APAS-VERUS-agent2
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c . -l src/vstdplus --project APAS --chapter Chap37 \
  -F src/Chap37/BSTRBMtEph.rs \
  -a -p --no-lib-min --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
