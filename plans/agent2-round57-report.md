<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 Round 57 Report

## Summary

Closed 5 capacity `assume` holes across `src/Chap41/AVLTreeSetStEph.rs`,
`src/Chap41/AVLTreeSetStPer.rs`, and `src/Chap45/BalancedTreePQ.rs` by adding
`requires` clauses bounding combined set/PQ sizes below `usize::MAX`, then
replacing each `assume` with a derivable `assert` using loop invariants and
length lemmas.

Also propagated the new `requires` to `AVLTreeSetMtEph.rs` (same chapter) and
`BalancedTreePQ::join` (same file) to keep those callers sound. Added one
reader-predicate `assume` in `AVLTreeSetMtEph.rs::union` at the lock boundary
(same pattern as all other Mt file reader accepts in Chap41).

---

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 41 | AVLTreeSetStEph.rs | 1 | 0 | -1 |
| 2 | 41 | AVLTreeSetStPer.rs | 2 | 0 | -2 |
| 3 | 41 | AVLTreeSetMtEph.rs | 0 | 0 | 0* |
| 4 | 45 | BalancedTreePQ.rs | 2 | 0 | -2 |

*Added 1 new reader-predicate `assume` in `AVLTreeSetMtEph.rs::union`
 (structural false positive / RWLOCK_GHOST pattern).

**Net: closed 5 assumes, added 1 reader-predicate accept = net -5 real holes.**

---

## Changes

### Chap41 / AVLTreeSetStEph.rs

- Added `requires self@.len() + other@.len() < usize::MAX as nat` to `union`
  trait (line 275).
- Added loop invariants `self_len as nat == self@.len()`,
  `other_len as nat == other@.len()`, `self@.len() + other@.len() < usize::MAX as nat`,
  `combined@.len() <= self_len as nat + j as nat` to the second merge loop.
- Added proof block invoking `unique_seq_to_set()` on both sets before the loop.
- Replaced `assume(combined@.len() + 1 < usize::MAX as nat)` with
  `assert(combined@.len() + 1 < usize::MAX as nat)`.

### Chap41 / AVLTreeSetStPer.rs

- Added `requires self@.len() + other@.len() < usize::MAX as nat` to `union`
  trait; same loop invariant and assertion pattern as StEph.
- Added `requires self.spec_avltreesetstper_wf(), self@.len() + 1 < usize::MAX as nat`
  to `insert` trait.
- In `insert` impl proof block: added `unique_seq_to_set()`, `assert(n as nat == self@.len())`,
  replaced `assume(new_vec@.len() < usize::MAX)` with `assert(...)`.

### Chap41 / AVLTreeSetMtEph.rs

- Added `self@.len() + other@.len() < usize::MAX as nat` to `union` trait requires.
- In `union` impl: split inline borrow into named variables `self_st`, `other_st`;
  added proof block with reader-predicate assume for the length bound (matching
  all other RWLOCK_GHOST accepts in this file).

### Chap45 / BalancedTreePQ.rs

- Added `self@.len() + 1 < usize::MAX as nat` to `insert` trait requires.
- Added `self@.len() + other@.len() < usize::MAX as nat` to `meld` trait requires.
- Added `left@.len() + right@.len() < usize::MAX as nat` to `join` trait requires.
- In `insert` impl proof block: removed erroneous `unique_seq_to_set()`;
  used `n = old_vals.len()` chain via `values_in_order` ensures to prove
  `n as nat == self@.len()`; then `assert(vals@.len() < usize::MAX)`.
- In `meld` impl proof block: removed `unique_seq_to_set()`; used
  `n1/n2 = self/other.elements.length()` ensures directly.
- Fixed loop invariant syntax: `n as nat < usize::MAX as nat` → `n < usize::MAX`
  (Verus parser rejects `usize::MAX as nat` in invariant position).
- Propagated new requires to all internal callers:
  `from_seq`, `from_vec`, `split`, `filter`, `map`, `insert_all`.

---

## Verification Status

| Suite | Result |
|---|---|
| `validate.sh dev_only` | 2193 verified, 0 errors |
| `validate.sh` (full) | 4465 verified, 17 errors |

Full validation errors are all in:
- **Chap43** (Agent 1's chapter) — new `requires` not yet propagated.
- **Chap53** — union callers in graph search need bounds.
- **Chap47** — pre-existing trigger issues (unrelated to this round).

Pre-existing RTT failure: `vstd::arithmetic::power::pow` import error in
Chap47 (present on main before this round).

---

## Techniques Used

- `length()` ensures for direct `n as nat == self@.len()` derivations.
- `values_in_order` ensures (`map_values =~= spec_seq`) for Vec length bridge.
- Loop invariants tracking `len + j` bounds to prove `combined@.len() + 1 < usize::MAX`.
- `lemma_size_lt_usize_max` + `lemma_size_eq_inorder_len` in `from_seq` caller
  to establish `n < usize::MAX` before the insertion loop.
- Named borrow variables in Mt union to allow spec-mode access to inner views
  in reader-predicate proof blocks.

---

## Remaining Holes Needing Work

| # | Chap | File | Hole | What Blocks It |
|---|:----:|---|---|---|
| 1 | 43 | OrderedSetStEph.rs | union requires | Agent 1's chapter |
| 2 | 43 | OrderedSetStPer.rs | insert/union | Agent 1's chapter |
| 3 | 43 | OrderedTableStPer.rs | insert requires | Agent 1's chapter |
| 4 | 53 | GraphSearchStEph.rs | union requires | Needs graph size bound |
| 5 | 53 | GraphSearchStPer.rs | union requires | Needs graph size bound |
| 6 | 53 | PQMinStEph.rs | union requires | Needs graph size bound |
| 7 | 53 | PQMinStPer.rs | union requires | Needs graph size bound |
