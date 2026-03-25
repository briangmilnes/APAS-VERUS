# Agent 5 — Round 76 Report

## Objective

Fold `obeys_feq_clone::<T>()` into `spec_bstsetavlmteph_wf()` in BSTSetAVLMtEph.rs,
eliminating 8 scattered assumes by consolidating them into 2 construction-point assumes.

## Changes

1. **Widened wf predicate** (`spec_bstsetavlmteph_wf`): added `&& obeys_feq_clone::<T>()`
   so every function requiring wf automatically gets `obeys_feq_clone`.

2. **Added `obeys_feq_clone::<T>()` to `values_vec` requires**: this free function takes
   `&BSTAVLMtEph<T>` (not `&Self`), so it doesn't get wf automatically.

3. **Added `obeys_feq_clone::<T>()` to `build_from_vec` requires**: needed to prove the
   new wf in its ensures. Removed its `// veracity: no_requires` annotation.

4. **Added `assume(obeys_feq_clone::<T>())` to `empty()` and `singleton()`**: these are
   the only two construction points where wf is ensured without a wf requires.

5. **Removed 8 assumes** from: `values_vec`, `delete`, `split`, `join_pair`, `join_m`,
   `filter`, `reduce`, `iter_in_order`.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 37 | BSTSetAVLMtEph.rs | 13 | 7 | -6 |

Remaining 7 holes:
- 2 assume (`obeys_feq_clone` in `empty`/`singleton` — construction points)
- 5 external_body (`union`, `intersection`, `difference`, `filter`, `reduce`)
- 1 accept (iterator clone-preserves-value, not counted as hole)

## Validation

- 4794 verified, 0 errors, 0 warnings
- 2619 RTT passed
- 157 PTT passed
