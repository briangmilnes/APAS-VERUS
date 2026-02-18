<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 44: Indexing and Searching — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory (Tool-Generated)

38 functions extracted across 2 source files.

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|------|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap44 | DocumentIndex | 15 | 15 | 0 | 2 | 0 | 17 | 0 | 0 | 17 |
| 2 | Chap44 | Example44_1 | 0 | 1 | 12 | 8 | 0 | 21 | 0 | 0 | 21 |

Key observations:
- **Zero functions inside `verus!`** — the entire chapter is unverified plain Rust.
- **Zero specs** — no `requires`/`ensures` on any function.
- **Zero proof holes** — because there is no verified code to have holes in.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Data Type 44.1 (Document Index) | ADT with types `word`, `id`, `contents`, `docs`, `index` and operations `makeIndex`, `find`, `queryAnd`, `queryOr`, `queryAndNot`, `size`, `toSeq` |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 44.2 (Make Index) | `makeIndex` using `tagWords`, `flatten`, `Table.collect`, and `Set.fromSeq` |
| 2 | Algorithm 44.3 (Index Functions) | `find = Table.find`, `queryAnd = Set.intersection`, `queryOr = Set.union`, `queryAndNot = Set.difference`, `size = Set.size`, `toSeq = Set.toSeq` |

### Cost Specs

| # | Operation | APAS Work | APAS Span |
|---|-----------|-----------|-----------|
| 1 | `makeIndex` | Θ(n log n) | Θ(log² n) |
| 2 | `find I w` | Θ(log n) | Θ(log n) |
| 3 | `queryAnd`, `queryOr`, `queryAndNot` | Θ(m log(1 + n/m)) | Θ(log n + log m) |
| 4 | `size` | (implied Θ(1)) | (implied Θ(1)) |
| 5 | `toSeq` | (not stated) | (not stated) |

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 44.1 | Tweet collection with 5 posts (jack, mary, nick, josefa, peter) |
| 2 | Example 44.2 | Staged computation: `fw = find (makeIndex T)` |
| 3 | Example 44.3 | Step-by-step walkthrough of `makeIndex` using `tagWords` and `Table.collect` |

### Theorems/Properties

None stated explicitly; the chapter is application-oriented.

### Exercises/Problems

None in this chapter.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Disagreements

| # | Function | APAS Cost | Claude-Opus-4.6 Cost | Disagreement |
|---|----------|-----------|---------------------|--------------|
| 1 | `make_index` | Work Θ(n log n), Span Θ(log² n) | Work Θ(n²), Span Θ(n²) | **Major.** The prose algorithm uses `flatten` + `Table.collect` (sort-based) + `Set.fromSeq` per entry. The implementation uses nested sequential loops: outer loop over word_entries, inner loop scanning all_pairs to find matching words. This is Θ(W × P) where W = unique words and P = total pairs, giving Θ(n²) in the worst case. No `Table.collect` is used; instead, manual grouping by linear scan. |
| 2 | `find` | Work Θ(log n), Span Θ(log n) | Work Θ(log n), Span Θ(log n) | Agrees. Delegates to `TableStPer::find`. |
| 3 | `query_and` | Work Θ(m log(1+n/m)), Span Θ(log n + log m) | Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) | **Work agrees, Span disagrees.** The set operations are sequential (StPer = single-threaded persistent). The APAS span assumes parallel set operations; this implementation has Span = Work. |
| 4 | `query_or` | Work Θ(m log(1+n/m)), Span Θ(log n + log m) | Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) | Same as query_and — sequential span. |
| 5 | `query_and_not` | Work Θ(m log(1+n/m)), Span Θ(log n + log m) | Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) | Same as query_and — sequential span. |
| 6 | `size` | Θ(1) | Θ(1) | Agrees. |
| 7 | `tokens` | (not stated, assumed Θ(m)) | Θ(m) | Agrees with implicit assumption. |

### Phase 3b: Implementation Fidelity

