# R34 Agent 4: Chap39 BSTTreapMtEph RwLock Assumes

## Goal

Prove 6 assume() holes in `src/Chap39/BSTTreapMtEph.rs`.
These are RwLock delegation patterns where the assume bridges
ghost state from the locked tree operation to the abstract set view.

## Targets

All in BSTTreapMtEph.rs:

| # | Line | Function | Assume |
|---|------|----------|--------|
| 1 | 1067 | find | `result.is_some() <==> self@.contains(target@)` |
| 2 | 1084 | size | `result as nat == self@.len()` |
| 3 | 1108 | minimum | `result.is_some() ==> self@.contains(result.unwrap()@)` |
| 4 | 1118 | maximum | `result.is_some() ==> self@.contains(result.unwrap()@)` |
| 5 | 1130 | in_order | `ordered@.len() == self@.len()` |
| 6 | 1142 | pre_order | `preordered@.len() == self@.len()` |

## Pattern

Each function follows this pattern:
1. `acquire_read()` on `self.locked_root`
2. Call the StEph link operation (e.g., `find_link`, `size_link`)
3. `release_read()`
4. `assume(...)` to bridge the gap

The bridge needed: the RwLock predicate (BSTTreapMtEphInv) connects
the locked Link<T> to `self@` (the ghost set). The link operation
has ensures about its result relative to the tree structure. The
assume closes the gap between "tree structure result" and "self@ result."

## Approach

1. Check what `find_link`, `size_link`, `min_link`, `max_link`,
   `in_order_collect`, `pre_order_collect` ensure.
2. Check what the RwLock invariant (`BSTTreapMtEphInv`) guarantees.
3. Chain: link operation ensures + RwLock invariant → function ensures.
4. Replace each `assume(...)` with proof assertions.

## Also consider

- BSTTreapMtEph has 3 EQ_CLONE_ASSUME structural FPs (lines 363,
  376, 1165). These are standard eq/clone pattern — skip.
- BSTParaTreapMtEph has 10 external_body holes — do NOT attempt
  these, they're parallel operations requiring more infrastructure.

## Rules

- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent4-round34-report.md`.
- Commit, push to `agent4/ready`.
