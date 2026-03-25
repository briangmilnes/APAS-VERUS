# R79 Agent 2 — Remove all_chapters cfg gates + fix broken tests

## Objective

28 test files in `tests/` have `#![cfg(feature = "all_chapters")]` on line 1. This feature
doesn't exist in Cargo.toml, so these tests are silently excluded from every RTT run.
Remove the gates and ensure all tests compile and pass.

## Baseline

- 4905 verified, 0 errors, 0 warnings
- 2774 RTT passed, 157 PTT passed

## Task 1: Remove cfg gates

Find all files:
```bash
grep -rl '^#!\[cfg(feature = "all_chapters")\]' tests/
```

Remove line 1 from each. There are 28 files.

## Task 2: Compile and fix

After removing gates, run `cargo build --tests 2>&1 | head -100` to find compile errors.

Known problem files (from agent1 R77 report — these were commented out in Cargo.toml
because their source modules are commented out in lib.rs):
- TestJohnsonMtEphF64 (Chap59) — module commented out
- TestGraphSearchMtPer (Chap53) — module commented out
- TestEdgeSetGraphMtPer, TestAdjTableGraphMtPer, TestAdjSeqGraphMtPer (Chap52) — modules commented out
- TestAVLTreeSetMtPer (Chap41) — tests have real bugs (thread explosion)
- TestAVLTreeSeqMtPer (Chap37) — check if module exists

For files whose source modules don't exist: **comment out the `[[test]]` entry in Cargo.toml**
with a reason, and **put the cfg gate back** on just those files. The goal is: every test
that CAN run DOES run, and broken tests are explicitly marked.

## Task 3: Verify test count increase

Run `scripts/rtt.sh`. The test count should increase from 2774. Report the new count.

## Important

- Do NOT modify source files in `src/`. Test-only task.
- Do NOT modify `scripts/validate.sh` or other scripts.
- For tests that fail at runtime (not compile time), investigate briefly. If it's a
  real bug in the test (wrong assertion, API change), fix the test. If it's a bug in
  the source module, leave the test and report the issue.

## Validation

Run `scripts/rtt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round79-report.md` with:
- Tests ungated (count)
- Tests that needed cfg gate restored (count + reasons)
- New RTT count
