# R142 Agent 3 — Fix set_edge DIFFERS (Chap52) + nth DIFFERS (Chap37). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap52/AdjMatrixGraphMtPer.rs` — set_edge (persistent seq row copy).
Read `src/Chap52/AdjSeqGraphMtEph.rs` — set_edge (rebuilds neighbor list).
Read `src/Chap37/AVLTreeSeqMtPer.rs` — nth (tree traversal).
Read `prompts/Chap52.txt` — APAS cost specs for graphs.
Read `prompts/Chap37.txt` — APAS cost specs for AVL tree sequences.

Report file: `plans/r142-agent3-set-edge-nth-report.md`

## Problem 1: set_edge in AdjMatrixGraphMtPer (1 DIFFERS)

```
AdjMatrixGraphMtPer.rs: set_edge — APAS says O(1); persistent seq requires row copy
```

The adjacency matrix is stored as a persistent sequence of rows. Setting one
entry requires copying the entire row (the persistent sequence update is O(n)).
APAS assumes O(1) random access update.

**Investigation:** Read the implementation. Is there a way to achieve O(1)?
With a persistent (functional) representation, single-element update is O(lg n)
at best (balanced tree). O(1) requires an ephemeral representation.

If O(1) is impossible with persistent sequences, update the annotation to
`ACCEPTED DIFFERENCE: persistent representation requires O(n) row copy` and
document why.

If there's a way to achieve O(lg n) or better (e.g., using the underlying
ParamBST or a different sequence representation for rows), implement it.

## Problem 2: set_edge in AdjSeqGraphMtEph (1 DIFFERS)

```
AdjSeqGraphMtEph.rs: set_edge — APAS says O(n), O(1); impl rebuilds neighbor list
```

Read what APAS actually says for the adjacency sequence representation. If APAS
says O(n) work, then our O(n) may already match. Check whether the DIFFERS
annotation is accurate or stale.

## Problem 3: nth in AVLTreeSeqMtPer (1 DIFFERS)

```
AVLTreeSeqMtPer.rs: nth — tree traversal to indexed node
```

APAS says nth on a balanced BST sequence is O(lg n) — descend using subtree
sizes. Our implementation should already do this (the BST stores sizes).

**Investigation:** Read the current nth implementation. If it's doing O(lg n)
descent via sizes, the annotation is wrong — update to matches APAS. If it's
doing in-order traversal, fix it to use size-based descent (same pattern as
OrderedTable's bst_select_by_rank in Chap43).

## Validation

Run `scripts/validate.sh isolate Chap52` then `isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- If a DIFFERS is unfixable due to representation choice, change annotation
  to `ACCEPTED DIFFERENCE: reason` (not DIFFERS).
- If a DIFFERS annotation is wrong (implementation already matches), fix the
  annotation.

## When done

RCP.
