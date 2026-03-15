# Chap41 Spec Audit Against ADT 41.1

Audited: ArraySetStEph, AVLTreeSetStEph, AVLTreeSetStPer.
Prose source: ADT 41.1 (Sets).

## Per-Function Classification

| # | Function | Prose Spec | ArraySetStEph | AVLTreeSetStEph | AVLTreeSetStPer | Status |
|---|----------|-----------|---------------|-----------------|-----------------|--------|
| 1 | size | \|a\| | count == self@.len() | same | same | Correct |
| 2 | toSeq | ordered elements | seq@.to_set() =~= self@ | same | same | Correct |
| 3 | empty | {} | empty@ == Set::empty() | same | same | Correct |
| 4 | singleton | {x} | {}.insert(x@) | same | same | Correct |
| 5 | fromSeq | range(a) | constructed@ =~= seq@.to_set() | same | same | **Fixed R17** |
| 6 | filter | {x in S \| f(x)} | filtered@.subset_of(self@) | same | same | Partial |
| 7 | intersection | a ∩ b | common@ == self@.intersect(other@) | same | same | Correct |
| 8 | difference | a \ b | remaining@ == self@.difference(other@) | same | same | Correct |
| 9 | union | a ∪ b | combined@ == self@.union(other@) | same | same | Correct |
| 10 | find | x in a | found == self@.contains(x@) | same | same | Correct |
| 11 | delete | a \ {x} | self@ == old(self)@.remove(x@) | same (persistent) | same | Correct |
| 12 | insert | a ∪ {x} | self@ == old(self)@.insert(x@) | same (persistent) | same | Correct |

## Fix Details

### fromSeq (Fixed in Round 17)

**Before:** `ensures constructed.spec_*_wf()` (+ finite). No relationship to input seq.
**After:** `ensures constructed@ =~= seq@.to_set()` (+ wf, finite).
**Prose:** `fromSeq(a) = range(a)` — the set of all values in the sequence.
**Proof:** Fully proved with loop invariant tracking forward/backward containment.
All 3 files fixed, 0 new holes.

### filter (Partial — No Fix)

**Current:** `filtered@.subset_of(self@)` — result is a subset of input.
**Prose:** `filter(f, S) = {x in S | f(x)}` — exact predicate filtering.
**Gap:** Missing completeness — can't express "every x satisfying f is in the result" without a `Ghost(spec_fn)` companion parameter. The closure-standard filter pattern uses `Ghost(spec_pred)` (see `src/standards/using_closures_standard.rs:91`), but the Set trait signature takes `F: PredSt<T>` without a ghost companion. Adding `Ghost(spec_pred)` would change the trait signature and all callers.
**Classification:** Partial. `subset_of` is the strongest expressible spec given the current signature.
