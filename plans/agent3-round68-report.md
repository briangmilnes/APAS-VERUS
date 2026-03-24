# Agent 3 Round 68 Report

## Objective

Reduce proof holes in `src/Chap43/OrderedTableStPer.rs` from 58 toward 0.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableStPer.rs | 58 | 24 | -34 |

## Verification

- validate: 4370 verified, 0 errors
- rtt: 2512 passed, 0 failed
- ptt: 145 passed, 0 failed

## Phase 1: Remove Redundant Axiom Assumes (24 holes removed)

Removed the 7-axiom-pack pattern from 6 methods that already have `self.spec_orderedtablestper_wf()` in requires. The wf predicate includes all 7 axiom predicates, making the explicit assumes redundant.

Methods cleaned: `find`, `insert`, `delete`, `domain`, `size`, `collect` (approximate, carried from prior session).

## Phase 2: Prove Admits (10 admits removed, 0 remain)

Every `admit()` in OrderedTableStPer.rs is now proved. Admits proved in this session:

| # | Chap | Function | Technique |
|---|------|----------|-----------|
| 1 | 43 | union (2 loops) | Two-loop invariant tracking: self-key completeness, other-only completeness, 3-category value tracking (self-only, intersection via existential witnesses, other-only). Post-loop domain union extensionality. |
| 2 | 43 | rank_key_iter | Ghost set `counted_keys` tracking filter membership. TotalOrder::cmp ensures connect exec to spec. `!filter_pred` proved in Equal (view equality contradiction) and Greater (antisymmetric + contradiction). Post-loop set extensionality against `dom().filter(pred)`. |
| 3 | 43 | select_key | Loop invariant on `result_key matches Some(rk)` with rank_key ensures. Postconditions are conditional on Some, so vacuously true if no match found. |

Admits proved in prior session (carried into this session's hole delta):
- difference, restrict, subtract, first_key_iter, get_key_range_iter
- split_key_iter, split_rank_key_iter
- map (2 admits), intersection

## Remaining 24 Holes

### Axiom assumes in constructor functions (20 holes)

| # | Chap | Function | Holes | Nature |
|---|------|----------|-------|--------|
| 1 | 43 | empty() | 7 | Axiom assumes for wf (no prior wf context) |
| 2 | 43 | from_sorted_entries() | 7 | Same |
| 3 | 43 | tabulate() pre-loop | 2 | Pair ordering axioms |
| 4 | 43 | tabulate() post-loop | 4 | Axiom assumes for wf |

These are in functions that CREATE new tables without an existing wf table to draw axiom predicates from. Removing them requires broadcast proofs or explicit proof infrastructure for type-level axiom predicates.

### Standard pattern holes (4 holes)

| # | Chap | Hole | Type |
|---|------|------|------|
| 1 | 43 | eq/clone workaround (x2) | warning, standard |
| 2 | 43 | iterator assume | standard |
| 3 | 43 | fn_missing_wf_ensures on from_sorted_entries | standard |

## Key Techniques

1. **Completeness invariant pattern**: `spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0)` maintained via `lemma_map_contains_pair_in_set` on old entries then `lemma_pair_in_set_map_contains` on new tree after insert.

2. **Value tracking via existential**: `exists|v1: V, v2: V, r: V| v1@ == old_map[p.0] && v2@ == other_map[p.0] && f.ensures((&v1, &v2), r) && p.1 == r@` maintained by providing explicit witnesses from closure calls.

3. **Ghost counted set for rank**: `ghost mut counted_keys: Set<K::V>` tracks which keys have been counted as < k. Insert on Less, prove `!filter_pred` on Equal/Greater. Post-loop set extensionality connects to `dom().filter(pred)`.

4. **TotalOrder::cmp to spec bridge**: `TotalOrder::cmp` ensures directly connect exec Ordering to spec-level `TotalOrder::le`. For Greater case, `TotalOrder::antisymmetric` proves contradiction.

5. **Conditional postcondition shortcut**: `select_key` postconditions are conditional on `key matches Some(...)`, making the proof much simpler — no need to prove existence of a key at each rank.

## Cross-File Finding

The `rank_key_iter` and `select_key` admits are identical in both OrderedTableStEph.rs and OrderedTableStPer.rs. The ghost counted set technique developed here could be applied to OrderedTableStEph.rs to eliminate those admits too.
