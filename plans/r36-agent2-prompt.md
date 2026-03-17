# R36 Agent 2: Chap43 OrderedTableStEph/StPer Ordering Operations

## Goal

Prove ordering operations in OrderedTableStEph.rs (10 ext_body) and
OrderedTableStPer.rs (9 ext_body). These are 2-level delegation:
OrderedTable → Table → AVLTreeSet.

## Background: TotalOrder bridging (SOLVED)

In R35 you were blocked by TotalOrder bridging. Agent 1 solved this in
OrderedSetStEph.rs. **Read `src/Chap43/OrderedSetStEph.rs` lines 369-460
BEFORE writing any code.**

The technique:
1. Iterate entries, track best candidate
2. `TotalOrder::cmp(elem_ref, &best)` → `core::cmp::Ordering`
3. Match on `Less / Equal / Greater`
4. `T::reflexive(x)` for Equal/self cases
5. `T::transitive(new_best, old_best, vals[j])` for Less case
6. Loop invariant: `forall|j| 0 <= j < i ==> TotalOrder::le(best, vals[j])`
7. Post-loop: `self@.contains(t@)` → `elements@.to_set().contains(t@)`
   → `elements@.contains(t@)` → index → covered by invariant

### Adaptation for OrderedTable

OrderedTable stores `Pair<K, V>` entries. Extract `.0` (key) for
comparisons. Use `TotalOrder::cmp` on keys, not PartialOrd.

For `&pair.0 >= k1` (get_key_range): replace with
`TotalOrder::cmp(&pair.0, k1)` — Verus doesn't support the blanket
`impl PartialOrd for &T`.

## Targets

### OrderedTableStEph.rs (10 external_body)

| # | Function | Pattern |
|---|----------|---------|
| 1 | first_key | min scan with TotalOrder::cmp |
| 2 | last_key | max scan with TotalOrder::cmp |
| 3 | previous_key | filtered max (less than k) |
| 4 | next_key | filtered min (greater than k) |
| 5 | rank_key | count elements less than k |
| 6 | select_key | sort + index (may need sorted entries) |
| 7 | get_key_range | filter by TotalOrder range |
| 8 | split_key | partition at key |
| 9 | filter | closure-based filtering |
| 10 | collect | BTree internal wrapper (may stay ext_body) |

### OrderedTableStPer.rs (9 external_body)

Same operations. StEph first, then mirror to StPer.

## Priority

1. first_key, last_key (simplest — min/max scan)
2. previous_key, next_key (filtered min/max)
3. rank_key (counting)
4. get_key_range (range filter with TotalOrder::cmp)
5. split_key, select_key, filter (harder)
6. collect (may stay external_body)

## Your R35 infrastructure

You already strengthened `from_sorted_entries` and `collect` ensures.
You proved `map` and `split_rank_key`. Build on that work.

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Read OrderedSetStEph.rs lines 369-460 FIRST for TotalOrder pattern.
- StEph first, then mirror to StPer.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent2-round36-report.md`.
- Commit, push to `agent2/ready`.
