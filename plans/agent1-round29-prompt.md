# Agent 1 — R29: Chap28 + Chap42 + Chap21 fn_missing_requires Sweep

## State

Main at latest commit. 4118 verified, 0 errors. You are Agent 1.

## Assignment

Fix all fn_missing_requires and requires_true warnings in Chap28, Chap42, and Chap21.
These are all mechanical — add the real precondition. **Do NOT add `requires true`.**

### Chap28 (8 warnings, 8 files)

All MaxContigSubSum variants. Each has 1 fn_missing_requires or requires_true:

| File | Warning |
|------|---------|
| MaxContigSubSumBruteStEph.rs | 1 |
| MaxContigSubSumDivConMtEph.rs | 1 |
| MaxContigSubSumDivConOptMtEph.rs | 1 |
| MaxContigSubSumDivConOptStEph.rs | 1 |
| MaxContigSubSumDivConStEph.rs | 1 |
| MaxContigSubSumIterStEph.rs | 1 |
| MaxContigSubSumReducedMcsseStEph.rs | 1 |
| MaxContigSubSumReducedStEph.rs | 1 |

Pattern: these are all implementations of max contiguous subsequence sum. The
fn_missing_requires is likely on the main algorithm function — needs `requires`
on the input sequence (e.g., length bounds). Read each function, understand what
it needs, add the real precondition.

For `requires_true` warnings: replace `requires true` with the real precondition,
or remove `requires true` if the function genuinely needs nothing (and has no ensures
that depend on a precondition).

### Chap42 (3 warnings, 3 files)

| File | Warning |
|------|---------|
| TableStEph.rs | 1 fn_missing_requires |
| TableStPer.rs | 1 fn_missing_requires |
| TableMtEph.rs | 1 fn_missing_requires |

These are table (dictionary) implementations. The fn_missing_requires is likely on
a helper function. Read it, add real requires.

### Chap21 (4 warnings, 3 files)

| File | Warning |
|------|---------|
| Algorithm21_5.rs | 1 |
| Exercise21_7.rs | 2 |
| Exercise21_8.rs | 1 |

These are algorithm/exercise files. Read each, add real requires.

## Rules

- Do NOT touch files outside Chap28, Chap42, and Chap21.
- Do NOT add `requires true`.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Skip no files — these chapters have few warnings each. Get them all.

## Deliverable

- `scripts/validate.sh` passes with 0 errors.
- Write report to `plans/agent1-round29-report.md`.
- `git add -A && git commit` with descriptive message.
- `git push origin agent1/ready`.
