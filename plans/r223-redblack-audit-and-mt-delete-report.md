# r223 report: red-black trees — MtEph public delete and audit (Chap37)

Branch `r223/redblack-audit` (worktree `~/projects/APAS-VERUS-r223`), from main
`c22859345`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0.

## Result

| # | Chap | Measure | Value |
|---|---|---|---|
| 1 | 37 | `validate.sh isolate Chap37` | 2063 verified, 0 errors, 0 warnings, 0 trigger notes |
| 2 | 37 | Seeds 1-8, BSTRBStEph alone | 8 of 8 runs, 80 verified, 0 errors |
| 3 | 37 | Seeds 1-8, BSTRBMtEph alone | 8 of 8 runs, 95 verified, 0 errors |
| 4 | 37 | `scripts/rtt.sh RB` | 41 of 41 pass (7 new) |
| 5 | — | Full `scripts/validate.sh` | 5820 verified, 0 errors |
| 6 | — | Full `scripts/rtt.sh` | 4374 of 4375; only Chap64 two-vertex (fixed in r224) |
| 7 | — | `scripts/ptt.sh` | 328 of 328 pass |

Seed log: `logs/seed-stability-r223-redblack.*.log`.

## Part A: MtEph public delete

`BSTRBMtEphTrait::delete(&mut self, target: &T) -> Result<(), ()>`:

- requires wf;
- ensures wf; on `Ok`, the view is a BST, `target` is absent, and the key set
  is the old key set minus `target`.

The body:

1. `acquire_write`;
2. one RWLOCK_GHOST assume, `assume(self.ghost_root@ == current)`, the same
   step `insert` uses; the user authorized it for this round;
3. if `find_link` does not find the key, return `Ok` with the tree unchanged;
4. otherwise color the root red if its left child is black;
5. the verified Layer-1 `delete_link`;
6. color the root black;
7. set the ghost root;
8. `release_write`.

The proof mirrors StEph `delete`. The lock predicate stays the full wf
(`spec_is_rb_tree_link`). Alg Analysis: Work O(lg n), Span O(lg n).

New MtEph tests (n = 500) check the red-black shape rebuilt from `pre_order()`
after every operation: BST order, no red-red edge, left-leaning, equal black
height, and height ≤ 2·lg(n+1). The tests:

- delete to empty in ascending, descending, and random order (3 seeds);
- deletes of absent keys;
- deletes from an empty tree;
- a 2000-step insert/delete interleaving checked against a `BTreeSet`.

All pass.

## Part B: audit

Spec strength: S = strong (full functional spec), P = partial, W = weak
(`ensures true`).

| # | Chap | File | Function | Spec | wf req/ens | Alg Analysis | Action |
|---|---|---|---|---|---|---|---|
| 1 | 37 | BSTRBStEph.rs | new | S | ens | O(1) ok | none |
| 2 | 37 | BSTRBStEph.rs | size | S | req | O(1) ok | none |
| 3 | 37 | BSTRBStEph.rs | is_empty | S | req | O(1) ok | none |
| 4 | 37 | BSTRBStEph.rs | height | S + bound | req | O(n) ok | none |
| 5 | 37 | BSTRBStEph.rs | insert | S | req/ens | O(lg n) ok | none |
| 6 | 37 | BSTRBStEph.rs | delete | S | req/ens | O(lg n) ok | none |
| 7 | 37 | BSTRBStEph.rs | contains, find | S | req | was O(h(T)) | now O(lg n) |
| 8 | 37 | BSTRBStEph.rs | minimum, maximum | S | req | — | added¹ |
| 9 | 37 | BSTRBStEph.rs | in_order, pre_order | S (exact) | req | O(n) ok | none |
| 10 | 37 | BSTRBStEph.rs | Layer 1 (15 fns) | S | llrb/bst/cache | find/min/max stale | now O(lg n) |
| 11 | 37 | BSTRBStEph.rs | update, toggle_color | S | — | O(1) ok | warning² |
| 12 | 37 | BSTRBMtEph.rs | new, from_sorted_slice | S | ens | ok | none |
| 13 | 37 | BSTRBMtEph.rs | insert | S | req/ens | O(lg n) ok | none |
| 14 | 37 | BSTRBMtEph.rs | delete | S | req/ens | O(lg n) | added (Part A) |
| 15 | 37 | BSTRBMtEph.rs | contains, size, is_empty | S³ | req | contains stale | now O(lg n) |
| 16 | 37 | BSTRBMtEph.rs | height | S³ + bound | req | O(n) ok | none |
| 17 | 37 | BSTRBMtEph.rs | find | S³ + accept | req | stale | now O(lg n) |
| 18 | 37 | BSTRBMtEph.rs | minimum, maximum | W | none | stale | O(lg n); spec⁴ |
| 19 | 37 | BSTRBMtEph.rs | in_order, pre_order | W | none | O(n) ok | spec⁴ |
| 20 | 37 | BSTRBMtEph.rs | filter, reduce | W | req | O(n) ok | spec⁴ |
| 21 | 37 | BSTRBMtEph.rs | iter | P | req | O(n) ok | none |
| 22 | 37 | BSTRBMtEph.rs | Layer 1 insert/delete | S | llrb/bst/cache | O(lg n) ok | none |
| 23 | 37 | BSTRBMtEph.rs | *_collect, *_parallel | W | size req | O(n) | finding⁵ |
| 24 | 37 | BSTRBMtEph.rs | filter/reduce_parallel | W | size req | O(n) | finding⁵ |
| 25 | 37 | BSTRBMtEph.rs | build_balanced, build_split | S | ens | O(n) ok | none |
| 26 | 37 | BSTRBMtEph.rs | update, toggle_color | S | — | O(1) ok | warning² |

