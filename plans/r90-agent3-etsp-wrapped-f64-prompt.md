# R90 Agent 3 — ETSPMtEph find_best_swap: refactor to WrappedF64, STEP 20

## Objective

Remove 2 external_body holes from ETSPMtEph.rs by refactoring raw f64 arithmetic
to use `WrappedF64` from `vstdplus/float.rs`. The WrappedF64 type has verified
exec bridges for comparison and arithmetic with specs through broadcast axioms.

## The 2 Holes

1. `find_best_swap_par` (line 687) — root cause. Recursive ParaPair! divide-and-conquer
   that finds the best edge swap. Uses `f64::MAX`, `+`, `<` on distances.
2. `find_best_swap_impl` (line 670) — downstream, calls find_best_swap_par.

## Strategy: Replace raw f64 with WrappedF64

The functions compute point distances and compare them:
```rust
let dist = point_distance(e1.to, e2.from) + point_distance(e2.to, e1.from);
if dist < best_dist { ... }
```

`WrappedF64` provides:
- `dist_add(&self, &other) -> WrappedF64` with `ensures r@ == f64_add_spec(self@, other@)`
- `dist_lt(&self, &other) -> bool` with `ensures b == (self.val.le(other.val) && self.val != other.val)`
- `dist_le(&self, &other) -> bool` with `ensures b == self.val.le(other.val)`
- `unreachable_dist() -> WrappedF64` with `ensures d@ == UNREACHABLE_SPEC(), !d.spec_is_finite()`

Replace:
- `f64::MAX` → `unreachable_dist()` or a large finite sentinel
- `a + b` → `a.dist_add(&b)`
- `a < b` → `a.dist_lt(&b)`
- `point_distance(p1, p2)` → should return `WrappedF64` instead of raw f64

## Read first

- `src/Chap26/ETSPMtEph.rs` — your file
- `src/Chap26/ETSPStEph.rs` — proved StEph version (uses same point_distance)
- `src/vstdplus/float.rs` — WrappedF64 API, broadcast axioms, f64_add_spec etc.
- `src/Chap65/PrimStEph.rs` — example of integerized algorithm (for pattern reference)

## Key: point_distance

`point_distance` computes `sqrt(dx² + dy²)`. Check if it already returns WrappedF64
or raw f64. If raw f64, you may need to wrap the result. The function likely has
`#[verifier::external_body]` — that's fine, just ensure its ensures uses WrappedF64 specs.

Do NOT try to verify the sqrt computation — keep external_body on point_distance.
The goal is to verify the COMPARISON and SELECTION logic in find_best_swap, not
the distance computation.

## What success looks like

The find_best_swap functions should verify with:
- WrappedF64 comparisons (dist_lt/dist_le) providing the spec
- broadcast axioms (FloatTotalOrder) providing transitivity for "if a < b and b < c then a < c"
- external_body remaining ONLY on point_distance (the sqrt computation)

## Isolation

```bash
scripts/validate.sh isolate Chap26
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify vstdplus/float.rs.
- Do NOT modify ETSPStEph.rs.
- Do NOT add assume or accept.
- If the WrappedF64 refactor cascades through too many functions, focus on
  find_best_swap_par first. find_best_swap_impl should follow.
- Use `broadcast use crate::vstdplus::float::float::group_float_finite_total_order;`
  and `broadcast use crate::vstdplus::float::float::group_float_arithmetic;` to
  bring in the axioms.

## STEP 20

## Report

Write `plans/agent3-r90-etsp-report.md`.
