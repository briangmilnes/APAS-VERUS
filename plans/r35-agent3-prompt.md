# R35 Agent 3: AugOrderedTableStEph + AugOrderedTableStPer

## Goal

Prove ordering operations in AugOrderedTableStEph.rs (10 external_body)
and AugOrderedTableStPer.rs (6 external_body + 2 assume). These are
3-level delegations with an added reduction/augmentation layer.

## Background: Delegation Chain

```
AugOrderedTableStEph<K, V, R>
  └── base_table: OrderedTableStEph<K, V>
        └── base_table: TableStEph<K, V>
              └── base_set: AVLTreeSetStEph<Pair<K, V>>
```

AugOrderedTable adds a `reduction` function (R) that computes an
aggregate over the table's values (e.g., sum, max). The ordering
operations delegate through OrderedTable to Table to AVLTreeSet.

Read AVLTreeSetStEph.rs sortedness infrastructure and trace the
delegation chain through OrderedTableStEph and TableStEph.

## Step 1: Understand the augmentation layer

AugOrderedTable wraps OrderedTable and adds:
- `calculate_reduction` / `recalculate_reduction` — compute R from values
- `join_key` — merge two tables at a key (used by split operations)

The ordering operations (first, last, previous, next, rank, select)
should delegate to the base OrderedTable. The augmentation only
affects operations that modify the table (insert, delete, join).

## Step 2: Prove ordering operations

### AugOrderedTableStEph.rs targets (10 holes)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 71 | calculate_reduction | Compute reduction from entries |
| 2 | 498 | map | Transform values with reduction |
| 3 | 578 | first_key | Delegate to base OrderedTable |
| 4 | 590 | last_key | Delegate to base |
| 5 | 602 | previous_key | Delegate to base |
| 6 | 614 | next_key | Delegate to base |
| 7 | 662 | join_key | Merge two tables at key |
| 8 | 701 | rank_key | Delegate to base |
| 9 | 712 | select_key | Delegate to base |
| 10 | 815 | clone | Clone the augmented table |

### AugOrderedTableStPer.rs targets (8 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 102 | calculate_reduction | assume | Reduction computation |
| 2 | 608 | first_key | external_body | Delegate to base |
| 3 | 620 | last_key | external_body | Delegate to base |
| 4 | 632 | previous_key | external_body | Delegate to base |
| 5 | 644 | next_key | external_body | Delegate to base |
| 6 | 705 | join_key | assume | Merge operation |
| 7 | 742 | rank_key | external_body | Delegate to base |
| 8 | 753 | select_key | external_body | Delegate to base |

## Priority

1. Pure delegations first: first_key, last_key, previous_key, next_key,
   rank_key, select_key (6 per file, should be mechanical)
2. calculate_reduction (may need closure/reduction specs)
3. join_key (construction — merge two sorted sequences)
4. map, clone (higher-order / structural)

## Also fix

- AugOrderedTableStPer.rs line 75: `fn_missing_requires` on `calculate_reduction`

## Rules

- Read AVLTreeSetStEph.rs sortedness infrastructure FIRST
- Read OrderedTableStEph.rs to understand how ordering operations work
- Read the relevant standards before modifying code
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent3-round35-report.md`
- Commit, push to `agent3/ready`
