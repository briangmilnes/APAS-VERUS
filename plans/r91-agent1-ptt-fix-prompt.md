# R91 Agent 1 — Fix PTT Infrastructure (308 failures), STEP 20

## Objective

Fix the PTT (Proof Time Tests) infrastructure. 308 of 311 PTT tests fail with
`can't find crate for apas_verus`. The root cause is that the Verus compilation
of the apas_verus lib crate fails with 1 error, so the crate artifact is never
produced and PTT test files can't link against it.

## Root Cause

`scripts/ptt.sh` compiles the apas_verus crate with Verus, then runs the PTT
tests against it. The crate compilation has 1 error:

```
error: invariant not satisfied at end of loop body
    --> src/Chap42/TableMtEph.rs:2086:21
```

This is a Z3 flaky failure in `TableMtEph::subtract()`. The invariant involves
a forall over `sources` indices maintaining key/value correspondence through a
filtering loop. It passes sometimes and fails sometimes depending on Z3 scheduling.

## Strategy

### Option A: Fix the flaky invariant (preferred)

Read `src/Chap42/TableMtEph.rs` around line 2086. The invariant:
```rust
forall|j: int| 0 <= j < sources.len() ==>
    0 <= #[trigger] sources[j] < old_view.len()
    && old_view[sources[j]].0 == kept@[j].0@
    && old_view[sources[j]].1 == kept@[j].1@
    && !keys@.contains(kept@[j].0@),
```

This is a conjunction flakiness issue — Z3 can prove each conjunct individually
but sometimes fails the conjunction. The fix (per R28 pattern): build the
conjunction incrementally with ghost variables, then assert equivalence.

Read the R28 workaround pattern used elsewhere in the codebase:
```bash
grep -r "incremental.*conjunction\|ghost.*conjunction" src/ --include="*.rs" | head -5
```

Or search for the pattern in ETSPStEph.rs which agent2 recently fixed for the
same issue.

### Option B: Bump rlimit

If the invariant is close to proving, `#[verifier::rlimit(20)]` on the function
may give Z3 enough budget. Try this first as a quick check.

### Option C: Split the invariant

Break the 4-conjunct forall into separate invariants:
```rust
invariant
    forall|j: int| 0 <= j < sources.len() ==> 0 <= #[trigger] sources[j] < old_view.len(),
    forall|j: int| 0 <= j < sources.len() ==> old_view[#[trigger] sources[j]].0 == kept@[j].0@,
    forall|j: int| 0 <= j < sources.len() ==> old_view[#[trigger] sources[j]].1 == kept@[j].1@,
    forall|j: int| 0 <= j < sources.len() ==> !keys@.contains(kept@[j].0@),
```

This avoids the conjunction issue entirely. Z3 proves each quantifier independently.

## After fixing

Once the crate compiles clean (0 errors), PTT tests should mostly pass. If
individual PTT tests still fail, those are separate issues — report them but
don't spend all 20 steps on individual test fixes.

Run:
1. `scripts/validate.sh` — confirm 0 errors
2. `scripts/ptt.sh` — confirm PTT tests pass

## Read first

- `src/Chap42/TableMtEph.rs` around line 2086 — the flaky invariant
- `src/Chap26/ETSPStEph.rs` — recent conjunction flakiness fix (search for
  "incremental" or "conjunction")

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify files outside Chap42 unless absolutely necessary.
- Do NOT add assume or accept.
- Do NOT weaken ensures.
- The fix should be STABLE — not just passing once, but reliably passing.
  Run validate twice if you can to confirm stability.
- If you fix the flaky invariant AND PTT passes, that's the whole task done.

## STEP 20

## Report

Write `plans/agent1-r91-ptt-report.md`.
