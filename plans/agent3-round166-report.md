# R166 Agent 3 Report: Lift Proof Patterns in Chap41 OrdKeyMap + Chap38 BSTParaSt/Mt

## Summary

Extracted repeated proof patterns into shared lemmas across OrdKeyMap (Chap41) and
BSTParaStEph/BSTParaMtEph (Chap38). Net reduction: **96 lines** across 4 files.

## Changes

### Chap41 OrdKeyMap.rs (4899 -> 4837, -62 lines)

Added 4 new proof lemmas to section 7:

| # | Chap | File | Lemma | Sites | Lines saved |
|---|------|------|-------|-------|-------------|
| 1 | 41 | OrdKeyMap.rs | `lemma_post_insert_invariants` | 8 | ~24 |
| 2 | 41 | OrdKeyMap.rs | `lemma_ordkeymap_wf_type_axioms` | 7 | ~42 |
| 3 | 41 | OrdKeyMap.rs | `lemma_values_preserved_from_subset` | 5 | ~35 |
| 4 | 41 | OrdKeyMap.rs | `lemma_loop_init_sorted` | 8 | ~16 |

**lemma_post_insert_invariants** — Combines the 4-lemma block
(view_gen_insert + key_unique_insert + pair_in_set_map_contains +
map_dom_preserved_by_superset) that appears after every BST insert in
iterate-and-insert operations.

**lemma_ordkeymap_wf_type_axioms** — Extracts the type-level axiom assertions
(pair_key_determines_order, obeys_cmp_spec, view_ord_consistent, obeys_feq_fulls)
from spec_ordkeymap_wf. Replaced 7 sites with 4-8 assertion lines each.

**lemma_values_preserved_from_subset** — Proves that map values are preserved
when one pair-set is a subset of another with key uniqueness. Used in intersect,
difference, filter, restrict, subtract.

**lemma_loop_init_sorted** — Combines the 3-lemma loop setup block
(sorted_keys_pairwise_distinct + key_unique_empty + view_gen_empty).

### Chap38 BSTParaSpecsAndLemmas.rs (new, 116 lines)

Created shared module with:
- `view_ord_consistent` spec fn
- 5 cmp proof lemmas (antisymmetry, transitivity, eq_subst, equal_congruent, equal_congruent_right)
- `lemma_cmp_order_axioms`

### Chap38 BSTParaStEph.rs (1664 -> 1589, -75 lines)

Removed local cmp lemmas and view_ord_consistent; imports from BSTParaSpecsAndLemmas.

### Chap38 BSTParaMtEph.rs (1821 -> 1745, -76 lines)

Same deduplication as StEph.

## Verification

- Full validation: 5583 verified, 0 errors
- RTT: 3776 passed
- PTT: 221 passed

## Net line delta

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 41 | OrdKeyMap.rs | 4899 | 4837 | -62 |
| 2 | 38 | BSTParaStEph.rs | 1664 | 1589 | -75 |
| 3 | 38 | BSTParaMtEph.rs | 1821 | 1745 | -76 |
| 4 | 38 | BSTParaSpecsAndLemmas.rs | 0 | 116 | +116 |
| 5 | - | lib.rs | - | - | +1 |
| | | **Total** | | | **-96** |

## Techniques

- Proof lemma extraction: identified repeated 4-lemma blocks and consolidated into
  single combined lemma calls.
- Module deduplication: factored identical proof lemmas from StEph and MtEph into
  a shared SpecsAndLemmas module, following the Chap37 BSTSpecsAndLemmas pattern.
- `pub use` re-export with `#[cfg(verus_keep_ghost)]` gating to maintain backward
  compatibility for downstream importers of view_ord_consistent.
