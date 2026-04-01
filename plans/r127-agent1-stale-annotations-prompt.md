# R127 Agent 1 — Fix stale DIFFERS annotations + remaining St parallel claims. AFK. DOT.

## Task A: Fix 14 stale Mt DIFFERS annotations (Chap35, 36, 38, 66)

These files already use `join()` or `ParaPair!` for parallelism, but their Code review
annotations still say "sequential." Re-read each flagged function's **implementation**
(not just the trait signature), determine the real Work/Span, and update the Code review
annotation.

Run this to find them:
```bash
~/projects/veracity/target/release/veracity-analyze-alg-analysis -c ~/projects/APAS-VERUS 2>&1 | grep 'Mt fn.*DIFFERS' | grep -E 'Chap(35|36|38|66)'
```

For each function:
1. Read the **impl body** (not just the trait decl) — look for `join()`, `ParaPair!`, recursive calls.
2. Compute the real Span from the code. If it uses `join()` for recursion, Span = depth of recursion.
3. Update the Code review line. If Span now matches APAS, use `— matches APAS`. If it still
   differs (e.g., sequential partition dominates in quicksort), keep `— DIFFERS: reason` but
   update the reason to reflect what the code actually does.

**Chap38 specifics**: `union_inner`, `intersect_inner`, `difference_inner` use `ParaPair!`
for recursive calls. `filter` delegates to `filter_parallel` which also uses `ParaPair!`.
`reduce` delegates to `reduce_parallel`. Check each one.

**Chap35/36 specifics**: These use `join()` for recursive calls but partition is sequential.
The DIFFERS reason should reflect this accurately: "parallel recursion via join(), but
sequential O(n) partition dominates span."

**Chap66 specifics**: Internal helpers (`hash_coin_flips_mt`, `compute_remaining_mt`,
`collect_mst_labels_mt`) use `ParaPair!`. The top-level `boruvka_mst_mt` orchestrates
these parallel steps in a sequential loop. Read the full structure before annotating.

## Task B: Fix 67 remaining St parallel Code review claims

These are annotations on St files where the annotation text includes "Parallelism O(1)"
or similar parallelism metadata. Work == Span (correct for St), but the "Parallelism"
text triggers veracity's detector.

Run this to find them:
```bash
~/projects/veracity/target/release/veracity-analyze-alg-analysis -c ~/projects/APAS-VERUS 2>&1 | grep 'St.*claims parallel'
```

Files affected: Chap06, Chap18, Chap19, Chap23, Chap35, Chap47, Chap56, Chap57, Chap64.

**Fix**: Remove the `Parallelism O(...)` portion from the Code review annotation on St
files. St files have no parallelism — the annotation should be just Work and Span.

Before:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n), Parallelism O(1) - sequential
```
After:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
```
Or if Work/Span differs from APAS:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: St sequential, APAS parallel
```

Do NOT change the Work or Span values — only remove the Parallelism metadata.

## Rules

- **DO NOT modify code.** Annotations only.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Do NOT change APAS lines — only Code review lines.
- Read the impl body before updating any DIFFERS annotation.

## When done

Commit all changes with `git add -A && git commit` and push.

## No step limit — finish all files.

## Report

Write `plans/agent1-r127-report.md` with tables:
- Task A: table of updated DIFFERS (# | Chap | File | Function | Old annotation | New annotation)
- Task B: count of St parallel fixes per file
