# Agent 3 — Round 76 Report

## Objective

Eliminate remaining holes in Chap66 Boruvka files: BoruvkaMtEph.rs and BoruvkaStEph.rs.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 66 | BoruvkaMtEph.rs | 1 | 0 | -1 |
| 2 | 66 | BoruvkaStEph.rs | 2 | 0 | -2 |

**Total: 3 holes eliminated. Chap66 is now clean (0 actionable holes).**

## Techniques

### BoruvkaStEph.rs — vertex_bridges (-1 hole)

Removed `external_body` from `vertex_bridges`. The function already had a complete
loop body. Key fixes:
- Added `ensures s == *self` to `Clone for LabeledEdge<V>` so destructured clone
  results connect back to the original reference.
- Replaced the `map(...).to_set()` loop invariant with the simpler
  `forall|j| 0 <= j < iter_seq.len() ==> edges@.contains(iter_seq[j]@)` from
  the `iter()` ensures.
- Added `forall|k| bridges@.contains_key(k) ==> bridges@[k].1.spec_is_finite()`
  invariant to track weight finiteness through bridge updates.
- Dropped the `spec_valid_bridge` invariant (not needed for ensures).

### BoruvkaStEph.rs — boruvka_mst (-1 hole)

Removed `external_body`, added `#[verifier::exec_allows_no_decreases_clause]` for
the recursive call. Key fixes:
- Added `spec_all_weights_finite(new_edges@)` invariant to the edge re-routing loop.
- Proved weight finiteness preservation: new edges get weights from original edges
  (which have finite weights), so the forall is maintained through insert.
- Added intermediate assertions connecting iterator elements to `edges@` membership.

### BoruvkaMtEph.rs — vertex_bridges_mt assume (-1 hole)

Eliminated `assume(w.spec_is_finite() && existing_w.spec_is_finite())` in the merge
loop by propagating weight finiteness through the call chain:
- Added `spec_all_weights_finite_seq(edges@)` to `vertex_bridges_mt` requires.
- Added `forall|k| bridges@.contains_key(k) ==> bridges@[k].1.spec_is_finite()`
  to `vertex_bridges_mt` ensures.
- Propagated finiteness through closure requires/ensures for ParaPair! arms.
- Added finiteness invariant to the merge loop, proved from right_bridges ensures.
- Added same requires to `boruvka_mst_mt` and its loop invariant.
- Added finiteness requires/ensures to `reroute_edges_mt` and its closures/loop.
- Added finiteness tracking to `boruvka_mst_mt_with_seed` edge collection loop.

## Chapters Closed

| # | Chap | Status |
|---|------|--------|
| 1 | 66 | Clean (was 3 holes) |

## Validation

- **4829 verified, 0 errors** (was 4794 at prompt baseline)
- **2619 RTT passed**
- **157 PTT passed**
- **43 clean chapters** (up from 42)
- **62 total holes** (down from 76)

## Remaining Holes in Chap66

None. Both BoruvkaMtEph.rs and BoruvkaStEph.rs are clean.

Warnings (non-actionable):
- `assume_eq_clone_workaround` in BoruvkaMtEph.rs PartialEq (standard pattern)
- `fn_missing_ensures` on 7 internal helper functions (not holes)
- Structural false positives: `hash_coin` external_body, `eq` external_body (info only)
