# Agent 2 — Round 165 Report

## Task

R165 Prompt B: Extract shared spec functions and proof lemmas from Chap39's BST treap
files into `src/Chap39/BSTTreapSpecsAndLemmas.rs`. Follow the pattern from
`src/Chap42/TableSpecsAndLemmas.rs`.

## Extracted Functions

| # | Kind | Name | Shared from |
|---|------|------|-------------|
| 1 | spec fn | `view_ord_consistent` | StEph (`_st`), ParaTreapMtEph |
| 2 | proof fn | `lemma_cmp_antisymmetry` | StEph (`_st`), ParaTreapMtEph |
| 3 | proof fn | `lemma_cmp_antisymmetry_less` | StEph (`_st`), ParaTreapMtEph |
| 4 | proof fn | `lemma_cmp_transitivity` | StEph (`_st`), ParaTreapMtEph |
| 5 | proof fn | `lemma_cmp_eq_subst` | StEph (`_st`), ParaTreapMtEph |
| 6 | proof fn | `lemma_cmp_equal_congruent` | StEph (`_st`), ParaTreapMtEph |
| 7 | proof fn | `lemma_cmp_equal_congruent_right` | StEph (`_st`), ParaTreapMtEph |
| 8 | proof fn | `lemma_joined_right_gt_lk` | StEph (`_st`), ParaTreapMtEph |
| 9 | proof fn | `lemma_joined_left_lt_rk` | StEph (`_st`), ParaTreapMtEph |

All 9 functions use `T: View + Ord` — the minimal common bound across `StT + Ord + IsLtTransitive`
and `MtKey`.

## Files Changed

| # | Chap | File | Lines Before | Lines After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 39 | BSTTreapSpecsAndLemmas.rs | 0 (new) | 186 | +186 |
| 2 | 39 | BSTTreapStEph.rs | 3249 | 3125 | −124 |
| 3 | 39 | BSTParaTreapMtEph.rs | 1942 | 1784 | −158 |
| 4 | — | lib.rs | — | — | +1 line |

**Net: −96 lines** across Chap39.

## Design Notes

- **Canonical names** (no `_st` suffix) in the shared module — match ParaTreapMtEph naming.
- **StEph aliases** via `#[cfg(verus_keep_ghost)] use X as X_st` for each of the 9 functions.
  The `#[cfg(verus_keep_ghost)]` is required because proof/spec fns are ghost-only and do
  not exist in regular cargo mode; Verus resolves them during its compilation pass.
- **ParaTreapMtEph** uses `pub use ...::*` (not just `use`) so that `BSTSetTreapMtEph`
  continues to pick up `view_ord_consistent`, `lemma_cmp_antisymmetry`, and
  `lemma_cmp_transitivity` via its existing glob import of BSTParaTreapMtEph.
- **BSTTreapMtEph** was not changed — its proof fns are BST-structural and reference the
  local `Node<T>` type; they are not candidates for extraction.

## Validation Results

- Verus isolate Chap39: **1299 verified, 0 errors**
- RTT: **3776 passed, 0 skipped**
- PTT: **221 passed, 0 skipped**
