# Chapter 5 Review Against Prose

## Phase 1: Function Inventory

### 1a. SetStEph.rs (22 functions)

| # | Chap | File | Function | Mode | Trait | V! | SpecStr |
|---|------|------|----------|------|-------|-----|---------|
| 1 | 5 | SetStEph.rs | `valid_key_type` | spec | - | Y | n/a |
| 2 | 5 | SetStEph.rs | `spec_setsteph_wf_generic` | spec | - | Y | n/a |
| 3 | 5 | SetStEph.rs | `lemma_singleton_choose` | proof | - | Y | strong |
| 4 | 5 | SetStEph.rs | `spec_setsteph_wf` | spec | SetStEphTrait | Y | n/a |
| 5 | 5 | SetStEph.rs | `spec_valid_key_type` | spec | SetStEphTrait | Y | n/a |
| 6 | 5 | SetStEph.rs | `from_vec` | exec | SetStEphTrait | Y | strong |
| 7 | 5 | SetStEph.rs | `iter` | exec | SetStEphTrait | Y | strong |
| 8 | 5 | SetStEph.rs | `to_seq` | exec | SetStEphTrait | Y | strong |
| 9 | 5 | SetStEph.rs | `empty` | exec | SetStEphTrait | Y | strong |
| 10 | 5 | SetStEph.rs | `singleton` | exec | SetStEphTrait | Y | strong |
| 11 | 5 | SetStEph.rs | `size` | exec | SetStEphTrait | Y | strong |
| 12 | 5 | SetStEph.rs | `mem` | exec | SetStEphTrait | Y | strong |
| 13 | 5 | SetStEph.rs | `insert` | exec | SetStEphTrait | Y | strong |
| 14 | 5 | SetStEph.rs | `union` | exec | SetStEphTrait | Y | strong |
| 15 | 5 | SetStEph.rs | `disjoint_union` | exec | SetStEphTrait | Y | strong |
| 16 | 5 | SetStEph.rs | `intersection` | exec | SetStEphTrait | Y | strong |
| 17 | 5 | SetStEph.rs | `elt_cross_set` | exec | SetStEphTrait | Y | strong |
| 18 | 5 | SetStEph.rs | `cartesian_product` | exec | SetStEphTrait | Y | strong |
| 19 | 5 | SetStEph.rs | `all_nonempty` | exec | SetStEphTrait | Y | strong |
| 20 | 5 | SetStEph.rs | `partition_on_elt` | exec | SetStEphTrait | Y | strong |
| 21 | 5 | SetStEph.rs | `partition` | exec | SetStEphTrait | Y | strong |
| 22 | 5 | SetStEph.rs | `split` | exec | SetStEphTrait | Y | strong |
| 23 | 5 | SetStEph.rs | `choose` | exec | SetStEphTrait | Y | strong |
| 24 | 5 | SetStEph.rs | `next` (Iterator) | exec | Iterator | Y | strong |
| 25 | 5 | SetStEph.rs | `hash` | exec | Hash | Y | none |
| 26 | 5 | SetStEph.rs | `eq` (PartialEq) | exec | PartialEq | Y | strong |

### 1b. SetMtEph.rs (21 functions + coarse-lock delegators)

| # | Chap | File | Function | Mode | Trait | V! | SpecStr |
|---|------|------|----------|------|-------|-----|---------|
| 1 | 5 | SetMtEph.rs | `valid_key_type` | spec | - | Y | n/a |
| 2 | 5 | SetMtEph.rs | `spec_setmteph_wf_generic` | spec | - | Y | n/a |
| 3 | 5 | SetMtEph.rs | `lemma_singleton_choose` | proof | - | Y | strong |
| 4 | 5 | SetMtEph.rs | `spec_setmteph_wf` | spec | SetMtEphTrait | Y | n/a |
| 5 | 5 | SetMtEph.rs | `spec_valid_key_type` | spec | SetMtEphTrait | Y | n/a |
| 6 | 5 | SetMtEph.rs | `from_vec` | exec | SetMtEphTrait | Y | strong |
| 7 | 5 | SetMtEph.rs | `iter` | exec | SetMtEphTrait | Y | strong |
| 8 | 5 | SetMtEph.rs | `to_seq` | exec | SetMtEphTrait | Y | strong |
| 9 | 5 | SetMtEph.rs | `empty` | exec | SetMtEphTrait | Y | strong |
| 10 | 5 | SetMtEph.rs | `singleton` | exec | SetMtEphTrait | Y | strong |
| 11 | 5 | SetMtEph.rs | `size` | exec | SetMtEphTrait | Y | strong |
| 12 | 5 | SetMtEph.rs | `mem` | exec | SetMtEphTrait | Y | strong |
| 13 | 5 | SetMtEph.rs | `insert` | exec | SetMtEphTrait | Y | strong |
| 14 | 5 | SetMtEph.rs | `union` | exec | SetMtEphTrait | Y | strong |
| 15 | 5 | SetMtEph.rs | `disjoint_union` | exec | SetMtEphTrait | Y | strong |
| 16 | 5 | SetMtEph.rs | `intersection` | exec | SetMtEphTrait | Y | strong |
| 17 | 5 | SetMtEph.rs | `elt_cross_set` | exec | SetMtEphTrait | Y | strong |
| 18 | 5 | SetMtEph.rs | `cartesian_product` | exec | SetMtEphTrait | Y | strong |
| 19 | 5 | SetMtEph.rs | `all_nonempty` | exec | SetMtEphTrait | Y | strong |
| 20 | 5 | SetMtEph.rs | `partition_on_elt` | exec | SetMtEphTrait | Y | strong |
| 21 | 5 | SetMtEph.rs | `partition` | exec | SetMtEphTrait | Y | strong |
| 22 | 5 | SetMtEph.rs | `choose` | exec | SetMtEphTrait | Y | strong |
| 23 | 5 | SetMtEph.rs | `next` (Iterator) | exec | Iterator | Y | strong |
| 24 | 5 | SetMtEph.rs | `hash` | exec | Hash | Y | none |
| 25 | 5 | SetMtEph.rs | `eq` (PartialEq) | exec | PartialEq | Y | strong |

