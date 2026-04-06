# R150 Agent 2 — Decompose StarPartitionMtEph parallel_star_partition. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Pay close attention to `helper_function_placement_standard.rs` — helpers that
operate on bare data types go as `pub(crate)` free functions.

Read `src/Chap62/StarPartitionMtEph.rs` completely before making any changes.
Read `src/Chap62/StarPartitionStEph.rs` for the sequential version as reference.

Report file: `plans/r150-agent2-starpartition-decompose-report.md`

## Problem

`parallel_star_partition` in `src/Chap62/StarPartitionMtEph.rs` has **211 assert
statements** in a single function body (~250 lines of proof in a ~1800 line file).
This causes flaky rlimit failures — Z3 drowns in quantifiers. The CLAUDE.md says
"20 assert lines means something is structurally wrong."

The function also has:
- 5 proof holes (the most of any file in the codebase)
- 3 `assert forall` using `==>` instead of `implies` (warnings)
- 7 auto-trigger warnings (missing explicit `#[trigger]`)

## Goal

Decompose `parallel_star_partition` into smaller helper functions, each with its
own requires/ensures and a manageable proof obligation. The rlimit should stabilize
because each helper gets its own Z3 context.

## Strategy

1. **Read the function** and identify logical phases. StarPartition typically has:
   - Phase 1: Generate random coin flips (heads/tails for each vertex)
   - Phase 2: Identify star centers (heads whose neighbors are all tails, or similar)
   - Phase 3: Build the partition map (assign each vertex to its star center)
   - Phase 4: Collect the centers into a set

2. **Factor each phase into a helper function** with clear requires/ensures.
   Each helper takes the inputs it needs and returns the intermediate result.
   The proof assertions that belong to that phase move into the helper.

3. **The main function becomes a pipeline** calling the helpers sequentially,
   with the helpers' ensures providing the assertions the next helper needs.

4. **Fix warnings while you're in there**: change `==>` to `implies` in the
   `assert forall` blocks, add explicit `#[trigger]` to kill auto-trigger warnings.

## Rules for decomposition

- Each helper should have < 30 assert statements. If a helper needs more, decompose
  further.
- Helpers that operate on bare data (`Vec`, `HashMapWithViewPlus`, `SetStEph`) go as
  `pub(crate)` free functions per Standard 19.
- Helpers that need `&self` on the module type go in the trait.
- Preserve ALL existing ensures on `parallel_star_partition` — the external contract
  must not change.
- Do NOT delete any proof assertions. Move them to the appropriate helper.
- Do NOT add assumes or accepts.
- Do NOT weaken any ensures.

## Validation

Use `scripts/validate.sh isolate Chap62` during development. Run full
`scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

The rlimit error should disappear after decomposition. If a helper still exceeds
rlimit, profile it: `scripts/validate.sh isolate Chap62 --profile`.

## When done

RCP. Report: how many helpers created, assert count per helper, whether rlimit
is stable, remaining holes.
