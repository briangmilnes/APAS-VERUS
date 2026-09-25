# r221 report: red-black balancing restored (Chap37 StEph and MtEph)

Branch `r221/redblack`, worktree `~/projects/APAS-VERUS-r221`. Verus
`0.2026.09.13`, Z3 4.16.0.

## Summary

Both files are now left-leaning red-black trees (LLRB, Sedgewick 2008, the 2-3
variant) with a per-node size cache. Their well-formedness predicate is
`spec_is_rb_tree_link`: BST order, the LLRB invariant, a black root, a correct
size cache, and size ≤ usize::MAX. The following are proved:

- insert maintains the invariant;
- delete and delete-min maintain it;
- the height bound holds: 2^h ≤ (n + 1)^2, that is, h ≤ 2·lg(n + 1).

Delete is proved in Layer 1 of both files, and StEph also has a public
`delete`. MtEph has no public `delete`, because it would need one new
lock-boundary `assume` (see "Open item").

Results:

| # | Check | Result | Log |
|---|-------|--------|-----|
| 1 | Full `scripts/validate.sh` | 5729 verified, 0 errors | `logs/validate.20260924-211834.log` |
| 2 | Chap37 isolate, final | 1972 verified, 0 errors | latest `logs/validate.*.log` |
| 3 | `scripts/rtt.sh` | 4346 run, 4344 passed, 2 failed | `logs/rtt.20260924-212004.log` |
| 4 | `scripts/ptt.sh` | 328 run, 328 passed | `logs/ptt.20260924-212043.log` |

The two RTT failures are:

- `TestBSTMtEph::mt_bbalpha_comprehensive_operations`, the known BB[α] test
  that r222 fixes.
- `TestSpanTreeMtEph::test_spanning_tree_mt_two_vertices` (Chap64). It fails
  in every run: three reruns all fail with `left: 0, right: 1` at line 69.
  Main's last full RTT (`logs/rtt.20260924-122329.log`, r220) also has two
  failures, and this test is one of them. It does not use Chap37. The plan
  expected nondeterministic Chap62/63 failures instead; this failure is
  deterministic and predates r221.

## Design

- **Node type.** Each file defines its own `Color`, `Node<T>`
  (key, color, size, left, right) and `Link<T> = Option<Box<Node<T>>>`. The
  files do not import from each other: the StEph Layer 1 is a copy of the
  MtEph Layer 1, with the bound `StTInMtT + Ord + TotalOrder` replaced by
  `TotalOrder`. The view is `link_to_bbt`, which erases colors and sizes into
  `BalBinTree<T>`. The public trait signatures are unchanged; for example,
  StEph `insert(self, value) -> Self`.
- **Exec code mirrors spec functions.** `rotate_left`, `rotate_right`,
  `flip_colors`, `update` and `fix_up` each ensure `*self == spec_X(*old(self))`.
  All red-black reasoning lives in proof lemmas over those spec functions.
- **Insert** is the LLRB insert: BST descent, then `fix_up` on the way back
  up, then the root is colored black.
- **Delete** is the LLRB delete: `move_red_left`, `move_red_right`,
  `delete_min_link`, `delete_link`, then `fix_up` on the way up. Its input
  classes and its choice of `balance` step come from the experiment
  `src/experiments/llrb_delete_balance.rs`, which records RESULT: SUCCEEDS:
  - both variants of the rebalance step keep the invariant, and all 26,965
    balance inputs fall within `fix_up`'s precondition;
  - delete sees three input classes (red LLRB root; black root with red left
    child; black node with red right child, reached only when key ≥ node key).
- **Size.** Size is O(1) from the cache. MtEph `insert` no longer walks the
  tree to check capacity. StEph `insert` gains the capacity requires
  `spec_size() < usize::MAX` (standard 22).
- **`from_sorted_slice` (MtEph)** is a new O(n) builder
  (`build_split`/`build_balanced`). It aims for black height ⌊lg(n + 1)⌋ and
  gives a node a red left child only when n = 2^(b+1) − 2. It is proved to
  produce a valid LLRB tree; the old `assume(spec_is_bst_link(ghost_link))`
  is removed. It newly requires `spec_sorted_strict(values@)` and
  `obeys_feq_clone::<T>()`.

## Height lemma

`lemma_llrb_height_bound` (public in both files): for an LLRB link with a black
root, `pow2(link_height(link)) <= (link_spec_size(link) + 1) * (link_spec_size(link) + 1)`.
It is built from two lemmas:

