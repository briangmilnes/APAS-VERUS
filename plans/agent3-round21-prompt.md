# Agent 3 — Round 21: Review Against Prose — BSTs & Ordered Collections

## Mission

Full review-against-prose for 10 chapters: Chap37, 38, 39, 40, 41, 42, 43, 44, 45, 47.
Follow the 8-phase procedure in `.cursor/rules/apas-verus/review-against-prose.mdc`.

## Your Chapters (62 files)

| # | Chap | Topic | Files |
|---|------|-------|-------|
| 1 | 37 | BST variants (Plain, AVL, RB, BBAlpha, Splay, Sets) | 19 |
| 2 | 38 | BST Parallel (BSTParaStEph, BSTParaMtEph) | 2 |
| 3 | 39 | Treap (BSTTreap, BSTParaTreap, BSTSetTreap) | 4 |
| 4 | 40 | BST Augmented (KeyValue, Size, Reduced) | 3 |
| 5 | 41 | Sets (ArraySet, AVLTreeSet) | 6 |
| 6 | 42 | Tables (TableStEph, StPer, MtEph) | 3 |
| 7 | 43 | Ordered Tables & Sets (Ordered, AugOrdered) | 10 |
| 8 | 44 | Document Index | 1 |
| 9 | 45 | Priority Queues (BinaryHeap, Leftist, Sorted, Unsorted, BalancedTree) | 5 |
| 10 | 47 | Hash Tables (Chained, Flat, LinProb, QuadProb, DoubleHash) | 9 |

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
- Chap37 has 19 files — many are Mt wrappers. Review the StEph files in detail;
  Mt files check parallelism (Phase 4) and that specs propagate.
- Chap37 BSTs share a common multi-struct pattern (Leaf/Interior/Node/Tree). Read
  `src/standards/multi_struct_standard.rs` to understand the expected style.
- Chap43 has both Ordered and AugOrdered variants — review StEph in detail,
  check that AugOrdered extends correctly.
- Chap45 PQ files implement different heap strategies — compare each against prose.
- Chap47 hash tables: review ParaHashTableStEph.rs (infrastructure) + strategy files.
  Note the ghost hash / opaque hash issue for lookup correctness.
- Do NOT modify requires/ensures or function signatures.
- Cost annotations (Phase 3a) go in source files as doc comments.
- `scripts/validate.sh` after adding cost annotations — 0 errors.

## Deliverables

- `src/ChapNN/analyses/review-against-prose.md` for each of 10 chapters.
- Cost annotations added to source files.
- `plans/agent3-round21-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
