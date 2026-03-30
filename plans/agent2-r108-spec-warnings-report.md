# R108 Agent 2 Report: Fix Veracity Spec Warnings

## Result: 8 of 19 warnings fixed, 11 classified

- Verification: 5433 verified, 0 errors (unchanged)
- RTT: 3083 passed
- PTT: 157 passed

## Fixes Applied

### Chap06 — 8 wf ensures added to graph constructors (FIXED)

All 4 graph StEph files had trait constructors (`empty`, `from_sets`/`from_vertices_*`)
returning concrete types instead of `Self`. Per the spec_wf_standard, when a trait
function returns `Self`, Verus uses abstract dispatch for `g.spec_wf()` in ensures --
no cycle. But when returning a concrete type, Verus resolves through the impl, creating
a 3-node cycle (method -> impl body -> trait declaration).

**Fix:** Changed return types from `DirGraphStEph<V>` etc. to `Self` and added
`g.spec_<module>_wf()` to each constructor's ensures.

| # | Chap | File | Function | Fix |
|---|------|------|----------|-----|
| 1 | 06 | DirGraphStEph.rs | empty | `-> Self`, added wf ensures |
| 2 | 06 | DirGraphStEph.rs | from_sets | `-> Self`, added wf ensures |
| 3 | 06 | LabDirGraphStEph.rs | empty | `-> Self`, added wf ensures |
| 4 | 06 | LabDirGraphStEph.rs | from_vertices_and_labeled_arcs | `-> Self`, added wf ensures |
| 5 | 06 | LabUnDirGraphStEph.rs | empty | `-> Self`, added wf ensures |
| 6 | 06 | LabUnDirGraphStEph.rs | from_vertices_and_labeled_edges | `-> Self`, added wf ensures |
| 7 | 06 | UnDirGraphStEph.rs | empty | `-> Self`, added wf ensures |
| 8 | 06 | UnDirGraphStEph.rs | from_sets | `-> Self`, added wf ensures |

## Not Fixed — Analysis

### Chap26 — `point_distance` (veracity: no_requires candidate)

| # | Chap | File | Function | Status |
|---|------|------|----------|--------|
| 9 | 26 | ETSPMtEph.rs | point_distance | No real requires needed |

Takes `&Point, &Point` (f64 coords). All f64 ops (`f64_sub`, `f64_mul`, `f64_add`,
`f64_sqrt`) have no requires. No wf predicate on Point. Genuine `// veracity: no_requires`
candidate.

### Chap43 — `from_sorted_entries` (too invasive)

| # | Chap | File | Function | Status |
|---|------|------|----------|--------|
| 10 | 43 | OrderedTableStPer.rs | from_sorted_entries | Needs substantial proof work |

`spec_orderedtablestper_wf()` requires 10 conjuncts. The function is missing 4 requires
clauses (`obeys_feq_fulls::<K,V>()`, `spec_pair_key_determines_order::<K,V>()`,
`vstd::laws_cmp::obeys_cmp_spec::<K>()`, `view_ord_consistent::<K>()`) and needs a
loop invariant for `spec_key_unique_pairs_set`. This is a significant proof task, not
a simple ensures addition.

### Chap44 — 2 warnings (1 false positive, 1 no_requires candidate)

| # | Chap | File | Function | Status |
|---|------|------|----------|--------|
| 11 | 44 | DocumentIndex.rs | QueryBuilder::new | SMT destabilization |
| 12 | 44 | DocumentIndex.rs | tokens | No real requires needed |

**#11:** Already has `requires spec_documentindex_wf(index)` (free function form).
Veracity wants `index.spec_documentindex_wf()` (method form). The method form is in
a different trait (DocumentIndexTrait), so no cycle. But changing from free function
to method form destabilizes Chap41/AVLTreeSetStPer (7 errors at full validate, passes
in isolation). The free function and method call are semantically identical --
`spec_documentindex_wf(&self)` body is `spec_documentindex_wf(self)`. Different SMT
encoding changes Z3 resource distribution.

**#12:** Takes `&String`, returns `ArraySeqStPerS<Word>`. No wf-bearing input types.
Genuine `// veracity: no_requires` candidate.

### Chap47 — 7 warnings (all false positives)

| # | Chap | File | Function | Status |
|---|------|------|----------|--------|
| 13 | 47 | ParaHashTableStEph.rs | createTable | Already has spec_hashtable_wf |
| 14 | 47 | ParaHashTableStEph.rs | insert | Uses Self::spec_impl_wf |
| 15 | 47 | ParaHashTableStEph.rs | lookup | Uses Self::spec_impl_wf |
| 16 | 47 | ParaHashTableStEph.rs | delete | Uses Self::spec_impl_wf |
| 17 | 47 | ParaHashTableStEph.rs | metrics | Uses Self::spec_impl_wf |
| 18 | 47 | ParaHashTableStEph.rs | loadAndSize | Uses Self::spec_impl_wf |
| 19 | 47 | ParaHashTableStEph.rs | resize | Uses Self::spec_impl_wf |

**Root cause:** The trait uses `Self::spec_impl_wf(table)` — a polymorphic wf predicate
that defaults to `spec_hashtable_wf(table)` but can be overridden by flat hash table
implementations. Veracity doesn't recognize this indirection and flags the functions as
missing `spec_hashtable_wf`. Adding explicit `spec_hashtable_wf` alongside `spec_impl_wf`
cascades postcondition failures across 3+ impl files (VecChainedHashTable, LinProbFlat,
DoubleHashFlat, QuadProbFlat) because the flat impls override `spec_impl_wf` and their
proofs don't automatically decompose to prove `spec_hashtable_wf` separately.

**Recommendation:** These are veracity false positives. The `Self::spec_impl_wf` pattern
is the correct design for a polymorphic trait. Veracity should recognize
`Self::spec_impl_wf` when `spec_impl_wf` defaults to `spec_hashtable_wf`.

## Veracity Improvement Suggestions

1. **Recognize free function wf calls.** Veracity flags `spec_hashtable_wf(&table)` and
   `spec_documentindex_wf(index)` as missing, but these are semantically equivalent to
   the method form. The tool should recognize `spec_<mod>_wf(x)` as equivalent to
   `x.spec_<mod>_wf()`.

2. **Recognize `Self::spec_impl_wf` indirection.** When a trait has an `open spec fn`
   that defaults to calling the module's wf predicate, functions using `Self::spec_impl_wf`
   should not be flagged as missing wf.

3. **no_requires for pure functions.** Functions like `tokens(&String) -> ArraySeqStPerS`
   and `point_distance(&Point, &Point) -> f64` have no wf-bearing inputs. Veracity
   should not flag these unless they have non-trivial input types with known wf predicates.
