# R111 Agent 2 — Fix iterator PTTs and two trigger warnings. AFK. PBOGH.

## Context

You wrote 89 iterator PTT patterns in R110 (14 new files, 7 updated). That work
is now merged to main. You never confirmed ptt.sh passes — the machine OOM'd
before you saw results.

Your R110 report is at `plans/agent2-r110-iterator-ptts-report.md`. Read it.

## Objective

Two tasks:

1. Get all iterator PTTs passing `scripts/ptt.sh`.
2. Fix two trigger warnings (see below).

**You are NOT writing new PTTs.** All PTTs were written in R110. This round is
fix-only. Do not create new test files or add new test patterns.

## Steps

1. Read your report: `plans/agent2-r110-iterator-ptts-report.md`
2. PTTs need a clean cargo build first. Run `cargo check -p rust_verify_test`.
   Fix compile errors before proceeding.
3. Run `scripts/ptt.sh`. Read the output. Show full output.
4. Fix any PTT failures. Common issues:
   - Wrong import paths
   - Wrong type parameters or bounds
   - Missing `View` impls on consume iterators
   - Ghost iterator type mismatches
   - `ensures` clauses that don't match the actual iterator spec
5. Re-run `scripts/ptt.sh` after fixes. Iterate until clean.
6. Fix the two trigger warnings (see below).
7. Run `scripts/validate.sh` once to confirm clean. Show the last 5 lines.
8. Run `scripts/rtt.sh` to confirm no RTT regressions.
9. Commit fixes.

## Trigger warnings to fix

Verus emits "automatically chose triggers" for two `choose` expressions. Add
explicit `#[trigger]` to silence them.

### 1. StarPartitionMtEph.rs line 695 (Chap62)

```rust
let j = choose|j: int| 0 <= j < nv as int && vertices_vec@[j]@ == v_view;
```

Needs `#[trigger]` on the indexing term:
```rust
let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices_vec@[j]@ == v_view;
```

### 2. AVLTreeSetMtPer.rs line 314 (Chap41)

```rust
let j = choose|j: int| 0 <= j < n as int && vals@[j]@ == v;
```

Needs `#[trigger]` on the indexing term:
```rust
let j = choose|j: int| 0 <= j < n as int && #[trigger] vals@[j]@ == v;
```

## Rules

- Do NOT write new PTTs. Fix-only.
- Run validate, ptt, rtt sequentially — never in parallel.
- Read each failing test's error carefully before fixing.
- Do NOT delete test patterns to make failures go away. Fix them.
- If a pattern is genuinely unprovable (e.g., iterator has `ensures true`
  with no useful spec), comment it out with `// SKIPPED: ensures true`
  and note it in your report.
- No subagents.

## STEP 20

## Report

Write `plans/agent2-r111-ptt-fix-report.md`. Include:
- PTT pass count before/after
- Which tests failed and how you fixed them
- Any patterns skipped with reason
- Trigger warning fix confirmation
