# R104 Agent 1 Report: Close spec_key_unique_pairs_set

## Result: No Effect — `closed` does not restrict visibility within the defining module

## What Was Done

1. Changed `pub open spec fn spec_key_unique_pairs_set` to `pub closed spec fn` in
   `src/Chap43/OrderedTableStPer.rs:81`.
2. Ran `scripts/validate.sh isolate Chap43` — 2582 verified, 0 errors (30s).
3. Ran `scripts/validate.sh isolate Chap52` — 2819 verified, 0 errors (30s).
4. Ran full `scripts/validate.sh` — 5426 verified, 0 errors (136s).
5. Profiled Chap43 and Chap52 before and after.
6. Reverted the change — zero benefit observed.

## Profile Comparison

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Chap43 `spec_key_unique_pairs_set_174` total | 45,753 | 45,753 | 0 |
| Chap43 `lemma_sorted_keys_pairwise_distinct_246` total | 83,367 | 83,367 | 0 |
| Chap43 `no_duplicates_104` total | 83,547 | 83,547 | 0 |
| Chap52 `spec_key_unique_pairs_set_174` total | 45,804 | 45,804 | 0 |

Instantiation counts are **identical** before and after. Zero reduction.

## Validation Time Comparison

| Run | Elapsed |
|-----|---------|
| Full validate (before, run 1) | 133s |
| Full validate (before, run 2) | 133s |
| Full validate (before, run 3) | 137s |
| Full validate (after) | 136s |

Within noise. No measurable speedup.

## Why `closed` Had No Effect

**Verus's `closed` keyword does not restrict the definition axiom within the defining
module.** All 45,753 instantiations of `spec_key_unique_pairs_set_174` occur within
`OrderedTableStPer::OrderedTableStPer` — the same module where the spec fn is defined.

Evidence:
- `lemma_key_unique_remove` has an empty proof body with
  `requires spec_key_unique_pairs_set(s)` / `ensures spec_key_unique_pairs_set(s.remove(pair))`.
  This cannot verify without the forall body being visible to Z3.
  It verified with `closed` and no `reveal()`, confirming that `closed` does not hide
  the definition from Z3 within the same module.

- No external module imports `spec_key_unique_pairs_set` from OrderedTableStPer
  (AugOrderedTableStPer, OrderedTableMtPer, and AdjTableGraphMtPer do not reference it).
  OrderedTableStEph has its own independent copy (chapter standalone pattern).

Therefore `closed` on this spec fn has exactly zero callers to restrict.

## What Would Actually Reduce Instantiations

To reduce the 45K instantiations of this 3-variable forall, the approach would need to be
structural rather than visibility-based:

1. **Factor proof functions into a separate submodule** and make the spec fn `closed` there,
   with `reveal()` only in the proof functions that need the forall body. This would prevent
   the quantifier from leaking into exec functions like `difference` (37K), `subtract` (27K),
   and `restrict` (17K) that reference `spec_key_unique_pairs_set` in invariants but don't
   need the forall unfolded.

2. **Replace the 3-variable forall with a 2-variable formulation** using a helper:
   `forall|p1, p2| s.contains(p1) && s.contains(p2) && p1.0 == p2.0 ==> p1.1 == p2.1`.
   This reduces trigger variables from 3 to 2.

3. **Use an opaque wrapper predicate** at the exec level that wraps
   `spec_key_unique_pairs_set` and is only revealed in proof functions.

None of these were in scope for this task.

## Holes

No change. No holes added or removed.

## Verification Counts

- 5426 verified, 0 errors (unchanged from baseline)
- 3083 RTT passed (confirmed)
