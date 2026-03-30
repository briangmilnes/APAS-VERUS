# R111 Agent 2 — Fix remaining trigger warnings in StarPartitionMtEph.rs. AFK. PBOGH.

## Objective

Eliminate all "automatically chose triggers" warnings in
`src/Chap62/StarPartitionMtEph.rs`. Two were fixed earlier this round
(lines 314 in AVLTreeSetMtPer.rs and 695 in this file). There are more
remaining in this file.

## Steps

1. Run `scripts/validate.sh isolate Chap62`. Show full output.
2. Find every "automatically chose triggers" note in the output.
3. For each one, read the quantifier, identify what Verus auto-selected,
   and add an explicit `#[trigger]` annotation that matches.
4. Re-run `scripts/validate.sh isolate Chap62` after fixes. Iterate until
   zero trigger notes.
5. Run `scripts/validate.sh` once to confirm no regressions crate-wide.
6. Run `scripts/rtt.sh` to confirm RTTs pass.
7. Also run `~/projects/veracity/target/release/veracity-compare-par-mut ~/projects/APAS-VERUS`
   and include the summary line (errors/warnings/info counts) in your report.
8. Commit.

## Rules

- Only add `#[trigger]` annotations. Do not change logic, specs, or proof structure.
- Run validate and rtt sequentially, not in parallel.
- No subagents.

## STEP 15

## Report

Write `plans/agent2-r111-starpartition-triggers-report.md`.
