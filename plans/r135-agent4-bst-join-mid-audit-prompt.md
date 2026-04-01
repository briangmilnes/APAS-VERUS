# R135 Agent 4 — Audit BSTParaMtEph join_mid rebalancing. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `prompts/Chap38.txt` for the textbook description of join.

Report file: `plans/r135-agent4-bst-join-mid-report.md`

## Problem

`src/Chap38/BSTParaMtEph.rs` — `join_mid` DIFFERS from APAS:
"parametric impl wraps node without rebalancing."

APAS CS 38.11 says join costs O(lg(|t1|+|t2|)). The textbook's join operation
maintains balance. A join that just wraps a node without rebalancing produces
an unbalanced tree — future operations degrade from O(lg n) to O(n).

## What to do

1. **Read `join_mid` in BSTParaMtEph.rs** (the DIFFERS annotation is around line 278).
   Understand what it does: does it just create a Node(left, key, right) without
   any rotation or rebalancing?

2. **Read the textbook** (`prompts/Chap38.txt`). What does APAS say `join` should do?
   For balanced BSTs (AVL, weight-balanced, treap), join must maintain the balance
   invariant. For plain (unbalanced) BSTs, join without rebalancing is correct but
   has O(n) worst-case cost.

3. **Check which BST variant BSTParaMtEph is.** Is it parametric (supports multiple
   balancing strategies)? If so, does the parametric impl have a hook for rebalancing
   that's not being called?

4. **Read `join_pair` and `join_m`** (also flagged as DIFFERS). Do they delegate to
   `join_mid`? Is the issue only in `join_mid` or is the whole join infrastructure
   missing rebalancing?

5. **Check the StEph counterpart** (`BSTParaStEph.rs`). Does its `join_mid` rebalance?
   If StEph rebalances and MtEph doesn't, that's a porting bug.

6. **Check callers**: who calls `join_mid`? If it's called after split in
   union/intersect/difference, an unbalanced join would degrade those operations
   from O(m lg(n/m)) to O(m*n).

## This is an AUDIT — report what you find

If the fix is straightforward (add a rotation call, delegate to a rebalancing join),
implement it. If it's complex (needs new infrastructure, affects the parametric
abstraction), document what's needed and don't implement.

## Also audit these related DIFFERS

| # | Line | Function | DIFFERS reason |
|---|------|----------|---------------|
| 1 | ~278 | join_mid | wraps node without rebalancing |
| 2 | ~343 | join_pair | delegates to union_inner (ParaPair) |
| 3 | ~419 | join_m | delegates to join_mid which is O(1) |

Are these three related? Is `join_pair` using `union_inner` as a workaround for
not having a proper balanced join?

## Validation

If you make code changes: `scripts/validate.sh isolate Chap38`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- If this is a design issue (parametric BST doesn't support rebalancing hooks),
  document it clearly in the report. Don't hack around it.

## When done

RCP.
