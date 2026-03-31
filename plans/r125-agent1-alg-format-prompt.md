# R125 Agent 1 — Standardize alg analysis annotations (Chap02-23). AFK. DOT.

## Task

Reformat all `/// - APAS: Work ...` lines in your chapters to the standard format,
and add Code review lines where missing.

## Your chapters

Chap02, Chap03, Chap05, Chap06, Chap11, Chap12, Chap17, Chap18, Chap19, Chap21, Chap23

## What to fix

### 1. Old format cost spec lines

Change:
```rust
/// - APAS: Work Θ(|v|), Span Θ(1)
```
To:
```rust
/// - Alg Analysis: APAS (ChNN ref NN.NN): Work O(|v|), Span O(1)
```

- Add the APAS reference: `(ChNN CS NN.NN)` or `(ChNN Alg NN.NN)` etc.
  Read `prompts/ChapNN.txt` to find which cost spec / algorithm / theorem
  the function corresponds to.
- Change `Θ` to `O` for consistency.
- Keep the Work/Span values — do NOT change the complexity, just the notation.

### 2. Old format algorithm reference lines

Change:
```rust
/// - APAS: Algorithm 19.8 — iterate (iterative).
```
To:
```rust
/// - Alg Analysis: APAS (Ch19 Alg 19.8): iterate (iterative).
```

### 3. Add Code review line where missing

If a function has an `Alg Analysis: APAS` line but no `Code review (Claude Opus 4.6):` line,
read the code and add one:
```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
```
Or `— DIFFERS: reason` if different.

### 4. Lines already in correct format

If a function already has both `Alg Analysis: APAS (ChNN ...)` and
`Code review (Claude Opus 4.6):` lines, leave them alone.

## Process

1. Read `prompts/ChapNN.txt` for the textbook references.
2. Do NOT run `veracity-annotate-alg-analysis-from-toml`.
3. For each file, find all `/// - APAS:` lines, reformat, add Code review.
4. Do NOT modify code — annotations only.

## No step limit — finish all chapters.

## When done

Commit all changes with `git add -A && git commit` and push.

## Report

Write `plans/agent1-r125-alg-format-report.md`.
