# R126 Agent 3 — Add missing Code review annotations (Chap18/19/38/40/41/52). AFK. DOT.

## Task

Your chapters: Chap18 (45), Chap19 (24), Chap38 (29), Chap40 (39), Chap41 (31), Chap52 (80) = 248 functions.

For each function that has `/// - Alg Analysis: APAS (...)` but no
`/// - Alg Analysis: Code review (Claude Opus 4.6):` line, read the code
and add the Code review line.

Run this to find your functions:
```bash
grep 'has APAS cost spec but no Code review' analyses/veracity-analyze-alg-analysis.log | grep -E 'Chap(18|19|38|40|41|52)'
```

## Format

```rust
/// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg n), Span O(lg n)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
```

Or `— DIFFERS: reason`. For St files: Span = Work (sequential).

## Rules

- **DO NOT modify code.** Annotations only.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Do NOT change existing lines. Only ADD missing Code review lines.

## When done

Commit all changes with `git add -A && git commit` and push.

## No step limit — finish all files.

## Report

Write `plans/agent3-r126-code-review-report.md`.
