#!/bin/bash
cd ~/projects/APAS-VERUS-agent3
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c . --project APAS --chapter Chap43 \
  --file AugOrderedTableMtEph.rs --file AugOrderedTableStEph.rs --file OrderedTableMtEph.rs \
  -a -p --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
