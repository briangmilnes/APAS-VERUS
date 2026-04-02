# R144 Agent 4 Report — Chap50 assume cleanup + DIFFERS annotations

## Task 1: Proved 3 Chap50 assumes

| # | Chap | File | Assume | Fix |
|---|------|------|--------|-----|
| 1 | 50 | OptBinSearchTreeMtEph.rs | `assume(ps@.len() > i + l)` | Added `(*prefix_sums)@.len() == n + 1` to requires of `obst_rec` and `parallel_min_split_cost`, propagated through closure requires. Prefix sums vec has n+1 elements (loop invariant), i+l <= n from requires. |
| 2 | 50 | OptBinSearchTreeMtPer.rs | `assume(ps@.len() > i + l)` | Same fix as #1. |
| 3 | 50 | OptBinSearchTreeMtPer.rs | `assume(self.memo.pred() == OptBSTMtPerMemoInv)` | Added `spec_optbinsearchtreemtper_wf` predicate to trait/impl capturing `self.memo.pred() == OptBSTMtPerMemoInv`. Constructors ensure wf, `optimal_cost` requires wf. Predicate identity flows from `new_arc_rwlock` ensures. |

## Task 2: Annotated 5 DIFFERS as ACCEPTED DIFFERENCE

| # | Chap | File | Function | Old annotation | New annotation |
|---|------|------|----------|----------------|----------------|
| 1 | 38 | BSTParaMtEph.rs | filter | DIFFERS: sequential recursion in filter_inner (spec_fn not Send) | ACCEPTED DIFFERENCE: Verus limitation; spec_fn not Send, blocks parallel filter |
| 2 | 41 | AVLTreeSetMtEph.rs | filter | DIFFERS: sequential filter (spec_fn not Send) | ACCEPTED DIFFERENCE: Verus limitation; spec_fn not Send, blocks parallel filter |
| 3 | 41 | AVLTreeSetMtPer.rs | filter | DIFFERS: sequential filter (spec_fn not Send) | ACCEPTED DIFFERENCE: Verus limitation; spec_fn not Send, blocks parallel filter |
| 4 | 18 | ArraySeqMtEphSlice.rs | flatten | DIFFERS: Vec concat at each level adds lg factor | ACCEPTED DIFFERENCE: Vec concat at each D&C level; O(1) rejoin needs PCell pre-allocated output |
| 5 | 19 | ArraySeqMtEphSlice.rs | flatten | DIFFERS: Vec concat at each level adds lg factor | ACCEPTED DIFFERENCE: Vec concat at each D&C level; O(1) rejoin needs PCell pre-allocated output |

## Validation

- Isolate Chap50: 829 verified, 0 errors
- Full: 5684 verified, 0 errors
- RTT: 3690 passed, 0 skipped
