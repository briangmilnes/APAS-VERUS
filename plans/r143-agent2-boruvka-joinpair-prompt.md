# R143 Agent 2 — Investigate Boruvka DIFFERS + BST join_pair. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap66/BoruvkaMtEph.rs` — Boruvka MST implementation.
Read `src/Chap38/BSTParaMtEph.rs` — join_pair function.
Read `prompts/Chap66.txt` — APAS description of Boruvka.
Read `prompts/Chap38.txt` — APAS cost specs for parametric BST.

Report file: `plans/r143-agent2-boruvka-joinpair-report.md`

## Problem 1: Boruvka (4 DIFFERS)

```
BoruvkaMtEph.rs:136  boruvka_mst_mt          — sequential O(lg n) loop, each round O(lg m) span
BoruvkaMtEph.rs:155  boruvka_mst_mt_with_seed — same
BoruvkaMtEph.rs:765  boruvka_mst_mt          — same (impl)
BoruvkaMtEph.rs:996  boruvka_mst_mt_with_seed — same (impl)
```

### Investigation

1. Read what APAS says about Boruvka's span. APAS Algorithm 66.3 describes
   Boruvka as O(lg n) rounds, each round doing parallel work. The outer loop
   IS sequential — you can't start round k+1 until round k contracts the graph.

2. If APAS gives Span O(lg² n) (lg n rounds × lg n per round), and our code
   does lg n sequential rounds with each round O(lg m) span via ParaPair
   helpers, then we may already match APAS. The annotation says "sequential
   O(lg n) loop" which sounds like a mismatch, but lg n sequential rounds
   with parallel internals IS the correct algorithm.

3. Check whether the DIFFERS is an annotation error. If the outer loop is
   inherently sequential (which it is for Boruvka), and each round is parallel
   (which it is via ParaPair), then the annotation should say "matches APAS"
   with the correct span.

4. If the annotation is wrong, fix it. If there's a genuine gap (e.g., some
   step within a round is sequential that APAS does in parallel), identify it
   and fix or document it.

## Problem 2: BST join_pair (1 DIFFERS)

```
BSTParaMtEph.rs:350  join_pair — delegates to union_inner (ParaPair)
```

### Investigation

1. Read what APAS says about join_pair's cost. Is it O(m lg(n/m + 1)) as
   the textbook specifies for BST union?

2. The annotation says "delegates to union_inner (ParaPair)" — this IS
   the APAS algorithm (BST union via split + recursive union of halves).
   ParaPair is parallel. So this might match APAS.

3. Check whether the work/span in the Code review annotation actually matches
   the APAS cost spec. If so, fix the annotation from DIFFERS to matches.

## Validation

Run `scripts/validate.sh isolate Chap66` then `isolate Chap38`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- These may be annotation errors, not code bugs. Read carefully before changing code.
- If the implementation matches APAS, fix the annotation. Don't change working code.

## When done

RCP.
