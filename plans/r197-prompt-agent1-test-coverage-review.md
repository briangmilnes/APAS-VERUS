# R197 Prompt — Agent 1: RTT coverage review and gap-fill. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER modify code under `src/` to make a test pass.** Tests are
   the consumers; if a test fails, it's the test that's wrong (or it
   exposes a real bug — STOP and report, do not silently change `src/`).
7. **NEVER touch `Example*.rs` or `Problem*.rs` files** (textbook
   demos / problem sets — out of scope per CLAUDE.md).

## Read all standards first.

Pay extra attention to: `iterators_standard.rs`, `using_closures_standard.rs`,
`mt_type_bounds_standard.rs`. Test patterns must match the standards.

## Context

We have **293 RTT files** across **43 of 44 src chapters** (Chap30 has
no tests). Coverage was never systematically audited. Some files have
many tests, others may have only `cargo test` smoke. APAS-VERUS now
ships ~5674 verified Verus items; the runtime test surface should
match.

## Goal

Audit RTT coverage across all `Algorithm*.rs` files in `src/Chap*/`
and add tests where coverage is missing or thin. **Do not test
`Example*.rs` or `Problem*.rs`** (CLAUDE.md rule).

## Plan

### Step 1: Inventory

Build a table `plans/r197-rtt-coverage-inventory.md` with one row per
`Algorithm*.rs` (and any other production file that isn't `Example*`
or `Problem*`):

| # | Chap | File | Tests in tests/ChapNN/ | Functions in file | Coverage status |
|---|------|------|------------------------|-------------------|-----------------|
| 1 | 03   | InsertionSortStEph.rs | 5 | 4 (sort, ...) | full |
| 2 | 65   | UnionFindPCStEph.rs | 24 | 8 | full |
| 3 | NN   | …                     | 0 | 12 | NONE |

Definitions:
- "Tests" = count of `#[test]` annotations in any file under
  `tests/ChapNN/` that imports the source file.
- "Functions in file" = `pub fn` count in the trait, excluding spec/proof.
- "Coverage status" = one of:
  - **NONE** — no `#[test]` exercises any function from the file.
  - **smoke** — only one or two tests, basic happy path.
  - **partial** — some functions tested, some not.
  - **full** — every public function has at least one test exercising it.

Use `grep`/`Grep`-tool to count, do not estimate.

### Step 2: Identify gaps

From the inventory, list every file that is **NONE**, **smoke**, or
**partial**. For partial files, list the specific untested functions.

Also flag:
- Chapters with zero tests (especially Chap30, currently uncovered).
- Mt files that lack thread-safety stress tests (concurrent insert/lookup).
- Iterator-bearing collections that lack the 6 iterator test patterns
  from `docs/APAS-VERUSIterators.rs` (loop-borrow-iter, loop-borrow-into,
  for-borrow-iter, for-borrow-into, loop-consume, for-consume).

### Step 3: Fill gaps — but bounded scope

Add tests for the **top 20 highest-impact gaps** this round. Do NOT
attempt to close every gap in one round (would explode scope).
"Highest impact" = combination of:
- Algorithm files with **NONE** coverage (max impact).
- Files with **0 hole**, **strong spec** verification — these are
  algorithms we can confidently exercise.
- Mt files lacking thread-stress tests.

For the remaining gaps, capture them in
`plans/r197-rtt-coverage-gaps-remaining.md` so future rounds have a
ready punch list.

### Step 4: Test patterns

Each new test should:
- Live in `tests/ChapNN/Test<FileBaseName>.rs`. If the file already
  exists, **append** new `#[test]` functions (do not create
  duplicates). If it doesn't exist, create it and register the entry
  in `Cargo.toml` under `[[test]]`.
- Use deterministic inputs (no `rand` without a fixed seed).
- Cover happy path AND at least one edge case (empty, singleton, full
  capacity, duplicate insert, etc.).
- For Mt files: use `std::thread::spawn` with a join, exercise
  concurrent reads at minimum, concurrent writes when the API permits.
  **Mt tests must have a timeout** (CLAUDE.md rule — use
  `std::sync::mpsc` with `recv_timeout` or `parking_lot`-style guard).
- Each test should run in **under 200ms** (RTT runs all 293+ tests
  serially via nextest; budget matters).

### Step 5: Validate that coverage actually grew

After adding tests:
- Re-run `scripts/rtt.sh`. All tests must pass.
- Re-count `#[test]` annotations in `tests/`.
- Update the inventory table to reflect the new state.
- Compute: tests added, files moved from NONE → smoke / partial /
  full, etc.

### Step 6: NO code changes in src/

If a new test exposes a bug (panic, wrong output), **STOP** and write
it up in the report under "Bugs found". Do NOT modify `src/` to make
the test pass. The user reviews bug reports and decides next steps.
This rule exists because src/ has been carefully verified — silent
edits to make tests pass can hide real issues.

If a test reveals that the trait API is missing something (e.g.,
collection has no way to query size), **note it but do not add it**.
API additions are user decisions.

## Out of scope

- Any modification to `src/`.
- `Example*.rs` and `Problem*.rs` files.
- Coverage of `vstdplus/` (it's a library, not algorithm code).
- Tests for chapters whose lib.rs registration is currently commented out.
- Trying to close >20 gaps in one round — split across rounds.

## Validation

```bash
scripts/rtt.sh
```

All 293+ tests must pass. Total wall time should stay under 2x current.
If a single test takes more than 200ms, replace with smaller input.

## Report

Write `plans/agent1-round197-report.md` with:

- Coverage inventory summary table (chapter-level: # files NONE /
  smoke / partial / full).
- Tests added this round (per file, count + brief description).
- Files moved into stronger coverage tier.
- New RTT count, wall time, vs prior.
- Any **bugs found** (separate prominent section).
- Path to the gap-list file for future rounds.

## RCP

`git add -A && git commit -m "R197 Agent 1: RTT coverage audit + gap-fill (top 20)"`,
then `git push`.