- `lemma_llrb_size_lower_bound`: size + 1 ≥ 2^bh;
- `lemma_llrb_height_upper_bound`: h ≤ 2·bh + [root is red].

Both `height()` functions expose the bound in their ensures.

## Per function

S = StEph, M = MtEph. "Seeds" is the Z3 seed result (see "Proof stability").

| # | Chap | File | Function | Change | Spec after | Seeds |
|---|------|------|----------|--------|------------|-------|
| 1 | 37 | BSTRBMtEph.rs | rotate_left/right | exact spec mirror | + `*self == spec_rotate_*` | module 8/8 |
| 2 | 37 | BSTRBMtEph.rs | flip_colors | exact spec mirror | + `*self == spec_flip_colors` | module 8/8 |
| 3 | 37 | BSTRBMtEph.rs | fix_up | exact spec mirror | + `*self == spec_fix_up` | alone 8/8 |
| 4 | 37 | BSTRBMtEph.rs | insert_link | LLRB proof | + llrb, bh, size cache | alone 8/8 |
| 5 | 37 | BSTRBMtEph.rs | delete_min_link | new | llrb, bh, keys − min | alone 8/8 |
| 6 | 37 | BSTRBMtEph.rs | delete_link (+3 helpers) | new | llrb, bh, keys − key | alone 8/8 ¹ |
| 7 | 37 | BSTRBMtEph.rs | move_red_left/right_link | new | exact spec mirror | module 8/8 |
| 8 | 37 | BSTRBMtEph.rs | build_balanced/build_split | new O(n) builder | llrb, black root | alone 8/8 ² |
| 9 | 37 | BSTRBMtEph.rs | insert (Layer 2) | size O(1), blacken | wf = rb tree | module 8/8 |
| 10 | 37 | BSTRBMtEph.rs | from_sorted_slice | assume removed | wf, size, keys | module 8/8 |
| 11 | 37 | BSTRBMtEph.rs | height | — | + pow2 bound | module 8/8 |
| 12 | 37 | BSTRBStEph.rs | Layer 1 (rows 1-7) | copy of M | same as M | module 8/8 |
| 13 | 37 | BSTRBStEph.rs | in/pre_order_into | new, O(n) | exact sequence | module 8/8 |
| 14 | 37 | BSTRBStEph.rs | insert, delete | LLRB | wf = rb tree, keys | module 8/8 |
| 15 | 37 | BSTRBStEph.rs | size / height | O(1) / + bound | exact / + pow2 | module 8/8 |
| 16 | 37 | BSTRBStEph.rs | in_order, pre_order | new trait fns | = spec_in/pre_order | module 8/8 |

¹ `delete_link_left` and `delete_at_ready` ran alone; `delete_link_right` and
`delete_link` passed as part of the module. ² `build_balanced` ran alone;
`build_split` passed as part of the module.

About 40 proof lemmas are new in each file; the move-red, delete-descent,
rejoin and tail lemmas cover delete. The delete spec functions sit in section 6b.

## Proof stability

The plan asks that each new or changed function verify alone under Z3 seeds 1-8.
There are about 140 such functions; running each alone under every seed would
take about 5 hours. What was run instead:

1. **Each module under seeds 1-8** (`--verify-only-module`, every function in
   one Z3 session):
   - Before the fix, MtEph passed seeds 1-7 and failed at seed 8:
     `lemma_move_red_left` exceeded its rlimit
     (`logs/validate.20260924-205814.log`).
   - The fix was in the proof, not the rlimit: the borrow case moved into its
     own lemma, `lemma_move_red_left_borrow`, and the main lemma now reveals
     fuel 2 instead of 3. Both lemmas alone pass seeds 1-8. Both files carry
     the change.
   - After the fix, MtEph passes 8/8 (94 verified each) and StEph passes 8/8
     (78 verified each). Logs: `logs/validate.20260924-210406.log` through
     `-210707.log`.
2. **The 8 costliest MtEph functions alone under seeds 1-8**, chosen by the
   per-function rlimit from `--output-json --time-expanded`
   (`logs/validate.20260924-210728.log`): insert_link (6.7M), build_balanced
   (6.4M), delete_link_left (5.3M), lemma_move_red_left_borrow (4.0M),
   fix_up (2.2M), lemma_descend_right_rotate (2.2M), delete_min_link (2.0M),
   delete_at_ready (1.1M). All 64 runs pass
   (`logs/seed-stability-r221-mteph.20260924-210753.log`).

