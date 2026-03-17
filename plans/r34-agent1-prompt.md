# R34 Agent 1: Chap41 AVLTreeSetStEph Sortedness Infrastructure

## Goal

Add a sortedness spec to `src/Chap41/AVLTreeSetStEph.rs` that enables
proving ordering operations (first, last, previous, next, rank, select)
in Chap43's OrderedSet and OrderedTable modules.

This is the critical infrastructure enabler for ~86 real holes in Chap43.

## Context

AVLTreeSetStEph wraps AVLTreeSeqStEph. Elements are stored in sorted
order (BST property maintained by insert/delete), but the spec never
states this. Current `spec_avltreesetsteph_wf` only includes:
- `elements.spec_avltreeseqsteph_wf()`
- `elements@.no_duplicates()`
- `self@.finite()`

Chap43's ordering operations (e.g., `first()` returns elements[0] and
must prove it's ≤ all others) need a sortedness predicate.

## Tasks

1. **Define a sorted predicate.** Add to AVLTreeSetStEph:
   ```
   open spec fn spec_elements_sorted(&self) -> bool {
       forall|i: int, j: int| 0 <= i < j < self.elements@.len()
           ==> #[trigger] TotalOrder::le(self.elements@[i], self.elements@[j])
   }
   ```
   Or integrate it into `spec_avltreesetsteph_wf`.

2. **Prove empty and singleton maintain it.** Trivially true.

3. **Prove insert maintains sortedness.** The implementation uses
   `position_in_sorted_seq` to find the insertion point, then
   inserts at that position. Prove the result is still sorted.

4. **Prove delete maintains sortedness.** Removing an element from
   a sorted sequence preserves sortedness.

5. **Prove from_sorted_elements maintains it.** If the input is
   already sorted, the result is sorted.

6. **Also prove the 1 remaining assume in AVLTreeSetStEph** (line 280,
   the insert off-by-one issue) if time permits.

## Key Files

- `src/Chap41/AVLTreeSetStEph.rs` — main target
- `src/Chap37/AVLTreeSeq.rs` — backing sequence (for understanding only)
- `src/standards/total_order_standard.rs` — TotalOrder trait patterns

## Rules

- Do NOT modify Chap43 files. Only Chap41.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent1-round34-report.md`.
- Commit, push to `agent1/ready`.
