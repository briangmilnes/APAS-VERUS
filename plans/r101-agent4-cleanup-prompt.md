# R101 Agent 4 — Cleanup: trigger warnings + derive Clone + validate, STEP 10

## Objective

Get the codebase to 0 warnings and confirm full green.

## Task 1: Fix 2 trigger warnings

AdjTableGraphStPer.rs has `choose` quantifiers that Verus auto-triggers.
Find them and add explicit `#[trigger]` annotations.

```bash
grep -n "automatically chose triggers" logs/validate.*.log | tail -5
```

The pattern: `choose|j: int| 0 <= j < len && ...` needs a `#[trigger]` on
one of the terms inside.

## Task 2: Fix 2 derive(Clone) warnings

`AdjTableGraphStPer.rs` line 55 and `AdjTableGraphMtPer.rs` line 50 have
`#[derive(Clone)]` on non-Copy structs, causing Verus warnings.

Fix: Replace derive with manual Clone impl outside verus! (section 14):

```rust
// Outside verus!:
impl<V: StT + Ord + Clone> Clone for AdjTableGraphStPer<V> {
    fn clone(&self) -> Self {
        AdjTableGraphStPer { adj: self.adj.clone() }
    }
}
```

Remove the `#[derive(Clone)]`. The manual impl won't have Verus specs but
it won't warn either — it's outside verus!.

Check that `self.clone()` calls in the verus! body still work. If Verus
complains about using an external Clone, you may need to restructure the
clone call site (use `adj.clone()` instead of `self.clone()`, same pattern
agent3 R96 used).

## Task 3: Full validate + RTT + PTT

```bash
scripts/validate.sh    # must be 0 errors, 0 warnings
scripts/rtt.sh         # must be 0 failures
scripts/ptt.sh         # must be 0 failures
```

## Isolation

```bash
scripts/validate.sh isolate Chap52   # for trigger + clone fixes
scripts/validate.sh                  # full
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 10

## Report

Write `plans/agent4-r101-cleanup-report.md`.
