# R30: Trigger Warning Cleanup (4 Agents)

## Problem

170 "automatically chose triggers" notes flooding verification output.
39 files use `#![auto]` instead of explicit `#[trigger]`.
CLAUDE.md rule: "Do not leave `#![auto]` or trigger warnings in final code."

## Task (same for all agents)

For every file in your assignment:

1. Find all `#![auto]` in quantifiers (`forall`, `exists`).
2. Run `scripts/validate.sh`, capture the "automatically chose triggers" notes
   for your files. Verus prints the triggers it selected — use those.
3. Replace `#![auto]` with explicit `#[trigger]` on the term Verus selected.
4. Also fix any bare `forall|...|` / `exists|...|` without any trigger annotation
   that generates auto-select notes.
5. Validate. Zero errors required. Zero trigger notes from your files.
6. Do NOT change any logic, specs, or proof structure. Only add `#[trigger]`.
7. Do NOT add `#![auto]` — that's what we're removing.
8. Commit and push to your agent branch.

## Agent Assignments

### Agent 1: Chap37 + Chap38 (9 files)
- src/Chap37/AVLTreeSeqStEph.rs
- src/Chap37/BSTAVLMtEph.rs
- src/Chap37/BSTAVLStEph.rs
- src/Chap37/BSTBBAlphaMtEph.rs
- src/Chap37/BSTBBAlphaStEph.rs
- src/Chap37/BSTPlainMtEph.rs
- src/Chap37/BSTPlainStEph.rs
- src/Chap37/BSTRBStEph.rs
- src/Chap38/BSTParaStEph.rs

### Agent 2: Chap40 + Chap41 + Chap42 + Chap43 (11 files)
- src/Chap40/BSTKeyValueStEph.rs
- src/Chap40/BSTReducedStEph.rs
- src/Chap40/BSTSizeStEph.rs
- src/Chap41/ArraySetEnumMtEph.rs
- src/Chap42/TableMtEph.rs
- src/Chap42/TableStEph.rs
- src/Chap42/TableStPer.rs
- src/Chap43/AugOrderedTableStEph.rs
- src/Chap43/AugOrderedTableStPer.rs
- src/Chap43/OrderedTableStEph.rs
- src/Chap43/OrderedTableStPer.rs

### Agent 3: Chap45 + Chap51 + Chap54 (10 files)
- src/Chap45/SortedListPQ.rs
- src/Chap45/UnsortedListPQ.rs
- src/Chap51/TopDownDPMtEph.rs
- src/Chap51/TopDownDPMtPer.rs
- src/Chap51/TopDownDPStEph.rs
- src/Chap51/TopDownDPStPer.rs
- src/Chap54/BFSMtEph.rs
- src/Chap54/BFSMtPer.rs
- src/Chap54/BFSStEph.rs
- src/Chap54/BFSStPer.rs

### Agent 4: Chap55 + vstdplus (9 files)
- src/Chap55/CycleDetectStEph.rs
- src/Chap55/CycleDetectStPer.rs
- src/Chap55/DFSStEph.rs
- src/Chap55/DFSStPer.rs
- src/Chap55/SCCStEph.rs
- src/Chap55/SCCStPer.rs
- src/Chap55/TopoSortStEph.rs
- src/Chap55/TopoSortStPer.rs
- src/vstdplus/seq_set.rs

## No File Conflicts

All assignments are chapter-disjoint.

## Verification

Each agent runs `scripts/validate.sh` after changes (0 errors, 0 trigger notes from assigned files).
