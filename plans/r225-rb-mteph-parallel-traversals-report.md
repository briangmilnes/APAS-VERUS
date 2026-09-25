# r225 report: fork-join traversals in Chap37 BSTRBMtEph

Branch `r225/rb-parallel`, from main `ec039f497`. Verus `0.2026.09.13.671956e`, Z3 4.16.0.
File: `src/Chap37/BSTRBMtEph.rs`.

## Design

`join` (`src/Chap02/HFSchedulerMtEph.rs`) requires `'static` arms, and the Layer-1
traversals receive `&Link<T>` borrowed from the read lock, so no arm may hold a
borrow of the tree. Each `_parallel` trait method therefore makes one verified deep
copy (`clone_link`, `ensures copy == *self`) and hands it to an owned fork-join
recursion (`in_order_owned`, `pre_order_owned`, `filter_owned`, `reduce_owned`).
Each owned recursion destructures the node, moves the two subtrees into two named
closures with explicit `requires`/`ensures`, and calls `join`. No `external_body`
is added; all code stays inside `verus!`; there is no sequential cutoff.

The output is a `Vec`. No verified sequence type in the codebase has a
lower-span append: `ArraySeqMtEph::append` and `flatten` (Chap18, Chap19) are
sequential loops with Span O(|a| + |b|). So each node concatenates its children's
vectors at a cost proportional to its subtree size.

### Cost, stated honestly

- Work of a concatenating traversal: the nodes at one depth hold at most n keys and
  the height is at most 2 lg(n + 1), so Work O(n lg n).
- Span: a root-to-leaf path pays the subtree size of every node on it, at most n per
  node, so Span O(n lg n). The plan's Span O(n) holds only when subtree sizes shrink
  geometrically down every path (for example trees from `from_sorted_slice` or from
  random inserts). It is not a worst-case bound for a left-leaning red-black tree:
  a tree whose right spine keeps a minimal left child at each black level and ends in
  a fully 3-node subtree keeps each spine subtree within a constant of n for about
  0.2 lg n levels, so the path sum is Theta(n lg n). The annotations state
  Span O(n lg n).
- The deep copy is sequential: Work O(n), Span O(n). It dominates `reduce`, whose owned
  recursion has Work O(n), Span O(lg n); the total is Work O(n), Span O(n).
- A lower span needs either Arc-shared nodes (O(1) hand-off to the arms, no copy) or a
  verified sequence with an O(lg n)-span append. Neither exists for this module today.

## Per-function changes

| # | Chap | File | Function | Body change | Work/Span before | Work/Span after |
|---|------|------|----------|-------------|------------------|-----------------|
| 1 | 37 | BSTRBMtEph.rs | `clone_link` (new, Layer 1) | recursive deep copy, `clone_plus` per key | — | O(n) / O(n) |
| 2 | 37 | BSTRBMtEph.rs | `in_order_collect` | proof added, `clone_plus` | O(n) / O(n) | O(n) / O(n) |
| 3 | 37 | BSTRBMtEph.rs | `pre_order_collect` | proof added, `clone_plus` | O(n) / O(n) | O(n) / O(n) |
| 4 | 37 | BSTRBMtEph.rs | `in_order_parallel` | copy, then `in_order_owned` | O(n) / O(n) seq. | O(n lg n) / O(n lg n) |
| 5 | 37 | BSTRBMtEph.rs | `pre_order_parallel` | copy, then `pre_order_owned` | O(n) / O(n) seq. | O(n lg n) / O(n lg n) |
| 6 | 37 | BSTRBMtEph.rs | `filter_parallel` | copy, then `filter_owned` | O(n) / O(n) seq.¹ | O(n lg n) / O(n lg n) |
| 7 | 37 | BSTRBMtEph.rs | `reduce_parallel` | copy, then `reduce_owned` | O(n) / O(n) seq. | O(n) / O(n) |
| 8 | 37 | BSTRBMtEph.rs | `in_order_owned` (new) | `join` over subtrees; L ++ [k] ++ R | — | O(n lg n) / O(n lg n) |
| 9 | 37 | BSTRBMtEph.rs | `pre_order_owned` (new) | `join` over subtrees; [k] ++ L ++ R | — | O(n lg n) / O(n lg n) |
| 10 | 37 | BSTRBMtEph.rs | `filter_owned` (new) | `join`; L ++ [k if kept] ++ R | — | O(n lg n) / O(n lg n) |
| 11 | 37 | BSTRBMtEph.rs | `reduce_owned` (new) | `join`; `op(op(l, k), r)` | — | O(n) / O(lg n) |

¹ As annotated. The old sequential body appended the right vector at every node, so its
worst case was also O(n lg n).

## Specs, before and after

| # | Chap | File | Function | Ensures before | Ensures after |
|---|------|------|----------|----------------|---------------|
| 1 | 37 | BSTRBMtEph.rs | `in_order_collect` | `true` | `out@ == old(out)@ + self.spec_in_order_seq()` |
| 2 | 37 | BSTRBMtEph.rs | `pre_order_collect` | `true` | `out@ == old(out)@ + self.spec_pre_order_seq()` |
| 3 | 37 | BSTRBMtEph.rs | `in_order_parallel` | `true` | `elements@ == self.spec_in_order_seq()` |
| 4 | 37 | BSTRBMtEph.rs | `pre_order_parallel` | `true` | `elements@ == self.spec_pre_order_seq()` |
| 5 | 37 | BSTRBMtEph.rs | `filter_parallel` | `true` | `spec_filtered`, plus key and predicate per element |
| 6 | 37 | BSTRBMtEph.rs | `reduce_parallel` | `true` | `self.spec_reduced(**op, identity, reduced)` |

