# R165 Prompt F — Agent 6: Extract specs from Chap18+19+43. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent6`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Extract shared specs and lemmas from three related chapters:

- Chap18 (ArraySeq): `src/Chap18/ArraySeqSpecsAndLemmas.rs` — ~10 duplicated
  proof fns, 8 files, 11719 lines
- Chap19 (ArraySeq augmented): `src/Chap19/ArraySeqSpecsAndLemmas.rs` — ~10
  duplicated proof fns, 4 files, 6267 lines
- Chap43 (OrderedTable/Set): `src/Chap43/OrderedSpecsAndLemmas.rs` — ~12
  duplicated proof fns, 10 files, 12086 lines

## Files

`src/Chap18/`: ArraySeq.rs, ArraySeq{St,Mt}{Eph,Per}.rs, ArraySeqMtEphSlice.rs,
LinkedList{St,Mt}Eph.rs, LinkedListStPer.rs, MathSeq.rs

`src/Chap19/`: ArraySeq{St,Mt}Eph.rs, ArraySeqMtEphSlice.rs, ArraySeqStPer.rs

`src/Chap43/`: OrderedTable{St,Mt}{Eph,Per}.rs, OrderedSet{St,Mt}{Eph,Per}.rs,
AugOrderedTable{St,Mt}Eph.rs, AugOrderedTableStPer.rs

## Approach

Three separate extractions, same pattern as `src/Chap42/TableSpecsAndLemmas.rs`:

1. **Read all files and all standards.**
2. **Chap18:** ArraySeq St/Mt/Per variants share sequence operation specs and
   multiset/permutation lemmas. Focus on the 8 algorithm files.
3. **Chap19:** Closely related to Chap18 (augmented sequences). Similar shared specs.
4. **Chap43:** OrderedTable and OrderedSet variants share BST-backed sorted
   structure specs and ordering lemmas.
5. **Register each in lib.rs** as first entry in its chapter module.
6. **Validate each chapter separately:**
   ```bash
   scripts/validate.sh isolate Chap18
   scripts/validate.sh isolate Chap19
   scripts/validate.sh isolate Chap43
   ```

## Report

Write `plans/agent6-round165-report.md`.

## RCP

`git add -A && git commit -m "R165 Agent 6: extract ArraySeq/Ordered SpecsAndLemmas (−N lines)"`, then `git push`.
