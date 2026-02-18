<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 44: Indexing and Searching — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Data Type 44.1 (Document Index) | ADT with types `word = string`, `id = string`, `contents = string`, `docs`, `index` and operations `makeIndex`, `find`, `queryAnd`, `queryOr`, `queryAndNot`, `size`, `toSeq` |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 44.2 (Make Index) | `makeIndex` using inner helper `tagWords(i, d) = ⟨(w, i) : w ∈ tokens(d)⟩`, then `flatten`, then `Table.collect`, then `Set.fromSeq` per entry |
| 2 | Algorithm 44.3 (Index Functions) | `find = Table.find`, `queryAnd = Set.intersection`, `queryOr = Set.union`, `queryAndNot = Set.difference`, `size = Set.size`, `toSeq = Set.toSeq` |

### Cost Specs

| # | Operation | APAS Work | APAS Span |
|---|-----------|-----------|-----------|
| 1 | `makeIndex` | O(n log n) | O(log² n) |
| 2 | `find I w` | O(log n) | O(log n) |
| 3 | `queryAnd`, `queryOr`, `queryAndNot` | O(m log(1 + n/m)) | O(log n + log m) |
| 4 | `size` | (implied O(1)) | (implied O(1)) |
| 5 | `toSeq` | (not stated) | (not stated) |

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 44.1 | Tweet collection: 5 posts by jack, mary, nick, josefa, peter. Searching 'fun' → {jack, mary, peter}. Searching 'club' → {mary}. |
| 2 | Example 44.2 | Staged computation: `fw = find (makeIndex T)` — build index once, partially apply `find`. `toSeq(queryAnd((fw 'fun'), queryOr((fw 'food'), (fw 'chess'))))` → ⟨jack, peter⟩. `size(queryAndNot((fw 'fun'), (fw 'chess')))` → 2. |
| 3 | Example 44.3 | Step-by-step `makeIndex` walkthrough: `tagWords('jack', 'chess is fun')` → `⟨('chess','jack'), ('is','jack'), ('fun','jack')⟩`, then `Table.collect` groups by word, then `Set.fromSeq` per entry. |

### Theorems/Properties

None stated explicitly; the chapter is application-oriented.

### Exercises/Problems

None in this chapter.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Disagreements

| # | Function | APAS Cost | Reviewer Cost | Disagreement |
|---|----------|-----------|---------------|--------------|
| 1 | `make_index` | Work O(n log n), Span O(log² n) | Work O(n²), Span O(n²) | **Major.** The prose uses `flatten` + `Table.collect` (sort-based grouping) + `Set.fromSeq`. The implementation uses three sequential phases with a nested inner loop in phase 3: for each unique word in `word_entries`, it rescans the entire `all_pairs` sequence to collect matching documents. This inner loop is O(W × P) where W = unique words and P = total pairs, giving O(n²) worst case. No sort-based `Table.collect` is used for grouping. |
| 2 | `find` | Work O(log n), Span O(log n) | Work O(log n), Span O(log n) | Agrees. Delegates to `TableStPer::find`. |
| 3 | `query_and` | Work O(m log(1+n/m)), Span O(log n + log m) | Work O(m log(1+n/m)), Span O(m log(1+n/m)) | **Span disagrees.** Work agrees. The set operations are sequential (`StPer` = single-threaded persistent), so Span = Work. The APAS span assumes parallel set operations. |
| 4 | `query_or` | Work O(m log(1+n/m)), Span O(log n + log m) | Work O(m log(1+n/m)), Span O(m log(1+n/m)) | Same as `query_and` — sequential span. |
| 5 | `query_and_not` | Work O(m log(1+n/m)), Span O(log n + log m) | Work O(m log(1+n/m)), Span O(m log(1+n/m)) | Same as `query_and` — sequential span. |
| 6 | `size` | O(1) | O(1) | Agrees. |
| 7 | `tokens` | (not stated, assumed O(m)) | O(m) | Agrees with implicit assumption. |
| 8 | `to_seq` | (not stated) | O(n²) | N/A — cost not stated in prose. The implementation builds the result one element at a time via repeated `singleton` + `append`, which is O(n²) for ArraySeqStPer. |

### Phase 3b: Implementation Fidelity

