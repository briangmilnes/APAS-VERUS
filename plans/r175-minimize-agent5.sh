#!/bin/bash
cd ~/projects/APAS-VERUS-agent5
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c . --project APAS --chapter Chap57,Chap59 \
  --file DijkstraStEphF64.rs --file DijkstraStEphU64.rs \
  --file JohnsonMtEphF64.rs --file JohnsonStEphF64.rs \
  --file JohnsonMtEphI64.rs --file JohnsonStEphI64.rs \
  -a -p --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