### 1c. RelationStEph.rs (12 functions)

| # | Chap | File | Function | Mode | Trait | V! | SpecStr |
|---|------|------|----------|------|-------|-----|---------|
| 1 | 5 | RelationStEph.rs | `spec_relationsteph_wf` | spec | RelationStEphTrait | Y | n/a |
| 2 | 5 | RelationStEph.rs | `spec_valid_key_type` | spec | RelationStEphTrait | Y | n/a |
| 3 | 5 | RelationStEph.rs | `spec_finite` | spec | RelationStEphTrait | Y | n/a |
| 4 | 5 | RelationStEph.rs | `empty` | exec | RelationStEphTrait | Y | strong |
| 5 | 5 | RelationStEph.rs | `from_set` | exec | RelationStEphTrait | Y | strong |
| 6 | 5 | RelationStEph.rs | `from_vec` | exec | RelationStEphTrait | Y | strong |
| 7 | 5 | RelationStEph.rs | `size` | exec | RelationStEphTrait | Y | strong |
| 8 | 5 | RelationStEph.rs | `domain` | exec | RelationStEphTrait | Y | strong |
| 9 | 5 | RelationStEph.rs | `range` | exec | RelationStEphTrait | Y | strong |
| 10 | 5 | RelationStEph.rs | `mem` | exec | RelationStEphTrait | Y | strong |
| 11 | 5 | RelationStEph.rs | `relates` | exec | RelationStEphTrait | Y | strong |
| 12 | 5 | RelationStEph.rs | `iter` | exec | RelationStEphTrait | Y | strong |
| 13 | 5 | RelationStEph.rs | `next` (Iterator) | exec | Iterator | Y | strong |
| 14 | 5 | RelationStEph.rs | `hash` | exec | Hash | Y | none |
| 15 | 5 | RelationStEph.rs | `eq` (PartialEq) | exec | PartialEq | Y | strong |

### 1d. MappingStEph.rs (16 functions)

| # | Chap | File | Function | Mode | Trait | V! | SpecStr |
|---|------|------|----------|------|-------|-----|---------|
| 1 | 5 | MappingStEph.rs | `is_functional_set` | spec | - | Y | n/a |
| 2 | 5 | MappingStEph.rs | `is_functional_seq` | spec | - | Y | n/a |
| 3 | 5 | MappingStEph.rs | `is_functional_seq_at` | spec | - | Y | n/a |
| 4 | 5 | MappingStEph.rs | `is_functional_relation` | spec | - | Y | n/a |
| 5 | 5 | MappingStEph.rs | `is_functional_set_at` | spec | - | Y | n/a |
| 6 | 5 | MappingStEph.rs | `is_functional_vec` | exec | MappingStEphTrait | Y | strong |
| 7 | 5 | MappingStEph.rs | `is_functional_vec_at` | exec | MappingStEphTrait | Y | strong |
| 8 | 5 | MappingStEph.rs | `is_functional_SetStEph_at` | exec | MappingStEphTrait | Y | strong |
| 9 | 5 | MappingStEph.rs | `is_functional_SetStEph` | exec | MappingStEphTrait | Y | strong |
| 10 | 5 | MappingStEph.rs | `is_functional_RelationStEph` | exec | MappingStEphTrait | Y | strong |
| 11 | 5 | MappingStEph.rs | `empty` | exec | MappingStEphTrait | Y | strong |
| 12 | 5 | MappingStEph.rs | `from_vec` | exec | MappingStEphTrait | Y | strong |
| 13 | 5 | MappingStEph.rs | `from_relation` | exec | MappingStEphTrait | Y | strong |
| 14 | 5 | MappingStEph.rs | `size` | exec | MappingStEphTrait | Y | **hole** |
| 15 | 5 | MappingStEph.rs | `domain` | exec | MappingStEphTrait | Y | strong |
| 16 | 5 | MappingStEph.rs | `range` | exec | MappingStEphTrait | Y | strong |
| 17 | 5 | MappingStEph.rs | `mem` | exec | MappingStEphTrait | Y | strong |
| 18 | 5 | MappingStEph.rs | `iter` | exec | MappingStEphTrait | Y | strong |
| 19 | 5 | MappingStEph.rs | `next` (Iterator) | exec | Iterator | Y | strong |
| 20 | 5 | MappingStEph.rs | `hash` | exec | Hash | Y | none |
| 21 | 5 | MappingStEph.rs | `eq` (PartialEq) | exec | PartialEq | Y | strong |

