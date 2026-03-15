# Chap51 Spec Audit — Implementing Dynamic Programming (Bottom-Up, Top-Down)

## Summary

Core DP functions have **strong** specs linking to spec_med. Accessors are structural (weak). Overall good.

## BottomUpDPStEph.rs / BottomUpDPStPer.rs

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | s, t set correctly | **weak** |
| 2 | s_length | — | len == spec_s_len() | **weak** |
| 3 | t_length | — | len == spec_t_len() | **weak** |
| 4 | is_empty | — | empty == (s_len==0 && t_len==0) | **weak** |
| 5 | set_s (Eph) | — | s updated, t preserved | **weak** |
| 6 | set_t (Eph) | — | t updated, s preserved | **weak** |
| 7 | med_bottom_up | sum < MAX | distance == spec_med(s_len, t_len) | **strong** |
| 8 | initialize_base_cases | bounds | table shape + base cases correct | **strong** |
| 9 | compute_cell_value | bounds, predecessors | val == spec_med(i, j) | **strong** |

## TopDownDPStEph.rs / TopDownDPStPer.rs

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | s, t set correctly | **weak** |
| 2 | s_length | — | len == spec_s_len() | **weak** |
| 3 | t_length | — | len == spec_t_len() | **weak** |
| 4 | is_empty | — | empty == (s_len==0 && t_len==0) | **weak** |
| 5 | memo_size | — | size == memo.len() | **weak** |
| 6 | is_memoized | — | memoized == memo.contains_key() | **weak** |
| 7 | get_memoized | — | val matches memo lookup | **weak** |
| 8 | insert_memo (Eph) | — | s/t preserved, memo updated | **weak** |
| 9 | clear_memo (Eph) | — | s/t preserved, memo cleared | **weak** |
| 10 | set_s (Eph) | — | s updated, memo cleared | **weak** |
| 11 | set_t (Eph) | — | t updated, memo cleared | **weak** |
| 12 | med_memoized | sum < MAX | distance == spec_med(s_len, t_len) | **strong** |
| 13 | med_recursive | bounds, memo_correct | distance == spec_med(i, j), memo_correct | **strong** |

## Notes

- Both DP implementations correctly link their core functions to spec_med.
- spec_med is the same recursive MED definition from Chap49 (imported or redefined).
- Bottom-up proves correctness cell-by-cell via diagonal computation.
- Top-down proves correctness via recursive memoization with spec_memo_correct invariant.
- Accessor weak specs are acceptable for container boilerplate.
