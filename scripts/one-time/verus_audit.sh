#!/bin/bash
# Audit verusified files for admits, assumes, external_body, and ensures counts

cd ~/projects/APAS-VERUS

FILES=(
  src/Chap06/DirGraphStEph.rs
  src/Chap06/UnDirGraphStEph.rs
  src/Chap06/LabDirGraphStEph.rs
  src/Chap06/LabUnDirGraphStEph.rs
  src/Chap06/WeightedDirGraphStEphInt.rs
  src/Chap06/WeightedUnDirGraphStEphInt.rs
)

printf "%-35s %7s %8s %14s %8s\n" "File" "admits" "assumes" "external_body" "ensures"
printf "%-35s %7s %8s %14s %8s\n" "---" "------" "-------" "-------------" "-------"

for f in "${FILES[@]}"; do
  name=$(basename "$f")
  admits=$(grep -c "admit\!" "$f" 2>/dev/null)
  [ -z "$admits" ] && admits=0
  assumes=$(grep -c "assume\!" "$f" 2>/dev/null)
  [ -z "$assumes" ] && assumes=0
  ext=$(grep -c "external_body" "$f" 2>/dev/null)
  [ -z "$ext" ] && ext=0
  ensures=$(grep -c "ensures" "$f" 2>/dev/null)
  [ -z "$ensures" ] && ensures=0
  printf "%-35s %7s %8s %14s %8s\n" "$name" "$admits" "$assumes" "$ext" "$ensures"
done