### 1e. KleeneStPer.rs (13 functions)

| # | Chap | File | Function | Mode | Trait | V! | SpecStr |
|---|------|------|----------|------|-------|-----|---------|
| 1 | 5 | KleeneStPer.rs | `in_star` | spec | - | Y | n/a |
| 2 | 5 | KleeneStPer.rs | `in_plus` | spec | - | Y | n/a |
| 3 | 5 | KleeneStPer.rs | `viewed` | spec | - | Y | n/a |
| 4 | 5 | KleeneStPer.rs | `lemma_star_closed_under_concat` | proof | - | Y | strong |
| 5 | 5 | KleeneStPer.rs | `lemma_plus_closed_under_concat` | proof | - | Y | strong |
| 6 | 5 | KleeneStPer.rs | `ptt_star_contains_empty` | proof | - | Y | strong |
| 7 | 5 | KleeneStPer.rs | `ptt_plus_rejects_empty` | proof | - | Y | strong |
| 8 | 5 | KleeneStPer.rs | `ptt_singleton_in_star_and_plus` | proof | - | Y | strong |
| 9 | 5 | KleeneStPer.rs | `ptt_plus_subset_of_star` | proof | - | Y | strong |
| 10 | 5 | KleeneStPer.rs | `ptt_star_property_transfer` | proof | - | Y | strong |
| 11 | 5 | KleeneStPer.rs | `ptt_star_concat_plus_is_plus` | proof | - | Y | strong |
| 12 | 5 | KleeneStPer.rs | `ptt_plus_concat_star_is_plus` | proof | - | Y | strong |
| 13 | 5 | KleeneStPer.rs | `new` | exec | KleeneStPerTrait | Y | strong |
| 14 | 5 | KleeneStPer.rs | `mem_star` | exec | KleeneStPerTrait | Y | strong |
| 15 | 5 | KleeneStPer.rs | `mem_plus` | exec | KleeneStPerTrait | Y | strong |
| 16 | 5 | KleeneStPer.rs | `alphabet` | exec | KleeneStPerTrait | Y | strong |

### Inventory Summary

| # | Chap | File | Total Fns | Spec | Proof | Exec | Holes |
|---|------|------|-----------|------|-------|------|-------|
| 1 | 5 | SetStEph.rs | 26 | 4 | 1 | 21 | 0 |
| 2 | 5 | SetMtEph.rs | 25 | 4 | 1 | 20 | 0 |
| 3 | 5 | RelationStEph.rs | 15 | 3 | 0 | 12 | 0 |
| 4 | 5 | MappingStEph.rs | 21 | 5 | 0 | 16 | 1 |
| 5 | 5 | KleeneStPer.rs | 16 | 3 | 9 | 4 | 0 |

Total: 103 functions, 1 proof hole (external_body on `MappingStEph::size`).
6 accept() markers (eq workaround patterns, not counted as holes).

---

## Phase 2: Prose Inventory

APAS Chapter 5 ("Sets and Relations") is purely definitional. It defines mathematical objects and their properties. There are no algorithms with computational cost specifications in the prose. The chapter contains:

### Definitions

| # | Prose Item | Type | APAS Section |
|---|-----------|------|--------------|
| 1 | Set (element, member, empty set) | Definition | 5.0 |
| 2 | Set comprehension | Definition | 5.0 |
| 3 | Definition 5.1: Union and Intersection | Definition | 5.1 |
| 4 | Definition 5.2: Cartesian Product | Definition | 5.1 |
| 5 | Definition 5.3: Set Partition | Definition | 5.1 |
| 6 | Definition 5.4: Kleene Star and Plus | Definition | 5.1 |
| 7 | Definition 5.5: Relation (domain, range) | Definition | 5.2 |
| 8 | Definition 5.6: Function/Mapping | Definition | 5.2 |

### Examples

| # | Prose Item | Type | APAS Section |
|---|-----------|------|--------------|
| 1 | Example 5.1: Cartesian product of {0,1,2,3} x {a,b} | Example | 5.1 |
| 2 | Example 5.2: Partition of {1,...,6} | Example | 5.1 |
| 3 | Example 5.3: Kleene star/plus of {a,b} | Example | 5.1 |
| 4 | Example 5.4: Relation vs function | Example | 5.2 |

### Exercises

| # | Prose Item | Type | APAS Section |
|---|-----------|------|--------------|
| 1 | Exercise 5.1: Kleene star and plus closed under concatenation | Exercise/Proof | 5.1 |

### Cost Specifications

None. Chapter 5 is purely definitional with no algorithmic cost analysis.

---

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All cost annotations are already present in the source files as doc comments on the trait declarations. The annotations follow the required format. Since Chapter 5 is purely definitional (no cost specifications in the prose), all APAS cost lines correctly state either "(no cost stated)" or describe hash-set-backed implementation costs. The annotations were added in prior rounds and are complete.

Summary of cost annotation status by file:

