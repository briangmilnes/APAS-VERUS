# R34 Agent 3: Quick Wins — Close Chap37, Chap45, Chap57, Chap59

## Goal

Close or reduce 4 chapters by proving small numbers of real holes.
5-6 real holes total across 4 chapters.

## Targets

### Chap37 (1 real hole, closes chapter)

- **AVLTreeSeqMtPer.rs** line 509: 1 external_body (NOT subseq_copy
  at line 630 — that's OPAQUE_EXTERNAL FP). Identify the function,
  remove external_body, prove it.
- AVLTreeSeq.rs line 1117: STD_TRAIT_IMPL FP (Iterator::next). Skip.

### Chap45 (1 real hole)

- **BalancedTreePQ.rs**: 1 assume in `extract_all_sorted` (sortedness
  of extracted sequence). Agent3's R33 work added proof infrastructure:
  `lemma_heap_parent_le`, `spec_almost_exec_heap`. Use these to prove
  that repeatedly extracting min from a heap produces sorted output.
  The loop invariant: result so far is sorted, and the last extracted
  element ≤ the heap minimum (which is ≤ all remaining elements).

### Chap57 (2 real holes)

- **DijkstraStEphU64.rs**:
  - Line 202: `assume(BinaryHeapPQ::spec_is_exec_heap(pq.spec_seq()))` —
    prove the PQ maintains heap property through insertions
  - Line 243: `assume(remaining_budget > 0)` — prove the budget
    tracking invariant
  - Lines 101, 118: STD_TRAIT_IMPL FP (cmp, partial_cmp). Skip.

### Chap59 (1 real hole)

- **JohnsonStEphI64.rs** line 445: `assume(reweighted@.A.len() * 2 + 2 <= usize::MAX)` —
  prove the graph size bound carries through from input graph to reweighted graph.

## Priority

1. Chap37 (quickest, closes a chapter)
2. Chap59 Johnson assume (single line)
3. Chap57 Dijkstra assumes
4. Chap45 BalancedTreePQ (hardest)

## Rules

- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent3-round34-report.md`.
- Commit, push to `agent3/ready`.
