# r221: restore red-black balancing (Chap37 StEph and MtEph)

Branch `r221/redblack`, worktree `~/projects/APAS-VERUS-r221`, from `main`.
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0. BB[α] is the next round,
`plans/r222-bbalpha-restore-balance.md`; build the test helper and the
height-lemma pattern here so r222 can reuse them.

## Problem

| # | Chap | File | Code | Spec (wf) |
|---|------|------|------|-----------|
| 1 | 37 | BSTRBStEph.rs | Plain BST insert. No color field; `rotate_left`/`rotate_right` are defined and never called. Header: "Color invariant requires extending BalBinTree with a color field (future work)." | `tree_is_bst()` only |
| 2 | 37 | BSTRBMtEph.rs | Left-leaning RB insert: `insert_link` ends with `fix_up` (rotate left, rotate right, flip colors), `insert` blackens the root. No `delete` in the trait. | `spec_is_bst_link` plus a size bound; no color invariant |

Neither file proves balance. The tests do not check it:
`test_rb_balancing_ascending` asserts `height() <= 20` for 20 keys, and
`TestBSTMtEph`'s RB test inserts keys in an already balanced order. The test
rewrite of 2026-02-18 (`fba845e39`) removed the balance assertions.

## Textbook

APAS Chap37, balancing schemes, item 2: "Red-Black trees maintain the invariant
that all leaves have a depth that is within a factor of 2 of each other. The
depth invariant is ensured by a scheme of coloring the nodes red and black."
Definition 37.6: near balance means every tree satisfying the invariant has
height O(lg n).

Conventions (Definition 37.1, 37.3): a tree is a full binary tree whose leaves
are empty; the depth of a leaf is the number of internal nodes on the path from
the root to it; the height is the maximum leaf depth, so an empty tree has
height 0 and a single node height 1. The existing `height` functions use this
convention; the specs, lemmas, and tests of this round must too.

The BST ADT (Data Type 37.7) has `insert, delete : (T × K) → T`, so delete is
in scope. Its bulk operations (union, intersection, difference, split,
joinPair, joinM, filter, reduce) are built on `joinMid` in Chapter 38 and are
out of scope for this round.

## Design

1. **Algorithm: left-leaning red-black trees** (Sedgewick 2008), for both
   files, because BSTRBMtEph's insert already implements it (`fix_up`,
   `flip_colors`, `rotate_left`, `rotate_right`). Keep its exec code for
   insert; the work there is specs and proofs. StEph gets the same algorithm.
2. **Representation.**
   - StEph gets its own node type with a `color` field (and a size field if
     `size()` should be O(1); state the choice in the report). Keep the
     public `View` as `BalBinTree<T>` through a spec fn that erases the colors,
     so `tree_is_bst` and `BSTSpecsAndLemmas.rs` still apply. The trait's
     signatures stay as they are (StEph `insert(self, value) -> Self`).
   - MtEph keeps `Node { key, color, size, left, right }`.
   - Standalone rule: StEph and MtEph do not import from each other.
3. **Invariant** (spec fns, in each file):
   - `is_llrb(t)`: no red right child; no red node with a red left child;
     every path from a node to a nil leaf has the same number of black nodes
     (`black_height`).
   - wf = BST ordering ∧ `is_llrb` ∧ root black ∧ (Mt) correct size cache.
   - Insert passes through states that break the invariant (a red right link,
     two reds in a row at the top). Give those an explicit "almost LLRB"
     predicate describing exactly what `fix_up` may receive, and prove
     `fix_up` maps it back to `is_llrb` with black height unchanged.
     `rotate_left`, `rotate_right`, `flip_colors` each get ensures on colors
     and black height, not only on keys.
4. **Near-balance lemma** (Definition 37.6), proved in each file:
   - a tree with black height b has at least 2^b − 1 keys;
   - its height is at most 2b (+1 if the root may be red during insert);
   - hence height ≤ 2·lg(n + 1).
   State with integer powers, no floats. Expose it in `height`'s ensures or as
   a public lemma.
5. **Delete.** Add `delete` to both traits (APAS BST ADT). LLRB delete uses
   `move_red_left`, `move_red_right`, `delete_min` (Sedgewick). Proving it is
   the hardest part of the round; do it after insert is fully verified. If
   the delete proof is not complete by the end of the round, leave its exec
   code in place with the strongest spec proved so far, report exactly which
   obligation is open, and do NOT add assume, accept, or external_body to
   close it.
