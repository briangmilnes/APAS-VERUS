# Agent 4 — Round 165 Report

## Summary

Extracted shared BFS and DFS specs/lemmas into two new dedicated files, following the
`src/Chap42/TableSpecsAndLemmas.rs` pattern. Both Chap54 and Chap55 verify clean.

---

## Holes Before/After

No holes changed this round (pure refactor — no new proofs, no new assumes).

---

## Chapters Closed

None. This was a structural compression round.

---

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 54 | BFSSpecsAndLemmas.rs | NEW (171 lines): shared BFS specs/lemmas over `Seq<usize>` |
| 2 | 54 | BFSStEph.rs | Use shared file; wrapper lemmas bridge spec_index→@ |
| 3 | 54 | BFSStPer.rs | Use shared file; wrapper lemmas bridge spec_index→@ |
| 4 | 54 | BFSMtEph.rs | Use shared file; wrapper lemmas bridge spec_index→@ |
| 5 | 54 | BFSMtPer.rs | Use shared file; wrapper lemmas bridge spec_index→@ |
| 6 | 55 | DFSSpecsAndLemmas.rs | NEW (188 lines): spec_num_false, 4 bool lemmas, 6 bridge lemmas |
| 7 | 55 | TopoSortStEph.rs | Removed 9 items now in DFSSpecsAndLemmas; added import |
| 8 | 55 | TopoSortStPer.rs | Removed 2 Per bridge lemmas; updated import |
| 9 | 55 | DFSStEph.rs | Removed 3 local bridge lemmas; updated import |
| 10 | 55 | DFSStPer.rs | Removed 2 local bridge lemmas; updated import |
| 11 | 55 | CycleDetectStEph.rs | Removed 2 local bridge lemmas; updated import |
| 12 | 55 | CycleDetectStPer.rs | Removed 2 local bridge lemmas; updated import |
| 13 | 55 | SCCStEph.rs | Removed 3 local bridge lemmas; updated import |
| 14 | 55 | SCCStPer.rs | Removed 2 local bridge lemmas; updated import |

Net across 13 files: **+108 insertions, −422 deletions** (−314 net lines).

---

## What DFSSpecsAndLemmas Contains

- `spec_num_false` — termination measure for all DFS algorithms
- `lemma_set_true_decreases_num_false` — decreases lemma for DFS termination
- `lemma_set_true_num_false_eq` — exact count for DFS termination
- `lemma_all_true_num_false_zero` — base case lemma
- `lemma_all_false_num_false_eq_len` — base case lemma
- `lemma_bool_view_eq_spec_index` (Eph) — view bridge for bool arrays
- `lemma_bool_array_set_view` (Eph) — set-update view bridge for bool arrays
- `lemma_usize_view_eq_spec_index` (Eph) — view bridge for usize arrays
- `lemma_graph_view_bridge` (Eph) — adjacency list view bridge
- `lemma_usize_per_view_eq_spec_index` (Per) — view bridge for persistent usize arrays
- `lemma_graph_per_view_bridge` (Per) — persistent adjacency list view bridge

---

## Key Technical Note: Why BFS Bridge Lemmas Are Wrappers

Chap54 BFS files use `spec_index`-based spec fns in their algorithm invariants (not `@`-based).
`BFSSpecsAndLemmas` operates on abstract `Seq<usize>`. Bridge lemmas in each BFS file:
1. Assert `forall|i| X@[i] == X.spec_index(i)` (bridge assertion, works via open spec fn unfolding)
2. Then call the shared Seq-based lemma with `X@`

Chap55 DFS files already use `@`-based notation in their spec fns, so no bridge wrappers
are needed — they call DFSSpecsAndLemmas lemmas directly.

---

## Verification Results

| Step | Count | Errors |
|------|-------|--------|
| `validate.sh isolate Chap54` | 1370 verified | 0 |
| `validate.sh isolate Chap55` | 2389 verified | 0 |
