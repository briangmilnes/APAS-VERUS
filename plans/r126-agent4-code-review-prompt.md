# R126 Agent 4 — Add missing Code review annotations (remaining chapters). AFK. DOT.

## Task

Your chapters: everything NOT assigned to agents 1-3. That includes:
Chap02(2), Chap03(1), Chap05(0), Chap06(13), Chap11(5), Chap23(1),
Chap26(6), Chap28(1), Chap42(10), Chap44(4), Chap45(18), Chap47(8),
Chap49(16), Chap50(4), Chap51(9), Chap54(8), Chap55(5), Chap57(4),
Chap58(4), Chap59(8), Chap61(4), Chap62(2), Chap63(2), Chap64(4),
Chap65(7), Chap66(5) = 151 functions.

For each function that has `/// - Alg Analysis: APAS (...)` but no
`/// - Alg Analysis: Code review (Claude Opus 4.6):` line, read the code
and add the Code review line.

Run this to find your functions:
```bash
grep 'has APAS cost spec but no Code review' analyses/veracity-analyze-alg-analysis.log | grep -vE 'Chap(18|19|37|38|39|40|41|43|52)'
```

## Format

```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
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

Write `plans/agent4-r126-code-review-report.md`.
