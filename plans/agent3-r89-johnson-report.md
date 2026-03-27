# Agent 3 — R89 Report: JohnsonMtEphI64 parallel_dijkstra_all

## Objective

Remove `#[verifier::external_body]` from `parallel_dijkstra_all` in
`src/Chap59/JohnsonMtEphI64.rs`.

## Result

**Done.** The external_body is removed. Chap59 has **0 holes**.

## What Was Done

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 59 | JohnsonMtEphI64.rs | Removed `#[verifier::external_body]` from `parallel_dijkstra_all` |
| 2 | 59 | JohnsonMtEphI64.rs | Added `adjust_distance` helper (i128 arithmetic, no overflow) |
| 3 | 59 | JohnsonMtEphI64.rs | Added `obeys_feq_clone` requires to `parallel_dijkstra_all`, closures, `johnson_apsp`, trait |
| 4 | 59 | JohnsonMtEphI64.rs | Fixed tabulate closure with explicit requires and `adjust_distance` call |

## Two Issues Resolved

### 1. feq_clone preconditions

`ArraySeqStEphS::singleton` and `append` require `obeys_feq_clone::<T>()`. The fix
threads `obeys_feq_clone::<ArraySeqStEphS<i64>>()` and
`obeys_feq_clone::<ArraySeqStEphS<usize>>()` through the requires chain:

- `parallel_dijkstra_all` requires them
- Both ParaPair! closures (f1, f2) require them
- `johnson_apsp` requires them (caller must provide)
- Trait signature updated with full requires/ensures

No `assume` added. The proof obligation flows to callers.

### 2. i64 overflow in distance adjustment

The original code computed `d_prime - p_u + p_v` directly as i64, which can overflow.
Added `adjust_distance` helper (mirrors the StEph version's pattern) that:
- Uses i128 intermediate arithmetic
- Clamps to i64::MAX / i64::MIN on overflow
- Preserves UNREACHABLE (i64::MAX) sentinel

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 59 | JohnsonMtEphI64.rs | 1 | 0 |
| 2 | 59 | JohnsonMtEphF64.rs | 0 | 0 |
| 3 | 59 | JohnsonStEphI64.rs | 0 | 0 |
| 4 | 59 | JohnsonStEphF64.rs | 0 | 0 |

## Verification

- Isolate: 2531 verified, 0 errors
- Full: 5266 verified, 0 errors (unchanged from baseline)
- Pre-existing E0373 error in AdjTableGraphMtPer.rs blocks RTT/PTT (not Chap59)

## Techniques Used

- **feq_clone threading**: Added `obeys_feq_clone` to requires chain without assume.
  Standard pattern per `constructor_feq_standard.rs`.
- **i128 arithmetic bridge**: Same pattern as `JohnsonStEphI64::adjust_distance`.
- **Explicit closure requires**: Added `obeys_feq_clone` to both ParaPair! closure
  requires so the recursive calls can satisfy their preconditions.

## Steps Used

1 of 20.
