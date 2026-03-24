# Agent 1 — Round 68 Report

## Summary

BSTTreapStEph parametric interface hole burndown: **28 → 3 holes (−25)**.

Added `spec fn spec_parambsttreapsteph_wf` to `ParamBSTTreapStEphTrait`, propagated
`requires`/`ensures` through all 15 parametric methods, deleted 19 `assume(spec_param_wf_link(...))`
calls, and replaced 3 `assume(self@.finite())` and 3 `assume(size_link == view.len())`
with proved lemmas.

## Changes (single file: `src/Chap39/BSTTreapStEph.rs`)

1. **Added two bridging proof lemmas** (section 7):
   - `lemma_wf_implies_finite`: structural induction, wf → view.finite()
   - `lemma_wf_size_eq_view_len`: structural induction, wf → size_link == view.len()

2. **Added `spec fn spec_parambsttreapsteph_wf`** to `ParamBSTTreapStEphTrait`:
   - Abstract in trait, open in impl with body `spec_param_wf_link(&self.root)`

3. **Propagated wf requires/ensures** through all parametric methods:
   - `requires self.spec_parambsttreapsteph_wf()` on all non-constructor methods
   - `ensures ...spec_parambsttreapsteph_wf()` on all methods returning `Self` or `&mut self`
   - `expose` and `join_mid` use `spec_param_wf_link(&l.root)` directly (avoids trait cycle)
   - `param_join_pair/union/intersect/difference` also require `other.spec_parambsttreapsteph_wf()`

4. **Deleted 19 wf assumes** (the "St analog of use_type_invariant" pattern)

5. **Replaced 6 finite/size-len assumes** with lemma calls

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 39 | BSTTreapStEph.rs | 28 | 3 | −25 |

## Remaining 3 Holes (all eq/clone workaround — accepted pattern)

| # | Line | Category | Code |
|---|------|----------|------|
| 1 | 141 | eq/clone | `assume(c@ == x@)` in `clone_elem_st` |
| 2 | 151 | eq/clone | `assume(cloned@ =~= tree@ && spec_param_wf_link(&cloned.root))` in `clone_with_view` |
| 3 | 2651 | eq/clone | `assume(left_base == identity)` in `reduce_inner_st` |

## Verification

- **4438 verified, 0 errors**
- **2528 RTT passed** (0 skipped)
- **145 PTT passed** (0 skipped)

## Techniques

- **Trait wf propagation**: The standard APAS pattern — require wf at entry, ensure wf at
  exit. All internal helpers (`split_inner_st`, `join_with_priority_st`, `union_inner_st`,
  etc.) already maintained `spec_param_wf_link` through their requires/ensures chains.
  Adding it to the parametric trait made the assumes redundant.
- **Concrete vs abstract spec fns in trait signatures**: `expose` and `join_mid` reference
  children of type `BSTTreapStEph<T>` (concrete, not `Self`). Using the abstract
  `spec_parambsttreapsteph_wf()` on concrete types creates a cycle. Fix: use the free
  spec fn `spec_param_wf_link(&l.root)` directly for concrete types.
- **Structural induction lemmas**: Both bridging lemmas (`lemma_wf_implies_finite` and
  `lemma_wf_size_eq_view_len`) follow by structural induction on `Link<T>`, using the
  wf invariant's conjuncts: `lv.finite() && rv.finite()` for finite, and
  `node.size == lv.len() + rv.len() + 1` with `lv.disjoint(rv)` for size-len.
