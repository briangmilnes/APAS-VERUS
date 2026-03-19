# Agent 2 — Round 41 Report

## Summary

Proved 11 `external_body` methods in `src/Chap43/OrderedTableStPer.rs`, reducing holes from 192 to 181.
Verification count: 4281 (baseline) → 4303 (+22 verified obligations).

## Methods Proved

| # | Chap | File | Method | Technique |
|---|------|------|--------|-----------|
| 1 | 43 | OrderedTableStPer.rs | `map` | Ghost `orig_vals: Seq<V>` for value tracking |
| 2 | 43 | OrderedTableStPer.rs | `filter` | Forward (`result_src`) + backward (`result_idx`) ghost tracking |
| 3 | 43 | OrderedTableStPer.rs | `get_key_range` | Forward tracking + `clone_plus` with feq trigger |
| 4 | 43 | OrderedTableStPer.rs | `split_key` | Bidirectional tracking + found value tracking + `reveal(obeys_view_eq)` |
| 5 | 43 | OrderedTableStPer.rs | `tabulate` | Ghost `key_args: Seq<K>` + `lemma_view_index` + feq trigger |
| 6 | 43 | OrderedTableStPer.rs | `restrict` | Bidirectional tracking + `seq_to_set_is_finite` for `keys@.finite()` |
| 7 | 43 | OrderedTableStPer.rs | `subtract` | Mirror of restrict with negated condition |
| 8 | 43 | OrderedTableStPer.rs | `intersection` | Bidirectional tracking + closure witness (`result_v1/v2/r`) |
| 9 | 43 | OrderedTableStPer.rs | `union` | Two-phase loop (self + other) + phase2 backward tracking |
| 10 | 43 | OrderedTableStPer.rs | `join_key` | Trivial delegation to proved `union` |
| 11 | 43 | OrderedTableStPer.rs | `split_rank_key` | Rewritten to iterate elements directly, split at index min(i, len) |

## Key Techniques

- **`obeys_feq_full_trigger::<T>()`**: Triggers the broadcast axiom `axiom_obeys_feq_full`, bridging `clone_plus()` results to view equality without needing `obeys_feq_full::<T>()` in function requires. Chain: trigger → `obeys_feq_full` → `obeys_feq_clone` → `axiom_cloned_implies_eq` → `cloned(*x, y) ==> *x == y`.
- **Bidirectional ghost tracking**: Forward `result_src: Seq<int>` maps result entries to source indices. Backward `result_idx: Seq<int>` maps source indices to result positions (for completeness proofs).
- **Separated `assert forall` from `assert(=~=)`**: Verus can't prove set extensionality inside a nested `assert by` block. Separating the element-wise proof from the `=~=` conclusion works.
- **Trigger-aware assertions**: When invariants use `phase1_src[j]` as trigger, need `assert(phase1_src[a] == a)` before using the invariant's conclusion.
- **`reveal(obeys_view_eq)` before `.eq(k)`**: Required to establish the connection between exec equality and view equality.

## Remaining Holes (OrderedTableStPer.rs)

| # | Chap | Method | Blocker |
|---|------|--------|---------|
| 1 | 43 | `domain` | No `self.spec_orderedtablestper_wf()` in requires |
| 2 | 43 | `collect` | No wf in requires |
| 3 | 43 | `first_key` | No wf in requires |
| 4 | 43 | `last_key` | No wf in requires |
| 5 | 43 | `previous_key` | No wf in requires |
| 6 | 43 | `next_key` | No wf in requires |
| 7 | 43 | `difference` | No `other.spec_orderedtablestper_wf()` in requires |
| 8 | 43 | `rank_key` | Complex `dom().filter()` + TotalOrder spec |
| 9 | 43 | `select_key` | Complex TotalOrder spec + depends on `collect` |
| 10 | 43 | `next` (iterator) | Structural `external_body` (std::iter::Iterator trait) |

Methods 1-7 are blocked by missing well-formedness preconditions in the trait signatures.
Methods 8-9 need complex TotalOrder reasoning with Set::filter length proofs.
Method 10 is a standard iterator external_body (structural, not algorithmic).

## Verification Results

- Verification: 4303 verified, 0 errors
- RTT: 2612 passed, 1 skipped
- PTT: 143 passed, 4 failed (pre-existing failures in OrderedTableStPer iterator tests)
- Total holes: 181 (down from 192)
