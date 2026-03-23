# Agent 4 Round 59 Report — Stabilize Flaky Z3 Proofs

## Summary

Fixed 3 flaky Z3 proofs that caused nondeterministic `4484 verified, 1 errors` results.
All fixes add intermediate assertions to reduce Z3 search; no specs weakened, no holes
added or removed.

## Fixes Applied

### Target 1: Chap62 StarContractionStEph.rs — `star_contract` (line 144)

**Problem:** Z3 flaked propagating the existential ensures from `star_contract_fuel` to
`star_contract`. The callee's antecedent is `(graph@.A.is_empty() || fuel == 0)` but
the caller only needs `graph@.A.is_empty()`.

**Fix:** Bound fuel to a variable, then added proof block asserting the disjunction
holds when `graph@.A.is_empty()`:
```rust
let fuel = graph.sizeV();
let result = star_contract_fuel(graph, base, expand, fuel);
proof {
    if graph@.A.is_empty() {
        assert(graph@.A.is_empty() || fuel == 0);
    }
}
result
```

### Target 2: Chap62 StarContractionMtEph.rs — `star_contract_mt` (line 163)

**Problem:** Identical pattern — Z3 flaked on the same existential propagation.

**Fix:** Same pattern as Target 1:
```rust
let fuel = graph.sizeV();
let result = star_contract_mt_fuel(graph, seed, base, expand, fuel);
proof {
    if graph@.A.is_empty() {
        assert(graph@.A.is_empty() || fuel == 0);
    }
}
result
```

### Target 3: Chap43 OrderedSetStPer.rs — `get_range` (line 904)

**Problem:** Z3 flaked maintaining `size as nat == self@.len()` loop invariant after
`result.insert(v)`. The bridge requires knowing `elements@.no_duplicates()` implies
`elements@.len() == elements@.to_set().len()`.

**Fix (two locations):**
1. Pre-loop: Added `elements@.unique_seq_to_set()` lemma call to anchor the
   `size == self@.len()` fact before loop entry (line 904).
2. In-loop: Added `assert(size as nat == self@.len())` reassertion after `result.insert(v)`
   proof block (line 953) to prevent Z3 from losing the immutability fact.

### OrderedSetStEph.rs — Not affected

The StEph version's `get_range` uses a different structure (`n as nat == self.base_set.elements@.len()`)
and does not have the `size as nat == self@.len()` invariant. No fix needed.

## Validation Results

| # | Run | Verified | Errors | Time |
|---|-----|----------|--------|------|
| 1 | validate run 1 | 4485 | 0 | 90s |
| 2 | validate run 2 | 4485 | 0 | 84s |
| 3 | validate run 3 | 4485 | 0 | 95s |

**Verification count:** 4485 (up from 4484 baseline due to new intermediate assertions).

**RTT:** 2610 passed, 0 failed.

## Hole Count Impact

Zero. No assumes, accepts, or external_body added or removed.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarContractionStEph.rs | Proof block in `star_contract` |
| 2 | 62 | StarContractionMtEph.rs | Proof block in `star_contract_mt` |
| 3 | 43 | OrderedSetStPer.rs | Pre-loop lemma + in-loop reassertion in `get_range` |
