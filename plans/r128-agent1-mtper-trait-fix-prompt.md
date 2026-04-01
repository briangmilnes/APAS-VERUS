# R128 Agent 1 — Fix Chap18 MtPer: wire map+reduce through trait methods. AFK. DOT.

## Problem

R127 agent 3 added parallel `_inner` helpers (map_inner, filter_inner, reduce_inner,
tabulate_inner) to `src/Chap18/ArraySeqMtPer.rs` but left the trait methods sequential.
The claim was "changing trait bounds breaks 21 callers." This is wrong — agents 2 and 4
changed the same trait bounds on Chap18 MtEph and Chap19 MtEph and verified clean.

Your job: properly parallelize the **map** and **reduce** trait methods in MtPer,
matching what agents 2 and 4 did for MtEph.

## What to do

1. Read `src/Chap18/ArraySeqMtEph.rs` to see how agent 2 parallelized map and reduce
   (the `map_dc` and `reduce_dc` helpers, trait bound changes).

2. In `src/Chap18/ArraySeqMtPer.rs`:
   - Add `Clone + Send + Sync + 'static` bounds to `F` in the trait declarations for
     `map` and `reduce` (in `ArraySeqMtPerRedefinableTrait` or whatever the trait is called).
   - Add matching bounds to `T`/`U` where needed (check what MtEph required).
   - Rewrite the `map` and `reduce` trait method bodies to use divide-and-conquer with
     `join()`, following the MtEph pattern.
   - Use `clone_fn`, `clone_fn2` from `crate::vstdplus::clone_plus::clone_plus::*`.

3. If agent 3's `_inner` helpers are good implementations, you may wire the trait methods
   to delegate to them (after adding the right trait bounds). If they have weaker specs
   than the trait methods, write proper D&C implementations instead.

4. Update the Code review annotations for map and reduce to reflect the new parallel span.

5. Do NOT touch filter, tabulate, scan, or the other functions — only map and reduce.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs`
2. `src/standards/hfscheduler_standard.rs`

## Validation

Run `scripts/validate.sh isolate Chap18` after changes. Fix verification errors.
Then run `scripts/rtt.sh`.

## Rules

- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Preserve all existing RTTs.

## When done

Commit with `git add -A && git commit` and push.

## Report

Write `plans/agent1-r128-report.md`.
