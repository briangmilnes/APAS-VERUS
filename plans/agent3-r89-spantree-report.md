# Agent 3 — R89 Report: SpanTreeMtEph (Chap64)

## Objective

Remove external_body from 2 functions in `src/Chap64/SpanTreeMtEph.rs`.

## Result

**1 of 2 holes removed.** Chap64: 2 holes → 1 hole.

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 64 | SpanTreeMtEph.rs | `verify_spanning_tree` | external_body | proved | Removed external_body |
| 2 | 64 | SpanTreeMtEph.rs | `spanning_tree_star_contraction_mt` | external_body | external_body | Blocked by Chap62 |

## What Was Done

### verify_spanning_tree — PROVED

Removed `#[verifier::external_body]`. The proof follows from:
- `sizeV()` ensures `n == self@.V.len()`
- `size()` ensures `count == self@.len()`
- The early-return on size mismatch makes the ensures trivially true (result=false)
- When returning true, `tree_edges.size() == expected_edges` gives the postcondition

Added:
- `#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]` before the for loop
- `graph_edges.spec_setsteph_wf()` and `valid_key_type_Edge::<V>()` to loop invariants
  (needed for `mem()` calls)

### spanning_tree_star_contraction_mt — BLOCKED

**Root cause:** `star_contract_mt` (Chap62) has a weaker contract than the StEph
version's `star_contract`. Specifically:

- **StEph `star_contract`** takes `Ghost(r_inv): Ghost<spec_fn(R) -> bool>` and
  ensures `r_inv(result)`. This allows propagating arbitrary result invariants
  (like `spec_setsteph_wf()`) through the recursion.

- **Mt `star_contract_mt`** lacks the `r_inv` parameter. Its ensures only covers
  the empty-graph base case: `graph@.A.is_empty() ==> exists|s| base.ensures(...)`.
  For non-empty graphs, it guarantees nothing about the result.

**Fix path:** Add `r_inv` ghost predicate to `star_contract_mt` and
`star_contract_mt_fuel` in `src/Chap62/StarContractionMtEph.rs`, mirroring the
StEph pattern. This is outside Chap64 scope.

**Improvements applied to the external_body function body:**
- Added explicit `requires`/`ensures` to both closures (matching StEph pattern)
- Changed `quotient_tree.iter()` → `quotient_tree.elements.iter()` (avoids
  needing `spec_setsteph_wf()` on closure arguments)
- Changed `original_edges.iter()` → `original_edges.elements.iter()` (same reason)
- Imported `HashSetWithViewPlusTrait` for `elements.iter()` access
- Added `#[cfg(verus_keep_ghost)]` guard on `obeys_key_model` import

## Verification

- Isolate Chap64: 1244 verified, 0 errors
- Full: 5296 verified, 0 errors in Chap64 (2 pre-existing flaky errors in Chap26/Chap42)
- Verified count +30 vs baseline (verify_spanning_tree body now verified)

## Steps Used

2 of 20.
