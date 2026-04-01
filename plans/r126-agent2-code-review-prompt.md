# R126 Agent 2 — Add missing Code review annotations (Chap37, Chap39, Chap43). AFK. DOT.

## Task

623 functions have APAS cost spec annotations but no Code review line.
Your chapters: Chap37 (58), Chap39 (82), Chap43 (84) = 224 functions.

For each function that has `/// - Alg Analysis: APAS (...)` but no
`/// - Alg Analysis: Code review (Claude Opus 4.6):` line, read the code
and add the Code review line.

Run this to find your functions:
```bash
grep 'has APAS cost spec but no Code review' analyses/veracity-analyze-alg-analysis.log | grep -E 'Chap(37|39|43)'
```

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

## Rules

- **DO NOT modify code.** Annotations only.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Do NOT change existing APAS or Code review lines.
- Only ADD missing Code review lines.
- Read `prompts/ChapNN.txt` if you need textbook context.

## When done

Commit all changes with `git add -A && git commit` and push.

## No step limit — finish all files.

## Report

Write `plans/agent2-r126-code-review-report.md`.
