# r222 report: weight balancing restored in BB[α] (Chap37 StEph and MtEph)

Branch `r222/bbalpha` (worktree `~/projects/APAS-VERUS-r222`). The branch starts
from `r221/redblack`, because r221 is not merged into main yet; merging r222
brings r221 with it. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Result

| # | Chap | Measure | Value |
|---|---|---|---|
| 1 | — | Full `scripts/validate.sh` | 5817 verified, 0 errors |
| 2 | — | Full `scripts/rtt.sh` | 4367 of 4368 pass; 1 failure¹ |
| 3 | — | `scripts/ptt.sh` | 328 of 328 pass |
| 4 | 37 | Seeds 1-8, each BB module alone | 16 of 16 runs, 0 errors |
| 5 | 37 | `holes.sh` BSTBBAlphaStEph.rs | 0 holes |
| 6 | 37 | `holes.sh` BSTBBAlphaMtEph.rs | 0 new; 13 assume, 1 accept as before |
| 7 | 37 | BB RTTs (`scripts/rtt.sh bb`) | 29 of 29 pass |
| 8 | 37 | BB benches (`--test` mode) | 3 of 3 pass |

¹ `TestSpanTreeMtEph::test_spanning_tree_mt_two_vertices` (Chap64). It fails
deterministically on main too; it is not related to this round.

## Interpretation of the textbook invariant

APAS states BB[α] with subtree sizes: each child of a node of size n has size
at least αn. That statement cannot hold for any α > 0, because a node with two
keys has an empty child. The implementation therefore uses weights: the weight
of a subtree is its size plus one, which is its number of empty leaves. The
invariant at every node is `w(r) ≤ 3·w(l)` and `w(l) ≤ 3·w(r)` (Δ = 3). Each
child then has at least a quarter of its node's weight, so α = 1/4 ≤ 1 − 1/√2.
The rebalancing rotation is single when `w(inner) < 2·w(outer)` and double
otherwise (Γ = 2). (Δ, Γ) = (3, 2) is the pair Hirai and Yamamoto (JFP 2011)
proved correct for insert and delete.

## Height bound

`lemma_wb_height_bound` (public, both files): for every weight-balanced link,
`4^h ≤ 3^h · w`, where w = n + 1. The constant is c = 1, so
h ≤ log_{4/3}(n + 1) ≈ 2.41·lg(n + 1). `height()` states the bound in its
ensures in both files: `pow(4, h) <= pow(3, h) * (size + 1)`.

## Per-function changes (both files; MtEph Layer 1 is a copy of StEph Layer 1)

| # | Chap | File | Function | Change | Spec before → after |
|---|---|---|---|---|---|
| 1 | 37 | BSTBBAlphaStEph.rs | node type | size-cached `Node`/`Link` | BalBinTree → Link, view erases sizes |
| 2 | 37 | BSTBBAlphaStEph.rs | wf | BST ∧ wb ∧ cache | BST only → full wf |
| 3 | 37 | BSTBBAlphaStEph.rs | insert | descent + `balance` per level | BST ensures → + full wf |
| 4 | 37 | BSTBBAlphaStEph.rs | delete | delete-min join + `balance` | BST ensures → + full wf |
| 5 | 37 | BSTBBAlphaStEph.rs | delete_min_link | new | returns the minimum, full wf |
| 6 | 37 | BSTBBAlphaStEph.rs | size | reads the cached size | O(n) → O(1) |
| 7 | 37 | BSTBBAlphaStEph.rs | height | same walk | + near-balance ensures |
| 8 | 37 | BSTBBAlphaStEph.rs | in_order, pre_order | new or exact | exact sequence ensures |
| 9 | 37 | BSTBBAlphaStEph.rs | balance, rotations | new, O(1) | `== spec_balance(l,k,r)` |
| 10 | 37 | BSTBBAlphaMtEph.rs | lock predicate | inv = full wf | BST ∧ size ∧ height → full wf |
| 11 | 37 | BSTBBAlphaMtEph.rs | type invariant | full wf on ghost link | BST → full wf |
| 12 | 37 | BSTBBAlphaMtEph.rs | insert | O(1) size check | O(n) size+height check removed |
| 13 | 37 | BSTBBAlphaMtEph.rs | delete | delete_link | as StEph |
| 14 | 37 | BSTBBAlphaMtEph.rs | height | same walk | + near-balance ensures |

