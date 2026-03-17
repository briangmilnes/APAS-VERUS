# Agent 2 — Round 33 Report

## Assignment

Chap43 trivial fixes + OrderedSetStEph delegation wrapper pilot.

## Results Summary

| # | Chap | File | Change | Result |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableMtPer.rs | Add `s.spec_orderedtablemtper_wf()` ensures to `from_st_table` | Done — trivial, wf is just `self@.dom().finite()` which was already ensured |
| 2 | 43 | OrderedTableMtEph.rs | Add `constructed.spec_orderedtablemteph_wf()` ensures to `from_sorted_entries` | BLOCKED — wf includes `spec_keys_no_dups` which `TableMtEph::from_sorted_entries` doesn't establish |
| 3 | 43 | OrderedSetStEph.rs | Remove `requires true` from `from_sorted_elements` | Done |
| 4 | 43 | OrderedSetStPer.rs | Remove `requires true` from `from_sorted_elements` | Done |
| 5 | 43 | AugOrderedTableStPer.rs | Remove `requires true` from `calculate_reduction` | Done |
| 6 | 43 | AugOrderedTableMtEph.rs | Remove `requires true` from `recalculate_reduction` | Done |
| 7 | 43 | OrderedSetStEph.rs | Remove `external_body` from delegation wrappers | BLOCKED — see analysis below |

## Verification Counts

- Verus: 4147 verified, 0 errors
- RTT: 2613 passed, 0 skipped
- PTT: 147 passed, 0 skipped

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableMtPer.rs | 8 | 8 | 0 |
| 2 | 43 | OrderedTableMtEph.rs | 7 | 7 | 0 |
| 3 | 43 | OrderedSetStEph.rs | 11 | 11 | 0 |
| 4 | 43 | OrderedSetStPer.rs | 9 | 9 | 0 |
| 5 | 43 | AugOrderedTableStPer.rs | 8 | 8 | 0 |
| 6 | 43 | AugOrderedTableMtEph.rs | 8 | 8 | 0 |

No external_body or assume holes were added or removed. Changes were to
fn_missing_wf_ensures and requires_true warnings only.

### Warning Changes

- **Eliminated:** 4 `requires_true` warnings (OrderedSetStEph, OrderedSetStPer, AugOrderedTableStPer, AugOrderedTableMtEph)
- **Eliminated:** 1 `fn_missing_wf_ensures` error (OrderedTableMtPer `from_st_table`)
- **Remaining:** 4 `fn_missing_requires` on functions that genuinely have no precondition (for user to annotate with `// veracity: no_requires` if appropriate)

## Task 2 Analysis: OrderedSetStEph External Body Removal

### What was expected

The task described OrderedSetStEph's functions as simple delegation wrappers
around AVLTreeSetStEph that could have external_body removed because "the base
method's ensures clause directly satisfies the wrapper's ensures clause."

### What was found

This description does not match the code. The simple delegations (size, find,
insert, delete, filter, intersection, union, difference, join) are **already
proved** — they don't have external_body. The remaining 11 external_body
functions fall into two categories:

**Ordering operations** (first, last, previous, next, rank, select): These
implement ordering from scratch by accessing the sorted backing sequence
(`self.base_set.elements`). AVLTreeSetStEph has NO ordering operations and
NO sorting spec. Its `spec_avltreesetsteph_wf` only checks structural
metadata (heights, cached sizes, no_duplicates, finiteness). The sorted
ordering is maintained by insert/delete but never expressed in specs. Without
a `sorted(elements@)` predicate, ordering properties are unprovable.

**Structural operations** (from_seq, split, get_range, split_rank): These
involve StPer-to-StEph sequence conversion (clone bridging) and/or complex
partitioning loops. The existing `to_seq` function uses `accept()` for the
clone/view bridge — the same issue would arise for `from_seq`. The StPer
versions of `get_range` and `split_rank` are proved with extensive loop
invariants and clone lemmas (`lemma_cloned_view_eq`, `obeys_feq_full_trigger`)
that rely on `clone_plus` — the StEph versions use regular `Clone` which
lacks these proof bridges.

### What would unblock this

1. **Add sortedness spec to AVLTreeSetStEph** (Chap41): something like
   `forall|i: int, j: int| 0 <= i < j < elements@.len() ==> le(elements@[i], elements@[j])`.
   This would enable proving first/last/previous/next/rank/select.

2. **Solve clone/view bridging for StT + Ord types**: Either prove
   `lemma_cloned_view_eq` works with regular `Clone` (not just `clone_plus`),
   or add `obeys_feq_full` to the requires of these functions.

### Task 1(a) Blocking Detail

`from_sorted_entries` in OrderedTableMtEph constructs via
`TableMtEph::from_sorted_entries(elements)`. The wf predicate
`spec_orderedtablemteph_wf` includes `self.base_table.spec_tablemteph_wf()`
which is `spec_keys_no_dups(entries@)`. But `TableMtEph::from_sorted_entries`
only ensures `constructed@.dom().finite()`, not `spec_keys_no_dups`. Proving
no-duplicate-keys requires connecting AVLTreeSeqStPer sortedness to key
uniqueness — similar to the sortedness infrastructure gap above.

## Techniques Used

- Direct ensures-clause strengthening (Task 1b)
- Vacuous precondition removal (Task 3)
- Codebase analysis of spec coverage gaps (Task 2)