¹ `minimum`/`maximum` were missing from StEph although Layer 1 had proved
`min_link`/`max_link`. They now require wf and ensure: none on an empty tree;
otherwise a key the tree contains that is ≤ (≥) every key.

² veracity reports `fn_missing_requires` on `update` and `toggle_color`.
They have no real precondition. Per CLAUDE.md, no `requires true` or
`// veracity: no_requires` was added; this is for the user to annotate.

³ Readers under the lock. Their result-to-ghost link is a lock-boundary
assume (RWLOCK_GHOST), unchanged from r221.

⁴ These MtEph readers ensure only `true`, or nothing about `self@`.
Stronger ensures need the same result-to-ghost assume the other readers use,
and this round may add no assumes, so they are unchanged. With approval they
could take the BB MtEph forms (`minimum`/`maximum`: 3 assumes each, as in BB).

⁵ `in_order_parallel`, `pre_order_parallel`, `filter_parallel`, and
`reduce_parallel` are sequential recursions, despite their names; this
predates r221. Making them fork-join changes span, which needs user approval.
The Layer-1 `in_order_collect`/`pre_order_collect` could get exact sequence
ensures (as StEph `in_order_into` has), but that would not reach Layer 2 while
item 19 stands.

Other checks:

- TOC section lists match the section headers in both files.
- Every quantifier has an explicit trigger: the isolate log has 0 trigger
  notes.
- The independent RB checker (`check_red_black`, `height_within_two_lg`) runs
  after every operation in every RB workload, StEph and MtEph.
- `BSTSetRBMtEph::delete` still removes a key by rebuilding the tree from a
  filtered in-order traversal: O(n). Calling the new tree `delete` would make
  it O(lg n). That changes an algorithm's cost, so it is left for the user to
  decide.

## Holes (`scripts/holes.sh`)

| # | Chap | File | Before (main) | After |
|---|---|---|---|---|
| 1 | 37 | BSTRBStEph.rs | 0 holes, 2 fn_missing_requires | same |
| 2 | 37 | BSTRBMtEph.rs | 6 RWLOCK_GHOST, 1 accept, 2 fn_missing_requires | 7 RWLOCK_GHOST (+ delete), 1 accept, 2 fn_missing_requires |

The only new assume is the authorized one in MtEph `delete`.

## Decisions for the user

| # | Chap | File | Decision |
|---|---|---|---|
| 1 | 37 | BSTRBMtEph.rs | Strengthen min/max/traversal readers with RWLOCK_GHOST assumes? |
| 2 | 37 | BSTRBMtEph.rs | Make the four `*_parallel` functions fork-join? |
| 3 | 37 | BSTSetRBMtEph.rs | Make the set's delete call the tree's delete (O(n) → O(lg n))? |
| 4 | 37 | both | Annotate `update`/`toggle_color` as having no precondition? |
