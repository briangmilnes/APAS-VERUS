# R37 Agent 2: OrderedTableStEph/StPer Remaining Operations

## Goal

Prove the remaining 5 operations in OrderedTableStEph.rs and 4 in
OrderedTableStPer.rs. You proved first/last/previous/next/get_key_range
in R36. Now tackle the harder ones: collect, filter, split_key, rank_key,
select_key.

## Context

You already know these files well from R36. The remaining functions are
more complex than the ordering ops you proved:

- **collect**: Sorts entries, returns sorted sequence. Needs sort correctness.
- **filter**: Closure-based filtering. Needs closure spec propagation.
- **split_key**: Splits table at key k. Needs disjoint postcondition.
- **rank_key**: Counts keys less than k via dom().filter().
- **select_key**: Finds k-th smallest key via dom().filter().

## Tier 1: OrderedTableStEph.rs (5 remaining)

| # | Line | Function | Difficulty |
|---|------|----------|-----------|
| 1 | 475 | collect | Medium: sort entries, prove sorted output |
| 2 | 556 | filter | Hard: closure requires/ensures propagation |
| 3 | 943 | split_key | Medium: partition by key, disjoint postcondition |
| 4 | 1094 | rank_key | Medium: count via linear scan, match dom().filter() |
| 5 | 1112 | select_key | Medium: sort + index, match dom().filter() |

### Approach for each

**collect**: The function sorts entries. Read how `Vec::sort_unstable_by`
interacts with Verus specs. You may need to prove that sorting preserves
the multiset of entries and produces a sorted sequence. If Vec::sort has
no Verus spec, wrap with external_body that states the sorting postcondition.

**filter**: Read `src/standards/using_closures_standard.rs` first. The
closure's requires/ensures must be propagated through the function's
requires. The pattern: add `requires forall|k: &K, v: &V| f.requires((k, v))`
and bridge the closure spec to the output table's view.

**split_key**: Partition entries into `< k` and `>= k`. The disjoint
postcondition says `left@.dom().disjoint(right@.dom())`. Prove by showing
no key appears in both partitions (by TotalOrder antisymmetry).

**rank_key**: Count entries with key < k. Should match `self@.dom().filter(|k'| lt(k', k)).len()`.
Use the same TotalOrder bridging from R36.

**select_key**: Sort keys, return k-th element. Similar to collect but
returns a single key. Match `self@.dom().filter(|k'| lt(k', k))` cardinality.

## Tier 2: OrderedTableStPer.rs (4 remaining)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 444 | collect | Mirror of StEph |
| 2 | 830 | split_key | Mirror of StEph |
| 3 | 973 | rank_key | Mirror of StEph |
| 4 | 991 | select_key | Mirror of StEph |

Note: StPer does NOT have a `filter` hole (it was proved or doesn't exist).
Mirror the StEph proofs. Use `lemma_view_index` from ArraySeqStPerS (which
you added in R36) to connect view-level and exec-level access.

## Priority

Start with rank_key and select_key — they use TotalOrder bridging you
already mastered. Then tackle split_key. Leave collect and filter for last
as they involve sort/closure complexity.

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Do NOT touch OrderedTableMtEph.rs (assigned to Agent 1).
- Read `src/standards/using_closures_standard.rs` before attempting filter.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent2-round37-report.md`.
- Commit, push to `agent2/ready`.