`spec_in_order_seq(self)` is `link_to_bbt(self).spec_in_order()` (likewise pre-order).

Filter and reduce specs are the strongest that follow from a closure's `ensures`, which
need not determine the closure's result:

- `spec_filtered_link(link, pred, kept)`: some tree of keep decisions shaped like
  `link` (`BalBinTree<bool>`) has, at each node, a result `pred` may return for that
  key, and `kept` is the in-order sequence of keys whose decision is `true`. With a
  deterministic predicate this is exactly the in-order filter. `filter_parallel` also
  ensures that every element is a key of the tree and `predicate.ensures((&x,), true)`.
- `spec_reduced_link(link, op, identity, reduced)`: some witness tree shaped like
  `link` (`BalBinTree<(T, T)>`) records at each node results `op` may return for
  `op(left, key)` and `op(op(left, key), right)`, with `identity` at empty links, and
  `reduced` is the root's result.

A first formulation put the existential inside the recursive spec function; Verus did
not instantiate the existential after unfolding the recursive definition, even with the
witness asserted. Moving the existential to a non-recursive wrapper over
quantifier-free recursive witness predicates proves directly with `choose`.

Requires added: `obeys_feq_clone::<T>()` on `clone_link`, the two `_collect`, and the
four `_parallel` methods (the copy clones keys); `F: Pred<T>` (adds `'static`) on
`filter_parallel`; `'static` on the `reduce_parallel` closure type.

## Layer-2 signature changes (r226 builds on these)

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 37 | BSTRBMtEph.rs | `in_order` | `requires obeys_feq_clone::<T>()` added |
| 2 | 37 | BSTRBMtEph.rs | `pre_order` | `requires obeys_feq_clone::<T>()` added |
| 3 | 37 | BSTRBMtEph.rs | `filter` | `obeys_feq_clone::<T>()` added; bound `F: Fn(&T) -> bool + Send + Sync` became `F: Pred<T>` (adds `'static`) |
| 4 | 37 | BSTRBMtEph.rs | `reduce` | `obeys_feq_clone::<T>()` added; `'static` added to `F` |
| 5 | 37 | BSTRBMtEph.rs | `iter` | `obeys_feq_clone::<T>()` added (it calls `in_order`) |

Ensures of these five are unchanged (`true`, or the iterator triple for `iter`).
`BSTSetRBMtEph` already carries `obeys_feq_clone` in its wf, so it needed no change.
The PTT `rust_verify_test/tests/Chap37/ProveBSTRBMtEph.rs` gained
`requires obeys_feq_clone::<u64>()` on its two `iter()` tests, following
`ProveBSTPlainMtEph.rs`. PTTs were not run this round (machine shared).

## Verification

- `scripts/validate.sh isolate Chap37`: `verification results:: 2071 verified, 0 errors`,
  no warnings, no trigger notes (`logs/validate.20260925-115436.log`).
- Seeds 1-8, `--verify-only-module Chap37::BSTRBMtEph::BSTRBMtEph`: each
  `103 verified, 0 errors` (`logs/validate.20260925-1156*.log` to `-115821.log`).
- `scripts/holes.sh src/Chap37/BSTRBMtEph.rs`: 0 actionable holes. The one `accept()`
  and seven RWLOCK_GHOST assumes are unchanged. Two new info-level
  `fn_missing_requires`, on `in_order_owned` and `pre_order_owned`: they have no real
  precondition, so none was added (no `requires true`, no annotation).

## Tests

`cargo nextest run --release -E 'binary(/TestBSTRBMtEph|TestBSTSetRBMtEph|TestBSTMtEph/)'`:
106 tests run, 106 passed, 0 skipped (87 in the RB MtEph and set binaries).

New tests in `tests/Chap37/TestBSTRBMtEph.rs`, each run on a thread under a 60-second
timeout; each compares `in_order`, `pre_order` (red-black pre-order shape, same key
set), `filter` (x % 3 == 0, all, none), and `reduce` (sum, max, and the associative,
non-commutative leftmost-nonzero) against a sorted `BTreeSet` model:

| # | Chap | File | Test | Workload |
|---|------|------|------|----------|
| 1 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_traversals_empty` | empty tree |
| 2 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_traversals_ascending` | 500 ascending inserts |
| 3 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_traversals_descending` | 500 descending inserts |
| 4 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_traversals_random` | 4 random permutations of 500 |
| 5 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_traversals_random_duplicates` | 500 random keys in [0, 200) |
| 6 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_traversals_from_sorted_slice` | n in {0,1,2,3,7,64,500} |
| 7 | 37 | TestBSTRBMtEph.rs | `test_rb_mt_parallel_iter` | `iter()` on 500 random keys |

## Open items

- Span O(n lg n) worst case (and O(n) for `reduce`) comes from the `Vec` output and the
  sequential copy. Reaching O(lg² n) needs Arc-shared nodes or a verified O(lg n)-span
  sequence append; both are larger changes than this round.
- PTT not run; the `ProveBSTRBMtEph.rs` edit follows the verified BSTPlainMtEph pattern.
