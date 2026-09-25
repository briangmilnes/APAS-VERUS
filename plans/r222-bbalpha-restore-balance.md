# r222: restore weight balancing to BB[α] (Chap37 StEph and MtEph)

Branch `r222/bbalpha`, worktree `~/projects/APAS-VERUS-r222`, from `main` after r221 (red-black trees) is merged.
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0. Red-black trees
(`BSTRBStEph.rs`, `BSTRBMtEph.rs`) are round r221, done first; reuse its test helper and height-lemma approach.

## Problem

`src/Chap37/BSTBBAlphaStEph.rs` and `BSTBBAlphaMtEph.rs` insert and delete as a
plain unbalanced BST. The rebalancing was removed in the first verification
pass (`7ce913d3a`, `a54240b52`, 2026-02-17; StEph header: "rebuild omitted
from verified core"). Both wf predicates are only `tree_is_bst()`;
`weight_balanced` (StEph line 68) is defined but used by no requires or ensures.
The tests that checked balance were deleted on 2026-02-18 (`fba845e39`) and
replaced by bounds a linked list satisfies (`height <= 64` for 64 keys). The
one remaining balance test, `TestBSTMtEph::mt_bbalpha_comprehensive_operations`,
fails (height 30 for 30 descending keys).

The APAS-AI original is not a reference: it checked the weight condition only
at the root after each insert, so any subtree below the root could become a
chain.

## Textbook

APAS Chap37, balancing schemes, item 3: "Weight balanced (BB[α]) trees maintain
the invariant that the left and right subtrees of a node of size n each have
size at least αn for 0 < α ≤ 1 − 1/√2." Definition 37.6: a scheme maintains
near balance if every tree satisfying its invariant has height O(lg n). APAS
gives no update algorithm.

## Design

1. **Algorithm: rotation-based weight balance** (Nievergelt and Reingold; the
   Adams formulation), with the integer parameters (Δ, Γ) = (3, 2) that Hirai
   and Yamamoto, "Balancing weight-balanced trees" (JFP 2011), proved to be
   the only integer pair for which insert and delete preserve the invariant
   (machine-checked in Coq).
   - Weight w(t) = size(t) + 1. The textbook statement, read with sizes,
     cannot hold for any α > 0: a node with 2 keys has an empty child, of size
     0 < α·2. Weights (size + 1, the number of empty leaves) are therefore
     required, not a choice; the report states this interpretation
     explicitly.
   - Invariant at every node: `Δ·w(l) ≥ w(r)` and `Δ·w(r) ≥ w(l)`. Each child
     then has weight at least w/4, so α = 1/4 ≤ 1 − 1/√2.
   - `balance(l, k, r)` after a one-element change: if `w(r) > Δ·w(l)`, rotate
     left, single if `w(r.l) < Γ·w(r.r)`, else double; symmetric on the other
     side.
   - Insert: BST descent, then `balance` at each node on the path back up.
   - Delete: remove the key; join the two children by extracting the minimum
     of the right subtree (or the maximum of the left), then `balance`.
   - Worst-case Work O(lg n), Span O(lg n) per insert and delete, not
     amortized. A partial-rebuild (scapegoat) scheme would give amortized
     bounds and a simpler proof, but APAS Chap37's Remark warns that
     "data structures that rely on amortization techniques can be challenging
     to support in the parallel setting," and rotations are also what the
     Chapter 38 `joinMid` interface (tree sequences, "rank is the log of the
     size") needs.
   - Conventions follow Definitions 37.1 and 37.3: empty leaves, height of an
     empty tree 0, of a single node 1.
2. **Representation.** Each node caches its subtree size, so that `size()` and
   the balance tests are O(1). Each file defines its own node type
   (StEph/MtEph standalone rule); the proof helpers already shared through
   `BSTSpecsAndLemmas.rs` stay shared. Keep the public `View` as
   `BalBinTree<T>` (a spec fn that erases the size fields), so the existing BST
   specs and the lemmas in `BSTSpecsAndLemmas.rs` still apply. wf includes
   "every cached size equals the spec size."
3. **Specs (strengthened, never weakened).**
   - `spec_bstbbalpha{st,mt}eph_wf` = BST ordering ∧ weight balance ∧ correct
     size cache.
   - Every operation that returns or mutates a tree ensures the full wf.
   - A proof lemma for near balance (Definition 37.6): a balanced tree of
     weight w has height h with `4^h ≤ c · 3^h · w`, stated with integer
     powers (derive the exact constant). Expose the height bound in `height`'s
     ensures or as a public lemma.
   - Mt: the RwLock predicate (`BSTBBAlphaMtEphInv`) carries the full wf.
     Existing lock-boundary assumes stay as they are; no new ones.
4. **Costs.** Update the Alg Analysis annotations of the functions whose bodies
   change: insert and delete O(lg n); size O(1). The user authorized this
   round to change algorithm bodies and costs; the rule freezing Alg Analysis
   applies to proof repair, not to this restoration.

## Balance testing (independent of the module's specs)

The tests must detect imbalance even if a later round weakens the specs.

1. The trait gets `pre_order` (StEph lacks it; MtEph has it) and `in_order`
   if missing. A BST is determined by its pre-order sequence, so a test helper
   in `tests/Chap37/` rebuilds the shape from `pre_order()` and checks, at
   every node: key order, the weight condition `3·w(l) ≥ w(r) ∧ 3·w(r) ≥ w(l)`,
   and height ≤ the proven bound. The helper uses no module internals.
2. Workloads, checked after every operation, for n up to 2000 (StEph) and
   500 (MtEph): ascending, descending, zigzag (alternating ends), seeded
   random, random insert/delete interleaving, delete in ascending, descending
   and random order down to empty, duplicate inserts, deletes of absent keys.
3. Replace the linked-list bounds (`height() <= 64`, `<= 20`-style asserts in
   `TestBSTBBAlphaStEph.rs`, `TestBSTBBAlphaMtEph.rs`) with the real bound.
   `mt_bbalpha_comprehensive_operations` must pass unchanged.
4. `TestBSTSetBBAlphaMtEph.rs` must still pass; add one balance workload
   through the set wrapper if it exposes traversal.

## Steps

1. Read CLAUDE.md and all of `src/standards/`.
2. StEph first: node type, size cache, `balance`, rotations, insert, delete,
   wf, height lemma. `scripts/validate.sh isolate Chap37` until 0 errors,
   0 warnings, 0 trigger notes.
3. StEph balance RTTs (`scripts/rtt.sh bbalpha`).
4. MtEph: same algorithm in layer 1, lock invariant, then its RTTs.
5. `BSTSetBBAlphaMtEph.rs` and the benches still compile and pass.
6. Proof stability: each new function verifies alone under Z3 seeds 1-8
   (the r219/r220 method); fix instability in the proof, not with rlimit.
7. Full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`,
   sequentially.
8. `scripts/holes.sh src/Chap37/`: no new holes in either file.
9. Report `plans/r222-bbalpha-restore-balance-report.md`: per function the
   change, spec before/after, proof size, seeds, the height-bound constant,
   and balance-test results (max height observed vs bound per workload).
   Tables with # and Chap columns.
10. Commit on the branch with `git add -A`; push the branch.

## Constraints

No assume, admit, accept, or external_body added; no rlimit raised; no spec
weakened; no `#![auto]`; no formatters; no subagents inside the round;
validate/rtt/ptt run sequentially.

## Success criteria

- Both files' wf includes weight balance, and insert and delete are proved to
  maintain it.
- The height bound is proved as a lemma.
- The independent balance tests pass on every workload.
- The previously failing BB[α] RTT passes.
- Full crate 0 errors; RTT failures limited to the known Chap62 and Chap63
  nondeterministic tests; PTT 328/328.
