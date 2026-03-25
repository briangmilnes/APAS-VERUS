# R78 Agent 4 — PrimStEph float TotalOrder + TSPApproxStEph fix (Chap64/65, 2 holes)

## Objective

Address 2 holes: PrimStEph float TotalOrder (structural) and TSPApproxStEph
fn_missing_requires (trivial fix).

## Baseline

- 4898 verified, 0 errors, 0 warnings

## Holes

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 1 | 65 | PrimStEph.rs | 72,74 | total + cmp | assume + external_body | Float TotalOrder |
| 2 | 64 | TSPApproxStEph.rs | 84 | vec_contains_pair | fn_missing_requires | Needs annotation |

## Task 1: TSPApproxStEph vec_contains_pair (quick fix)

Read `vec_contains_pair` in `src/Chap64/TSPApproxStEph.rs` at line 84. Determine if it
has a real precondition or genuinely works on any input. If no precondition, add
`// veracity: no_requires`. If it needs a real requires, add it.

**IMPORTANT**: Do NOT add `requires true`. Read the function body first.

## Task 2: PrimStEph float TotalOrder

PrimStEph implements `TotalOrder` for `PQEntry` which wraps `WrappedF64` priority.
The 4 proof functions (reflexive, transitive, antisymmetric, total) all have assumes.

Agent 2 R77 found:
- `FloatTotalOrder` axioms require finiteness preconditions that `TotalOrder` trait doesn't carry
- PQEntry is a preorder (equal priority ≠ equal entry), so antisymmetric is semantically impossible
- Same pattern as String's TotalOrder impl

**Approaches to try**:
1. Check if `vstdplus/float.rs` has axioms for `WrappedF64` that could prove reflexive/
   transitive/total (antisymmetric is genuinely false for PQEntry).
2. Check if PQEntry's `le` spec can be tightened to make antisymmetric provable
   (e.g., tie-break on vertex when priorities equal).
3. If structurally impossible, document why and leave as-is. These are accepted holes
   in the float strategy.

## Key resources

- `src/Chap65/PrimStEph.rs` — Read fully
- `src/vstdplus/float.rs` — Float axioms
- `src/vstdplus/total_order.rs` — TotalOrder trait

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round78-report.md` with holes before/after (table with Chap column).
