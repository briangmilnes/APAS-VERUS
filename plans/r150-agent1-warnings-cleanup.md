# R150 Agent 1 — Fix All Verus Warnings + AVLTreeSeqStPer rlimit. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r150-agent1-warnings-cleanup-report.md`

## Goal

Zero warnings from `scripts/validate.sh`. Currently 6 warnings + 1 error.

## Task 1: Fix `assert forall` implies warnings (5 instances)

These use `==>` instead of `implies` in `assert forall` statements. Verus warns
because with `==>`, the antecedent is NOT assumed in the proof body. With `implies`,
it IS assumed. This is almost always what you want.

Change `==>` to `implies` in these `assert forall` statements:

| # | Chap | File | Line |
|---|------|------|------|
| 1 | 37 | BSTRBMtEph.rs | 1304 |
| 2 | 37 | BSTRBMtEph.rs | 1310 |
| 3 | 62 | StarPartitionMtEph.rs | 1217 |
| 4 | 62 | StarPartitionMtEph.rs | 1401 |
| 5 | 62 | StarPartitionMtEph.rs | 1429 |

Read each `assert forall` block. Change `==>` to `implies` on the line with the
quantifier condition. The proof body (`by { ... }`) should not need changes — it
was already proving the conclusion, just without the antecedent assumed.

Validate after each file change with `scripts/validate.sh isolate ChapNN`.

## Task 2: Fix auto-trigger warnings (8 instances)

Verus is choosing triggers automatically instead of using explicit `#[trigger]`.
Add explicit trigger annotations.

| # | Chap | File | Line |
|---|------|------|------|
| 1 | 37 | AVLTreeSeqStPer.rs | 440 |
| 2 | 62 | StarPartitionMtEph.rs | 1265 |
| 3 | 62 | StarPartitionMtEph.rs | 1303 |
| 4 | 62 | StarPartitionMtEph.rs | 1320 |
| 5 | 62 | StarPartitionMtEph.rs | 1362 |
| 6 | 62 | StarPartitionMtEph.rs | 1369 |
| 7 | 62 | StarPartitionMtEph.rs | 1403 |
| 8 | 62 | StarPartitionMtEph.rs | 1431 |

For each, read the Verus output to see what trigger it auto-chose, then add that
as an explicit `#[trigger]` annotation. Common patterns:

```rust
// Single trigger:
forall|i: int| 0 <= i < n ==> #[trigger] seq[i] == f(i)

// Multi-trigger:
forall|i: int, j: int| #![trigger seq1[i], seq2[j]] ...
```

Lines 1403/1431 overlap with Task 1 (same assert forall blocks) — fix both the
`==>` → `implies` and add triggers in one edit.

## Task 3: Fix AVLTreeSeqStPer.rs:440 rotate_right rlimit

This function exceeds its rlimit. Profile it first:

```bash
scripts/validate.sh isolate Chap37 --profile
```

Then read the profile:

```bash
ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs cat
```

Look for quantifiers with >100K instantiations — those are matching loop candidates.

Possible fixes (try in order):
1. Add explicit `#[trigger]` to kill the auto-trigger warning (Task 2 line 1) —
   bad trigger choice may be causing the blowup.
2. Add intermediate `assert` steps to guide Z3.
3. Add `#[verifier::rlimit(N)]` with a specific budget if the function is just
   slightly over the default.
4. Factor out a proof lemma if the function body is doing too much.

Do NOT just bump the rlimit without profiling first.

## Validation

After all changes, run `scripts/validate.sh` (full). Target: 0 warnings, 0 errors
(or document why a warning/error cannot be fixed).

Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or specs (only fix warnings and rlimit).
- Do NOT weaken ensures.
- All existing RTTs must pass.

## When done

RCP.
