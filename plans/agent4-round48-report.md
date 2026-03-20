# Agent 4 Round 48 Report

## Assignment
Chap26 ETSP (ETSPMtEph.rs) — prove float distance holes.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 26 | ETSPMtEph.rs | 4 | 2 | -2 |

- **Verified**: 4419 (unchanged count — removed 2 external_body, no new fns)
- **RTT**: 2613 pass
- **Warnings**: 0

## What Was Done

### 1. Proved `distance` by adding `spec_point_distance` (-1 hole)

- Added uninterpreted `spec_point_distance(a: Point, b: Point) -> f64` spec fn
- Added `ensures d == spec_point_distance(*a, *b)` to `point_distance` (kept `external_body` — f64 arithmetic in body)
- Added matching ensures to `ETSPPointTrait::distance` trait method
- Removed `external_body` from `distance` impl — body is trivial delegation to `point_distance`

### 2. Proved `find_best_swap_impl` through Arc/Vec specs (-1 hole)

- Added requires/ensures to `find_best_swap_par` (kept `external_body`):
  - `requires (*left_tour)@.len() >= 1, (*right_tour)@.len() >= 1, hi <= (*left_tour)@.len()`
  - `ensures result.0 < (*left_tour)@.len(), result.1 < (*right_tour)@.len()`
- Removed `external_body` from `find_best_swap_impl` — body is:
  1. `left_tour.clone()` — Vec::clone with Edge::clone (`ensures cloned == *self`) gives view equality
  2. `Arc::new(cloned_vec)` — vstd spec `ensures v == t` connects Arc inner to cloned vec
  3. `find_best_swap_par(lt, rt, 0, left_tour.len())` — requires satisfied via the equality chain
  4. Index bounds propagate from ensures through the chain back to original vec lengths
- Z3 handled the full chain (Vec::clone + Arc::new + external_body ensures) without proof hints

## Remaining Holes (2)

| # | Chap | File | Line | Function | Blocker |
|---|------|------|------|----------|---------|
| 1 | 26 | ETSPMtEph.rs | 611 | `point_distance` | f64 arithmetic: `-`, `*`, `.sqrt()` not verifiable in Verus |
| 2 | 26 | ETSPMtEph.rs | 670 | `find_best_swap_par` | f64 comparison, `f64::MAX`, recursive parallel structure |

Both are irreducible without f64 arithmetic verification support in Verus. `point_distance` would need `f64_mul_spec` and `f64_sqrt_spec` axioms (neither exists). `find_best_swap_par` additionally needs f64 comparison specs and `f64::MAX` handling.

## Techniques Used

- **Uninterpreted spec function bridge**: `spec_point_distance` gives `point_distance` a verifiable ensures without reasoning about f64 arithmetic
- **Vec::clone + Arc::new spec chain**: Edge's Copy/Clone (`ensures cloned == *self`) propagates through vstd's Vec::clone (per-element `cloned` + extensional equality) and Arc::new (`ensures v == t`) to establish `(*arc)@.len() == original@.len()`
- **External_body with tight ensures**: Adding real requires/ensures to `find_best_swap_par` (keeping external_body) enabled proving the caller

## Global Impact

- Baseline: 38 holes total, 4419 verified
- After: 36 holes total, 4419 verified (-2 holes)
