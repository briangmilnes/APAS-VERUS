# R138 Agent 1 — Fix remaining 8 OrderedTable in_order bugs. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `plans/r136-agent4-chap43-in-order-audit-report.md` — the full audit.
Read `plans/r137-agent4-chap43-fix-report.md` — what agent 4 already fixed + infrastructure.

Report file: `plans/r138-agent1-orderedtable-remaining-report.md`

## Background

Agent 4 (R136) audited 41 in_order() sites in Chap43 and found 11 BUGs. Of those:
- `bst_find_by_key`: YOU fixed this in R135 (O(lg n) BST descent)
- `first_key_iter`: Agent 4 fixed in R137
- `last_key_iter`: Agent 4 fixed in R137 (using new `max_key`)

8 remain. Agent 4 also built infrastructure you can use:
- `max_key` on ParamBST (Chap38) — traverse right branches, O(lg n)
- `TotalOrderBridge` trait — bridges Ord::cmp_spec to TotalOrder::le
- `reveal_param_bst_backings` — exposes BST type invariant for view witnesses

## The 8 remaining bugs (each in BOTH StEph and StPer)

| # | Function | Current | Fix (from APAS) | Blocker |
|---|----------|---------|-----------------|---------|
| 1 | next_key_iter | in_order + scan forward | split(k) → right half → min_key | None |
| 2 | previous_key_iter | in_order + scan backward | split(k) → left half → max_key | None |
| 3 | split_key_iter | in_order + rebuild two trees | BST split directly | None |
| 4 | get_key_range_iter | in_order + scan range | Two splits: split(k1), split(k2) | None |
| 5 | rank_key_iter | in_order + count | Walk BST using NodeInner.size | Need rank helper |
| 6 | select_key | in_order + index | Walk BST using NodeInner.size | Need select helper |
| 7 | split_rank_key_iter | in_order + rebuild | select + split | Needs 5+6 |

That's 7 unique functions × 2 files = 14 fixes. (The audit listed 8 remaining because
one was counted differently — the total is the same.)

## For functions 1-4 (no blockers)

Use the existing BST operations directly:

- `next_key`: `self.tree.split(&search_pair)` → take right half → `right.min_key()`
- `previous_key`: `self.tree.split(&search_pair)` → take left half → `left.max_key()`
- `split_key`: `self.tree.split(&search_pair)` → wrap both halves in OrderedTable
- `get_key_range`: split at k1, split right half at k2, middle is the range

For split, you need a search Pair. Use the same pattern as your `bst_find_by_key`:
construct via expose/descent, or use a dummy value if the BST's Ord only compares keys.

Use `TotalOrderBridge` lemmas to bridge BST cmp_spec guarantees to TotalOrder::le
postconditions. Use `reveal_param_bst_backings` for view-level witnesses.

## For functions 5-7 (need rank/select)

`ParamBST` stores `NodeInner.size` (subtree size). Write helper functions:

`bst_rank_by_key(tree, k)`: Walk the BST. At each node, if k < node.key, go left.
If k > node.key, add left.size + 1, go right. If k == node.key, return left.size.
O(lg n). Reference: `src/Chap40/BSTSizeStEph.rs` lines ~496 and ~503.

`bst_select_by_rank(tree, rank)`: Walk the BST. If rank < left.size, go left.
If rank == left.size, return this node. If rank > left.size, go right with
rank - left.size - 1. O(lg n). Reference: same file.

These go as free functions in OrderedTableStEph.rs (Pair-aware, keyed by K).

## Validation

Run `scripts/validate.sh isolate Chap43`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Every fix must be O(lg n).
- Use the infrastructure agent 4 built (TotalOrderBridge, max_key, reveal_param_bst_backings).
- Update annotations from O(n) to O(lg n).

## When done

RCP.
