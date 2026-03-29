# Agent 3 R102 — DocumentIndex: Move into verus!, add requires/ensures

## Summary

Moved `DocumentIndexTrait`, its impl, `QueryBuilderTrait`, and its impl from outside
`verus!` into `verus!`. Added requires/ensures to all functions. Added `spec_documentindex_wf`
predicate. Added `spec_index_wf` spec to `QueryBuilderTrait`.

## Changes

### Moved inside verus!

| # | Chap | Item | Status |
|---|------|------|--------|
| 1 | 44 | `DocumentIndexTrait` (8 exec fns + 1 spec fn) | Verified |
| 2 | 44 | `impl DocumentIndexTrait for DocumentIndex` | Verified |
| 3 | 44 | `QueryBuilderTrait` (6 exec fns + 1 spec fn) | Verified |
| 4 | 44 | `impl QueryBuilderTrait for QueryBuilder` | Verified |

### Stays outside verus!

| # | Chap | Item | Reason |
|---|------|------|--------|
| 1 | 44 | `tokens()` | String::to_lowercase, chars, is_alphabetic |
| 2 | 44 | `create_finder()` | Returns `impl Fn` with lifetime capture |
| 3 | 44 | `Display`, `Debug` | Standard pattern (section 14) |
| 4 | 44 | `DocumentCollectionLit!` macro | Macros always outside (section 13) |

### Functions verified (no external_body)

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 44 | DocumentIndex.rs | `find` | Delegation to `TableStPer::find` + `AVLTreeSetStPer::empty` |
| 2 | 44 | DocumentIndex.rs | `query_and` | Delegation to `AVLTreeSetStPer::intersection` |
| 3 | 44 | DocumentIndex.rs | `query_or` | Delegation to `AVLTreeSetStPer::union` |
| 4 | 44 | DocumentIndex.rs | `query_and_not` | Delegation to `AVLTreeSetStPer::difference` |
| 5 | 44 | DocumentIndex.rs | `size` | Delegation to `AVLTreeSetStPer::size` |
| 6 | 44 | DocumentIndex.rs | `empty` | Delegation to `TableStPer::empty` |
| 7 | 44 | DocumentIndex.rs | `word_count` | Delegation to `TableStPer::size` |
| 8 | 44 | DocumentIndex.rs | `QueryBuilder::new` | Trivial construction |
| 9 | 44 | DocumentIndex.rs | `QueryBuilder::find` | Delegation to `DocumentIndex::find` |
| 10 | 44 | DocumentIndex.rs | `QueryBuilder::and` | Delegation to `DocumentIndex::query_and` |
| 11 | 44 | DocumentIndex.rs | `QueryBuilder::or` | Delegation to `DocumentIndex::query_or` |
| 12 | 44 | DocumentIndex.rs | `QueryBuilder::and_not` | Delegation to `DocumentIndex::query_and_not` |

### Functions with external_body (4 holes)

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 44 | DocumentIndex.rs | `make_index` | String::to_lowercase, chars, sort_unstable_by |
| 2 | 44 | DocumentIndex.rs | `to_seq` | Loop with String clone needs feq predicates |
| 3 | 44 | DocumentIndex.rs | `get_all_words` | Loop with Pair destructuring and String clone |
| 4 | 44 | DocumentIndex.rs | `complex_query` | Chains find/and/or/and_not; wf propagation through intermediate results not provable without find ensures on result wf |

### Specs added

- `spec_documentindex_wf(di)` — free spec fn, `di.word_to_docs.spec_tablestper_wf()`
- `DocumentIndexTrait::spec_documentindex_wf(&self)` — trait spec fn
- `QueryBuilderTrait::spec_index_wf(&self)` — trait spec fn for index wf

### New imports

- `vstd::laws_eq::obeys_view_eq` (cfg-gated)
- `crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent` (cfg-gated)
- `crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_clone}` (cfg-gated)

### TOC restructured

Updated to standard bottom-up per-type ordering with proper section headers.

## Holes before/after

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 44 | DocumentIndex.rs | 0* | 4 | +4 |

*Before: 0 holes reported because all functions were outside `verus!` and invisible
to veracity. The 4 "new" holes are pre-existing unverified code now properly tracked.

## Verification counts

- Full validate: 5400 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed

## Note on fn_missing_wf_requires

Veracity flags `QueryBuilderTrait::new` for using `spec_documentindex_wf(index)` (free function)
instead of `index.spec_documentindex_wf()` (method call). Using the method-call form triggers
Z3 instability in Chap41/Chap42 dependencies (3 errors in ArraySetStEph.rs and TableStPer.rs).
The free-function form is semantically equivalent and verifies cleanly.
