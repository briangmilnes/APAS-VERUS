# Agent 1 — Round 187 Report

## Task

Close the 7 HashMap UnionFind gaps from R186.

## Results

| # | Metric | Before | After |
|---|--------|--------|-------|
| 1 | Verified | 724 | 724 |
| 2 | Errors | 7 | 7 |

## Changes made

1. Added explicit `parent.dom().contains(pv_v)` triggers in all proof lemma
   recursive calls (lemma_find_in_dom, lemma_find_is_root, lemma_more_fuel,
   lemma_find_after_link).
2. Added `spec_is_root` to the trait and find()'s ensures.
3. Added domain bridges (`po.dom().contains(u@)`, etc.) before every
   `lemma_find_after_link` call in all three union branches.
4. Added fuel-equivalence assertions at find() return points.

## Root cause discovered

The remaining errors are NOT domain-tracking. They are **exec-to-view bridges**.

The core issue: `*parent_val == curr` (exec PartialEq) does NOT automatically
tell Z3 that `parent_val@ == curr@` (view equality). The connection requires
`obeys_key_model::<V>()` which is in the wf, but Z3 doesn't instantiate it
for specific exec equality checks.

In the array version, `parent[i]` is `usize` and `parent[i] as int` is a
direct cast — no key model needed. In the HashMap version, `parent@[k]` is
a `V` value, and converting to `V::V` via `@` requires the `obeys_key_model`
bridge.

This affects:
- `find()`: when `*parent_val == curr`, we need `pv(parent, curr@) == curr@`
  to recognize curr as a root. The exec equality doesn't give view equality.
- `union_sets()`: after `self.parent.insert(root_v, root_u)`, the new map
  has `pv(pn, root_v@) == root_u@` which should follow from insert's ensures
  + `root_u@ == root_u.clone_view()@`. But the clone_view bridge may not
  propagate automatically.

## Fix strategy for next round

1. After `*parent_val == curr`, add: `proof { assume(parent_val@ == curr@); }`
   — NO, that violates safety rules.

2. Instead: use `self.parent.get(&curr)` which ensures `*v == self@[k@]`.
   Then `*parent_val == self.parent@[curr@]`. And `*parent_val == curr` (exec).
   So `self.parent@[curr@] == curr` (exec). Need: `(self.parent@[curr@])@ == curr@`.
   This IS the obeys_key_model bridge. Add as a separate lemma that
   derives `a@ == b@` from `a == b` using obeys_key_model.

3. Check if vstdplus has an `eq_implies_view_eq` helper. If not, write one
   as a proof fn.
