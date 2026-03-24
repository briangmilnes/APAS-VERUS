# Agent 2 — Round 68 Report: OrderedTableStEph Hole Burndown

## Task
R68 Agent 2: Reduce proof holes in `src/Chap43/OrderedTableStEph.rs` from 43 toward 0.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 43 | OrderedTableStEph.rs | 43 | 10 | −33 |

Verification: 4367 verified, 0 errors.

## Work Performed (This Session + Previous Session)

### Phase 1: Eliminate Redundant Type Axiom Assumes (Previous Session)
Replaced 24 standalone type-axiom `assume()` calls with assertions derived from `spec_orderedtablesteph_wf()` conjuncts.

### Phase 3: Attack Admits with Bridge Proofs

| # | Chap | Function | Technique | Status |
|---|------|----------|-----------|--------|
| 1 | 43 | difference | Subset + completeness loop invariants | Proved |
| 2 | 43 | restrict | Subset + key condition + completeness | Proved |
| 3 | 43 | subtract | Same pattern as restrict | Proved |
| 4 | 43 | get_key_range | TotalOrder::cmp bridge + key condition | Proved |
| 5 | 43 | split_key_iter | reveal(obeys_cmp_ord) + 3-branch partition | Proved |
| 6 | 43 | map | Completeness invariant via spec_pair_set_to_map | Proved |
| 7 | 43 | first_key_iter | Rewrite to iterative TotalOrder::cmp pattern | Proved |
| 8 | 43 | split_rank_key_iter | Subset + disjoint + completeness + sorted↔tree equiv | Proved |
| 9 | 43 | intersection | Domain intersect + value tracking with existential witnesses | Proved |
| 10 | 43 | union | Two-phase loop: Phase 1 (merge self+other), Phase 2 (add other-only), 3-way value tracking, full post-loop domain+value proof | Proved |
| 11 | 43 | rank_key_iter | Agent 3 owns — marked external_body | Deferred |
| 12 | 43 | select_key | Depends on rank_key — marked external_body | Deferred |

### Key Proof Techniques
- **Completeness invariant**: `forall|j| 0 <= j < i ==> spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0)` — used in map, intersection, union, split_rank_key
- **Per-pair value tracking**: `forall|p| new_tree@.contains(p) ==> <value condition>` — avoids nested existentials
- **3-way value tracking (union)**: self-only (`old_tree.contains(p)`), combined (`exists v1 v2 r...`), other-only (`other.tree@.contains(p)`)
- **reveal(obeys_cmp_ord) without turbofish**: E0401 fix — `reveal` doesn't take generic params from outer item
- **lemma_pair_in_set_map_contains**: Bridge from `set.contains((k,v))` to `map.dom().contains(k) && map[k] == v`
- **Freshness by contradiction**: If key already in tree, trace to sorted index, contradiction with pairwise distinct keys

## Remaining Holes (10)

| # | Chap | Line | Type | Description | Blocker |
|---|------|------|------|-------------|---------|
| 1 | 43 | 1326 | assume | obeys_cmp_spec in tabulate | No &self → no wf |
| 2 | 43 | 1327 | assume | view_ord_consistent in tabulate | No &self → no wf |
| 3 | 43 | 1453 | assume | spec_pair_key_determines_order in tabulate | No &self → no wf |
| 4 | 43 | 1454 | assume | obeys_cmp_spec::\<K\> in tabulate | No &self → no wf |
| 5 | 43 | 1455 | assume | view_ord_consistent::\<K\> in tabulate | No &self → no wf |
| 6 | 43 | 1456 | assume | obeys_feq_fulls in tabulate | No &self → no wf |
| 7 | 43 | 3408 | external_body | rank_key_iter | Agent 3 owns |
| 8 | 43 | 3435 | external_body | select_key | Agent 3 owns (depends on rank_key) |
| 9 | 43 | 3728 | assume | iter_invariant in iterator next | Stays per task |
| 10 | 43 | 3752 | warning | fn_missing_wf_ensures on from_sorted_entries | Deferred |