6. **Costs.** insert and delete Work O(lg n), Span O(lg n); update the Alg
   Analysis annotations only for the functions whose bodies change. The user
   authorized this algorithm restoration.
7. `BSTSetRBMtEph.rs` keeps working; its wf now includes the RB invariant
   through the inner tree. `from_sorted_slice` and `build_balanced` in
   BSTRBMtEph must produce a valid LLRB tree (color the levels so black
   heights are equal; prove it), or their ensures must not claim more than
   they produce.

## Balance testing (independent of the module's specs)

The tests must detect imbalance even if a later round weakens the specs.

1. Both traits expose `pre_order` (StEph lacks it; add it) and `in_order`. A
   BST is determined by its pre-order sequence, so a helper in
   `tests/Chap37/` rebuilds the shape from `pre_order()` and checks, at every
   node:
   - key order;
   - the APAS invariant, "all leaves have a depth that is within a factor of
     2 of each other": at the root, max leaf depth ≤ 2 × min leaf depth; and
     at every node, the same for the subtree's leaf depths measured from that
     node. Every valid red-black tree satisfies this at every node: below a
     node with B black nodes on each path (not counting the node), the
     shortest path has ≥ B nodes and the longest ≤ 2B if the node is black,
     ≥ B+1 and ≤ 2B+1 if red. The check is necessary, not claimed
     sufficient; it tests the textbook property without seeing colors;
   - height ≤ 2·lg(n + 1).
   The helper uses no module internals. Write it so r222 can add a
   weight-balance check beside it.
2. Workloads, checked after every operation, n up to 2000 (StEph) and 500
   (MtEph): ascending, descending, zigzag (alternating ends), seeded random,
   duplicate inserts, `from_sorted_slice` of sizes 0 to 300; once delete
   exists: random insert/delete interleaving, delete in ascending, descending
   and random order down to empty, deletes of absent keys.
3. Replace the linked-list bounds in `TestBSTRBStEph.rs`,
   `TestBSTRBMtEph.rs`, and the RB test in `TestBSTMtEph.rs` with the real
   bound. `TestBSTSetRBMtEph.rs` and PTT `ProveBSTRBMtEph.rs` must still
   pass.

## Steps

1. Read CLAUDE.md and all of `src/standards/`.
2. Write the balance test helper and workloads first, and run them on the
   current code (`scripts/rtt.sh rb`, or narrower filters). Record which
   fail: StEph is expected to fail; MtEph is expected to pass because its
   insert balances. That is the before-measurement.
3. MtEph insert: invariant specs, "almost LLRB", proofs for the rotations,
   `flip_colors`, `fix_up`, `insert_link`, `insert`, `from_sorted_slice`;
   height lemma. `scripts/validate.sh isolate Chap37` to 0 errors, 0 warnings,
   0 trigger notes.
4. StEph: node type with color, the same insert algorithm and proofs.
5. Delete in both files, then its tests.
6. Proof stability: each new or changed function verifies alone under Z3
   seeds 1-8 (the r219/r220 method); fix instability in the proof, not with
   rlimit.
7. Full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`,
   sequentially.
8. `scripts/holes.sh src/Chap37/`: no new holes in the two files.
9. Report `plans/r221-redblack-restore-balance-report.md`: per function the
   change, spec before/after, seeds; the height-lemma statement; balance-test
   results before and after (max height observed vs bound per workload);
   delete status. Tables with # and Chap columns.
10. Commit on the branch with `git add -A`; push the branch.

## Constraints

No assume, admit, accept, or external_body added (the existing Mt lock-boundary
assumes stay as they are); no rlimit raised; no spec weakened; no `#![auto]`;
no formatters; no subagents inside the round; validate/rtt/ptt run
sequentially.

## Success criteria

- Both files' wf includes the red-black invariant; insert is proved to
  maintain it; the height lemma is proved.
- Delete exists in both and is proved, or its open obligation is reported
  exactly.
- The independent balance tests pass on every workload.
- Full crate 0 errors; RTT failures limited to the known BB[α] test
  (fixed in r222) and the nondeterministic Chap62 and Chap63 tests; PTT
  328/328.
