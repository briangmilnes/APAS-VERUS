# R165 Prompt D — Agent 4: Extract graph algorithm specs from Chap54+55. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent4`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Extract shared specs and lemmas from two related graph algorithm chapters:

- Chap54 (BFS): `src/Chap54/BFSSpecsAndLemmas.rs` — ~13 duplicated proof fns,
  4 files, 2993 lines
- Chap55 (DFS/TopoSort/CycleDetect): `src/Chap55/DFSSpecsAndLemmas.rs` — ~14
  duplicated proof fns, 8 files, 5821 lines

## Files

`src/Chap54/`: BFSMtEph.rs, BFSMtPer.rs, BFSStEph.rs (+ BFSStPer if present)

`src/Chap55/`: CycleDetect{St,Mt}{Eph,Per}.rs, TopoSort{St,Mt}{Eph,Per}.rs,
DFS{St,Mt}{Eph,Per}.rs, SCC{St,Mt}Eph.rs

## Approach

Two separate extractions, same pattern as `src/Chap42/TableSpecsAndLemmas.rs`:

1. **Read all files in both chapters and all standards.**
2. **Chap54:** BFS variants share visited-set invariants, distance-bounded proofs,
   parent-array properties. Extract into `BFSSpecsAndLemmas.rs`.
3. **Chap55:** DFS variants share visited-set monotonicity, stack invariants,
   back-edge properties, finish-order proofs. The St/Per pairs
   (CycleDetectStEph/StPer, TopoSortStEph/StPer) have heavy overlap.
   Extract into `DFSSpecsAndLemmas.rs`.
4. **Register both in lib.rs** as first entries in their chapter modules.
5. **Validate each chapter separately:**
   ```bash
   scripts/validate.sh isolate Chap54
   scripts/validate.sh isolate Chap55
   ```

## Report

Write `plans/agent4-round165-report.md`.

## RCP

`git add -A && git commit -m "R165 Agent 4: extract BFS/DFS SpecsAndLemmas (−N lines)"`, then `git push`.
