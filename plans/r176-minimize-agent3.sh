#!/bin/bash
cd ~/projects/APAS-VERUS-agent3

# Strip prior-round markers from target file so --resume tests everything fresh
sed -i '/^[[:space:]]*\/\/ Veracity: NEEDED/d' src/Chap43/AugOrderedTableMtEph.rs
sed -i 's|^// Veracity: UNNEEDED [a-z ]* *||' src/Chap43/AugOrderedTableMtEph.rs

~/projects/veracity/target/release/veracity-minimize-proofs \
  -c . -l src/vstdplus --project APAS --chapter Chap43 \
  -F src/Chap43/AugOrderedTableMtEph.rs \
  -a -p --no-lib-min --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
