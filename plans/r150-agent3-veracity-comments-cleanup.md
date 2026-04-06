# R150 Agent 3 — Remove "Veracity: added" Comments. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r150-agent3-veracity-comments-report.md`

## Problem

79 lines across the codebase have `// Veracity: added` comments. These are
annotations left by a veracity tool pass marking broadcast groups it inserted.
The annotations add no value — the broadcast groups are real code, the comments
are noise.

## How to find them

```bash
grep -rn '// Veracity: added' src/ --include="*.rs" | grep -v experiments
```

## What to do

For each occurrence, remove ONLY the `// Veracity: added ...` comment text.
These are inline comments on real code lines, so you must preserve the code.

Two patterns:

### Pattern 1: Standalone comment line

```rust
// Veracity: added broadcast group
broadcast use vstd::seq::group_seq_axioms;
```

Delete the entire comment line. Keep the `broadcast use` line.

### Pattern 2: End-of-line comment

```rust
broadcast use vstd::seq::group_seq_axioms; // Veracity: added
```

Remove ` // Veracity: added` from the end. Keep the code.

## Rules

- Do NOT modify any code — only remove comment text.
- Do NOT modify files in `src/experiments/`.
- Do NOT add assumes, accepts, or external_body.
- Preserve all whitespace and formatting of the code lines.

## Validation

Run `scripts/validate.sh` (full). Then `scripts/rtt.sh`.

## When done

RCP. Report: files modified, comments removed.
