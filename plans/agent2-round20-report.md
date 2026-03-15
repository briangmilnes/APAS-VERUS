# Agent 2 — Round 20 Report

## Summary

Fixed 5 proof holes across Chap05 and Chap38. Strengthened `collect_in_order` and
`reduce_inner` ensures. All 9 BSTParaMtEph holes are thread-spawn boundaries — not fixable
without restructuring.

## Holes Before/After

| # | Chap | File | Before | After | Fixed |
|---|------|------|--------|-------|-------|
| 1 | 05 | MappingStEph.rs | 3 | 1 | 2 |
| 2 | 38 | BSTParaStEph.rs | 8 | 5 | 3 |
| 3 | 38 | BSTParaMtEph.rs | 9 | 9 | 0 |
| | | **Total** | **20** | **15** | **5** |

## Chap05 MappingStEph — 2 holes fixed

| # | Function | Was | Now | Technique |
|---|----------|-----|-----|-----------|
| 1 | from_vec | external_body | proved | seq_set lemmas bridge View ↔ Set containment |
| 2 | from_relation | external_body | proved | Clone ensures + is_functional_set uniqueness |
| 3 | size | external_body | **kept** | Needs set-bijection lemma (functional projection cardinality) |

## Chap38 BSTParaStEph — 3 holes fixed

| # | Function | Was | Now | Technique |
|---|----------|-----|-----|-----------|
| 1 | expose | assume | **kept** | Clone workaround (allowed pattern) |
| 2 | insert | assume | **kept** | Overflow bound (practical axiom) |
| 3 | delete | assume | **kept** | Overflow bound (practical axiom) |
| 4 | union | assume | **kept** | Overflow bound (practical axiom) |
| 5 | reduce | external_body | proved | Strengthened reduce_inner ensures: `len==0 ==> result@==identity@` |
| 6 | in_order | external_body | proved | Strengthened collect_in_order with containment/completeness ensures |
| 7 | filter_inner | external_body | proved | Added disjointness + ordering proofs for join_m if-branch |
| 8 | clone | external_body | **kept** | Cyclic reference: ParamBST::clone → NodeInner::clone → ParamBST::clone |

### collect_in_order strengthening

Added three new ensures to `collect_in_order`:
- Preservation: elements before `old(out)@.len()` unchanged.
- Containment: all appended elements come from `self@`.
- Completeness: all elements of `self@` appear in the appended portion.

This enabled removing external_body from `in_order`.

### filter_inner proof completion

The `if predicate(&key)` branch was missing proofs for `join_m` requires:
- Disjointness of `left_filtered@` and `right_filtered@` (subsets of disjoint sets).
- Ordering properties (inherited from parent via subset).

## Chap38 BSTParaMtEph — 0 holes fixable

All 9 holes are structural:
- View spec is intentionally opaque (no ghost field).
- All algorithmic functions delegate to inner functions **outside `verus!`** using `ParaPair!` thread spawning.
- `assume_specification` on `split_inner` bridges verified ↔ unverified code.
- These are thread-spawn boundaries; `external_body` is appropriate.

## Remaining holes by category

| Category | Count | Files |
|---|---|---|
| Clone workaround assume | 1 | BSTParaStEph |
| Overflow bound assume | 3 | BSTParaStEph (insert/delete/union) |
| Clone cyclic external_body | 1 | BSTParaStEph |
| Set-bijection external_body | 1 | MappingStEph (size) |
| Thread boundary external_body | 8 | BSTParaMtEph |
| Thread boundary assume_spec | 1 | BSTParaMtEph |

## Verification

- 3945 verified, 0 errors
- 2600 RTT passed
- 147 PTT passed

## Commit

TBD
