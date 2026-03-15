# Agent 4 — Round 21: Review Against Prose — DP & Graphs

## Mission

Full review-against-prose for 17 chapters: Chap49–59, 61–66.
Follow the 8-phase procedure in `.cursor/rules/apas-verus/review-against-prose.mdc`.

## Your Chapters (92 files)

| # | Chap | Topic | Files |
|---|------|-------|-------|
| 1 | 49 | DP: MinEditDist, SubsetSum | 8 |
| 2 | 50 | DP: MatrixChain, OptBinSearchTree | 8 |
| 3 | 51 | DP: BottomUpDP, TopDownDP | 8 |
| 4 | 52 | Graph Representations (AdjMatrix, AdjSeq, AdjTable, EdgeSet) | 14 |
| 5 | 53 | Graph Search (GraphSearch, PQMin) | 5 |
| 6 | 54 | BFS | 4 |
| 7 | 55 | DFS, CycleDetect, SCC, TopoSort | 8 |
| 8 | 56 | Shortest Paths Results (SSSP, AllPairs, PathWeightUtils) | 10 |
| 9 | 57 | Dijkstra, Stack | 3 |
| 10 | 58 | BellmanFord | 2 |
| 11 | 59 | Johnson | 4 |
| 12 | 61 | Edge Contraction, Vertex Matching | 4 |
| 13 | 62 | Star Contraction, Star Partition | 4 |
| 14 | 63 | Connectivity | 2 |
| 15 | 64 | SpanTree, TSPApprox | 3 |
| 16 | 65 | Kruskal, Prim, UnionFind | 3 |
| 17 | 66 | Boruvka | 2 |

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
- You have the most chapters (17) but many are small (2–4 files). Batch the
  small graph chapters (Chap57–59, 61–66) efficiently.
- Chap49–51 (DP) have both StPer and MtEph variants — review StEph/StPer in detail,
  check Mt for parallelism (Phase 4).
- Chap52 (Graph Representations) has 14 files across 4 representations × variants.
  Review one StEph per representation in detail; variants check spec propagation.
- Chap56 has F64/I64 duplicate files. Review the I64 version; note any F64 differences.
  The float axiom situation (see CLAUDE.md Float/Graph section) may affect specs.
- Chap57–59 (Dijkstra, BellmanFord, Johnson) are the shortest-path family. Compare
  against prose carefully — these algorithms have subtle spec requirements.
- Chap61–66 (graph contraction, connectivity, MST) — check that parallel algorithms
  (MtEph) actually use fork-join, not sequential loops.
- Do NOT modify requires/ensures or function signatures.
- Cost annotations (Phase 3a) go in source files as doc comments.
- `scripts/validate.sh` after adding cost annotations — 0 errors.

## Deliverables

- `src/ChapNN/analyses/review-against-prose.md` for each of 17 chapters.
- Cost annotations added to source files.
- `plans/agent4-round21-report.md`
- 0 errors on validate.
- Commit + push to `agent4/ready`.
