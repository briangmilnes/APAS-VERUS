# Agent 1 — Round 10 Prompt

## Mission

Close Chap37 (6 real holes), close Chap45 (4 holes), close Chap50 (2 fn_missing_ensures).

## Your Files (no other agent touches these)

**Chap37** (6 real holes):
- `AVLTreeSeq.rs` — 1 external_body (next() iterator)
- `AVLTreeSeqStPer.rs` — 2 assume (set_rec obeys_feq_clone, build_balanced clone bridge)
- `AVLTreeSeqMtPer.rs` — 1 assume (set_rec) + 2 external_body (build_balanced, subseq_copy)
- `BSTSplayStEph.rs` — trivial_wf only (not a real hole — give it a real wf spec)
- BST*MtEph files — fn_missing_requires cleanup (not real holes but errors in analysis)

**Chap45** (4 real holes):
- `BinaryHeapPQ.rs` — 1 assume (sorted after extract_all_sorted)
- `BalancedTreePQ.rs` — 1 external_body (contains)
- `Example45_2.rs` — SKIP (Example files are demo code, not worth proving)
- `HeapsortExample.rs` — SKIP (Example file)

**Chap50** (0 real holes, 2 fn_missing_ensures):
- `OptBinSearchTreeMtEph.rs` — add ensures to obst_rec
- `OptBinSearchTreeMtPer.rs` — add ensures to obst_rec

## Priority Order

1. **Chap50** — trivial. Copy ensures from StEph/StPer versions of obst_rec. 10 minutes.
2. **Chap37** — you have deep context from Round 9. The 6 remaining are feq/clone bridges.
3. **Chap45** — you said "structural" last round. This round, actually try.

## Specific Guidance

### Chap37 Remaining Holes

| # | File | Hole | What You Said Blocks It | What to Try |
|---|------|------|------------------------|-------------|
| 1 | AVLTreeSeq.rs | ext_body next() | "feq bridge for generic clone" | Strengthen next() spec. If clone is the issue, use the recursive Box clone pattern you already proved. |
| 2 | AVLTreeSeqStPer.rs | assume set_rec | "obeys_feq_clone bridge for Arc" | Lift feq_clone into requires. Read using_closures_standard.rs. |
| 3 | AVLTreeSeqStPer.rs | assume build_balanced | "clone bridge: val@ == a@[mid]@" | Strengthen Clone::clone ensures for the element type. |
| 4 | AVLTreeSeqMtPer.rs | assume set_rec | Same as StPer #2 | Same approach. |
| 5 | AVLTreeSeqMtPer.rs | ext_body build_balanced | "&[T] lifetime vs 'static thread boundary" | This is a real thread boundary. May be irreducible. |
| 6 | AVLTreeSeqMtPer.rs | ext_body subseq_copy | "Mutex slots" | Same — thread boundary. |

For #5 and #6: if truly thread boundary, that's acceptable. Document what you tried.

### Chap45 — Actually Try

- **BinaryHeapPQ sorted assume**: The heap property IS the sortedness invariant for
  extract_all_sorted. Each delete_min removes the minimum. If delete_min ensures the
  result is ≤ all remaining elements, induction gives sorted output. Strengthen
  delete_min's ensures if needed.

- **BalancedTreePQ contains**: This is an AVL tree search returning whether element exists.
  The tree is sorted. Write the contains body as a tree traversal with `decreases`.

- **Example45_2 and HeapsortExample**: SKIP. Example files are demo code, not proof targets.

### Chap50

Copy the ensures from `obst_rec` in OptBinSearchTreeStEph.rs to the MtEph/MtPer versions.
The function signatures should match. Validate. Done.

## Rules

- Read `src/standards/using_closures_standard.rs` before touching closure requires.
- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversions.
- Push to `agent1/ready`.
- Write `plans/agent1-round10-report.md`.

## Targets

- Chap37: ≤ 3 holes
- Chap45: ≤ 2 holes
- Chap50: closed (0 holes)
