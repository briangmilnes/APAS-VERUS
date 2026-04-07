# R158 Agent 3 — Fix Debug/Display [14] + Copyright [24]. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r158-agent3-debug-copyright-report.md`

## Task A: Debug/Display [14] — ~181 warnings

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[14\]'
```

Add `impl Debug` and `impl Display` outside `verus!` but inside `pub mod`,
in section 14. Skip structs with `#[derive(Debug)]` inside verus!.

## Task B: Copyright [24] — ~68 warnings

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS \
  -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments \
  -e lib.rs -e standards 2>&1 | grep 'warning: \[24\]'
```

Fix line 1 to: `//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.`

## Validation

`scripts/validate.sh` (full). Then `scripts/rtt.sh`.

## Rules

- Do NOT modify anything inside `verus!`.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