| # | Chap | File | Annotated Fns | Status |
|---|------|------|--------------|--------|
| 1 | 5 | SetStEph.rs | 18 exec fns | Complete |
| 2 | 5 | SetMtEph.rs | 17 exec fns | Complete |
| 3 | 5 | RelationStEph.rs | 9 exec fns | Complete |
| 4 | 5 | MappingStEph.rs | 13 exec fns | Complete |
| 5 | 5 | KleeneStPer.rs | 4 exec fns | Complete |

### Phase 3b: Implementation Deviations from Prose

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 5 | SetStEph.rs | `split` | Not in prose. Utility for splitting a set into two disjoint parts of specified sizes. |
| 2 | 5 | SetStEph.rs | `to_seq` | Not in prose. Conversion utility from set to Vec. |
| 3 | 5 | SetStEph.rs | `from_vec` | Not in prose. Construction utility. |
| 4 | 5 | SetStEph.rs | `singleton` | Not in prose. Convenience constructor. |
| 5 | 5 | SetStEph.rs | `insert` | Not in prose. Mutation operation for ephemeral set building. |
| 6 | 5 | SetStEph.rs | `choose` | Not in prose. Maps to vstd `Set::choose()`. |
| 7 | 5 | MappingStEph.rs | `is_functional_*` | Not in prose as executable. Definition 5.6 defines functionality as a property; the code provides executable checks. |
| 8 | 5 | MappingStEph.rs | `is_functional_SetStEph` | Quadratic work O(|s|^2). If the prose had stated a cost, it would likely expect O(|s|) via a hash-based approach. |

All prose-defined operations (union, intersection, cartesian product, partition, Kleene star/plus, relation domain/range, mapping) are faithfully implemented. The operational behavior matches the mathematical definitions.

### Phase 3c: Ensures vs Prose Postconditions

| # | Chap | File | Function | Prose Spec | Code ensures | Match |
|---|------|------|----------|-----------|--------------|-------|
| 1 | 5 | SetStEph.rs | `union` | A union B | `union@ == self@.union(s2@)` | exact |
| 2 | 5 | SetStEph.rs | `intersection` | A intersect B | `intersection@ == self@.intersect(s2@)` | exact |
| 3 | 5 | SetStEph.rs | `cartesian_product` | {(a,b) : a in A, b in B} | `forall av bv: product@.contains((av,bv)) <==> self@.contains(av) && s2@.contains(bv)` | exact |
| 4 | 5 | SetStEph.rs | `partition` | Def 5.3 (nonempty, covers, disjoint) | Three-conjunct ensures matching all three conditions | exact |
| 5 | 5 | RelationStEph.rs | `domain` | {a : (a,b) in R} | `domain@ == Set::new(\|x\| exists \|y\| self@.contains((x,y)))` | exact |
| 6 | 5 | RelationStEph.rs | `range` | {b : (a,b) in R} | `range@ == Set::new(\|y\| exists \|x\| self@.contains((x,y)))` | exact |
| 7 | 5 | MappingStEph.rs | `is_functional_*` | Def 5.6: |R| = |domain(R)| | `is_functional_set(s)` checks unique domain keys | equivalent |
| 8 | 5 | KleeneStPer.rs | `mem_star` | Def 5.4: all chars in alphabet | `member == in_star(self@, viewed(s@))` | exact |
| 9 | 5 | KleeneStPer.rs | `mem_plus` | Def 5.4: nonempty + all chars | `member == in_plus(self@, viewed(s@))` | exact |
| 10 | 5 | KleeneStPer.rs | `lemma_star_closed_under_concat` | Exercise 5.1 | `in_star(alphabet, s1.add(s2))` | exact |
| 11 | 5 | KleeneStPer.rs | `lemma_plus_closed_under_concat` | Exercise 5.1 | `in_plus(alphabet, s1.add(s2))` | exact |

All ensures clauses faithfully match the prose definitions. No weakened postconditions detected.

---

## Phase 4: Parallelism Review (SetMtEph)

SetMtEph is the multi-threaded variant of SetStEph. It uses `HFSchedulerMtEph` for fork-join parallelism.

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|---------------|-------|
| 1 | 5 | SetMtEph.rs | `from_vec` | sequential | Loop insertion, no parallelism |
| 2 | 5 | SetMtEph.rs | `iter` | delegating | Delegates to inner hash set |
| 3 | 5 | SetMtEph.rs | `to_seq` | sequential | Loop clone, no parallelism |
| 4 | 5 | SetMtEph.rs | `empty` | delegating | O(1), no parallelism needed |
| 5 | 5 | SetMtEph.rs | `singleton` | delegating | O(1), no parallelism needed |
| 6 | 5 | SetMtEph.rs | `size` | delegating | O(1), no parallelism needed |
| 7 | 5 | SetMtEph.rs | `mem` | delegating | O(1), no parallelism needed |
| 8 | 5 | SetMtEph.rs | `insert` | delegating | O(1), no parallelism needed |
| 9 | 5 | SetMtEph.rs | `union` | sequential | Iterate + insert, no parallelism |
| 10 | 5 | SetMtEph.rs | `disjoint_union` | sequential | Iterate + insert, no parallelism |
| 11 | 5 | SetMtEph.rs | `intersection` | sequential | Iterate + filter, no parallelism |
| 12 | 5 | SetMtEph.rs | `elt_cross_set` | sequential | Inner loop of cartesian_product |
| 13 | 5 | SetMtEph.rs | `cartesian_product` | **parallel** | spawn/wait per element via HFScheduler |
| 14 | 5 | SetMtEph.rs | `all_nonempty` | sequential | Loop scan |
| 15 | 5 | SetMtEph.rs | `partition_on_elt` | sequential | Loop scan |
| 16 | 5 | SetMtEph.rs | `partition` | sequential | Loop over elements |
| 17 | 5 | SetMtEph.rs | `choose` | delegating | O(1), no parallelism needed |