| # | Function | Prose Algorithm | Implementation | Fidelity |
|---|----------|----------------|----------------|----------|
| 1 | `make_index` | Uses `tagWords` to tag, `flatten` to collect, `Table.collect` to group by word, `Set.fromSeq` per entry. | Manual nested loops: first loop builds all_pairs, second inserts into word_table one-by-one (losing the collect semantics), third loop re-scans all_pairs per unique word to rebuild doc sets. | **Low.** The three-loop approach is quadratic and does not follow the prose's `collect` + `fromSeq` pattern. The `word_table.insert()` with a merge function overwrites rather than collecting, then a separate scan re-groups. This is functionally correct but algorithmically inferior. |
| 2 | `find` | `Table.find T v` | `self.word_to_docs.find(word)` with fallback to empty set. | **High.** Matches prose exactly. |
| 3 | `query_and` | `Set.intersection A B` | `docs_a.intersection(docs_b)` | **High.** Direct delegation. |
| 4 | `query_or` | `Set.union A B` | `docs_a.union(docs_b)` | **High.** Direct delegation. |
| 5 | `query_and_not` | `Set.difference A B` | `docs_a.difference(docs_b)` | **High.** Direct delegation. |
| 6 | `size` | `Set.size A` | `docs.size()` | **High.** Direct delegation. |
| 7 | `to_seq` | `Set.toSeq A` | Iterates AVL tree sequence and builds ArraySeqStPerS one element at a time. | **Medium.** Functionally correct but uses O(n²) append pattern instead of a bulk conversion. |

### Phase 3c: Spec Fidelity

No specs exist. All 38 functions have `spec_strength = none`. There are no `requires`/`ensures` on any function. The chapter is entirely unverified.

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 44.** The chapter implements only `StPer` (single-threaded persistent) variants. The APAS prose describes parallel span bounds for the set operations and `makeIndex`, but no parallel implementation exists.

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | `make_index` | Θ(log² n) | Θ(n²) | No | Sequential nested loops |
| 2 | `find` | Θ(log n) | Θ(log n) | No | Sequential but matches APAS Work |
| 3 | `query_and` | Θ(log n + log m) | Θ(m log(1+n/m)) | No | Sequential AVL set intersection |
| 4 | `query_or` | Θ(log n + log m) | Θ(m log(1+n/m)) | No | Sequential AVL set union |
| 5 | `query_and_not` | Θ(log n + log m) | Θ(m log(1+n/m)) | No | Sequential AVL set difference |

## Phase 5: Runtime Test Review

### Phase 5a: Test Coverage

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | DocumentIndex.rs | `tests/Chap44/TestDocumentIndex.rs` | Present |
| 2 | Example44_1.rs | `tests/Chap44/TestExample44_1.rs` | Present |

**TestDocumentIndex.rs** (15 tests):
- `test_documentcollectionlit_macro_functionality` — macro edge cases
- `test_empty_index` — empty collection
- `test_single_document_index` — singleton
- `test_make_index_basic` — basic index construction
- `test_find_operation` — multi-word find with expected results
- `test_query_and` — intersection
- `test_query_or` — union
- `test_query_and_not` — difference
- `test_size_operation` — size queries
- `test_to_seq_operation` — sequence conversion
- `test_tokens_function` — tokenization
- `test_tokens_edge_cases` — empty, punctuation, single word, multiple spaces
- `test_complex_query_combinations` — compound queries
- `test_case_insensitive_tokenization` — case handling
- `test_duplicate_words_in_document` — deduplication
- `test_query_builder` — QueryBuilder trait
- `test_empty_query_results` — operations on empty sets
- `test_large_document_collection` — 100-document scalability
- `test_create_finder` — staged computation pattern
- `test_algorithmic_costs_verification` — timing sanity check

**TestExample44_1.rs** (16 tests):
- `test_create_tweet_collection` — collection construction
- `test_create_tweet_index` — index construction
- `test_tweet_query_examples` — all 4 basic search queries
- `test_complex_query_fun_and_food_or_chess` — textbook complex query
- `test_count_fun_but_not_chess` — textbook count query
- `test_search_food_or_fun` — OR combination
- `test_search_party_and_food` — AND combination
- `test_get_all_words` — word enumeration
- `test_get_word_count` — word count
- `test_query_builder_example` — builder pattern
- `test_verify_textbook_examples` — all textbook examples
- `test_performance_comparison_demo` — comparison demo
- `test_tokenization_demo` — tokenization demo
- `test_index_statistics` — statistics
- `test_staged_computation_pattern` — Example 44.2 pattern
- `test_case_insensitive_search` — case insensitive
- `test_empty_search_results` — empty results
- `test_single_word_documents` — edge case
- `test_query_combinations` — compound queries
- `test_algorithmic_complexity_verification` — timing

### Phase 5b: Test Quality

Test coverage is **excellent**. All core ADT operations (`make_index`, `find`, `query_and`, `query_or`, `query_and_not`, `size`, `to_seq`) are tested with:
- Happy path cases
- Edge cases (empty inputs, singletons, non-existent words)
- The exact textbook examples (Example 44.1, 44.2)
- Compound queries matching textbook descriptions
- Case insensitivity
- Duplicate word handling
- Large collection scalability (100 documents)
- Query builder pattern

### Phase 5c: Missing Tests

