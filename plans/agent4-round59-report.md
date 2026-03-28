# Agent 4 Round 59 Report — Stabilize Flaky Z3 Proofs

## Summary

Added existential re-assertions to two StarContraction wrapper functions to stabilize
Z3's propagation of existential postconditions from callee to caller. Target 3
(OrderedSetStPer `get_range`) does not exist as described — there is no loop with
`size as nat == self@.len()` invariant in that function.

## Fixes Applied

### Target 1: Chap62 StarContractionStEph.rs — `star_contract` (line 186)

**Problem:** Z3 flaked propagating the existential ensures from `star_contract_fuel` to
`star_contract`. The callee's antecedent is `(graph@.A.is_empty() || fuel == 0)` but
the caller only needs `graph@.A.is_empty()`.

**Fix:** Added explicit existential re-assertion after the disjunction assertion:
```rust
proof {
    if graph@.A.is_empty() {
        assert(graph@.A.is_empty() || fuel == 0);
        assert(exists|s: &SetStEph<V>| s@ == graph@.V
            && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), result));
    }
}
```

### Target 2: Chap62 StarContractionMtEph.rs — `star_contract_mt` (line 206)

**Problem:** Identical pattern — Z3 flaked on the same existential propagation.

**Fix:** Same existential re-assertion pattern as Target 1.

### Target 3: Chap43 OrderedSetStPer.rs — NOT APPLICABLE

The described pattern (loop with `size as nat == self@.len()` invariant in `get_range`)
does not exist. `get_range` delegates to `get_range_iter`, which uses tree `split`
operations without loops. No changes made.

## Validation Results

| # | Mode | Result | RSS |
|---|------|--------|-----|
| 1 | full | 5386 verified, 0 errors | 9.6 GB |
| 2 | full | OOM killed (memory pressure) | 19.3 GB |
| 3 | full | 5386 verified, 0 errors | 12.2 GB |
| 4 | isolate Chap62 | 1231 verified, 0 errors | 11.2 GB |
| 5 | isolate Chap62 | 1231 verified, 0 errors | 11.2 GB |
| 6 | isolate Chap62 | 1231 verified, 0 errors | 11.2 GB |
| 7 | full | 5386 verified, 0 errors | 20.4 GB |
| 8 | full | 5386 verified, 0 errors | 9.6 GB |

**3 successful full validations, 3 successful isolated validations.** The one OOM kill
was system memory contention (19.3 GB RSS), not a verification failure.

**Verification count:** 5386 verified, 0 errors.

## Hole Count Impact

Zero. No assumes, accepts, or external_body added or removed.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarContractionStEph.rs | Existential re-assertion in `star_contract` proof block |
| 2 | 62 | StarContractionMtEph.rs | Existential re-assertion in `star_contract_mt` proof block |
