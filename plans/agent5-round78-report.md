# Agent 5 — Round 78 Report

## Objective

Prove 1 external_body hole in `spanning_tree_star_contraction` (Chap64/SpanTreeStEph.rs).

## Result: 1 hole closed, Chap64 now clean

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 64 | SpanTreeStEph.rs | 1 | 0 | -1 |

## Technique

The blocker was `star_contract`'s closure interface: it required `expand` to be a total
function (accept ANY inputs), but the expand closure needed `spec_setsteph_wf()` on its
set arguments.

**Two-part fix:**

1. **StarContractionStEph.rs — Ghost(r_inv) invariant parameter.** Added a ghost
   `spec_fn(R) -> bool` parameter to `star_contract` and `star_contract_fuel` that threads
   a result invariant through the recursion. The API now:
   - Guards the expand quantifier: expand only needs to accept wf inputs where r_inv holds
   - Requires base to produce r_inv-satisfying results
   - Requires expand to preserve r_inv
   - Ensures r_inv(result) on the output

   Existing callers (`contract_to_vertices`, `count_components_hof`,
   `connected_components_hof`) pass `Ghost(|r| true)` — trivially satisfied, no proof
   changes needed.

2. **SpanTreeStEph.rs — total closures via elements.iter().** Replaced `SetStEph::iter()`
   (which requires `spec_setsteph_wf()`) with `HashSetWithViewPlus::iter()` via the public
   `elements` field (no wf requirement). This makes the expand closure's requires purely
   type-level (`valid_key_type_Edge`, `obeys_key_model`), satisfying the universal
   quantifier trivially. The wf invariant is threaded via `Ghost(|r| r.spec_setsteph_wf())`.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarContractionStEph.rs | Added Ghost(r_inv) to star_contract API |
| 2 | 63 | ConnectivityStEph.rs | Updated 2 star_contract calls with Ghost(true) |
| 3 | 64 | SpanTreeStEph.rs | Removed external_body, total closures |
| 4 | — | TestStarContractionStEph.rs | Updated test call with Ghost |

## Verification

- 4899 verified, 0 errors, 0 warnings (+1 verified from removing external_body)
- 2774 RTT passed
- 157 PTT passed

## Global Status

- 14 holes (was 15)
- 44 clean chapters (was 43)
- Chap64 is now fully clean