| # | Function | Prose Algorithm | Implementation | Fidelity |
|---|----------|----------------|----------------|----------|
| 1 | `make_index` | `tagWords(i,d)` tags each word with doc id. `flatten` collects all pairs into one sequence. `Table.collect` groups pairs by key (word). `Set.fromSeq` converts each group's value sequence to a set. | Three sequential phases: (1) Build `all_pairs` by iterating docs and tokens. (2) Insert each pair into `word_table` via `table.insert()` with a merge function that discards the old value (loses all but one doc per word). (3) For each unique word in `word_table`, rescan `all_pairs` to collect all matching doc ids, convert to `AVLTreeSeqStPerS`, then `Set.fromSeq`. | **Low.** Phase 2 is effectively wasted since it only retains one doc per word, and phase 3 re-does the grouping via quadratic scan. The prose's `Table.collect` (which groups by key in O(n log n) via sort) is not used. The result is functionally correct but algorithmically O(n²) instead of O(n log n). |
| 2 | `find` | `Table.find T v` | `self.word_to_docs.find(word)` with fallback to empty set for `None`. | **High.** Direct match. The fallback to empty set is a reasonable Rust adaptation of the prose (which implicitly returns empty docs for missing words). |
| 3 | `query_and` | `Set.intersection A B` | `docs_a.intersection(docs_b)` | **High.** Direct delegation. |
| 4 | `query_or` | `Set.union A B` | `docs_a.union(docs_b)` | **High.** Direct delegation. |
| 5 | `query_and_not` | `Set.difference A B` | `docs_a.difference(docs_b)` | **High.** Direct delegation. |
| 6 | `size` | `Set.size A` | `docs.size()` | **High.** Direct delegation. |
| 7 | `to_seq` | `Set.toSeq A` | Calls `docs.to_seq()` to get `AVLTreeSeqStPerS`, then copies element-by-element into `ArraySeqStPerS` via repeated `singleton` + `append`. | **Medium.** Functionally correct but the element-by-element copy is O(n²). A bulk conversion (or returning the AVL sequence directly) would match the prose's implied O(n) better. |
| 8 | `tokens` | `tokens(d) : string → string sequence` — splits string into words. | Iterates characters, accumulates alphabetic runs, lowercases. Non-alphabetic characters act as delimiters. | **High.** Reasonable tokenizer. The lowercase normalization is an implementation choice not in the prose (which uses exact string matching), but is sensible for a practical search engine. |
| 9 | `tagWords` | `tagWords(i,d) = ⟨(w,i) : w ∈ tokens(d)⟩` — named helper. | Inlined in the first loop of `make_index`. Not a separate named function. | **Medium.** The logic is present but not factored out. Naming it `tag_words` would improve readability and match the prose structure. |

### Phase 3c: Spec Fidelity

No specs exist. All functions have `spec_strength = none`. There are no `requires`/`ensures` on any function. The chapter is entirely unverified Rust outside `verus!` blocks.

The module is gated with `#[cfg(all(not(any(feature = "experiments_only", feature = "dev_only")), not(verus_keep_ghost)))]`, explicitly excluding it from Verus verification. This means it compiles only under `cargo build`/`cargo test`, never under Verus.

## Phase 4: Parallelism Review

No Mt (multi-threaded) modules exist in Chapter 44. The chapter implements only StPer (single-threaded persistent) variants. The APAS prose states parallel span bounds for `makeIndex` (O(log² n)) and the set query operations (O(log n + log m)), but no parallel implementation exists.

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | `make_index` | O(log² n) | O(n²) | No | Sequential nested loops; both work and span far exceed APAS |
| 2 | `find` | O(log n) | O(log n) | No | Sequential but matches APAS work = span for this op |
| 3 | `query_and` | O(log n + log m) | O(m log(1+n/m)) | No | Sequential AVL set intersection |
| 4 | `query_or` | O(log n + log m) | O(m log(1+n/m)) | No | Sequential AVL set union |
| 5 | `query_and_not` | O(log n + log m) | O(m log(1+n/m)) | No | Sequential AVL set difference |

## Phase 5: Runtime Test Review

### Phase 5a: Test Coverage

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | `DocumentIndex.rs` | `tests/Chap44/TestDocumentIndex.rs` | Present |
| 2 | `Example44_1.rs` | `tests/Chap44/TestExample44_1.rs` | Present |

**TestDocumentIndex.rs** (20 tests):

