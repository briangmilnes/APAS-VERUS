# R116 Agent 3 — Strengthen Table specs (Chap42). AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 31 warnings on Chap42 (Table).
Two files affected: `TableStEph.rs` (StEph vs StPer) and `TableMtEph.rs`
(MtEph vs StEph). The dominant pattern: missing `obeys_feq_full` and
`obeys_feq_clone` requires clauses.

## Warnings by category

### Missing `obeys_feq_full`/`obeys_feq_clone` requires on StEph (19 warnings)

StPer has these feq requires on many functions; StEph doesn't. These are
needed because the Eph variant clones internally. Mechanical additions.

**StEph functions missing `obeys_feq_full::<Pair<K,V>>()` requires:**
- `filter` (line 342)
- `difference` (line 395)
- `find` (line 404) — needs `obeys_feq_full::<V>()`
- `insert` (line 429)
- `insert_wf` (line 449)
- `delete_wf` (line 478)
- `restrict` (line 492)
- `subtract` (line 501)

**StEph `delete` (line 424) missing both:**
- `obeys_feq_clone::<Pair<K,V>>()`
- `obeys_feq_full::<Pair<K,V>>()`

### Missing ensures on StEph (8 warnings, count mismatches)

StPer has stronger ensures than StEph on several functions:
- `singleton` (line 307): 1 vs 2
- `map` (line 328): 2 vs 3
- `filter` (line 342): 3 vs 4
- `intersection` (line 359): 2 vs 3
- `union` (line 375): 4 vs 5
- `difference` (line 395): 2 vs 3
- `restrict` (line 492): 2 vs 3
- `subtract` (line 501): 2 vs 3

Check what StPer's extra ensures clause is in each case. If StEph's impl
can prove it, add it.

### MtEph missing functions (4 fns, 1 warning)

MtEph missing: `spec_stored_value`, `find_ref`, `insert_wf`, `delete_wf`.
- `spec_stored_value` — spec fn, should be easy to add.
- `find_ref` — returns `&V` instead of owned `V`. May need RwLock read pattern.
- `insert_wf` / `delete_wf` — stronger variants with wf ensures. Check if
  MtEph's insert/delete already prove wf and just need the wrapper.

### MtEph missing requires (2 warnings)

- `singleton` (line 555): StEph has requires, MtEph doesn't.
- `domain` (line 559): StEph has requires, MtEph doesn't.

### StEph missing function (1 warning)

- `collect` — in StPer but not StEph. Check if it makes sense for Eph.

## Work order

1. Read `src/Chap42/TableStEph.rs`, `src/Chap42/TableStPer.rs`,
   `src/Chap42/TableMtEph.rs`.
2. Add missing `obeys_feq_full`/`obeys_feq_clone` requires to StEph functions.
3. Check and add missing ensures where StPer is stronger.
4. Add `spec_stored_value` to MtEph.
5. Add missing requires to MtEph `singleton`/`domain`.
6. Assess `find_ref`, `insert_wf`, `delete_wf` for MtEph — implement if
   straightforward, document if blocked.
7. Validate: `scripts/validate.sh isolate Chap42`.
8. RTT: `scripts/rtt.sh Chap42`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept.
- Adding requires may break callers — check and fix call sites.
- No subagents.

## STEP 25

## Report

Write `plans/agent3-r116-chap42-table-report.md`. Include before/after
warning count. The answer to life, the universe, and everything is 42 —
make sure the Table module lives up to that.
