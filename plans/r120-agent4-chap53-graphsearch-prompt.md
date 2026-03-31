# R119 Agent 4 — Strengthen Chap53 GraphSearch specs. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 18 warnings on Chap53. Graph search
algorithms with missing Mt functions, weak requires/ensures, and param
mismatches. You did Chap52 graph representations in R118 — Chap53 builds
on those.

## Warnings

Get the full list by running:
```bash
~/projects/veracity/target/release/veracity-compare-par-mut -c ~/projects/APAS-VERUS 2>&1 | grep 'src/Chap53/.*warning'
```

The warnings will include missing functions in MtPer/MtEph variants,
weak requires/ensures vs StPer/StEph, and possibly param count mismatches
(Mt variants may add seed/scheduler parameters).

## Strategy

1. Run the compare-par-mut command above to get the current warning list.
2. Read all Chap53 files (St variants first for reference).
3. Categorize: missing fns, weak requires, weak ensures, false positives.
4. Fix mechanical warnings first (adding requires/ensures).
5. Assess missing Mt functions — implement if lock-delegate-unlock,
   document if they need parallel algorithm implementation.
6. Validate: `scripts/validate.sh isolate Chap53`.
7. RTT: `scripts/rtt.sh Chap53`.

## Important context

Chap53 depends on Chap52 (graph representations). Your R118 work
strengthened Chap52 specs. If Chap53 callers now have stronger ensures
available from Chap52, use them.

Also: `src/Chap53/GraphSearchMtPer.rs` has 1 proof hole at line 179:
```rust
assume(neighbors.spec_avltreesetmtper_wf());
```
Agent2 is separately working on this hole. Do NOT modify that assume.
Focus on compare-par-mut warnings only.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Do NOT touch the assume at GraphSearchMtPer.rs:179 (agent2's target).
- Mt standalone: do NOT import from St counterparts.
- Adding requires may break callers — check and fix.
- No subagents.

## STEP 25

## Report

Write `plans/agent4-r119-chap53-graphsearch-report.md`. Include before/after
warning count per file.