| # | Test Name | What It Tests |
|---|-----------|---------------|
| 1 | `test_documentcollectionlit_macro_functionality` | Macro: empty + 2-document creation |
| 2 | `test_empty_index` | Empty collection: word_count=0, find returns empty |
| 3 | `test_single_document_index` | Singleton: word_count=2, find returns doc, missing word returns empty |
| 4 | `test_make_index_basic` | 5-doc collection: word_count ≥ 9, all_words ≥ 9 |
| 5 | `test_find_operation` | Multi-word find with exact result sets for "programming", "world", "peace", nonexistent |
| 6 | `test_query_and` | Intersection: "programming" AND "world" → {doc1, doc4} |
| 7 | `test_query_or` | Union: "programming" OR "peace" → {doc1..doc5} |
| 8 | `test_query_and_not` | Difference: "programming" AND NOT "world" → {doc3} |
| 9 | `test_size_operation` | Size queries: programming=3, peace=2, nonexistent=0 |
| 10 | `test_to_seq_operation` | Sequence conversion: 3 elements, sorted comparison |
| 11 | `test_tokens_function` | Tokenization: "Hello, World! This is a test." → 6 lowercase tokens |
| 12 | `test_tokens_edge_cases` | Edge cases: empty, punctuation-only, single word, multiple spaces |
| 13 | `test_complex_query_combinations` | Compound: (hello AND world) OR (programming AND peace) → {doc1, doc4} |
| 14 | `test_case_insensitive_tokenization` | Case: "Hello WORLD Programming" found via lowercase queries |
| 15 | `test_duplicate_words_in_document` | Dedup: "hello hello world world" → size 1 per word |
| 16 | `test_query_builder` | QueryBuilder: find, and, complex_query |
| 17 | `test_empty_query_results` | Empty set operations: AND, OR, AND NOT all return empty |
| 18 | `test_large_document_collection` | 100-document scalability: all docs found for common words |
| 19 | `test_create_finder` | Staged computation: `create_finder` + multiple queries + batch word search |
| 20 | `test_algorithmic_costs_verification` | Timing: find, query_and, query_or, query_and_not all < 10ms |

**TestExample44_1.rs** (19 tests):

| # | Test Name | What It Tests |
|---|-----------|---------------|
| 1 | `test_create_tweet_collection` | Collection: length=5, jack/mary content correct |
| 2 | `test_create_tweet_index` | Index: word_count > 10, find "fun" returns 3 docs |
| 3 | `test_tweet_query_examples` | All 4 basic searches: fun→{jack,mary,peter}, club→{mary}, food→{nick,peter}, chess→{jack} |
| 4 | `test_complex_query_fun_and_food_or_chess` | Textbook: `toSeq(queryAnd(fw 'fun', queryOr(fw 'food', fw 'chess')))` → {jack, peter} |
| 5 | `test_count_fun_but_not_chess` | Textbook: `size(queryAndNot(fw 'fun', fw 'chess'))` → 2 |
| 6 | `test_search_food_or_fun` | OR: food OR fun → {jack, mary, nick, peter} |
| 7 | `test_search_party_and_food` | AND: party AND food → {peter} |
| 8 | `test_get_all_words` | Enumeration: > 15 unique words |
| 9 | `test_get_word_count` | Count: 15 < word_count < 50 |
| 10 | `test_query_builder_example` | Builder: (fun AND party) OR (chess AND NOT food) → size 2 |
| 11 | `test_verify_textbook_examples` | All 4 textbook examples via `verify_textbook_examples()` |
| 12 | `test_performance_comparison_demo` | Demo: indexed_work=1 < brute_force_work=5 |
| 13 | `test_tokenization_demo` | Demo: "I had fun in dance club today!" → 7 tokens |
| 14 | `test_index_statistics` | Stats: 5 docs, >15 unique words, >20 total words |
| 15 | `test_staged_computation_pattern` | Example 44.2: `create_tweet_finder()` returns working closure |
| 16 | `test_case_insensitive_search` | Case: fun search returns 3 |
| 17 | `test_empty_search_results` | Empty: nonexistent words, AND/OR on empty sets |
| 18 | `test_single_word_documents` | Edge: fun appears in multiple docs (size > 1) |
| 19 | `test_query_combinations` | Compound: AND, OR, AND NOT with exact result checking |

### Phase 5b: Test Quality

Test coverage is **excellent**. All 7 core ADT operations from Data Type 44.1 are tested with:
- Happy path cases covering the full operation semantics
- Edge cases: empty inputs, singletons, non-existent words, duplicate words, punctuation-only content
- Exact textbook examples: Example 44.1 (tweet data), Example 44.2 (staged computation with `fw`), and the compound queries `toSeq(queryAnd(fw 'fun', queryOr(fw 'food', fw 'chess')))` → {jack, peter} and `size(queryAndNot(fw 'fun', fw 'chess'))` → 2
- Case insensitivity validation
- Large collection scalability (100 documents)
- QueryBuilder pattern coverage
- Timing sanity checks

