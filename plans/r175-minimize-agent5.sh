#!/bin/bash
cd ~/projects/APAS-VERUS-agent5

for f in \
  src/Chap57/DijkstraStEphF64.rs \
  src/Chap57/DijkstraStEphU64.rs \
  src/Chap59/JohnsonMtEphF64.rs \
  src/Chap59/JohnsonStEphF64.rs \
  src/Chap59/JohnsonMtEphI64.rs \
  src/Chap59/JohnsonStEphI64.rs; do

  # Determine chapter from path
  CHAP=$(echo "$f" | grep -oP 'Chap\d+')

  ~/projects/veracity/target/release/veracity-minimize-proofs \
    -c . -l src/vstdplus --project APAS --chapter "$CHAP" \
    -F "$f" \
    -a -p --no-lib-min --fresh --danger \
    --max-incremental 0.00 --max-memory-increase 0.00
done
