# R152 Agent 3 — Fix Remaining Debug/Display + Copyright. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r152-agent3-styling-report.md`

## Task A: Remaining Debug/Display [14] — ~181 warnings

Run the styler to find them:

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[14\]'
```

Tocify may have introduced new [14] warnings by reformatting files. Fix all
of them. Pattern: add `impl Debug` and `impl Display` outside `verus!` but
inside `pub mod`, in section 14.

See `plans/r149-agent1-debug-display-chap05-19.md` for the full pattern
reference (simple structs, generic structs, ghost/marker structs, iterators).

Skip structs that already have `#[derive(Debug)]` inside verus! — adding
a manual impl would conflict.

## Task B: Copyright format [24] — ~68 warnings

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[24\]'
```

Fix line 1 of each flagged file to:
```
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
```

Tocify may have reverted some copyright fixes from R149. Fix them all again.

## Validation

Run `scripts/validate.sh` (full). Then `scripts/rtt.sh`.

## Rules

- Do NOT modify anything inside `verus!`.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
