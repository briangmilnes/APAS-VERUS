# Agent 1 — Round 42 Report

## Summary

Proved 7 external_body methods in Chap43 OrderedTableStEph across two commits,
reducing holes from 12 to 5. 4334 verified, 0 errors. 2613 RTT pass.

## Commits

| # | Commit | Description |
|---|--------|-------------|
| 1 | 7468ab4e | Prove filter, intersection, split_key, get_key_range (12→8) |
| 2 | 69e0cb9f | Prove domain, rank_key, select_key (8→5) |

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 12 | 5 | -7 |

## Methods Proved

| # | Chap | Method | Technique |
|---|------|--------|-----------|
| 1 | 43 | filter | Ghost kept-index seq, positional mapping |
| 2 | 43 | intersection | Same pattern as filter |
| 3 | 43 | split_key | Two-pass collect with clone_plus |
| 4 | 43 | get_key_range | cmp-based filtering with clone_plus |
| 5 | 43 | domain | Bidirectional ghost set tracking, ArraySetStEph::insert |
| 6 | 43 | rank_key | Ghost Set + obeys_feq_view_injective contrapositive |
| 7 | 43 | select_key | Delegates to rank_key, view_injective for structural eq |

## Key Techniques

**obeys_feq_view_injective for structural↔view bridging**: The rank_key proof
discovered that `obeys_feq_full::<K>()` (triggered via broadcast axiom) provides
`obeys_feq_view_injective::<K>()`: `forall|x, y| x@ == y@ ==> x == y`. The
contrapositive `x != y ==> x@ != y@` bridges TotalOrder::cmp's structural
inequality ensures to the view-level inequality needed by the filter predicate.
This is a reusable pattern for any proof needing to connect spec-level structural
equality to view-level equality for generic types.

**Previous failed approach**: `reveal(obeys_view_eq)` alone is insufficient because
`obeys_eq_spec()` is abstract on the PartialEq trait — the solver can't see its body
to connect structural `==` to `eq_spec`. The feq view_injective bypass avoids this
entirely.

## Remaining Holes (5)

| # | Chap | Method | Blocker |
|---|------|--------|---------|
| 1 | 43 | avl_seq_length | Intentional wf bypass wrapper |
| 2 | 43 | avl_seq_nth | Intentional wf bypass wrapper |
| 3 | 43 | insert | from_vec requires values@.len() < usize::MAX; insert can grow by 1 |
| 4 | 43 | tabulate | from_vec requires values@.len() < usize::MAX; key set size unbounded |
| 5 | 43 | union | from_vec requires values@.len() < usize::MAX; combined size unbounded |

**avl_seq_length/nth** (holes 1-2): These are intentional external_body wrappers that
call AVLTreeSeqStEphS::length()/nth() without the spec_avltreeseqsteph_wf() precondition.
Removing them requires propagating wf to key_in_other and difference's trait signature.

**insert/tabulate/union** (holes 3-5): All blocked by the same issue:
AVLTreeSeqStEphS::from_vec requires `values@.len() < usize::MAX`, but the wf predicate
only guarantees `len < usize::MAX`. When insert adds a new key, length becomes `len + 1`
which could equal `usize::MAX`, violating from_vec's strict inequality. Fix options:
(a) tighten wf to `len + 1 < usize::MAX`, (b) add size bound to insert's requires,
(c) relax from_vec to `<=`. All require coordinated changes.

## Verification

- 4334 verified, 0 errors
- 2613 runtime tests pass
- No new warnings introduced in OrderedTableStEph
