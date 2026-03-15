# Agent 1 — Round 21: Review Against Prose — Foundation & Sequences

## Mission

Full review-against-prose for 9 chapters: Chap02, 03, 05, 06, 11, 12, 17, 18, 19.
Follow the 8-phase procedure in `.cursor/rules/apas-verus/review-against-prose.mdc`.

## Your Chapters (48 files)

| # | Chap | Topic | Files |
|---|------|-------|-------|
| 1 | 02 | HFScheduler, InsertionSort, Fibonacci | 2 |
| 2 | 03 | InsertionSort | 1 |
| 3 | 05 | Sets, Relations, Mappings, Kleene | 5 |
| 4 | 06 | Directed/Undirected/Labeled/Weighted Graphs | 20 |
| 5 | 11 | LinkedList | 5 |
| 6 | 12 | Fibonacci numbers | 3 |
| 7 | 17 | MathSeq | 1 |
| 8 | 18 | ArraySeq (Sequences) | 7 |
| 9 | 19 | ArraySeq (Parametric Sequences) | 4 |

## Pre-Generated Inputs (DO NOT regenerate these)

The function inventory is already generated. Read it, don't rerun the tool:

- `src/ChapNN/analyses/veracity-review-module-fn-impls.md` — function inventory per chapter
- `prompts/ChapNN.txt` — APAS textbook prose per chapter
- `src/ChapNN/analyses/veracity-review-verus-proof-holes.log` — proof holes per chapter

## The 8 Phases

For each chapter, execute all 8 phases from the review-against-prose procedure:

### Phase 1: Inventory
Already done — read `src/ChapNN/analyses/veracity-review-module-fn-impls.md`.

### Phase 2: Prose Inventory
Read `prompts/ChapNN.txt`. Extract every named Definition, Algorithm, Cost spec,
Theorem. List them in the review output.

### Phase 3: Algorithmic Analysis
For each exec fn:
- **3a**: Write cost annotation comments in source (APAS line + Claude-Opus-4.6 line).
- **3b**: Note implementation deviations from prose.
- **3c**: Compare ensures against prose postconditions — is the spec faithful?

### Phase 4: Parallelism Review
For Mt modules (SetMtEph, ArraySeqMtEph, etc.), classify each function as
parallel/sequential/delegating. Produce the parallelism gap table.

### Phase 5: Runtime Test Review
Check `tests/ChapNN/` for test coverage of exec functions.

### Phase 6: PTT Review
Check `rust_verify_test/tests/ChapNN/` for iterator/loop coverage.

### Phase 7: Gap Analysis
Prose items with no implementation. Code with no prose counterpart.

### Phase 8: TOC Review
Audit section ordering and in/out placement.

## Output

For each chapter, write: `src/ChapNN/analyses/review-against-prose.md`

This file is the persistent record. Include all 8 phases, tables, and findings.

## Important

- **Every table must have a Chap column** (just the number) after the # index column.
- Read the fn-impls inventory — do NOT rerun `veracity-review-module-fn-impls`.
- Read the prose file before writing cost annotations.
- Read the implementation before writing the Claude-Opus-4.6 cost line.
- Do NOT modify requires/ensures or function signatures. This is a REVIEW, not a fix round.
- Cost annotations (Phase 3a doc comments) ARE code changes — those go in the source files.
- Skip Example files (Example*.rs) per CLAUDE.md policy.
- For Chap06, the 20 files are 4 graph types × 5 variants. Review the StEph files in
  detail; variants just check that specs propagate.
- `scripts/validate.sh` after adding cost annotations — 0 errors.

## Deliverables

- `src/ChapNN/analyses/review-against-prose.md` for each of 9 chapters.
- Cost annotations added to source files (Phase 3a).
- `plans/agent1-round21-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
