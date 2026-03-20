# Agent 1 — Round 50 Report

## Verification Summary

| Metric | Count |
|--------|-------|
| Verified | 4445 |
| Errors | 0 |
| RTT | 2613 passed, 0 failed |
| PTT | 147 passed, 0 failed |

## Primary Target: Chap43 Holes (5 before, 5 after)

All 5 holes analyzed in depth. Each is structural — blocked by Verus limitations or missing
cross-chapter proof infrastructure, not by missing local proof work.

| # | Chap | File | Line | Hole Type | Blocker |
|---|------|------|------|-----------|---------|
| 1 | 43 | AugOrderedTableMtEph.rs | 672 | external_body | Needs closure-clone bridge + wf propagation through ParaPair |
| 2 | 43 | AugOrderedTableStPer.rs | 117 | assume (closure) | Verus cannot prove clone preserves closure requires (confirmed by experiments/clone_fn.rs) |
| 3 | 43 | OrderedSetStEph.rs | 1134 | assume (algorithmic) | Filter cardinality proof needs sortedness in wf; AVL wf does not include spec_elements_sorted |
| 4 | 43 | OrderedSetStPer.rs | 1031 | assume (algorithmic) | Same as #3 (standalone files, same pattern) |
| 5 | 43 | OrderedTableMtPer.rs | 321 | assume (algorithmic) | RwLock ghost boundary loses capacity bound |

### Detailed Blocker Analysis

**Holes #3 and #4 (select filter assumes):** The select() function needs to prove that the
number of elements <= the i-th element equals i. This is a rank-cardinality fact that
follows from sortedness of the AVL-backed sequence. However,
`spec_avltreesetsteph_wf()` does not include `spec_elements_sorted()`. Adding sortedness
to wf would require changes in Chap41 AVLTreeSetStEph/StPer and all callers that establish
wf. A multi-chapter infrastructure project.

**Hole #2 (closure clone):** `lemma_reducer_clone_total` assumes that cloning a closure
preserves its requires. The experiment file `src/experiments/clone_fn.rs` confirms this
cannot be proved in Verus — the Clone trait has no spec for closures.

**Hole #1 (reduce_range_parallel):** Removing external_body requires: (1) wf propagation
through get_key_range (done this round — see below), (2) closure-clone bridge for reducer
(blocked by hole #2), (3) ParaPair closure verification. Cascading dependencies.

**Hole #5 (capacity bounds):** The assume in domain() exists because
OrderedSetMtEph::insert requires `old(self)@.len() + 1 < usize::MAX`. The capacity bound
is lost across the RwLock acquire/release boundary. Same class as all RWLOCK_GHOST structural
false positives.

## Code Change Made

Strengthened `get_key_range` ensures in OrderedTableMtEph.rs (Chap43) to include wf:

```rust
// Before (line 274):
ensures range@.dom().finite();

// After:
ensures range@.dom().finite(), range.spec_orderedtablemteph_wf();
```

The impl already calls `from_st()` which ensures wf. No new assumes needed. This
is a prerequisite for eventually proving `reduce_range_parallel` (hole #1) — the body
calls `select_key` on the range, which requires wf.

## Secondary Target: fn_missing_requires Warnings

| # | Chap | File | Line | Function | Finding |
|---|------|------|------|----------|---------|
| 1 | 57 | DijkstraStEphU64.rs | 104 | pq_entry_new | No real precondition — struct constructor |
| 2 | 59 | JohnsonStEphI64.rs | 73 | adjust_distance | No real precondition — i128 clamped arithmetic |
| 3 | 59 | JohnsonStEphI64.rs | 89 | reweight_edge | No real precondition — i128 clamped arithmetic |
| 4 | 43 | OrderedSetStPer.rs | 1157 | from_sorted_elements | No real precondition — calls from_vec (no requires) then from_seq |

All 4 functions genuinely have no precondition. Per CLAUDE.md rules:
- Cannot add tautological requires clauses
- Cannot add `// veracity: no_requires` annotations (only user adds these)
- Flagged for user review

## Techniques Used

- Traced closure-clone proof obligations through experiments/clone_fn.rs
- Analyzed AVL wf predicate vs sortedness predicate in Chap41
- Traced RwLock ghost boundary capacity bound loss pattern
- Strengthened ensures on get_key_range to propagate wf (structural improvement)

## Outcome vs Success Criteria

| Criterion | Target | Actual | Met? |
|-----------|--------|--------|------|
| fn_missing_requires fixed | 3 | 0 (all genuinely precondition-free) | No |
| Net hole reduction Chap43 | -2 | 0 (all structural) | No |
| Verification errors | 0 | 0 | Yes |
| RTT failures | 0 | 0 | Yes |
| PTT failures | 0 | 0 | Yes |

The 5 Chap43 holes are all blocked by cross-cutting Verus limitations (closure-clone,
RwLock ghost boundary, missing sortedness infrastructure). No local proof work can
eliminate them without first addressing those infrastructure gaps.
