# Chap50 Spec Audit — Dynamic Programming Optimization (OBST, Matrix Chain)

## Summary

MatrixChain has **strong** specs on core DP functions (optimal_cost, multiply_cost, matrix_chain_rec). OBST has **no spec function** for optimal cost — critical gap.

## OptBinSearchTreeStEph.rs / OptBinSearchTreeStPer.rs

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | keys.len==0, memo empty | **weak** |
| 2 | from_keys_probs | keys.len==probs.len | keys.len==input, memo empty | **weak** |
| 3 | from_key_probs | — | keys==input, memo empty | **weak** |
| 4 | optimal_cost (Eph) | — | (none) | **missing** |
| 5 | optimal_cost (Per) | — | (none) | **missing** |
| 6 | keys | — | keys@ == self@.keys | **strong** |
| 7 | set_key_prob (Eph) | index < len | keys updated, memo cleared | **weak** |
| 8 | update_prob (Eph) | index < len | keys.len preserved, memo cleared | **weak** |
| 9 | num_keys | — | count == keys.len | **strong** |
| 10 | clear_memo (Eph) | — | keys preserved, memo cleared | **weak** |
| 11 | memo_size | — | count == memo.len | **weak** |

## MatrixChainStEph.rs / MatrixChainStPer.rs

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | dims.len==0, memo empty | **weak** |
| 2 | from_dimensions | — | dims match, memo empty | **weak** |
| 3 | from_dim_pairs | — | dims.len==input, memo empty | **weak** |
| 4 | optimal_cost | dims_bounded, costs_fit | cost == spec_chain_cost(..., 0) | **strong** |
| 5 | dimensions | — | dims match view | **weak** |
| 6 | num_matrices | — | n == dims.len | **weak** |
| 7 | set_dimension (Eph) | index < len | dims updated, memo cleared | **weak** |
| 8 | update_dimension (Eph) | index < len | dims updated, memo cleared | **weak** |
| 9 | clear_memo (Eph) | — | dims preserved, memo cleared | **weak** |
| 10 | memo_size | — | n == memo.len | **weak** |
| 11 | multiply_cost | bounds | cost == spec_multiply_cost | **strong** |
| 12 | matrix_chain_rec | bounds, costs_fit, memo_correct | cost == spec_chain_cost, memo_correct | **strong** |

## Critical Gap: OBST

OBST has no spec function defining optimal search cost. The textbook defines:
```
OBST(keys, probs, i, j) = min over r in [i,j] of:
  OBST(keys, probs, i, r-1) + OBST(keys, probs, r+1, j) + sum(probs[i..j])
```

Fix requires: (1) define `spec_obst_cost` recursive spec fn, (2) add ensures to `optimal_cost` and `obst_rec`, (3) prove connection. The Probability type uses f64 which complicates spec-level reasoning (no f64 arithmetic axioms in spec mode).

## Notes

- MatrixChain spec_chain_cost is well-defined and fully connected to exec code.
- OBST gap is architectural: Probability wraps f64, which has no spec arithmetic.
- Constructor/accessor weak specs are acceptable (structural operations).