### Parallelism Gap Analysis

`cartesian_product` is the only function with genuine parallelism: it spawns one task per element in `self`, each computing `elt_cross_set` for that element against `s2`. This is the only operation where parallelism makes algorithmic sense for these hash-set-backed data structures. The remaining operations (union, intersection, etc.) are inherently sequential due to hash table mutation -- there is no parallel algorithm in APAS for these set operations.

The `cartesian_product` implementation uses `spawn`/`wait` with a join phase that calls `disjoint_union` to merge the per-element cross products. The span analysis in the cost annotation correctly notes that the sequential join phase dominates span.

Parallelism status: **cartesian_product is correctly parallel.** All O(1) operations correctly delegate. Sequential operations are appropriately sequential for hash-set-backed implementations. No parallelism gaps.

---

## Phase 5: Runtime Test Review

### SetStEph RTT coverage

| # | Chap | File | Function | Tested | Test Functions |
|---|------|------|----------|--------|---------------|
| 1 | 5 | SetStEph.rs | `from_vec` | yes | `test_set_fromvec`, `test_trait_from_vec` |
| 2 | 5 | SetStEph.rs | `iter` | yes | `test_set_iter`, `test_set_iterator_boundaries` |
| 3 | 5 | SetStEph.rs | `to_seq` | no | - |
| 4 | 5 | SetStEph.rs | `empty` | yes | `test_set_empty`, `test_trait_empty` |
| 5 | 5 | SetStEph.rs | `singleton` | yes | `test_set_singleton`, `test_trait_singleton` |
| 6 | 5 | SetStEph.rs | `size` | yes | `test_set_size_comprehensive` |
| 7 | 5 | SetStEph.rs | `mem` | yes | `test_set_mem_comprehensive` |
| 8 | 5 | SetStEph.rs | `insert` | yes | `test_set_insert` |
| 9 | 5 | SetStEph.rs | `union` | yes | `test_set_union`, `test_trait_union` |
| 10 | 5 | SetStEph.rs | `disjoint_union` | no | - |
| 11 | 5 | SetStEph.rs | `intersection` | yes | `test_set_intersection`, `test_trait_intersection` |
| 12 | 5 | SetStEph.rs | `elt_cross_set` | indirect | Tested via `cartesian_product` |
| 13 | 5 | SetStEph.rs | `cartesian_product` | yes | `test_cartesian_product_example_5_1`, `test_trait_cartesian_product` |
| 14 | 5 | SetStEph.rs | `all_nonempty` | indirect | Tested via `partition` |
| 15 | 5 | SetStEph.rs | `partition_on_elt` | indirect | Tested via `partition` |
| 16 | 5 | SetStEph.rs | `partition` | yes | `test_partition_example_5_2_true/false`, `test_trait_partition` |
| 17 | 5 | SetStEph.rs | `split` | no | - |
| 18 | 5 | SetStEph.rs | `choose` | no | - |

**Uncovered:** `to_seq`, `disjoint_union`, `split`, `choose`.

### SetMtEph RTT coverage

| # | Chap | File | Function | Tested | Test Functions |
|---|------|------|----------|--------|---------------|
| 1 | 5 | SetMtEph.rs | `from_vec` | yes | `test_set_basic` |
| 2 | 5 | SetMtEph.rs | `union` | yes | `test_set_union` |
| 3 | 5 | SetMtEph.rs | `cartesian_product` | yes | `test_cartesian_product`, `test_cartesian_product_larger` |
| 4 | 5 | SetMtEph.rs | `empty` | indirect | Used in from_vec |
| 5 | 5 | SetMtEph.rs | `size` | yes | `test_set_basic` |
| 6 | 5 | SetMtEph.rs | `mem` | yes | `test_set_basic` |
| 7 | 5 | SetMtEph.rs | Other (disjoint_union, intersection, partition, etc.) | no | - |

**Uncovered:** `iter` (directly), `to_seq`, `singleton`, `insert`, `disjoint_union`, `intersection`, `elt_cross_set`, `all_nonempty`, `partition_on_elt`, `partition`, `choose`.

### RelationStEph RTT coverage

