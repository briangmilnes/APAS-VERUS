# r228: algorithmic-analysis review of every chapter (review only, no fixes)

Branch `r228/alg-review`, worktree `~/projects/APAS-VERUS-r228`, from main
`fc9b61116`. Started 2026-09-26.

## Goal

Every `/// - Alg Analysis:` annotation in `src/ChapNN/` is reviewed against two
things: the code, and the APAS textbook (`prompts/ChapNN.txt`). The review adds
one new comment line per annotated function. It changes no code, spec, proof,
or existing annotation. After all chapters are done, one document summarizes
the findings.

Scale: 44 chapters, about 250 files, about 7,700 annotation lines (1,840 APAS
lines and 5,880 Code-review lines).

## The new line

Directly below the function's existing `/// - Alg Analysis:` lines, add:

```
/// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(...), Span O(...) — <verdict>
```

- The Work and Span are your own analysis of the function body, from reading
  the code. Use `Work O(...), Span O(...)` in the variables of
  `src/standards/alg_analysis_notation_standard.rs`.
- `<verdict>` is one or more of these phrases, separated by `; `:
  - `matches textbook`: the new analysis equals the APAS cost (the APAS line,
    or the cost the textbook prose states for this algorithm).
  - `does not match textbook: <reason>`: the new analysis differs from APAS.
    Give the concrete cause, for example "Vec append copies, Span O(n) not
    O(1)", or "sequential loop; APAS parallel".
  - `does not match old analysis: <old> vs new; <reason>`: the new analysis
    differs from the latest existing Code-review line.
  - `no textbook cost`: APAS states no cost for this function (helpers,
    accessors, spec-support code). Then only the old-analysis comparison
    applies.
- If the new analysis matches both the textbook and the old analysis, the
  verdict is `matches textbook` alone.
- Use the date of the day you do the review (`date +%F`).

## Rules

- Read each function body before writing its line. Never infer cost from the
  name. Never batch-insert lines with sed or other text substitution. Insert
  each line with the Edit tool after reading the function.
- Record the textbook costs of each algorithm (Algorithm, Cost
  Specification, and Exercise numbers). Read the chapter's prose,
  `prompts/ChapNN.txt`, before reviewing its files.
- Include span: follow `join`/`ParaPair`/HFScheduler calls, closures, and
  callees. A callee's cost comes from its own body or its reviewed annotation.
- Change nothing else: no code, spec, proof, existing comment, or formatting.
  If an existing annotation is malformed (for example, glued onto a
  `// Veracity:` line), leave it and record it in the chapter report.
- Functions with no Alg Analysis annotation: do not add one. List them in the
  chapter report as unannotated.
- No verification runs by the agents (validate, rtt, ptt). The main session
  validates each batch. Doc comments in the wrong position can break parsing:
  place the new line inside the existing `///` block, directly after the last
  `Alg Analysis` line.
- No git commands that change state (commit, checkout, reset, stash, clean).
  The main session commits.
- No formatters, no Python, and no Perl.

## Per-chapter report

`plans/r228-alg-review/ChapNN.md`:

1. The textbook cost specs found in `prompts/ChapNN.txt` (a table).
2. One row per reviewed function:

   | # | Chap | File | Function | APAS | Old review | New review | Verdict |

   Keep cells under 40 characters and abbreviate; use footnotes for longer
   reasons.
3. Counts: matches textbook; does not match textbook; does not match old
   analysis; no textbook cost; unannotated functions; malformed annotations.
4. Notable findings: wrong old analyses, misleading names, missing
   parallelism, and cost regressions against APAS.

## Batches (3 agents at a time, in order)

| # | Chap | Chapters | Annotation lines |
|---|---|---|---|
| 1 | 02-17 | 02, 03, 05, 11, 12, 17 | 266 |
| 2 | 06 | 06 | 814 |
| 3 | 18 | 18 | 478 |
| 4 | 19-23 | 19, 21, 23 | 402 |
| 5 | 26-36 | 26, 27, 28, 30, 35, 36 | 257 |
| 6 | 37 | 37 | 999 |
| 7 | 38-39 | 38, 39 | 497 |
| 8 | 40-41 | 40, 41 | 644 |
| 9 | 42-45 | 42, 44, 45 | 523 |
| 10 | 43 | 43 | 1059 |
| 11 | 47-50 | 47, 49, 50 | 524 |
| 12 | 51-52 | 51, 52 | 618 |
| 13 | 53-56 | 53, 54, 55, 56 | 362 |
| 14 | 57-66 | 57, 58, 59, 61, 62, 63, 64, 65, 66 | 298 |

## Main-session steps per batch

1. Check that the diff adds only comment lines
   (`git diff --numstat`; every changed line starts with `///`).
2. Run `scripts/validate.sh isolate ChapNN` for the batch's highest chapter.
   Only one validate runs at a time.
3. Commit `r228: alg-analysis review ChapNN..` and push the branch.
4. Launch the next batch.

## Final document

`analyses/r228-alg-analysis-review.md`, compiled from the chapter reports:

- totals, per chapter and overall;
- every "does not match textbook" item, grouped by cause (Vec-backed,
  sequentialized, representation choice, analysis precision, true defect);
- every "does not match old analysis" item (old annotations that were wrong);
- unannotated and malformed lists;
- `scripts/check-alg-analysis.sh` output before and after.

It adds no fixes. Fixes are a later round.