### Phase 5c: Missing Tests

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | No test for `tokens` with Unicode/non-ASCII input | Low | The tokenizer uses `is_alphabetic()` which handles Unicode, but no test exercises this |
| 2 | No test for documents with empty content | Low | What happens when a document has `""` as content? |
| 3 | No test for very long single-word documents | Low | Edge case: document with one word repeated many times |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs exist or are needed.** The chapter contains no `verus!` blocks, no iterators with ghost state, and no verified loops. All code is plain Rust gated with `not(verus_keep_ghost)`, intentionally excluded from Verus verification.

### Phase 6a: Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | `DocumentIndex.rs` | `tests/Chap44/TestDocumentIndex.rs` (20 tests) | N/A | RTT present, PTT not applicable |
| 2 | `Example44_1.rs` | `tests/Chap44/TestExample44_1.rs` (19 tests) | N/A | RTT present, PTT not applicable |

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | `tagWords` helper | Inlined | The prose defines `tagWords(i,d) = ⟨(w,i) : w ∈ tokens(d)⟩` as a named helper. The implementation inlines this logic in the first loop of `make_index` (lines 77-90) rather than factoring it into a separate function. Functionally equivalent but less readable. |
| 2 | `Table.collect` for grouping | Not used | The prose relies on `Table.collect` to sort and group word-document pairs by key. The implementation instead does a quadratic rescan: for each unique word, it iterates all pairs to find matches (lines 111-137). This is the root cause of the O(n²) cost. |
| 3 | Parallel `makeIndex` | Not implemented | APAS span O(log² n) implies parallel `flatten` + parallel sort (`Table.collect`) + parallel `Set.fromSeq`. No Mt variant exists. |

### Code with No Prose Counterpart

| # | Function/Item | Kind | Notes |
|---|--------------|------|-------|
| 1 | `empty()` | Constructor | Verus-pattern scaffolding — creates empty index |
| 2 | `get_all_words()` | Utility | Returns all indexed words as a sequence |
| 3 | `word_count()` | Utility | Returns number of unique words in the index |
| 4 | `create_finder()` | Helper | Implements Example 44.2's `fw = find(makeIndex T)` staged pattern |
| 5 | `QueryBuilder` struct + trait | Query DSL | Fluent interface for composing queries — no prose counterpart |
| 6 | `DocumentCollectionLit!` macro | Macro | Convenience macro for constructing test document collections |
| 7 | `TweetQueryExamples` struct | Example | Wrapper encapsulating Example 44.1/44.2 state for test convenience |
| 8 | `doc_set_to_sorted_vec()` | Test helper | Converts `DocumentSet` to sorted `Vec` for assertion comparison |
| 9 | `verify_textbook_examples()` | Test helper | Runs all textbook examples and returns bool |
| 10 | `performance_comparison_demo()` | Demo | Returns symbolic cost comparison (1 vs 5) — not a real benchmark |
| 11 | `tokenization_demo()` | Demo | Demonstrates tokenization on sample input |
| 12 | `index_statistics()` | Demo | Returns (doc_count, unique_words, total_words) tuple |

### Structural Observations

1. **Type mapping is clean.** `Word = String`, `DocumentId = String`, `Contents = String`, `DocumentSet = AVLTreeSetStPer<DocumentId>`, `DocumentCollection = ArraySeqStPerS<Pair<DocumentId, Contents>>`. These correspond directly to the prose types.

2. **`DocumentIndex` wraps a `TableStPer<Word, DocumentSet>`.** This is the correct representation: a table mapping words to sets of document identifiers.

3. **The trait pattern (`DocumentIndexTrait`) is well-structured** but unconventional for Verus code. Since this chapter has only one implementation, a direct `impl DocumentIndex` would suffice.

4. **`create_finder` correctly implements Example 44.2's staged computation.** The closure `move |word| index.find(word)` captures the index and returns a finder function, matching `fw = find(makeIndex T)`.

## Phase 8: Table of Contents & Style Review

### TOC Presence

| # | File | TOC Present | Section Headers | Notes |
|---|------|:-----------:|:---------------:|-------|
| 1 | `DocumentIndex.rs` | No | No | No TOC comment block |
| 2 | `Example44_1.rs` | No | No | No TOC comment block |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | `DocumentIndex.rs` | ❌ out | ❌ out | - | - | - | ✅ out | ✅ out | ✅ out | - |
| 2 | `Example44_1.rs` | - | - | ❌ out | - | - | - | - | - | - |