| # | Chap | File | Function | Tested | Test Functions |
|---|------|------|----------|--------|---------------|
| 1 | 5 | RelationStEph.rs | `empty` | yes | `test_relation_empty` |
| 2 | 5 | RelationStEph.rs | `from_set` | yes | `test_relation_domain_range_and_mem` |
| 3 | 5 | RelationStEph.rs | `from_vec` | yes | `test_relation_fromvec` |
| 4 | 5 | RelationStEph.rs | `size` | yes | `test_relation_size` |
| 5 | 5 | RelationStEph.rs | `domain` | yes | `test_relation_domain_range_and_mem` |
| 6 | 5 | RelationStEph.rs | `range` | yes | `test_relation_domain_range_and_mem` |
| 7 | 5 | RelationStEph.rs | `mem` | yes | `test_relation_domain_range_and_mem` |
| 8 | 5 | RelationStEph.rs | `relates` | no | - |
| 9 | 5 | RelationStEph.rs | `iter` | yes | `test_relation_iter` |

**Uncovered:** `relates` (directly; it is exercised indirectly through MappingStEph.mem).

### MappingStEph RTT coverage

| # | Chap | File | Function | Tested | Test Functions |
|---|------|------|----------|--------|---------------|
| 1 | 5 | MappingStEph.rs | `empty` | yes | `test_empty_mapping`, `test_empty_mapping_trait` |
| 2 | 5 | MappingStEph.rs | `from_vec` | yes | `test_from_vec_basic` |
| 3 | 5 | MappingStEph.rs | `from_relation` | yes | `test_from_relation` |
| 4 | 5 | MappingStEph.rs | `size` | yes | `test_from_vec_basic` |
| 5 | 5 | MappingStEph.rs | `domain` | yes | `test_domain_and_range` |
| 6 | 5 | MappingStEph.rs | `range` | yes | `test_domain_and_range` |
| 7 | 5 | MappingStEph.rs | `mem` | yes | `test_mem_comprehensive` |
| 8 | 5 | MappingStEph.rs | `iter` | yes | `test_iter` |
| 9 | 5 | MappingStEph.rs | `is_functional_vec` | no | - |
| 10 | 5 | MappingStEph.rs | `is_functional_vec_at` | no | - |
| 11 | 5 | MappingStEph.rs | `is_functional_SetStEph_at` | no | - |
| 12 | 5 | MappingStEph.rs | `is_functional_SetStEph` | no | - |
| 13 | 5 | MappingStEph.rs | `is_functional_RelationStEph` | no | - |

**Uncovered:** All five `is_functional_*` functions (precondition checkers for `from_vec`/`from_relation`).

### KleeneStPer RTT coverage

| # | Chap | File | Function | Tested | Test Functions |
|---|------|------|----------|--------|---------------|
| 1 | 5 | KleeneStPer.rs | `new` | yes | `test_star_empty_string_always_accepted` |
| 2 | 5 | KleeneStPer.rs | `mem_star` | yes | `test_star_*` (5 tests) |
| 3 | 5 | KleeneStPer.rs | `mem_plus` | yes | `test_plus_*` (3 tests) |
| 4 | 5 | KleeneStPer.rs | `alphabet` | yes | `test_alphabet_accessor` |

**Uncovered:** None. Full RTT coverage for KleeneStPer.

### RTT Summary

| # | Chap | File | Exec Fns | Covered | Uncovered |
|---|------|------|----------|---------|-----------|
| 1 | 5 | SetStEph.rs | 21 | 17 | 4 (to_seq, disjoint_union, split, choose) |
| 2 | 5 | SetMtEph.rs | 20 | 6 | 14 |
| 3 | 5 | RelationStEph.rs | 12 | 11 | 1 (relates) |
| 4 | 5 | MappingStEph.rs | 16 | 11 | 5 (is_functional_*) |
| 5 | 5 | KleeneStPer.rs | 4 | 4 | 0 |

---

## Phase 6: PTT Review

### PTT Files and Coverage

| # | Chap | File | PTT File | Loop Patterns Tested |
|---|------|------|----------|---------------------|
| 1 | 5 | SetStEph.rs | `SetStEph.rs` | loop-loop, for-iter, from_vec |
| 2 | 5 | SetMtEph.rs | `SetMtEph.rs` | loop-loop, for-iter, from_vec |
| 3 | 5 | RelationStEph.rs | `RelationStEph.rs` | loop-loop, for-iter |
| 4 | 5 | MappingStEph.rs | `MappingStEph.rs` | loop-loop, for-iter |
| 5 | 5 | SetStEph.rs | `ProveSetMtEph.rs` | loop-borrow-iter, for-borrow-iter, for-borrow-into |
| 6 | 5 | RelationStEph.rs | `ProveRelationStEph.rs` | loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into |
| 7 | 5 | MappingStEph.rs | `ProveMappingStEph.rs` | loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into |
| 8 | 5 | KleeneStPer.rs | `ProveKleeneStPer.rs` | mem_star, mem_plus, alphabet (no iterator) |

### PTT Loop Pattern Coverage Matrix

| # | Chap | File | loop-loop | for-iter | loop-borrow-iter | loop-borrow-into | for-borrow-iter | for-borrow-into |
|---|------|------|-----------|----------|-----------------|-----------------|-----------------|-----------------|
| 1 | 5 | SetStEph.rs | yes | yes | - | - | - | - |
| 2 | 5 | SetMtEph.rs | yes | yes | yes | - | yes | yes |
| 3 | 5 | RelationStEph.rs | yes | yes | yes | yes | yes | yes |
| 4 | 5 | MappingStEph.rs | yes | yes | yes | yes | yes | yes |

