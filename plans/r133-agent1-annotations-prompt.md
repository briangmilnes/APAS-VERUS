# R133 Agent 1 — Add missing alg analysis annotations. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r133-agent1-annotations-report.md`

## Task

Add `/// - Alg Analysis:` annotations to functions that are missing them.

Your chapters: **Chap37** (594 functions) and **Chap39** (140 functions) = 734 total.

Run this to find your functions:
```bash
~/projects/veracity/target/release/veracity-analyze-alg-analysis -c ~/projects/APAS-VERUS 2>&1 | grep 'missing alg analysis' | grep -E 'Chap(37|39)'
```

## Format

For each function, read the implementation and add:
```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(...), Span O(...)
```

For St files: Span = Work (sequential).
For Mt files: Span may differ if the function uses join/ParaPair.

If the function has no meaningful cost (e.g., constructor returning a constant, 
trivial getter), use:
```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
```

If the function is a spec/proof function (not exec), skip it — veracity should not
flag those. If it does, the function may be misclassified.

## Rules

- **DO NOT modify code.** Annotations only.
- Do NOT change existing APAS or Code review lines.
- Only ADD missing annotations.
- Read the function body before annotating — get the cost right.

## When done

Commit with `git add -A && git commit` and push.
