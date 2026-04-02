# R142 Agent 2 — Parallelize AVL from_seq, Update to_seq Annotations (Chap41)

## Summary

Parallelized `from_seq` in both `AVLTreeSetMtEph.rs` and `AVLTreeSetMtPer.rs` using
divide-and-conquer with `join()` + `union()`. Updated `to_seq` annotations to explain
why sequential is correct for Vec-backed output.

## Changes

### from_seq — Parallelized (2 files)

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | AVLTreeSetMtEph.rs | Added `from_vec_dc` free function; replaced sequential loop with parallel D&C |
| 2 | 41 | AVLTreeSetMtPer.rs | Added `from_vec_dc_per` free function; replaced sequential loop with parallel D&C |

**Algorithm**: Collect seq elements into Vec, then D&C: split Vec at midpoint, clone
elements into two half-Vecs, recurse in parallel via `join()`, union the two resulting
ParamBSTs. Base cases: empty → `ParamBST::new()`, singleton → `ParamBST::singleton()`.

**Cost**: Work O(n lg n), Span O(lg² n) — matches APAS Ex 41.3 parallel approach.
Previously: Work O(n lg n), Span O(n lg n) (sequential loop of inserts).

**Proof**: Uses `seq_to_set_distributes_over_add` to show union of left/right to_set
equals full to_set. Uses `lemma_cardinality_of_set` to bound tree sizes for union's
requires. Explicit triggers on all quantifiers.

### to_seq — Annotation Update Only (2 files)

| # | Chap | File | Change |
|---|------|------|--------|
| 3 | 41 | AVLTreeSetMtEph.rs | Updated annotation: "sequential; APAS O(lg n) span assumes tree-based sequence output, Vec output requires O(n) materialization" |
| 4 | 41 | AVLTreeSetMtPer.rs | Same annotation update |

**Rationale**: APAS Cost Spec 41.4 gives to_seq Work O(|a|), Span O(lg|a|). The O(lg n)
span assumes a tree-based sequence output where concat is O(lg n). Our Vec-backed output
requires O(n) materialization. D&C with Vec concat would increase work to O(n lg n)
while keeping span at O(n) — strictly worse than sequential O(n) work, O(n) span.
The sequential implementation is correct and optimal for our output type.

## DIFFERS Resolution

| # | Chap | File | Function | Old Status | New Status |
|---|------|------|----------|-----------|------------|
| 1 | 41 | AVLTreeSetMtEph.rs | to_seq | DIFFERS | Explained (Vec output limitation) |
| 2 | 41 | AVLTreeSetMtEph.rs | from_seq | DIFFERS | Fixed — parallel D&C |
| 3 | 41 | AVLTreeSetMtPer.rs | to_seq | DIFFERS | Explained (Vec output limitation) |
| 4 | 41 | AVLTreeSetMtPer.rs | from_seq | DIFFERS | Fixed — parallel D&C |

## Verification

- Full validate: 5679 verified, 0 errors
- RTT: 3690 passed
- PTT: 221 passed
- Zero trigger warnings in Chap41
