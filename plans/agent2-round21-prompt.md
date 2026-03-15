# Agent 2 — Round 21: Review Against Prose — Trees, Sorting, Contraction

## Mission

Full review-against-prose for 8 chapters: Chap21, 23, 26, 27, 28, 30, 35, 36.
Follow the 8-phase procedure in `.cursor/rules/apas-verus/review-against-prose.mdc`.

## Your Chapters (45 files)

| # | Chap | Topic | Files |
|---|------|-------|-------|
| 1 | 21 | Examples (primes, points, Cartesian) | 12 |
| 2 | 23 | Tree Sequences (BalBinTree, PrimTreeSeq) | 2 |
| 3 | 26 | Divide & Conquer (MergeSort, Scan, eTSP) | 8 |
| 4 | 27 | Contraction (list/star contraction) | 4 |
| 5 | 28 | MCSS (max contiguous subsequence sum) | 11 |
| 6 | 30 | Probability | 1 |
| 7 | 35 | OrderStatSelect (quickselect) | 4 |
| 8 | 36 | QuickSort | 3 |

## Pre-Generated Inputs (DO NOT regenerate these)

- `src/ChapNN/analyses/veracity-review-module-fn-impls.md` — function inventory
- `prompts/ChapNN.txt` — APAS textbook prose
- `src/ChapNN/analyses/veracity-review-verus-proof-holes.log` — proof holes

## The 8 Phases

Execute all 8 phases from `.cursor/rules/apas-verus/review-against-prose.mdc` for
each chapter:

1. **Inventory** — read the fn-impls file (already generated).
2. **Prose Inventory** — read `prompts/ChapNN.txt`, extract named items.
3. **Algorithmic Analysis** — cost annotations (3a), implementation fidelity (3b),
   spec fidelity (3c).
4. **Parallelism Review** — Mt modules only.
5. **Runtime Test Review** — check `tests/ChapNN/`.
6. **PTT Review** — check `rust_verify_test/tests/ChapNN/`.
7. **Gap Analysis** — prose with no code, code with no prose.
8. **TOC Review** — section ordering, in/out.

## Output

For each chapter, write: `src/ChapNN/analyses/review-against-prose.md`

## Important

- **Every table must have a Chap column** after the # index column.
- Chap21 is all Example files — skip per CLAUDE.md policy. Write a brief
  review-against-prose.md noting they're examples.
- Chap30 (Probability) may have limited prose. Note what's there.
- Chap26/28 have both StPer and MtEph variants — review StPer in detail,
  check Mt for parallelism (Phase 4).
- Do NOT modify requires/ensures or function signatures.
- Cost annotations (Phase 3a) go in source files as doc comments.
- `scripts/validate.sh` after adding cost annotations — 0 errors.

## Deliverables

- `src/ChapNN/analyses/review-against-prose.md` for each of 8 chapters.
- Cost annotations added to source files.
- `plans/agent2-round21-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
