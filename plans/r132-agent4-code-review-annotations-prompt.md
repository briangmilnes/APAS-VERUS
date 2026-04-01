# R132 Agent 4 — Add missing Code review annotations. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r132-agent4-annotations-report.md`

## Problem

692 functions have APAS cost spec annotations but no Code review line.

Run this to find them:
```bash
~/projects/veracity/target/release/veracity-analyze-alg-analysis -c ~/projects/APAS-VERUS 2>&1 | grep 'has APAS cost spec but no Code review'
```

## Task

For each function that has `/// - Alg Analysis: APAS (...)` but no
`/// - Alg Analysis: Code review (Claude Opus 4.6):` line, read the code
and add the Code review line.

## Format

```rust
/// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
```

Or if different:
```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential loop
```

For St files: Span = Work (sequential). Use `— DIFFERS: St sequential, APAS parallel`
if the APAS has parallel span.

## Prioritize

Start with chapters that have the most missing annotations. Check the output of the
veracity command above and sort by chapter.

## Rules

- **DO NOT modify code.** Annotations only.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Do NOT change existing APAS or Code review lines.
- Only ADD missing Code review lines.
- Read the function implementation before writing the annotation.

## When done

Commit all changes with `git add -A && git commit` and push.
