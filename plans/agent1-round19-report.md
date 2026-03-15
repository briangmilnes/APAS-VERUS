# Agent 1 Round 19 Report — Strengthen Chap43 Table Operation Specs

## Summary

Strengthened ensures clauses for table operations across 4 Chap43 files, replacing
weak `ensures self@.dom().finite()` with full domain-level and value-level specs
matching the Chap42 R17 patterns.

## Files Modified

| # | Chap | File | Functions Strengthened |
|---|------|------|----------------------|
| 1 | 43 | OrderedTableStEph.rs | 11 (insert, domain, tabulate, map, filter, intersection, union, difference, restrict, subtract, join_key) |
| 2 | 43 | OrderedTableStPer.rs | 11 (insert, domain, tabulate, map, filter, intersection, union, difference, restrict, subtract, join_key) |
| 3 | 43 | AugOrderedTableStEph.rs | 11 (insert, domain, tabulate, map, filter, intersection, union, difference, restrict, subtract, join_key) |
| 4 | 43 | AugOrderedTableStPer.rs | 11 (insert, domain, tabulate, map, filter, intersection, union, difference, restrict, subtract, join_key) |

**Total: 44 functions strengthened** across 4 files (both trait and impl for each).
Reduce (2 files) unchanged — generic return type R prevents meaningful spec.

## Spec Patterns Applied

| # | Operation | Ephemeral Ensures | Persistent Ensures |
|---|-----------|-------------------|-------------------|
| 1 | insert | dom =~= old.dom.insert(k@), contains_key, value specs | dom =~= self.dom.insert(k@) |
| 2 | domain | domain@ =~= self@.dom() | keys@ =~= self@.dom() |
| 3 | tabulate | dom =~= keys@, wf, closure ensures linkage | same |
| 4 | map | dom =~= self@.dom() | dom == self@.dom(), closure value specs |
| 5 | filter | subset_of, value preservation, backward completeness | same |
| 6 | intersection | dom =~= intersect, combine ensures | same |
| 7 | union | dom =~= union, 3-case value preservation | same |
| 8 | difference | dom =~= difference, value preservation | same |
| 9 | restrict | dom =~= intersect(keys@), value preservation | same |
| 10 | subtract | dom =~= difference(keys@), value preservation | same |
| 11 | join_key | dom =~= union | same |

## Hole Impact

| File | Before | After | Delta | Notes |
|------|--------|-------|-------|-------|
| OrderedTableStEph.rs | 5 | 6 | +1 | New external_body on map (strong spec) |
| OrderedTableStPer.rs | 4 | 4 | 0 | No change |
| AugOrderedTableStEph.rs | 3 | 4 | +1 | New external_body on map (strong spec) |
| AugOrderedTableStPer.rs | 2 | 2 | 0 | No change |
| **Chap43 Total** | **40** | **42** | **+2** | |

The +2 holes are from adding `external_body` to `map` in OrderedTableStEph and
AugOrderedTableStEph. These functions have custom loop implementations that cannot
automatically prove the stronger domain-preservation spec. The external_body now
carries `mapped@.dom() =~= self@.dom()` instead of just `finite()`.

## Impl Strategy

- **Delegating impls** (insert, domain, tabulate, filter, intersection, union,
  difference, restrict, subtract, join_key): Removed explicit ensures; inherited
  from trait. Proofs flow through base table delegation since views are `open spec fn`.
- **Custom loop impls** (map in OrdTableStEph, AugOrdTableStEph): Added
  `#[verifier::external_body]` with strong spec.
- **Already external_body** (filter in OrdTableStEph, join_key in AugOrdTableStEph):
  Inherited stronger trait ensures automatically.

## Verification

- **Verus**: 4036 verified, 0 errors
- **RTT**: 2600 tests passed
- **PTT**: 147 tests passed
- **Total holes**: 169 (project-wide)

## Commit

Commit hash: (pending)
