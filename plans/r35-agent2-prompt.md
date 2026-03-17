# R35 Agent 2: OrderedTableStEph + OrderedTableStPer Delegation

## Goal

Prove ordering operations in OrderedTableStEph.rs (12 external_body)
and OrderedTableStPer.rs (10 external_body). These are 2-level
delegations: OrderedTable wraps TableStEph which wraps AVLTreeSetStEph.

## Background: Delegation Chain

```
OrderedTableStEph<K, V>
  └── base_table: TableStEph<K, V>
        └── base_set: AVLTreeSetStEph<Pair<K, V>>
```

The sortedness infrastructure lives on AVLTreeSetStEph. The ordering
operations on OrderedTable need to propagate through Table to reach
the sorted sequence of Pair<K, V> entries.

Read AVLTreeSetStEph.rs sortedness infrastructure (section 6+7+9)
and TableStEph.rs to understand the delegation chain.

## Step 1: Understand Table → Set connection

TableStEph stores key-value pairs as `Pair<K, V>` in an
AVLTreeSetStEph. The ordering is by key (K). Check:
- Does TableStEph expose sortedness of its backing set?
- Does TableStEph's insert/delete maintain sortedness?
- You may need to add sortedness-aware wrappers in TableStEph first.

## Step 2: Strengthen wf

Add sortedness to `spec_orderedtablesteph_wf()` — the backing Table's
AVLTreeSet must be sorted by key.

## Step 3: Prove ordering operations

### OrderedTableStEph.rs targets (12 holes)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 421 | map | Transform values, preserve keys+order |
| 2 | 445 | filter | Subset of sorted is sorted |
| 3 | 526 | collect | Return sorted entries |
| 4 | 544 | first_key | Min key = first in sorted pairs |
| 5 | 558 | last_key | Max key = last in sorted pairs |
| 6 | 572 | previous_key | Predecessor key |
| 7 | 590 | next_key | Successor key |
| 8 | 608 | split_key | Split at key |
| 9 | 658 | get_key_range | Entries between keys |
| 10 | 680 | rank_key | Position of key |
| 11 | 698 | select_key | Key at position |
| 12 | 712 | split_rank_key | Split at position |

### OrderedTableStPer.rs targets (10 holes)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 440 | collect | Return sorted entries |
| 2 | 458 | first_key | Same pattern as StEph |
| 3 | 472 | last_key | Same pattern |
| 4 | 486 | previous_key | Same pattern |
| 5 | 504 | next_key | Same pattern |
| 6 | 522 | split_key | Same pattern |
| 7 | 570 | get_key_range | Same pattern |
| 8 | 600 | rank_key | Same pattern |
| 9 | 618 | select_key | Same pattern |
| 10 | 632 | split_rank_key | Same pattern |

## Priority

1. first_key, last_key (simplest)
2. rank_key, select_key (index-based)
3. previous_key, next_key (search-based)
4. collect (return sorted sequence)
5. split_key, get_key_range, split_rank_key (construction)
6. map, filter (higher-order — may need closure specs)

## Also fix

- OrderedTableStEph.rs line 887: `fn_missing_wf_ensures` on `from_sorted_entries`
- OrderedTableStPer.rs line 668: `fn_missing_wf_ensures` on `from_sorted_entries`

## Rules

- Read AVLTreeSetStEph.rs sortedness infrastructure FIRST
- Read TableStEph.rs to understand the Table → Set delegation
- Read the relevant standards before modifying code
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent2-round35-report.md`
- Commit, push to `agent2/ready`
