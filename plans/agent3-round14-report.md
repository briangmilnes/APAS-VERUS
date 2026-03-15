# Agent 3 — Round 14 Report

## Summary

Eliminated **10 proof holes** in `src/Chap38/BSTParaStEph.rs` (15 → 5), exceeding the
stretch target of -8. The core technique was adding a **T::V witness property** to
ParamBST's `type_invariant`, enabling ordering-based cross-disjointness proofs that
bridge the T::V gap.

Chap41 AVLTreeSetStEph had only 2 holes at round start (not 6), and AVLTreeSetStPer had
0 holes (not 5). Both were already reduced by R13 work. The 2 remaining StEph holes are
Vec capacity assumes (`len < usize::MAX`) that cannot be eliminated without structural
changes to AVLTreeSeqStEph's wf spec.

## Verification

- **4026 verified, 0 errors**
- **0 trigger warnings**

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 38 | BSTParaStEph.rs | 15 | 5 | -10 |
| 2 | 41 | AVLTreeSetStEph.rs | 2 | 2 | 0 |
| 3 | 41 | AVLTreeSetStPer.rs | 0 | 0 | 0 |
| **Total** | | | **17** | **7** | **-10** |

## Holes Eliminated (BSTParaStEph.rs)

| # | Category | Original Line | What Was Proved |
|---|----------|---------------|-----------------|
| 1 | join_pre | 895 | intersect found: join_m disjointness via subset |
| 2 | join_pre | 931 | intersect !found: join_pair ordering via subset |
| 3 | join_pre | 997 | difference found: join_pair ordering via subset |
| 4 | join_pre | 1033 | difference !found: join_m disjointness via subset |
| 5 | containment | 975 | intersect found: result containment via witness routing |
| 6 | containment | 1021 | intersect !found: result containment via witness routing |
| 7 | containment | 1012 | difference found lrv: !other via witness ordering |
| 8 | containment | 1017 | difference found rrv: !other via witness ordering |
| 9 | containment | 1049 | difference !found lrv: !other via witness ordering |
| 10 | containment | 1054 | difference !found rrv: !other via witness ordering |

Plus union: proved 5 of 6 conjuncts (ordering, non-containment, disjointness), leaving
only the size bound assume. Net effect: 1 hole before, 1 hole after (no count change,
but the assume is now minimal).

## Techniques

### 1. Subset Disjointness (holes 1-4)
For join preconditions: `lrv ⊂ alv`, `rrv ⊂ arv`, and `alv ⊥ arv` (from expose) implies
`lrv ⊥ rrv`. Size bounds from `vstd::set_lib::lemma_len_subset`. T-level ordering inherited
from parent set membership.

### 2. T::V Witness Property (holes 5-10)
Added to ParamBST's `#[verifier::type_invariant]`:
```rust
forall|v: <T as View>::V| self.ghost_locked_root@.contains(v)
    ==> exists|t: T| t@ == v
```
This bridges the gap between set operations (on `T::V`) and ordering quantifiers (on `T`).
At proof sites: `use_type_invariant(&tree)` then `choose|t: T| t@ == x` to get a T witness,
then instantiate ordering quantifiers for contradiction.

Key proof pattern for cross-set exclusion:
```
x ∈ alv → witness t with t@ == x → t.cmp_spec(&ak) == Less
if x ∈ brv → brv.contains(t@) → t.cmp_spec(&ak) == Greater → contradiction
```

### 3. Explicit Set Equality for !found branches
In `!found` difference branch, solver needed `assert(blv.union(brv) =~= other@)` to
connect the split postcondition (`bl@ ∪ br@ =~= other@.remove(akv)`) with the fact
that `remove` on a non-member is identity.

## Remaining Holes (BSTParaStEph.rs, 5 total)

| # | Line | Category | Hole | Fixable? |
|---|------|----------|------|----------|
| 1 | 431 | clone | expose clone assume (3 conjuncts) | No — needs Verus clone ensures |
| 2 | 515 | size_bound | insert: left + right < usize::MAX | No — usize::MAX cascade |
| 3 | 526 | size_bound | delete: left + right < usize::MAX | No — usize::MAX cascade |
| 4 | 877 | size_bound | union: luv + ruv < usize::MAX | No — usize::MAX cascade |
| 5 | 1399 | clone | external_body on Clone::clone | No — standard clone pattern |

All 5 remaining holes are structural limitations (Verus clone support, usize::MAX bounds),
not algorithmic proof gaps.

## Commit

Branch: `agent3/ready`