**Gaps:** SetStEph is missing the four "Prove" loop patterns (loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into). The older-style PTT only covers loop-loop, for-iter, and from_vec. However, since SetStEph.rs does not have an `IntoIterator` for `&'a SetStEph`, the loop-borrow-into and for-borrow-into patterns would not apply. The loop-borrow-iter and for-borrow-iter patterns are applicable and would be beneficial to add.

KleeneStPer has no iterator and correctly tests only its exec functions (mem_star, mem_plus, alphabet).

---

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Set comprehension | Not directly implemented. Verus `Set::new` provides the same capability at spec level. No exec-level set comprehension is needed or provided. |
| 2 | Example 5.1 (Cartesian product) | Tested in RTT `test_cartesian_product_example_5_1`. |
| 3 | Example 5.2 (Partition) | Tested in RTT `test_partition_example_5_2_true/false`. |
| 4 | Example 5.3 (Kleene star/plus) | Tested in RTT `test_star_multi_element_strings`. |
| 5 | Example 5.4 (Relation vs function) | Partially tested via `test_from_relation` and `is_functional` checks in MappingLit macro. |

No prose-defined operations are missing from the implementation.

### Code with no prose counterpart

| # | Chap | File | Function | Purpose |
|---|------|------|----------|---------|
| 1 | 5 | SetStEph.rs | `from_vec` | Construction utility |
| 2 | 5 | SetStEph.rs | `singleton` | Convenience constructor |
| 3 | 5 | SetStEph.rs | `insert` | Ephemeral mutation |
| 4 | 5 | SetStEph.rs | `to_seq` | Conversion utility |
| 5 | 5 | SetStEph.rs | `split` | Set splitting utility |
| 6 | 5 | SetStEph.rs | `choose` | vstd Set::choose() bridge |
| 7 | 5 | SetStEph.rs | `iter` | Iterator access |
| 8 | 5 | MappingStEph.rs | `is_functional_*` (5) | Executable checks for Def 5.6 |
| 9 | 5 | RelationStEph.rs | `relates` | Alias for `mem` with Pair arg |

All are legitimate Verus/APAS-VERUS scaffolding. None represent algorithmic deviations.

### Proof Holes

| # | Chap | File | Function | Hole Type | Description |
|---|------|------|----------|-----------|-------------|
| 1 | 5 | MappingStEph.rs | `size` | external_body | `MappingStEph::size` delegates to `self.mapping.size()` (RelationStEph). The ensures clause `size == self@.dom().len()` requires proving that the relation's pair count equals the domain size under the functionality invariant. The proof connects `self.mapping@.len()` (pair count) to `self@.dom().len()` (unique domain keys), which requires the `is_functional_set` invariant. This is provable but the proof has not been written. |

### Accept Markers (not holes, informational)

| # | Chap | File | Line | Type | Pattern |
|---|------|------|------|------|---------|
| 1 | 5 | SetStEph.rs | 896 | accept | PartialEq eq workaround |
| 2 | 5 | SetMtEph.rs | 1045 | accept | Coarse-lock delegating empty |
| 3 | 5 | SetMtEph.rs | 1054 | accept | Coarse-lock delegating size |
| 4 | 5 | SetMtEph.rs | 1062 | accept | Coarse-lock delegating mem |
| 5 | 5 | SetMtEph.rs | 1073 | accept | Coarse-lock delegating insert |
| 6 | 5 | SetMtEph.rs | 1099 | accept | PartialEq eq workaround |
| 7 | 5 | MappingStEph.rs | 606 | accept | PartialEq eq workaround |

All accept markers follow approved patterns (eq/clone workaround or coarse-lock delegation).

---

## Phase 8: TOC Review

### SetStEph.rs

```
1. module         -- present
2. imports        -- present
3. broadcast use  -- present
4. type defs      -- present
5. view impls     -- present
6. spec fns       -- present
7. proof fns      -- present
8. traits         -- present
9. impls          -- present
10. iterators     -- present
11. derive impls in verus! -- present
12. macros        -- present (outside verus!)
13. derive impls outside verus! -- present
```

Sections correctly omit section 11 (top-level coarse locking -- not an Mt module). All sections are in correct order. Iterator infrastructure (section 10) includes Iter struct, View impl, Iterator::next, GhostIterator, ForLoopGhostIteratorNew, ForLoopGhostIterator. In/out placement is correct: sections 1-11 inside `verus!`, sections 12-13 outside.

### SetMtEph.rs

```
1. module         -- present
2. imports        -- present
3. broadcast use  -- present
4. type defs      -- present
5. view impls     -- present
6. spec fns       -- present
7. proof fns      -- present
8. traits         -- present
9. impls          -- present
10. iterators     -- present
11. top level coarse locking -- present (Mt module)
12. derive impls in verus! -- present
13. macros        -- present (outside verus!)
14. derive impls outside verus! -- present
```

All 14 sections present and correctly ordered. Section 11 correctly included for the Mt module. Coarse-lock pattern uses `SetMtEphInv` RwLockPredicate following the naming convention. In/out placement is correct.

