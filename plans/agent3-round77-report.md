# Agent 3 — Round 77 Report

## Objective

Remove 15 `external_body` holes from BSTSetAVLMtEph, BSTSetRBMtEph, and
BSTSetSplayMtEph by applying the verified ParaPair named-closure pattern
(from BSTSetPlainMtEph) and FnMut requires propagation.

## Technique

The BSTSetPlainMtEph file (already verified by agent5 in R77) demonstrates
the pattern for all five functions:

1. **union/intersection/difference**: Replace `#[verifier::external_body]`
   with `#[verifier::exec_allows_no_decreases_clause]` and named closures
   with explicit `ensures r.spec_wf()`. The closure ensures propagate through
   `para_pair`'s verified spec. Also replaced `.unwrap()` with `match` to
   avoid unprovable `Option::is_some()` preconditions.

2. **filter/reduce**: Remove `external_body`, add `forall|t: &T| #[trigger]
   predicate.requires((t,))` (filter) and `forall|a: T, b: T| #[trigger]
   op.requires((a, b))` (reduce) to both trait requires and while loop
   invariants. The FnMut requires flow lets Verus verify the closure calls.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 37 | BSTSetAVLMtEph.rs | 5 | 0 | -5 |
| 2 | 37 | BSTSetRBMtEph.rs | 5 | 0 | -5 |
| 3 | 37 | BSTSetSplayMtEph.rs | 5 | 0 | -5 |

## Verification

- **Verified**: 4890 (was 4869, +21)
- **Errors**: 0
- **Warnings**: 0
- **RTT**: 2619 passed
- **PTT**: 157 passed

## Project totals

- **Holes**: 19 (was 34, -15)
- **Clean chapters**: 43 (was 43)
- **Dirty chapters**: 3 (was 3)

## Remaining holes in Chap37

8 holes remain (not in scope for this round):
- 2 × `assume()` algorithmic (iterator clone-preserves-value in BSTSplayMtEph)
- 6 × `external_body` (RwLock thread-spawn boundaries in underlying BST Mt modules)
