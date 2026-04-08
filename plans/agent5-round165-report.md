# Agent 5 — Round 165 Report

## Task
R165 Prompt E: Extract shared specs and lemmas into SpecsAndLemmas files for Chap27 and Chap51.

## Work Performed

### Chap27 — ContractSpecsAndLemmas.rs

Created `src/Chap27/ContractSpecsAndLemmas.rs` (+196 lines) with 8 public proof functions
shared across all four contract/scan algorithm variants:

| # | Function | Description |
|---|----------|-------------|
| 1 | `lemma_fold_left_monoid` | fold_left(s, x, f) == f(x, fold_left(s, id, f)) under monoid |
| 2 | `lemma_fold_left_pair` | fold_left([a,b], id, f) == f(a, b) |
| 3 | `lemma_fold_left_singleton` | fold_left([a], id, f) == a |
| 4 | `lemma_contraction_even` | even-length fold equals fold of pairwise-contracted seq |
| 5 | `lemma_prefix_contraction` | fold of even prefix equals fold of contracted prefix |
| 6 | `lemma_expand_even` | b_seq.take(j) fold == s.take(2j) fold |
| 7 | `lemma_expand_odd` | f(fold(take(2j)), s[2j]) == fold(take(2j+1)) |
| 8 | `lemma_expand_odd_tail` | last-element step for odd n |

Removed local copies from all 4 Chap27 files and added import.

### Chap51 — SeqSpecsAndLemmas.rs

Created `src/Chap51/SeqSpecsAndLemmas.rs` (+76 lines) with 4 definitions shared across
all eight DP variant files:

| # | Function | Used by |
|---|----------|---------|
| 1 | `spec_min` | all 8 files |
| 2 | `spec_med_fn` | 4 TopDown files |
| 3 | `spec_memo_correct` | 2 Mt TopDown files |
| 4 | `lemma_spec_med_fn_bounded` | 4 TopDown files |

Removed local copies from all 8 Chap51 files and added import.

### lib.rs

Added `ContractSpecsAndLemmas` as first entry in Chap27 block.
Added `SeqSpecsAndLemmas` as first entry in Chap51 block.

## Line Counts

### Chap27

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 27 | ContractSpecsAndLemmas.rs | 0 | 196 | +196 |
| 2 | 27 | ReduceContractStEph.rs | 290 | 183 | -107 |
| 3 | 27 | ScanContractStEph.rs | 448 | 285 | -163 |
| 4 | 27 | ReduceContractMtEph.rs | 407 | 319 | -88 |
| 5 | 27 | ScanContractMtEph.rs | 430 | 267 | -163 |
| — | — | **Chap27 net** | **1575** | **1250** | **-325** |

### Chap51

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 51 | SeqSpecsAndLemmas.rs | 0 | 76 | +76 |
| 2 | 51 | BottomUpDPStEph.rs | 533 | 529 | -4 |
| 3 | 51 | BottomUpDPStPer.rs | 497 | 493 | -4 |
| 4 | 51 | BottomUpDPMtEph.rs | 514 | 510 | -4 |
| 5 | 51 | BottomUpDPMtPer.rs | 485 | 481 | -4 |
| 6 | 51 | TopDownDPStEph.rs | 451 | 418 | -33 |
| 7 | 51 | TopDownDPStPer.rs | 415 | 382 | -33 |
| 8 | 51 | TopDownDPMtEph.rs | 585 | 538 | -47 |
| 9 | 51 | TopDownDPMtPer.rs | 546 | 499 | -47 |
| — | — | **Chap51 net** | **4026** | **3926** | **-100** |

**Total net: -425 lines across Chap27 + Chap51.**

## Verification

```
Chap27 isolate: 929 verified, 0 errors (9s)
Chap51 isolate: 1426 verified, 0 errors (19s)
```

## Holes

No new holes introduced. No holes closed (extraction only).