### RelationStEph.rs

```
1. module         -- present
2. imports        -- present
3. broadcast use  -- present
4. type defs      -- present
5. view impls     -- present
8. traits         -- present (sections 6, 7 correctly omitted)
9. impls          -- present
10. iterators     -- present
11. derive impls in verus! -- present
12. macros        -- present (outside verus!)
13. derive impls outside verus! -- present
```

Note: TOC header shows section 11 as "derive impls in verus!" but the standard number for that is 12 when section 11 (coarse locking) is omitted. However, the file's internal numbering (11, 12, 13) is consistent with its own TOC header. The standard says "Omit sections that don't apply." The file omits sections 6 (spec fns) and 7 (proof fns) since RelationStEph has none -- this is correct. In/out placement is correct.

### MappingStEph.rs

```
1. module         -- present
2. imports        -- present
3. broadcast use  -- present
4. type defs      -- present
5. view impls     -- present
6. spec fns       -- present
8. traits         -- present (section 7 omitted -- no proof fns)
9. impls          -- present
10. iterators     -- present
11. derive impls in verus! -- present
12. macros        -- present (outside verus!)
13. derive impls outside verus! -- present
```

Section 7 correctly omitted (no proof fns). In/out placement correct.

### KleeneStPer.rs

```
1. module         -- present
2. imports        -- present
3. broadcast use  -- present
4. type defs      -- present
5. view impls     -- present
6. spec fns       -- present
7. proof fns      -- present
8. traits         -- present
9. impls          -- present
```

Sections 10-14 correctly omitted: KleeneStPer has no iterator, no derive impls, no macros, and no outside-verus code. In/out placement correct (all inside verus!).

### In/Out Placement Table

| # | Chap | File | Section | Expected | Actual | Status |
|---|------|------|---------|----------|--------|--------|
| 1 | 5 | SetStEph.rs | Clone | in | in | correct |
| 2 | 5 | SetStEph.rs | PartialEq/Eq | in | in | correct |
| 3 | 5 | SetStEph.rs | Hash | in | in | correct |
| 4 | 5 | SetStEph.rs | Debug | out | out | correct |
| 5 | 5 | SetStEph.rs | Display | out | out | correct |
| 6 | 5 | SetStEph.rs | SetLit macro | out | out | correct |
| 7 | 5 | SetMtEph.rs | Clone | in | in | correct |
| 8 | 5 | SetMtEph.rs | PartialEq/Eq | in | in | correct |
| 9 | 5 | SetMtEph.rs | Hash | in | in | correct |
| 10 | 5 | SetMtEph.rs | Debug | out | out | correct |
| 11 | 5 | SetMtEph.rs | Display | out | out | correct |
| 12 | 5 | SetMtEph.rs | SetMtLit macro | out | out | correct |
| 13 | 5 | RelationStEph.rs | Clone | in | in | correct |
| 14 | 5 | RelationStEph.rs | PartialEq/Eq | in | in | correct |
| 15 | 5 | RelationStEph.rs | Hash | in | in | correct |
| 16 | 5 | RelationStEph.rs | Debug | out | out | correct |
| 17 | 5 | RelationStEph.rs | Display | out | out | correct |
| 18 | 5 | RelationStEph.rs | RelationLit macro | out | out | correct |
| 19 | 5 | MappingStEph.rs | Clone | in | in | correct |
| 20 | 5 | MappingStEph.rs | PartialEq/Eq | in | in | correct |
| 21 | 5 | MappingStEph.rs | Hash | in | in | correct |
| 22 | 5 | MappingStEph.rs | Debug | out | out | correct |
| 23 | 5 | MappingStEph.rs | Display | out | out | correct |
| 24 | 5 | MappingStEph.rs | MappingLit macro | out | out | correct |

All in/out placements are correct.

---

## Summary

### Chapter 5 Health

| Metric | Value |
|--------|-------|
| Files reviewed | 5 |
| Total functions | 103 |
| Proof holes | 1 (`MappingStEph::size` external_body) |
| Accept markers | 7 (all approved patterns) |
| Clean modules | 4 of 5 (80%) |
| Prose coverage | 100% (all definitions, examples, and exercises implemented) |
| Spec strength | Strong across all modules |
| Parallelism | cartesian_product is correctly parallel in SetMtEph |
| TOC compliance | Full compliance across all 5 files |
| In/out compliance | Full compliance across all 5 files |

### Actionable Items

1. **MappingStEph::size external_body (1 hole).** The proof connecting `self.mapping@.len()` to `self@.dom().len()` under `is_functional_set` is the sole remaining proof obligation. Difficulty: moderate. Requires showing that for a functional set of pairs, the number of pairs equals the number of distinct first components.

2. **RTT gaps.** SetMtEph has significant RTT coverage gaps (14 uncovered functions). SetStEph is missing tests for `to_seq`, `disjoint_union`, `split`, and `choose`. MappingStEph is missing tests for the five `is_functional_*` functions.

3. **PTT gap.** SetStEph is missing the "Prove" PTT patterns (loop-borrow-iter, for-borrow-iter). These would be low-effort to add.

---

Review date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
