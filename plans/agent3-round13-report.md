# Agent 3 — Round 13 Report

## Mission

Apply feq broadcast axiom pattern to eliminate `assume(obeys_feq_full::<T>())` holes
in Chap41, and prove set operation postconditions (filter subset, intersection/difference/union
equality) where possible. Target: -15 holes.

## Results Summary

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 41 | ArraySetStEph.rs | 3 | 0 | -3 |
| 2 | 41 | AVLTreeSetStEph.rs | 14 | 6 | -8 |
| 3 | 41 | AVLTreeSetStPer.rs | 10 | 5 | -5 |
| 4 | 38 | BSTParaStEph.rs | 15 | 15 | 0 |
| | | **Total** | **42** | **26** | **-16** |

Target was -15, achieved **-16**.

## Chap41 Total: 53 → 37 (-16)

## Verification

- 4000 verified, 0 errors
- 2600 RTT passed
- 147 PTT passed

## Techniques Used

### 1. feq broadcast axiom (new infrastructure in feq.rs)

Added `obeys_feq_full_trigger::<T>()` spec fn (always true) and broadcast proof
`axiom_obeys_feq_full` to `group_feq_axioms`. Any file that `broadcast use group_feq_axioms`
can now replace `assume(obeys_feq_full::<T>())` with `assert(obeys_feq_full_trigger::<T>())`.

### 2. Clone view equality via lemma_cloned_view_eq

After `let c = elem.clone()`, call `lemma_cloned_view_eq(*elem, c)` in proof block to
establish `c@ == elem@`. Required for filter/intersection/difference/union proofs where
cloned elements are inserted into result sets.

### 3. Set operation loop invariants

For intersection/difference/union, used two-invariant pattern:
- **Subset**: accumulated result ⊆ target set
- **Completeness**: all processed elements meeting condition are in result

After loop, use `choose` to get witness index proving any element in the target set was
processed, then `assert(result@ =~= target_set)` via extensional equality.

## Per-File Details

### ArraySetStEph.rs (3 → 0, CLEAN)

Replaced 3 `assume(obeys_feq_full::<T>())` in `empty()`, `singleton()`, `find()` with
`assert(obeys_feq_full_trigger::<T>())`. File is now completely clean.

### AVLTreeSetStEph.rs (14 → 6)

- 4 feq assumes eliminated via trigger assert (singleton, find, delete, insert)
- filter: proved `filtered@.subset_of(self@)` via loop invariant + clone view equality
- intersection: proved equality via subset + completeness invariants
- difference: proved equality (same pattern, `!other@.contains(...)` condition)
- union: proved equality (two-loop pattern for self and other elements)

Remaining 6 holes:
- `size`: seq.len vs set.len needs `no_duplicates` invariant in wf
- `find` not-found branch: needs sorted invariant in wf
- `delete`/`insert` postconditions (2 each): `from_vec` to `to_set()` gap

### AVLTreeSetStPer.rs (10 → 5)

- 1 feq assume eliminated in `find` via trigger assert
- filter: proved subset (same pattern as StEph, persistent insert style)
- intersection: proved equality
- difference: proved equality
- union: proved equality

Remaining 5 holes:
- `size`: same seq.len vs set.len gap
- `filter`: `assume(f.requires((&*elem,)))` — trait spec missing precondition,
  adding it breaks Chap43/OrderedSetStPer.rs caller
- `find` not-found: same sorted invariant gap

### BSTParaStEph.rs (15 → 15, no change)

All 15 holes blocked by T::V witness gap: set elements are view values (`<T as View>::V`)
but ordering properties use `forall|t: T|` quantifiers. Cannot bridge from view-level set
membership to value-level `cmp_spec` without a concrete `T` witness. This is an architectural
limitation requiring either:
- A `view_ord_consistent` spec relating view membership to value ordering
- Restructuring to use value-level sets instead of view-level sets

## Files Modified

- `src/vstdplus/feq.rs` — Added broadcast axiom infrastructure
- `src/Chap41/ArraySetStEph.rs` — 3 feq assumes → trigger asserts
- `src/Chap41/AVLTreeSetStEph.rs` — 4 feq assumes + 4 set operation proofs
- `src/Chap41/AVLTreeSetStPer.rs` — 1 feq assume + 4 set operation proofs

## Commit Hash

64027d5dce4b4d7f66e6af973b2ea7c5d7f88448
