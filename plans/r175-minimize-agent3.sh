#!/bin/bash
cd ~/projects/APAS-VERUS-agent3
for f in src/Chap43/AugOrderedTableMtEph.rs src/Chap43/AugOrderedTableStEph.rs src/Chap43/OrderedTableMtEph.rs; do
  ~/projects/veracity/target/release/veracity-minimize-proofs \
    -c . -l src/vstdplus --project APAS --chapter Chap43 \
    -F "$f" \
    -a -p --no-lib-min --fresh --danger \
    --max-incremental 0.00 --max-memory-increase 0.00
done
