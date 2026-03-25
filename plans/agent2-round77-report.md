# Agent 2 — Round 77 Report

## Summary

- **Holes**: 34 → 33 (−1)
- **Verified**: 4869 → 4871 (+2)
- **RTT**: 2619 passed
- **PTT**: 157 passed

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 65 | UnionFindStEph.rs | 5 | 4 | −1 |
| 2 | 65 | KruskalStEph.rs | 1 | 1 | 0 |
| 3 | 65 | PrimStEph.rs | 2 | 2 | 0 |

## Changes Made

### UnionFindStEph.rs (Chap65) — 1 hole eliminated

- **Proved `insert`**: Removed `external_body`. Key techniques:
  - **Frame lemma approach**: One `assert forall` establishes that for all existing keys w ≠ v@, parent/rank/roots values are unchanged AND old roots map to values ≠ v@. This single assertion covers the "unchanged" case for all 15 wf conjuncts at once.
  - **3-clone pattern**: Bind 3 clones to named variables, use `lemma_three_clones_eq` to establish equalities. Use the original `v` directly for `elements.push(v)` (avoiding a 4th clone whose equality is hard to prove in isolated `by` blocks).
  - **Proof function isolation**: Extract the wf preservation proof into a standalone `proof fn lemma_insert_preserves_wf` with `rlimit(50)`, keeping the exec `insert` body simple.
  - **Strictly_cloned broadcast limitation**: `assert(v4 == v)` fails inside `by` blocks because the `strictly_cloned(v, v4)` fact from clone calls is not available in isolated proof contexts. Solved by using 3 clones + original instead of 4 clones.

## What Blocked Further Progress

### UnionFind `find` (Chap65)
Path-following with path compression requires:
1. Loop decreases measure based on rank ordering (each parent step increases rank)
2. Loop invariant establishing current node is in domain and on the path to root
3. Second loop (path compression) must prove wf preservation after each parent pointer update
4. Each loop iteration involves `self.parent.get().unwrap().clone()` with equality chains

The proof infrastructure is now in place (wf predicate, frame lemma pattern), but the loop invariants and decreases measures are substantial work.

### UnionFind `union` (Chap65)
Depends on `find` being proved. Also needs to prove that rank-based union preserves the root mapping and rank ordering — requires showing new root is correctly propagated through the roots ghost map.

### UnionFind `equals`, `num_sets` (Chap65)
Both call `find` internally. Blocked by `find`.

### KruskalStEph `kruskal_mst` (Chap65)
Calls `insert` (now proved), `equals`, and `union`. Blocked by `find`/`union`/`equals`.

### PrimStEph TotalOrder holes (Chap65)
Structural float limitations:
- `FloatTotalOrder` axioms require finiteness preconditions that `TotalOrder` trait doesn't carry
- PQEntry is a preorder (equal priority ≠ equal entry), so antisymmetric is semantically impossible
- Same pattern as String's TotalOrder impl in total_order.rs
- Cannot be proved without changing TotalOrder trait signature or PQEntry equality semantics

## Techniques Used

- **Frame lemma pattern**: Assert one quantified fact covering all old keys, proving preservation of all fields at once. Dramatically reduces Z3 budget vs asserting each wf conjunct individually.
- **3-clone + original pattern**: Avoid the 4th clone by using the original value for one operation, sidestepping `strictly_cloned` broadcast isolation in `by` blocks.
- **Proof function isolation**: Extract complex wf proofs into standalone `proof fn` with `#[verifier::rlimit(50)]`, keeping exec function bodies clean and fast.
