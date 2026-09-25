# r226 report: strong specs for the BSTRBMtEph readers (Chap37)

Branch `r226/rb-reader-specs`, from `r225/rb-parallel` (`cbe48973f`).
Verus `0.2026.09.13.671956e`, Z3 4.16.0. File: `src/Chap37/BSTRBMtEph.rs`.

## Design

Each strengthened reader adds one lock-boundary assume right after `borrow()`,
`proof { assume(self.ghost_root@ == *data); }`, the writers' shape. Everything
else is proved from the Layer-1 ensures (r225) and the `lemma_link_to_bbt_*`
bridges. No result property is assumed. Exec bodies and cost annotations are
unchanged; the three sequence readers bind the `from_vec` result to a local to
state a proof about it, which is not an exec change.

`spec_filtered_link` and `spec_reduced_link` are stated over `Link<T>`, while
the Layer-2 view is `BalBinTree<T>`. To state the filter and reduce relations on
`self@`, the round adds their `BalBinTree` counterparts and two bridge lemmas:

| # | Chap | File | Item | Kind |
|---|------|------|------|------|
| 1 | 37 | BSTRBMtEph.rs | `spec_kept_tree` | spec fn (in-order kept keys under decisions) |
| 2 | 37 | BSTRBMtEph.rs | `spec_keep_decisions_tree` | spec fn (decision tree shaped like the tree) |
| 3 | 37 | BSTRBMtEph.rs | `spec_filtered_tree` | spec fn (exists decisions) |
| 4 | 37 | BSTRBMtEph.rs | `spec_reduce_witness_tree` | spec fn (witness tree for `op(op(l, k), r)`) |
| 5 | 37 | BSTRBMtEph.rs | `spec_reduced_tree` | spec fn (exists witness) |
| 6 | 37 | BSTRBMtEph.rs | `lemma_link_to_bbt_kept` | proof fn: link decisions are view decisions, same kept seq |
| 7 | 37 | BSTRBMtEph.rs | `lemma_link_to_bbt_reduce_witness` | proof fn: link witness is a view witness |

The Layer-2 bodies `choose` the Layer-1 existential witness and apply the bridge.

## Specs, before and after

All on `BSTRBMtEphTrait` in `src/Chap37/BSTRBMtEph.rs`.

| # | Chap | Function | Requires before | Requires after | Ensures before | Ensures after |
|---|------|----------|-----------------|----------------|----------------|---------------|
| 1 | 37 | `minimum` | none | wf, `obeys_feq_clone` | `true` | size 0 ⇒ None; size > 0 ⇒ Some; Some ⇒ `self@.tree_contains(m)` and `∀x. contains(x) ⇒ le(m, x)` |
| 2 | 37 | `maximum` | none | wf, `obeys_feq_clone` | `true` | mirror of `minimum` with `le(x, m)` |
| 3 | 37 | `in_order` | `obeys_feq_clone` | wf, `obeys_feq_clone` | `true` | `seq.seq@ == self@.spec_in_order()` |
| 4 | 37 | `pre_order` | `obeys_feq_clone` | wf, `obeys_feq_clone` | `true` | `seq.seq@ == self@.spec_pre_order()` |
| 5 | 37 | `filter` | unchanged | unchanged | `true` | `spec_filtered_tree(self@, predicate, seq.seq@)`; each element is a key of `self@` with `predicate.ensures((&x,), true)` |
| 6 | 37 | `reduce` | unchanged | unchanged | `true` | `spec_reduced_tree(self@, op, identity, accumulated)` |
| 7 | 37 | `iter` | unchanged | unchanged | `elts == remaining`, decrease Some | `remaining == self@.spec_in_order()`, `elts == self@.spec_in_order()`, decrease Some |

Notes:
- `minimum`/`maximum` return a clone (`Option::cloned`), so relating the result to
  the tree needs `obeys_feq_clone::<T>()`; it is required, not assumed
  (constructor_feq_standard). The ensures follow BSTRBStEph `minimum`/`maximum`
  (r223), stated on `self@`.
