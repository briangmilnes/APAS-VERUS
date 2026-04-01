# R128 Agent 4 — Parallelize Chap41 AVLTreeSet Mt operations. AFK.

## Background

Chap41 has 14 DIFFERS across three Mt files:
- `src/Chap41/AVLTreeSetMtEph.rs` — 6 functions
- `src/Chap41/AVLTreeSetMtPer.rs` — 6 functions
- `src/Chap41/ArraySetEnumMtEph.rs` — 2 functions

These are set operations (union, intersect, difference, filter, to_seq, from_seq)
that delegate to sequential St implementations through the RwLock.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs`
2. `src/standards/hfscheduler_standard.rs`
3. `src/standards/arc_usage_standard.rs`

## Key context

Chap41 set operations are built ON TOP of the BST implementations from Chap38/39.
Chap38 `BSTParaMtEph` already has **parallel** union/intersect/difference/reduce
using `ParaPair!` (confirmed in R127 — agent 1 promoted their DIFFERS to "matches APAS").

So the question is: can the Chap41 Mt set operations delegate to the parallel BST
operations instead of the sequential St ones?

## Approach

1. Read all three files. Understand the delegation chain:
   - AVLTreeSetMtEph → acquires lock → calls StEph method → releases
   - The StEph methods may delegate to BST operations

2. For each DIFFERS function, check if there's a parallel counterpart available:
   - **union/intersect/difference**: Chap38 BSTParaMtEph has parallel versions.
     Can AVLTreeSet delegate to the parallel BST? Or does it need its own D&C?
   - **filter**: Same question — Chap38 has filter_parallel.
   - **to_seq**: In-order traversal. Could be parallelized with D&C on the tree
     (left subtree || right subtree, then concat). But with Vec backing, concat is O(n).
   - **from_seq**: Sequential insert loop. Could use D&C build (split seq, build both
     halves in parallel, merge). But BST merge/join is needed.

3. For parallelizable functions: rewrite the Mt method to use parallel delegation
   or D&C with `join()`.

4. For functions that remain sequential: update DIFFERS annotation with accurate reason.

## Validation

Run `scripts/validate.sh isolate Chap41` after changes. Fix verification errors.
Then run `scripts/rtt.sh`.

## Rules

- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Preserve all existing RTTs.
- If a function can't be parallelized, leave it sequential with an accurate DIFFERS annotation.

## When done

Commit with `git add -A && git commit` and push.

## Report

Write `plans/agent4-r128-report.md` with:
- Table: # | Chap | File | Function | Parallelized? | Old Span | New Span | Reason if not
- Verification count
