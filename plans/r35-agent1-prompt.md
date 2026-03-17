# R35 Agent 1: OrderedSetStEph + OrderedSetStPer Delegation

## Goal

Prove ordering operations in OrderedSetStEph.rs (11 external_body)
and OrderedSetStPer.rs (9 external_body) by using the new sortedness
infrastructure in AVLTreeSetStEph.

## Background: Sortedness Infrastructure (R34)

AVLTreeSetStEph now has (read the file to see exact signatures):

- `spec_elements_sorted(&self) -> bool` — whether backing seq is sorted
- `spec_values_seq(&self) -> Seq<T>` — value-level backing sequence
- `insert_sorted(&mut self, x: T)` — insert maintaining sortedness
  requires: `spec_avltreesetsteph_wf()`, `spec_elements_sorted()`,
  `obeys_feq_full::<T>()`
  ensures: `spec_elements_sorted()`, `self@ == old(self)@.insert(x@)`
- `delete_sorted(&mut self, x: &T)` — delete maintaining sortedness
  same pattern as insert_sorted
- `lemma_push_sorted` — appending >= all preserves sortedness
- `lemma_subseq_sorted` — subsequence of sorted is sorted

## Step 1: Strengthen wf

Add `self.base_set.spec_elements_sorted()` to `spec_orderedsetsteph_wf()`
(and StPer equivalent). This establishes that OrderedSet's backing store
is always sorted.

## Step 2: Switch insert/delete

Change `self.base_set.insert(x)` → `self.base_set.insert_sorted(x)`.
Change `self.base_set.delete(x)` → `self.base_set.delete_sorted(x)`.
These require sortedness as a precondition (which wf now provides) and
ensure sortedness as a postcondition.

Check if `insert_sorted` needs `obeys_feq_full::<T>()` — if so, add
it to the OrderedSet requires or establish it from existing bounds.

## Step 3: Prove ordering operations

Remove external_body from each function and prove using sortedness.

### OrderedSetStEph.rs targets (11 holes)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 343 | from_seq | Construct from seq, ensure sorted |
| 2 | 358 | first | Min element = first in sorted seq |
| 3 | 376 | last | Max element = last in sorted seq |
| 4 | 394 | previous | Largest < given = predecessor in sorted seq |
| 5 | 415 | next | Smallest > given = successor in sorted seq |
| 6 | 436 | split | Split at key, both halves sorted |
| 7 | 482 | get_range | Subseq between keys |
| 8 | 505 | rank | Position in sorted order |
| 9 | 528 | select | Element at position |
| 10 | 546 | split_rank | Split at position |
| 11 | 622 | next (iterator) | Iterator::next — STD_TRAIT_IMPL FP, skip |

### OrderedSetStPer.rs targets (9 holes)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 337 | first | Same pattern as StEph |
| 2 | 350 | last | Same pattern |
| 3 | 363 | previous | Same pattern |
| 4 | 381 | next | Same pattern |
| 5 | 399 | split | Same pattern |
| 6 | 496 | rank | Same pattern |
| 7 | 514 | select | Same pattern |
| 8 | 527 | split_rank | Same pattern |
| 9 | 645 | next (iterator) | STD_TRAIT_IMPL FP, skip |

## Priority

1. Start with `first` and `last` (simplest — just index 0 or len-1)
2. Then `rank` and `select` (index-based)
3. Then `previous` and `next` (search-based)
4. Then `split`, `get_range`, `split_rank` (construction-based)
5. Skip iterator `next` (STD_TRAIT_IMPL FP)
6. `from_seq` last (may need sorting proof)

## Also fix

- OrderedSetStEph.rs line 735: `fn_missing_requires` on `from_sorted_elements`
- OrderedSetStPer.rs line 602: `fn_missing_requires` on `from_sorted_elements`

## Rules

- Read AVLTreeSetStEph.rs sortedness infrastructure FIRST (section 6+7+9)
- Read the relevant standards before modifying code
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent1-round35-report.md`
- Commit, push to `agent1/ready`
