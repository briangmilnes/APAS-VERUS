# Agent 4 — Round 54: Chap47 ParaHashTable + Chap43 overflow

## Goal

Fix spec warnings and holes in ParaHashTableStEph, close QuadProb hole, then help with Chap43.

## Priority 1: src/Chap47/ParaHashTableStEph.rs (2 holes + 8 warnings)

**Hole 1**: `assume(c == *x)` at line 115 — same clone bridge as Chap38. Read
`src/standards/partial_eq_eq_clone_standard.rs` and downgrade to clone workaround pattern.

**Hole 2**: `#[verifier::external_body]` on `resize` at line 493. Read the function
body — if it's algorithmic logic (not just threading), the external_body must be replaced
with a real proof. If it's a thread-spawn boundary, document and leave.

**Warnings (8)**: `fn_missing_wf_requires` and `fn_missing_wf_ensures` on `createTable`,
`insert`, `lookup`, `delete`, `metrics`, `loadAndSize`, `resize`. These functions need
`requires table.spec_hashtable_wf()` and/or `ensures result.spec_hashtable_wf()` added
to their signatures.

Read the trait definition to understand the wf predicate, then add the appropriate
requires/ensures to each function. The trait signature is the source of truth — update
both trait and impl.

## Priority 2: src/Chap47/QuadProbFlatHashTableStEph.rs (1 hole)

**Hole**: `assume(false)` at line 383 — "Table full: unreachable with load factor < ...".
This needs `assume(false); diverge()` pattern if it's truly unreachable, plus a proof
that the load factor invariant prevents reaching this point. Read the surrounding loop
to understand the full-table guard.

## Priority 3: src/Chap43/AugOrderedTableMtEph.rs (1 hole)

**Hole**: `#[verifier::external_body]` on `reduce_range_parallel` at line 672. This is
a parallel algorithm — read `src/standards/hfscheduler_standard.rs` and the fork-join
rules. The structural logic should be verified; only the thread-spawn boundary should
be external_body.

Check if this was previously sequentialized and reverted. If so, the current external_body
is the intended state until someone writes the parallel proof with HF scheduler.

## Priority 4: src/Chap43/OrderedSetStEph.rs (1 hole)

**Hole**: `assume(self@.filter(...))` at line 1134. Same pattern as OrderedSetStPer
(Agent 2 is doing that one). If Agent 2 finds a fix, apply the same technique here.

## Rules

- Read `src/standards/partial_eq_eq_clone_standard.rs` for clone bridge pattern.
- Read `src/standards/capacity_bounds_standard.rs` for wf requires/ensures pattern.
- Do NOT add `accept()` or weaken ensures. Prove or leave the hole.
- Do NOT modify files outside Chap47/Chap43.
- Validate after each file. Fix trigger warnings.
- Write `plans/agent4-round54-report.md` when done.
