# R37 Agent 3: OrderedSetStEph/StPer Remaining + AugOrderedTableMtEph

## Goal

Prove the remaining 4 operations in OrderedSetStEph.rs and 4 in
OrderedSetStPer.rs, then tackle AugOrderedTableMtEph.rs (2 holes).

## Context

OrderedSet wraps AVLTreeSetStEph/StPer. The remaining external_body
functions are: split, rank, select, and Iterator::next.

In R36 you proved 12 Mt delegations and fixed 46 trigger annotations.
This round focuses on the St-level algorithmic proofs.

## Tier 1: OrderedSetStEph.rs (4 external_body)

| # | Line | Function | Description |
|---|------|----------|-------------|
| 1 | 796 | split | Splits set at key k into (left, found, right) |
| 2 | 885 | rank | Returns count of elements < k |
| 3 | 908 | select | Returns i-th smallest element |
| 4 | 1075 | Iterator::next | Returns next element in iteration order |

### Approach

**split**: Delegates to `self.base_set.split(k)` (AVLTreeSetStEph::split).
Check if the base method's ensures match the OrderedSet trait ensures.
If yes, remove external_body and call through. The disjoint postcondition
needs: `forall|x| left@.contains(x) ==> TotalOrder::lt(x, k@)` and
`forall|x| right@.contains(x) ==> TotalOrder::le(k@, x)`.

**rank**: Delegates to `self.base_set.rank(k)`. The spec says
`rank == self@.filter(|k'| TotalOrder::lt(k', k@)).len()`. Check if the
base AVLTreeSetStEph has a `rank` method with compatible ensures.

**select**: Delegates to `self.base_set.select(i)`. The spec says
`selected matches Some(v) ==> v == sorted_elements[i as int]`. Check base
method compatibility.

**Iterator::next**: The iterator wraps an AVLTreeSetStEph iterator.
Delegate to `self.inner.next()` and bridge the ensures. Read
`src/standards/iterators_standard.rs` for the iteration pattern.

### Pattern: check base method ensures

Before writing complex proofs, check if the base AVLTreeSetStEph methods
already have the ensures you need. If so, the proof is just:
```rust
fn rank(&self, k: &T) -> (rank: usize)
    where T: TotalOrder
{
    self.base_set.rank(k)  // base ensures propagate
}
```

Read `src/Chap41/AVLTreeSetStEph.rs` to see what split/rank/select ensure.

## Tier 2: OrderedSetStPer.rs (4 external_body, mirror)

| # | Line | Function |
|---|------|----------|
| 1 | 709 | split |
| 2 | 806 | rank |
| 3 | 824 | select |
| 4 | 1000 | Iterator::next |

Mirror the StEph proofs. StPer wraps AVLTreeSetStPer.

## Tier 3: AugOrderedTableMtEph.rs (2 external_body)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 85 | calculate_reduction | Closure requires cascade |
| 2 | 659 | reduce_range_parallel | ParaPair! fork-join parallelism |

**calculate_reduction**: Read `src/Chap43/AugOrderedTableStEph.rs` to see
how agent3 solved the closure cascade pattern in R35. The reducer's totality
(`forall|v1, v2| reducer.requires((v1, v2))`) needs to be in wf or requires.

**reduce_range_parallel**: Uses ParaPair! for parallelism. If blocked by
fork-join closure specs, leave as external_body and report.

**Expected: -6 to -10 holes total.**

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Do NOT touch OrderedTableStEph/StPer (assigned to Agent 2).
- Do NOT touch OrderedTableMtEph (assigned to Agent 1).
- Read base method ensures in AVLTreeSetStEph.rs FIRST.
- Read `src/standards/using_closures_standard.rs` before calculate_reduction.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent3-round37-report.md`.
- Commit, push to `agent3/ready`.