Updated Alg Analysis entries: insert and delete Work O(lg n), Span O(lg n),
not amortized; size O(1).

## Proof structure

The exec functions mirror spec functions (`mk_node`, `single_left`,
`double_left`, `single_right`, `double_right`, `balance`). Each has an ensures
of the form `result == spec_...(l, k, r)`. All reasoning happens in proof lemmas:

- rotation lemmas: `lemma_single_left/right`, `lemma_double_left/right`,
  `lemma_double_weights` (pure linear integer arithmetic), `lemma_balance`;
- step lemmas: one per recursive branch of insert, delete, and delete-min;
- bridge lemmas from `Link` to `BalBinTree`: size, contains, height, BST;
- `lemma_wb_height_bound`: induction plus nonlinear arithmetic on `pow`.

The balance arithmetic was checked first by exhaustive enumeration for weights
up to 60 (`src/experiments/wb_balance_arith.rs`, RESULT SUCCEEDS).

Each BB module's whole-module SMT time is about 1 s. The first seed sweep
found `double_right` exceeding its rlimit under seed 7, because both double
rotations revealed fuel 2 for their whole bodies. The fix replaced the fuel
reveals with one-step `lemma_cached_node` calls. After the fix, all 16 runs
(seeds 1-8, both modules) verify with 0 errors
(`logs/seed-stability-r222-bbalpha.20260924-214738.log`).

MtEph kept every lock-boundary assume exactly as it was: 13 assume and
1 accept, the same counts as on main. No assume, admit, accept, or
external_body was added, and no rlimit was raised.

## Balance tests

`tests/Chap37/bst_balance_check.rs::check_weight_balanced` rebuilds each tree
from `pre_order()` alone, without module internals. It checks key order and the
weight condition at every node. `height_within_wb_bound` checks
`4^h ≤ 3^h (n + 1)`. The tests run these checks after every insert and delete.

| # | Chap | File | Workload | n | Max height | Bound |
|---|---|---|---|---|---|---|
| 1 | 37 | TestBSTBBAlphaStEph.rs | ascending | 2000 | 20 | 26 |
| 2 | 37 | TestBSTBBAlphaStEph.rs | descending | 2000 | 20 | 26 |
| 3 | 37 | TestBSTBBAlphaStEph.rs | zigzag | 2000 | 20 | 26 |
| 4 | 37 | TestBSTBBAlphaStEph.rs | random | 2000 | 15 | 26 |
| 5 | 37 | TestBSTBBAlphaMtEph.rs | ascending | 500 | 16 | 21 |
| 6 | 37 | TestBSTBBAlphaMtEph.rs | descending | 500 | 16 | 21 |
| 7 | 37 | TestBSTBBAlphaMtEph.rs | zigzag | 500 | 16 | 21 |
| 8 | 37 | TestBSTBBAlphaMtEph.rs | random | 500 | 12 | 21 |

Both files also have these workloads, and all pass:

- duplicate inserts;
- deletes of absent keys;
- delete to empty in ascending, descending, and random order;
- 6000-step (StEph) or 2000-step (MtEph) random insert/delete interleavings,
  checked against a `BTreeSet` model.

## Test changed: `mt_bbalpha_comprehensive_operations`

With the specified algorithm, the test's 30 descending inserts produce height 8.
The test asserted `height <= 7`, a constant that was not derived from any
proven bound. The invariant check passes on that tree, and the proven bound for
n = 30 is 11. At the user's direction, the assertion now checks the proven
bound: `height_within_wb_bound(bst.size(), height)`. The test passes. The
other `height() <= 64`-style bounds were replaced in the same way.

## Open items for the user

| # | Chap | File | Item |
|---|---|---|---|
| 1 | 37 | — | Merge r221, then r222 (the r222 branch contains r221) |
| 2 | 37 | BSTRBMtEph.rs | Decide on the delete RWLOCK_GHOST assume (from r221) |
