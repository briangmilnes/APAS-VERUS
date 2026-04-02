# R136 Agent 4 — Audit Chap43 OrderedTable for unnecessary in_order traversals. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `prompts/Chap43.txt` for APAS cost expectations.
Read your own R135 report on BSTParaMtEph — you already know the BST internals.

Report file: `plans/r136-agent4-chap43-in-order-audit-report.md`

## Problem

`src/Chap43/OrderedTableStEph.rs` calls `in_order()` (O(n) full tree traversal)
in at least 12 operations. Many of these should use BST operations (O(lg n)) or
BST-level operations that don't require flattening the entire tree.

Agent 1 is fixing `bst_find_by_key` (the find case). Your job: audit ALL other
uses of `in_order()` in Chap43 and classify each as:

- **Bug**: should use BST search/split/etc. instead of in_order + scan. O(n) → O(lg n).
- **Correct**: the operation genuinely needs all elements (e.g., domain, map, values).
- **Improvable**: could use a smarter BST operation but it's not wrong per se.

## What to audit

Check these files:
- `src/Chap43/OrderedTableStEph.rs`
- `src/Chap43/OrderedTableStPer.rs`
- `src/Chap43/AugOrderedTableStEph.rs`
- `src/Chap43/AugOrderedTableStPer.rs`
- `src/Chap43/OrderedSetMtEph.rs`

For each `in_order()` call site, report:
```
/full/path/file.rs:LINE: fn FUNCTION — in_order usage: BUG/CORRECT/IMPROVABLE — reason
```

## Which operations SHOULD be O(lg n)?

APAS CS 43.2: "The work and span for all the operations in ADT 43.1 is O(lg n)."
This includes: find, insert, delete, first, last, previous, next, split, join,
getRange, rank, select, splitRank.

Operations that legitimately need O(n): domain (return all keys), map (transform
all values), filter (test all entries), reduce (aggregate all entries), values
(return all values), tabulate (build from scratch).

## Do NOT fix the code in this round

This is an audit. Report what you find. Agent 1 is fixing find — don't duplicate
that work. List the other operations that need fixing, what BST operations they
should use instead, and estimated difficulty.

If any fix is trivial (one-line change to use an existing BST method), you may
implement it. But don't attempt complex rewrites.

## When done

RCP.
