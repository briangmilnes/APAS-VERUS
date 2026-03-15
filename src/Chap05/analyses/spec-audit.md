# Chap05 Spec Audit — Sets, Relations, Mappings, Kleene

Auditor: Agent 3, Round 19.
Prose source: `prompts/Chap05.txt` (Definitions 5.1–5.6, Exercise 5.1).

## SetStEph.rs — ADT 5.1 Sets

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `empty` | Def 5.1 (empty set) | **strong** | `@ == Set::empty()` |
| 2 | `singleton` | Def 5.1 | **strong** | `@ == Set::empty().insert(x@)` |
| 3 | `size` | Def 5.1 | **strong** | `size == self@.len()` |
| 4 | `mem` | Def 5.1 (x in A) | **strong** | `contains == self@.contains(x@)` |
| 5 | `insert` | Def 5.1 | **strong** | `@ == old(@).insert(x@)`, inserted flag |
| 6 | `union` | Def 5.1 (A union B) | **strong** | `@ == self@.union(s2@)` |
| 7 | `disjoint_union` | Def 5.1 (disjoint) | **strong** | union + disjoint precondition + len sum |
| 8 | `intersection` | Def 5.1 (A inter B) | **strong** | `@ == self@.intersect(s2@)` |
| 9 | `cartesian_product` | Def 5.2 (A x B) | **strong** | biconditional on (av, bv) |
| 10 | `partition` | Def 5.3 | **strong** | full 3-condition partition spec |
| 11 | `split` | N/A (utility) | **strong** | disjoint, union, sizes |
| 12 | `choose` | N/A (utility) | **strong** | `self@.contains(element@)` |
| 13 | `from_vec` | N/A (utility) | **strong** | `@ == v@.map(…).to_set()` |
| 14 | `iter` | N/A (iterator) | **strong** | bijection, no_duplicates |
| 15 | `to_seq` | N/A (conversion) | **strong** | no_duplicates, biconditional contains |

**Verdict: 15/15 strong.** All operations faithfully encode APAS prose.

## SetMtEph.rs — Mt wrapper for ADT 5.1

| # | Function | Classification | Notes |
|---|----------|---------------|-------|
| 1–15 | All ops | **strong** | Mirrors SetStEph specs exactly |

**Verdict: 15/15 strong.** Mt specs match St specs.

## RelationStEph.rs — Def 5.5 Relations

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `empty` | Def 5.5 | **strong** | `@ == Set::empty()` |
| 2 | `from_set` | Def 5.5 | **strong** | `@ == pairs@` |
| 3 | `from_vec` | Def 5.5 | **strong** | `@ == v@.map(…).to_set()` |
| 4 | `size` | Def 5.5 | **strong** | `size == self@.len()` |
| 5 | `domain` | Def 5.5 (domain) | **strong** | `{x \| exists y, (x,y) in R}` |
| 6 | `range` | Def 5.5 (range) | **strong** | `{y \| exists x, (x,y) in R}` |
| 7 | `mem` | Def 5.5 | **strong** | `contains == self@.contains((a@, b@))` |
| 8 | `relates` | Def 5.5 | **strong** | `contains == self@.contains(p@)` |
| 9 | `iter` | N/A (iterator) | **strong** | bijection, no_duplicates |

**Verdict: 9/9 strong.**

## MappingStEph.rs — Def 5.6 Mappings/Functions

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `empty` | Def 5.6 | **strong** | `@ == Map::empty()` |
| 2 | `from_vec` | Def 5.6 | **strong** | wf + content bridge (fixed R19) |
| 3 | `from_relation` | Def 5.6 | **strong** | wf + content bridge (fixed R19) |
| 4 | `size` | Def 5.6 | **strong** | `size == self@.dom().len()` (fixed R19) |
| 5 | `domain` | Def 5.6 (domain) | **strong** | `@ == self@.dom()` |
| 6 | `range` | Def 5.6 (range) | **strong** | extensional + values() equivalence |
| 7 | `mem` | Def 5.6 | **strong** | dom/value biconditional + contains_pair |
| 8 | `iter` | N/A (iterator) | **strong** | bijection, no_duplicates |
| 9 | `is_functional_vec` | Def 5.6 (validator) | **strong** | iff is_functional_seq |
| 10 | `is_functional_vec_at` | Def 5.6 (validator) | **strong** | iff is_functional_seq_at |
| 11 | `is_functional_SetStEph` | Def 5.6 (validator) | **strong** | iff is_functional_set |
| 12 | `is_functional_SetStEph_at` | Def 5.6 (validator) | **strong** | iff is_functional_set_at |
| 13 | `is_functional_RelationStEph` | Def 5.6 (validator) | **strong** | iff is_functional_relation |

**Verdict: 13/13 strong.** Three gaps fixed in Round 19: `size` (added ensures), `from_vec` (added content bridge), `from_relation` (added content bridge). Impl bodies marked `external_body`.

## KleeneStPer.rs — Def 5.4 Kleene Star/Plus

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `new` | Def 5.4 | **strong** | `@ == alphabet@` |
| 2 | `mem_star` | Def 5.4 (Sigma*) | **strong** | `== in_star(self@, viewed(s@))` |
| 3 | `mem_plus` | Def 5.4 (Sigma+) | **strong** | `== in_plus(self@, viewed(s@))` |
| 4 | `alphabet` | N/A (accessor) | **strong** | `alpha@ == self@` |

Proof lemmas also strong: Exercise 5.1 closure under concatenation proved.

**Verdict: 4/4 strong.**

## Chapter 5 Summary

| File | Strong | Partial | Weak | Missing | Total |
|------|--------|---------|------|---------|-------|
| SetStEph.rs | 15 | 0 | 0 | 0 | 15 |
| SetMtEph.rs | 15 | 0 | 0 | 0 | 15 |
| RelationStEph.rs | 9 | 0 | 0 | 0 | 9 |
| MappingStEph.rs | 13 | 0 | 0 | 0 | 13 |
| KleeneStPer.rs | 4 | 0 | 0 | 0 | 4 |
| **Total** | **56** | **0** | **0** | **0** | **56** |
