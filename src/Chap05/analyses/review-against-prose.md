<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap05 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-17
**Last mechanical audit:** 2026-02-19 — full review regeneration; proof holes log updated.
**Prose file:** `prompts/Chap05.txt`
**Source files:** `SetStEph.rs`, `SetMtEph.rs`, `RelationStEph.rs`, `MappingStEph.rs`, `KleeneStPer.rs`

## Phase 1: Inventory

| # | File | Lines | Type | Exec fns | Proof holes |
|---|------|-------|------|----------|-------------|
| 1 | SetStEph.rs | 865 | St | 16 | 1 (PartialEq assume) |
| 2 | SetMtEph.rs | 992 | Mt | ~16 | 1 (PartialEq assume) |
| 3 | RelationStEph.rs | 377 | St | 6 | 0 — clean |
| 4 | MappingStEph.rs | 556 | St | 12 | 1 (PartialEq assume) |
| 5 | KleeneStPer.rs | 265 | St | 3 exec, 2 lemmas, 7 PTTs | 0 — clean |

## Phase 2: Prose Inventory (from prompts/Chap05.txt)

Chapter 5 is purely definitional — no algorithms, no pseudocode, no cost specs.

| # | Item | Type |
|---|------|------|
| 1 | Sets — distinct objects, empty set, membership | Definitions |
| 2 | Def 5.1: Union, Intersection, Disjoint | Definition |
| 3 | Def 5.2: Cartesian Product | Definition |
| 4 | Def 5.3: Set Partition — non-empty blocks, covering, pairwise disjoint | Definition |
| 5 | Def 5.4: Kleene Operators — Sigma-star, Sigma-plus | Definition |
| 6 | Exercise 5.1: Kleene star/plus closed under concatenation | Exercise |
| 7 | Def 5.5: Binary relation, domain, range | Definition |
| 8 | Def 5.6: Function/mapping — relation where each domain element maps to one range element | Definition |

## Phase 3: Algorithmic Analysis — Cost Annotations

All exec functions have cost annotations in the required format:

```
/// - APAS: Work Θ(...), Span Θ(...)
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...) — [reason if different]
```

**Added during this review:** `KleeneStPer::new()` and `KleeneStPer::alphabet()` — both now have full APAS + Claude-Opus annotations.

## Phase 4: Parallelism Review (SetMtEph)

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | from_vec | Theta(1) | Theta(v) | No | Sequential loop |
| 2 | union | Theta(1) | Theta(a+b) | No | Sequential loop |
| 3 | disjoint_union | Theta(1) | Theta(a+b) | No | Sequential loop |
| 4 | intersection | Theta(1) | Theta(a+b) | No | Sequential loop |
| 5 | partition | Theta(1) | Theta(a * parts) | No | Sequential loop |
| 6 | cartesian_product | Theta(b) | Theta(a * b) | Partially | Spawns a tasks but sequential join dominates |

Only cartesian_product uses spawn/wait. All other Mt operations are sequential loops — thread-safe but not parallel. See `docs/WhatIsTheAPASThreadingModelCosts.md` for the open question on PRAM vs fork-join cost models.

## Phase 5: RTT Review

| # | Test file | Coverage |
|---|-----------|----------|
| 1 | TestSetStEph.rs | Comprehensive: macros, cartesian product, partition, equality |
| 2 | TestSetMtEph.rs | Mirrors StEph tests for Mt variant |
| 3 | TestRelationStEph.rs | Relation construction, domain, range, membership |
| 4 | TestMappingStEph.rs | Mapping construction, functional checks, domain, range |
| 5 | TestKleeneStPer.rs | 10 tests: star/plus membership, empty/singleton/multi, integer alphabet |

## Phase 6: PTT Review

