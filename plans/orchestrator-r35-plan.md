# R35 Plan: Chap43 Mass Delegation — All 4 Agents

## State After R34

- 4190 verified, 0 errors, 2613 RTT, 147 PTT
- 37 clean chapters, 9 holed
- 159 total holes, 135 real actionable
- Chap43: 98 holes across 10 files (86 external_body, 11 assume, 1 fn_missing)
- Agent1 R34 added sortedness infrastructure to AVLTreeSetStEph:
  `insert_sorted`, `delete_sorted`, `spec_elements_sorted`, `spec_values_seq`

## R35 Strategy

All 4 agents on Chap43. The sortedness infrastructure unblocks the
ordering operations (first, last, previous, next, rank, select, split,
get_range, split_rank). This is the mass production phase.

## Key Insight: insert_sorted / delete_sorted

The existing OrderedSet/OrderedTable files call `self.base_set.insert(x)`
and `self.base_set.delete(x)`. These DON'T maintain sortedness. Agent1
added `insert_sorted` and `delete_sorted` which DO. To prove ordering
operations, each file needs:

1. Strengthen wf: add `self.base_set.spec_elements_sorted()` (or
   equivalent through delegation layers)
2. Switch insert/delete to call `insert_sorted`/`delete_sorted`
3. Remove external_body from ordering operations, prove via sortedness

## Agent Assignments

| # | Agent | Files | Holes | Pattern |
|---|-------|-------|-------|---------|
| 1 | 1 | OrderedSetStEph + OrderedSetStPer | 20 | 1-level delegation |
| 2 | 2 | OrderedTableStEph + OrderedTableStPer | 21 | 2-level delegation |
| 3 | 3 | AugOrderedTableStEph + AugOrderedTableStPer | 18 | 3-level + reduce |
| 4 | 4 | OrderedSetMtEph + OrderedTableMtEph | 22 | Mt RwLock delegation |

No file conflicts between agents.

Deferred to R36: OrderedTableMtPer (8), AugOrderedTableMtEph (8).

## Expected Outcomes

- Agent 1: Pilot the pattern on OrderedSetStEph. -8 to -15 holes.
- Agent 2: Two-level delegation (Table wraps Set). -8 to -15 holes.
- Agent 3: AugOrderedTable adds reduce. -6 to -12 holes.
- Agent 4: Mt delegation + RwLock accepts. -8 to -15 holes.

**Expected total: -30 to -55 holes.**

## Merge Order

Any order (file-disjoint). Validate after each merge.
