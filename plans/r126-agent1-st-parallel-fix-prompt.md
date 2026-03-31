# R126 Agent 1 — Fix St files claiming parallel span in Code review. AFK. DOT.

## Problem

49 Code review annotations on St files (StEph/StPer) incorrectly claim
parallel span (Span != Work). St implementations are sequential — Span
must equal Work.

Run this to get the error list:
```bash
~/projects/veracity/target/release/veracity-analyze-alg-analysis -c ~/projects/APAS-VERUS 2>&1 | grep 'St.*claims parallel'
```

**Ignore** lines where Work == Span (e.g., `Work O(1) but Span O(1)`) — those
are tool false positives and are already correct.

**Fix** lines where Work != Span. For example:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(1)
```
Should be:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — DIFFERS: St sequential, APAS parallel
```

On St files, Span always equals Work because there's no parallelism.
If the line doesn't already say `— DIFFERS`, add it with reason
`St sequential, APAS parallel` or similar.

## Chapters with errors

Chap05 (16), Chap06 (11), Chap18 (26), Chap19 (13), Chap23 (8),
Chap28 (1), Chap35 (2), Chap39 (6), Chap47 (13), Chap56 (8),
Chap57 (6), Chap64 (1)

## Rules

- **DO NOT modify code.** Annotations only.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Only fix lines where Work != Span. Leave correct lines alone.
- Do NOT change APAS lines — only Code review lines.

## When done

Commit all changes with `git add -A && git commit` and push.

## No step limit — finish all files.

## Report

Write `plans/agent1-r126-st-parallel-fix-report.md`.