- `in_order`/`pre_order` state equality on the `seq` field (`Seq<T>`), which is
  stronger than the mapped `ArraySeqStPerS` view (`Seq<T::V>`); the view form
  follows from it.
- `filter` is the strongest form that follows from a closure's `ensures`: with a
  deterministic predicate it is exactly the in-order filter.

## Assumes added

Each is `proof { assume(self.ghost_root@ == *data); }` placed right after
`borrow()`, one per reader:

| # | Chap | File | Function | Assume |
|---|------|------|----------|--------|
| 1 | 37 | BSTRBMtEph.rs | `minimum` | `self.ghost_root@ == *data` |
| 2 | 37 | BSTRBMtEph.rs | `maximum` | `self.ghost_root@ == *data` |
| 3 | 37 | BSTRBMtEph.rs | `in_order` | `self.ghost_root@ == *data` |
| 4 | 37 | BSTRBMtEph.rs | `pre_order` | `self.ghost_root@ == *data` |
| 5 | 37 | BSTRBMtEph.rs | `filter` | `self.ghost_root@ == *data` |
| 6 | 37 | BSTRBMtEph.rs | `reduce` | `self.ghost_root@ == *data` |

`iter` needs no assume: it calls `in_order`, whose new ensures give the result.
The assumes in `contains`, `size`, `is_empty`, `height`, and `find` are unchanged.

## Holes

`scripts/holes.sh src/Chap37/BSTRBMtEph.rs`: 6 actionable `assume() [algorithmic]`,
7 RWLOCK_GHOST (the pre-existing ones), 1 `accept()` (pre-existing, `find`),
4 accepted `fn_missing_requires` (pre-existing helpers).

The 6 "algorithmic" holes are exactly the six assumes above. Veracity does not
classify `assume(self.ghost_root@ == *data)` as RWLOCK_GHOST, although it does
classify the writers' `assume(self.ghost_root@ == current)`. The difference is the
dereferenced borrow `*data`. The form was fixed by the user's authorization, so
it was not rewritten; the classifier gap is written up in
`plans/r226-veracity-rwlock-ghost-deref.md` for the veracity maintainer.

## Set wrapper and PTT

- `BSTSetRBMtEph`: no change. Its wf is `self.tree.spec_bstrbmteph_wf() &&
  obeys_feq_clone::<T>()`, and every call to `minimum`, `maximum`, and
  `in_order` is in a function that requires the set's wf (or, in `values_vec`,
  the tree's wf and `obeys_feq_clone`), so the new requires were already met.
  Its specs were not strengthened.
- PTT `rust_verify_test/tests/Chap37/ProveBSTRBMtEph.rs`: no change; it calls only
  `iter()`, whose requires are unchanged. PTTs were not run (main session runs them).

## Verification

- `scripts/validate.sh isolate Chap37`: `verification results:: 2076 verified, 0 errors`,
  no warnings, no trigger notes (`logs/validate.20260925-120635.log`).
- Seeds 1-8, `--verify-only-module Chap37::BSTRBMtEph::BSTRBMtEph --smt-option smt.random_seed=N`:
  each `108 verified, 0 errors` (`logs/validate.20260925-120749.log` to `-120919.log`).
  The eight logs `validate.20260925-12070*.log` are a first attempt with a wrong
  flag name (`--smt-options`) that Verus rejected before verifying.

## Tests

`cargo nextest run --release -E 'binary(/TestBSTRBMtEph|TestBSTSetRBMtEph|TestBSTMtEph/)'`:
106 tests run, 106 passed, 0 skipped.

## Open items

- Veracity classification of the `*data` form (see the feedback file above).
- `BSTSetRBMtEph` reader specs are still `ensures true`; strengthening them is a
  separate round.