| # | PTT file | Module | Purpose |
|---|----------|--------|---------|
| 1 | SetStEph.rs | SetStEph | Iterator verification, callability |
| 2 | SetMtEph.rs | SetMtEph | Iterator verification, callability |
| 3 | RelationStEph.rs | RelationStEph | Iterator verification, callability |
| 4 | MappingStEph.rs | MappingStEph | Iterator verification, callability |
| 5 | ProveRelationStEph.rs | RelationStEph | Additional proof tests |
| 6 | ProveMappingStEph.rs | MappingStEph | Additional proof tests |

**KleeneStPer:** No standalone PTT file in rust_verify_test. The module contains 7 inline PTT proof functions: `ptt_star_contains_empty`, `ptt_plus_rejects_empty`, `ptt_singleton_in_star_and_plus`, `ptt_plus_subset_of_star`, `ptt_star_property_transfer`, `ptt_star_concat_plus_is_plus`, `ptt_plus_concat_star_is_plus`.

## Phase 7: Gap Analysis

**Prose items with no implementation:**
- Kleene operators: Sigma-star, Sigma-plus — IMPLEMENTED (KleeneStPer.rs).
- Exercise 5.1: closure under concatenation — PROVED.

**Code with no prose counterpart:**
- singleton, from_vec, insert, size, split, choose, iter, to_seq, clone
- elt_cross_set, all_nonempty, partition_on_elt (internal helpers)
- Iterator infrastructure (SetStEphIter, GhostIterator, ForLoopGhostIterator)
- PartialEq/Eq implementations
- SetMtEph (parallel variant — same interface, different impl)
- Hash, Debug, Display implementations and macros

## Phase 8: TOC Review

| # | File | Has TOC | Notes |
|---|------|---------|-------|
| 1 | SetStEph.rs | No | — |
| 2 | SetMtEph.rs | Yes | Full 13-section TOC |
| 3 | RelationStEph.rs | No | — |
| 4 | MappingStEph.rs | No | — |
| 5 | KleeneStPer.rs | Yes | 9-section TOC |

## Prose-to-Code Mapping

| # | Prose Definition | Code | Spec Fidelity |
|---|-----------------|------|---------------|
| 1 | Empty set | empty() | Strong — ensures view equals Set::empty() |
| 2 | Membership x in A | mem() | Strong — ensures result matches self@.contains(x@) |
| 3 | Union (Def 5.1) | union() | Strong — ensures view equals self@.union(s2@) |
| 4 | Disjoint (Def 5.1) | disjoint_union() | Strong — requires self@.disjoint(s2@), ensures union + len additivity |
| 5 | Intersection (Def 5.1) | intersection() | Strong — ensures view equals self@.intersect(s2@) |
| 6 | Cartesian Product (Def 5.2) | cartesian_product() | Strong — ensures product contains (a,b) iff self contains a and s2 contains b |
| 7 | Partition (Def 5.3) | partition() | Strong — checks all three conditions from Def 5.3 |
| 8 | Kleene Star (Def 5.4) | KleeneStPer::mem_star() | Strong — ensures result matches in_star spec |
| 9 | Kleene Plus (Def 5.4) | KleeneStPer::mem_plus() | Strong — ensures result matches in_plus spec |
| 10 | Exercise 5.1 | lemma_star/plus_closed_under_concat | Proved — both closure lemmas verified |
| 11 | Relation (Def 5.5) | RelationStEph | Strong — modeled as Set of pairs |
| 12 | Domain of R (Def 5.5) | domain() | Strong — ensures domain contains x iff exists y such that self contains (x,y) |
| 13 | Range of R (Def 5.5) | range() | Strong — symmetric to domain |
| 14 | Function (Def 5.6) | MappingStEph | Strong — is_functional_set captures Def 5.6 exactly |

## Cost Disagreements

The prose has no cost specs (Chapter 5 is purely definitional). Cost annotations in the code come from ADT interface chapters elsewhere in APAS.