No significant gaps. The `create_finder` and `QueryBuilder` helpers are covered. The `tokens` function has dedicated edge case tests.

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** The chapter contains no `verus!` blocks, no iterators with ghost state, and no verified loops. All code is plain Rust outside verification scope.

### Phase 6a: Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | DocumentIndex.rs | `tests/Chap44/TestDocumentIndex.rs` | N/A | RTT present, PTT not applicable |
| 2 | Example44_1.rs | `tests/Chap44/TestExample44_1.rs` | N/A | RTT present, PTT not applicable |

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | `tagWords` helper | Partially implemented | The concept exists in `make_index` but not as a named function. The inner loop in `make_index` performs the tagging inline rather than via a separate `tagWords` function. |
| 2 | `Table.collect` | Not used | The prose algorithm relies on `Table.collect` to group word-document pairs by word. The implementation uses manual scanning instead. |

### Code with No Prose Counterpart

| # | Function | Notes |
|---|----------|-------|
| 1 | `empty()` | Constructor — Verus-specific scaffolding |
| 2 | `get_all_words()` | Utility — no prose equivalent |
| 3 | `word_count()` | Utility — no prose equivalent |
| 4 | `create_finder()` | Implements Example 44.2's `fw` pattern as a helper |
| 5 | `QueryBuilder` + trait | Query composition helper — no prose equivalent |
| 6 | `DocumentCollectionLit!` macro | Convenience macro for test data |
| 7 | `TweetQueryExamples` struct | Example wrapper — no prose equivalent |
| 8 | `doc_set_to_sorted_vec()` | Test helper |
| 9 | `verify_textbook_examples()` | Test helper |
| 10 | `performance_comparison_demo()` | Demo helper |
| 11 | `tokenization_demo()` | Demo helper |
| 12 | `index_statistics()` | Demo helper |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present | Section Headers | Notes |
|---|------|:-----------:|:---------------:|-------|
| 1 | DocumentIndex.rs | No | No | No TOC comment block |
| 2 | Example44_1.rs | No | No | No TOC comment block |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | DocumentIndex.rs | ❌ out | ❌ out | - | - | - | ✅ out | ✅ out | ✅ out | - |
| 2 | Example44_1.rs | - | - | ❌ out | - | - | - | - | - | - |

Notes:
- `DocumentIndex` derives `Clone` and `PartialEq` outside `verus!` (line 23) — should be inside with specs if verusified.
- `TweetQueryExamples` has `Default` outside `verus!` (line 41) — acceptable since no verus block exists.
- `Debug` and `Display` impls for `DocumentIndex` are correctly outside `verus!`.
- `DocumentCollectionLit!` macro is correctly outside `verus!`.

**Action items:** If the chapter is verusified in future, `Clone` and `PartialEq` must move inside `verus!` with specs.

## Proof Holes Summary

```
✓ DocumentIndex.rs
✓ Example44_1.rs

Modules:  2 clean, 0 holed
Holes Found: 0 total
```

**Zero proof holes** — because the chapter contains no verified code whatsoever.

## Spec Strength Summary

| Classification | Count |
|---------------|-------|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 38 |

All 38 functions have no Verus specifications. The entire chapter is plain Rust outside `verus!` blocks.

## Overall Assessment

Chapter 44 is a **functionally complete but unverified** implementation of the Document Index ADT from the APAS textbook.

**Strengths:**
1. All 7 ADT operations from Data Type 44.1 are implemented.
2. All textbook examples (44.1, 44.2, 44.3) are exercised in tests.
3. Excellent runtime test coverage (31 tests across 2 test files) with edge cases, scalability, and textbook example verification.
4. The `find`, `query_and`, `query_or`, `query_and_not`, and `size` functions directly delegate to the underlying `TableStPer` and `AVLTreeSetStPer` operations, matching the prose faithfully.

**Weaknesses:**
1. **`make_index` has quadratic cost** instead of the prose's Θ(n log n). The implementation uses nested sequential loops to group words instead of `Table.collect`.
2. **No Verus verification at all** — zero functions inside `verus!`, zero specs, zero proof functions.
3. **No parallelism** — all operations are sequential. The APAS span bounds assume parallel set operations; this implementation has Span = Work for set operations.
4. **No TOC headers** in either source file.
5. **`to_seq` uses quadratic append pattern** — builds the sequence one element at a time via repeated singleton + append.
6. Module is gated with `#[cfg(all(not(any(feature = "experiments_only", feature = "dev_only")), not(verus_keep_ghost)))]` — intentionally excluded from Verus verification.

**Priority recommendations:**
1. Fix `make_index` to use `Table.collect` (or equivalent) for Θ(n log n) work.
2. If verusification is planned, move the module inside `verus!` and add specs.
3. Add TOC headers per the standard.