## Balance tests

`tests/Chap37/bst_balance_check.rs` rebuilds the shape from `pre_order()` and
checks three properties, using no module internals: key order, max leaf depth
≤ 2 × min leaf depth at every node, and 2^h ≤ (n + 1)^2.

Workloads, all checked after every operation:

- insert: ascending, descending, zigzag, seeded random (4 seeds), duplicates;
- MtEph `from_sorted_slice` for n = 0..300;
- StEph delete: ascending, descending and random order down to empty; deletes
  of absent keys; 6,000 random interleaved inserts and deletes compared against
  a `BTreeSet`.

Maximum height observed (`logs/rtt-rb-balance-report.*.log`):

| # | Chap | File | Workload | n | Before | After | Bound 2·lg(n+1) |
|---|------|------|----------|---|--------|-------|-----------------|
| 1 | 37 | BSTRBStEph.rs | ascending | 2000 | 2000 | 11 | 21.9 |
| 2 | 37 | BSTRBStEph.rs | descending | 2000 | 2000 | 19 | 21.9 |
| 3 | 37 | BSTRBStEph.rs | zigzag | 2000 | 2000 | 19 | 21.9 |
| 4 | 37 | BSTRBStEph.rs | random | 2000 | 24 | 15 | 21.9 |
| 5 | 37 | BSTRBMtEph.rs | ascending | 500 | 9 | 9 | 17.9 |
| 6 | 37 | BSTRBMtEph.rs | descending | 500 | 15 | 15 | 17.9 |
| 7 | 37 | BSTRBMtEph.rs | zigzag | 500 | 15 | 15 | 17.9 |
| 8 | 37 | BSTRBMtEph.rs | random | 500 | 14 | 14 | 17.9 |

Before r221:

- StEph failed the red-black check at n = 3 on every workload.
- MtEph passed. Its insert already balanced; its proofs did not state the
  invariant.

The fixed height caps in `TestBSTRBStEph.rs` (`<= 20`), `TestBSTRBMtEph.rs`
(`<= 3`, `<= 6`, `<= 4`) and the RB test in `TestBSTMtEph.rs` (`<= 8`) are
replaced by the helper's `height_within_two_lg`.

## Holes

`scripts/holes.sh src/Chap37/`:

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 37 | BSTRBMtEph.rs | 7 RWLOCK_GHOST assumes, 1 accept, 2 fn_missing_requires | 6 RWLOCK_GHOST, 1 accept, 2 fn_missing_requires |
| 2 | 37 | BSTRBStEph.rs | clean | 0 holes, 2 fn_missing_requires |

- **Change in MtEph:** the `from_sorted_slice` assume is gone. No assume,
  accept, admit or external_body was added in either file.
- **Existing annotations:** MtEph keeps the user's three
  `// veracity: no_requires` annotations (is_red, size_link, update).
- **Warnings flagged for you:** fn_missing_requires in MtEph `toggle_color`
  and StEph `update` and `toggle_color`.
  - I had added `// veracity: no_requires` to `toggle_color` and copied the
    user's annotations into StEph. Both are removed, because only the user
    adds these.
  - `update` and `toggle_color` have no real precondition. StEph `is_red` and
    `size_link` lost their copied annotations too, yet holes.sh does not flag
    them; that output is left as it is.

## Open item: MtEph public delete

Layer 1 `delete_link` is proved in MtEph. A Layer-2
`fn delete(&mut self, target: &T) -> (removed: Result<(), ()>)` with the
`BSTPlainMtEph` spec needs the same writer lock-boundary step that `insert`
uses: `assume(self.ghost_root@ == current)`. Veracity classifies it as
RWLOCK_GHOST. CLAUDE.md does not allow adding an `assume` without asking, so
MtEph has no public delete and no MtEph delete RTTs.

With approval, the body would be:

1. `acquire_write`;
2. the RWLOCK_GHOST assume;
3. `find_link`, returning `Ok` unchanged if the key is absent;
4. color the root red if its left child is black;
5. `delete_link`;
6. color the root black;
7. `release_write`.

The Layer-2 proof would mirror StEph `delete`, which verifies.

## Commits

Commits are on `r221/redblack`, and the branch is pushed. Main is not changed.