Notes:
- `DocumentIndex` derives `Clone` and `PartialEq` outside `verus!` (line 23) — should be inside with specs if verusified.
- `Eq` is not derived (only `PartialEq`).
- `TweetQueryExamples` has `Default` outside `verus!` (line 46) — acceptable since no verus block exists.
- `Debug` and `Display` impls for `DocumentIndex` are correctly outside `verus!`.
- `DocumentCollectionLit!` macro is correctly outside `verus!`.
- **Action:** If the chapter is verusified in future, `Clone` and `PartialEq`/`Eq` must move inside `verus!` with specs per project standard.

### Copyright Headers

| # | File | Copyright | Module Doc (`//!`) | Status |
|---|------|:---------:|:------------------:|--------|
| 1 | `DocumentIndex.rs` | `//!` (incorrect — should be `//`) | Present | Copyright uses `//!` instead of `//` |
| 2 | `Example44_1.rs` | `//!` (incorrect — should be `//`) | Present | Copyright uses `//!` instead of `//` |

Per `module-header.mdc`, copyright should use `//` (regular comment), not `//!` (doc comment).

## Proof Holes Summary

```
✓ DocumentIndex.rs
✓ Example44_1.rs

Modules:  2 clean, 0 holed
Holes Found: 0 total
```

**Zero proof holes** — because the chapter contains no verified code. The module is gated with `not(verus_keep_ghost)`, explicitly excluding it from Verus verification.

## Review TODOs

| # | Priority | Item | Notes |
|---|----------|------|-------|
| 1 | **High** | Fix `make_index` to use `Table.collect` (or sort-based grouping) | Current O(n²) nested-scan approach does not match the prose's O(n log n) algorithm. The phase-2 `word_table.insert()` loses all but one doc per word, and phase-3 rescans all pairs per unique word. Should: (a) build pairs, (b) sort/collect by word, (c) `Set.fromSeq` per group. |
| 2 | **High** | Fix `to_seq` to avoid O(n²) append pattern | Building `ArraySeqStPerS` one element at a time via repeated `singleton` + `append` is O(n²). Use a bulk conversion or `from_vec`. |
| 3 | **Medium** | Extract `tag_words` as a named function | Matches prose structure. Currently inlined in `make_index` loop. |
| 4 | **Medium** | Add TOC headers per project standard | Both source files lack the standard section-header format. |
| 5 | **Medium** | Fix copyright headers | Both files use `//!` for copyright; should use `//` per `module-header.mdc`. |
| 6 | **Low** | Consider verusification | The entire chapter is plain Rust outside `verus!`. Moving inside would enable specs and formal verification of the Document Index ADT. |
| 7 | **Low** | Add `Eq` derivation to `DocumentIndex` | Currently only derives `PartialEq`, not `Eq`. If equality is needed, both should be present. |
| 8 | **Low** | Consider Mt variants for parallel span | APAS specifies parallel spans for `makeIndex` and query operations. No Mt modules exist. |

## Overall Assessment

Chapter 44 is a **functionally complete but algorithmically flawed and unverified** implementation of the Document Index ADT.

**Strengths:**
1. All 7 ADT operations from Data Type 44.1 are implemented and produce correct results.
2. All three textbook examples (44.1, 44.2, 44.3) are faithfully exercised in tests, including the exact compound queries and expected results.
3. Excellent runtime test coverage: 39 tests across 2 test files, covering happy paths, edge cases, scalability, and textbook examples.
4. The `find`, `query_and`, `query_or`, `query_and_not`, and `size` functions directly delegate to the underlying `TableStPer` and `AVLTreeSetStPer` operations, matching the prose faithfully.
5. The staged computation pattern from Example 44.2 (`fw = find(makeIndex T)`) is correctly implemented via `create_finder` and `create_tweet_finder`.
6. Clean type mapping from prose types to Rust types.

**Weaknesses:**
1. **`make_index` has O(n²) cost** instead of the prose's O(n log n). The implementation's phase-2 `insert` discards all but one doc per word, then phase-3 rescans all pairs per unique word. The prose's `Table.collect` (sort-based grouping) is not used.
2. **`to_seq` has O(n²) cost** from repeated singleton + append pattern.
3. **No Verus verification at all** — zero functions inside `verus!`, zero specs, zero proof functions. The module is explicitly gated out of Verus compilation.
4. **No parallelism** — all operations are sequential StPer. The APAS span bounds assume parallel operations.
5. **No TOC headers** in either source file.
6. **`tagWords` not factored out** as a named function, reducing correspondence with the prose.