| # | Function | APAS annotation | Claude-Opus-4.6 | Issue |
|---|----------|----------------|-----------------|-------|
| 1 | partition (SetStEph) | Was Theta(parts * a^2) | Theta(a * parts) | Fixed: no quadratic factor, O(1) hash lookups |
| 2 | partition (SetMtEph) | Was Theta(parts * a^2) | Theta(a * parts) | Fixed: same as StEph |
| 3 | is_functional_SetStEph | Theta(s) | Theta(s^2) | Iterates set, calls O(s) check per element |
| 4 | is_functional_RelationStEph | Theta(r) | Theta(r^2) | Delegates to is_functional_SetStEph |

Items 1-2 were corrected in the source. Items 3-4 are annotated as disagreements in the source.

## Proof Holes

```
✓ KleeneStPer.rs
   9 clean proof functions
❌ MappingStEph.rs
/home/milnes/projects/APAS-VERUS-agent1/src/Chap05/MappingStEph.rs:518: assume() - assume(r == (self@ == other@));
       508 |         fn eq(&self, other: &Self) -> (equal: bool)
            ...
       516 |                 }
       517 |                 // Verus BUG is preventing this as of Version: 0.2026.02.05.80fb5a4.
       519 |             }
       520 |             r
   Holes: 1 total
      1 × assume()
✓ RelationStEph.rs
❌ SetMtEph.rs
/home/milnes/projects/APAS-VERUS-agent1/src/Chap05/SetMtEph.rs:955: assume() - proof { assume(r == (self@ == other@)); }
       951 |         fn eq(&self, other: &Self) -> (r: bool)
            ...
       953 |         {
       954 |             let r = self.elements == other.elements;
       955 |             r
       956 |         }
   Holes: 1 total
      1 × assume()
   Proof functions: 1 total (1 clean, 0 holed)
❌ SetStEph.rs
/home/milnes/projects/APAS-VERUS-agent1/src/Chap05/SetStEph.rs:832: assume() - proof { assume(equal == (self@ == other@)); }
       827 |         fn eq(&self, other: &Self) -> (equal: bool)
            ...
       830 |             let equal = self.elements == other.elements;
       831 |             // HashSetWithView* eq is external_body so we have to trust it here.
       833 |             equal
       834 |         }
   Holes: 1 total
      1 × assume()
   Proof functions: 1 total (1 clean, 0 holed)

═══════════════════════════════════════════════════════════════
SUMMARY
═══════════════════════════════════════════════════════════════

Modules:
   2 clean (no holes)
   3 holed (contains holes)
   5 total

Proof Functions:
   11 clean
   0 holed
   11 total

Holes Found: 3 total
   3 × assume()
```

## Proof Holes Justification

| # | File | Hole | Justification |
|---|------|------|---------------|
| 1 | SetStEph.rs | assume() in PartialEq::eq | Standard pattern — Verus can't resolve eq_spec through trait machinery |
| 2 | SetMtEph.rs | assume() in PartialEq::eq | Same pattern |
| 3 | MappingStEph.rs | assume() in PartialEq::eq | Same pattern |

All three are the approved PartialEq pattern from the `partialeq-eq-pattern` rule. RelationStEph and KleeneStPer are fully clean.

## Summary

Chap05 is the foundation ADT chapter. All 8 prose definitions are implemented with strong specs that faithfully capture the mathematical definitions. KleeneStPer and Exercise 5.1 were added in a prior review. The partition cost annotation was corrected from quadratic to linear. Two is_functional functions have quadratic cost disagreements. The Mt variant has minimal actual parallelism — only cartesian_product spawns threads, and its sequential join negates the span benefit.

**Changes in this review (2026-02-17):**
- Added cost annotations for `KleeneStPer::new()` and `KleeneStPer::alphabet()`.
- Restructured review into 8 phases (Inventory, Prose Inventory, Algorithmic Analysis, Parallelism, RTT, PTT, Gap Analysis, TOC).
- Updated date and proof-holes output.
- Corrected line counts (SetStEph 865, SetMtEph 992, RelationStEph 377, MappingStEph 556, KleeneStPer 265).
- Documented PTT structure: 4 standalone files + 2 Prove* files; KleeneStPer has 7 inline PTT proof functions.
- Documented TOC status: SetMtEph and KleeneStPer have TOC; others do not.
