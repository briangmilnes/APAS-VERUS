# Chapter 44: Document Indexing and Searching -- Review Against Prose

Reviewer: Claude-Opus-4.6 (Agent 3)
Date: 2026-03-15

## Phase 1: Inventory

| # | Chap | File | Type | Lines | Holes | Status |
|---|------|------|------|-------|-------|--------|
| 1 | 44 | DocumentIndex.rs | StPer | 341 | 0 | Clean |

Total: 1 file, 341 lines, 0 holes.
Skipped: Example44_1.rs (per project rules).

## Phase 2: Prose Inventory

### APAS Textbook Coverage (Chapter 44)

| # | Prose Item | Type | Files Implementing |
|---|-----------|------|--------------------|
| 1 | ADT 44.1 (Document Index) | Interface | DocumentIndex.rs |
| 2 | Algorithm 44.2 (makeIndex) | Algorithm | DocumentIndex.rs (make_index) |
| 3 | Algorithm 44.3 (find) | Algorithm | DocumentIndex.rs (find) |
| 4 | Algorithm 44.3 (queryAnd) | Algorithm | DocumentIndex.rs (query_and) |
| 5 | Algorithm 44.3 (queryOr) | Algorithm | DocumentIndex.rs (query_or) |
| 6 | Algorithm 44.3 (queryAndNot) | Algorithm | DocumentIndex.rs (query_and_not) |
| 7 | Algorithm 44.3 (size) | Algorithm | DocumentIndex.rs (size) |
| 8 | Algorithm 44.3 (toSeq) | Algorithm | DocumentIndex.rs (to_seq) |
| 9 | Example 44.1 (tweets) | Example | Example44_1.rs (skipped) |
| 10 | Example 44.2 (staged computation) | Example | DocumentIndex.rs (create_finder) |
| 11 | Costs (makeIndex, find, queries) | Cost | Annotated in DocumentIndex.rs |

All prose items from Chapter 44 are implemented. The chapter is small and focused: one ADT, two algorithms, and cost statements. Coverage is complete.

### Operations Not in APAS

| # | Operation | Files | Notes |
|---|-----------|-------|-------|
| 1 | empty() | DocumentIndex.rs | Scaffolding for Verus verification |
| 2 | get_all_words() | DocumentIndex.rs | Scaffolding; collects table keys |
| 3 | word_count() | DocumentIndex.rs | Scaffolding; delegates to Table.size |
| 4 | tokens() | DocumentIndex.rs | Helper; APAS references it but does not define it |
| 5 | create_finder() | DocumentIndex.rs | Staged computation from Example 44.2 |
| 6 | QueryBuilder | DocumentIndex.rs | Builder pattern for chained queries |
| 7 | DocumentCollectionLit! | DocumentIndex.rs | Macro for test construction |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations are present in DocumentIndex.rs in the required dual format:
```
/// - APAS: Work ..., Span ...
/// - Claude-Opus-4.6: Work ..., Span ... -- explanation
```

All exec functions in both DocumentIndexTrait and QueryBuilderTrait have annotations.

| # | Chap | Function | APAS Cost | Actual Cost | Deviation |
|---|------|----------|-----------|-------------|-----------|
| 1 | 44 | make_index | W O(n log n), S O(log^2 n) | W O(n^2), S O(n^2) | Sequential nested loops; no parallel flatten/collect |
| 2 | 44 | find | W O(log n), S O(log n) | W O(log n), S O(log n) | Agrees; delegates to Table.find |
| 3 | 44 | query_and | W O(m log(1+n/m)), S O(log n + log m) | W O(m log(1+n/m)), S O(m log(1+n/m)) | Work agrees; span sequential (not parallel) |
| 4 | 44 | query_or | W O(m log(1+n/m)), S O(log n + log m) | W O(m log(1+n/m)), S O(m log(1+n/m)) | Work agrees; span sequential |
| 5 | 44 | query_and_not | W O(m log(1+n/m)), S O(log n + log m) | W O(m log(1+n/m)), S O(m log(1+n/m)) | Work agrees; span sequential |
| 6 | 44 | size | W O(1), S O(1) | W O(1), S O(1) | Agrees |
| 7 | 44 | to_seq | (not stated) | W O(n), S O(n) | N/A |

Key cost observation: The make_index implementation is the primary deviation. APAS specifies O(n log n) work and O(log^2 n) span using parallel flatten + Table.collect + Set.fromSeq. The implementation uses sequential nested loops over documents/words (quadratic worst case on documents with many shared words), then sort_unstable_by (O(n log n)), then a sequential grouping pass. The overall work is O(n log n) in the number of word-document pairs (dominated by the sort), but the implementation uses sequential iteration rather than parallel flatten, so span is O(n) rather than O(log^2 n). The cost annotation in the source conservatively marks it O(n^2) because the outer loop over `all_pairs` in older versions was quadratic; the current sort-based version is O(n log n) in practice.

The query operations (query_and, query_or, query_and_not) delegate to AVLTreeSetStPer, which uses balanced BSTs. Work matches APAS. Span deviates because AVLTreeSetStPer's intersection/union/difference are sequential (single-threaded), giving O(m log(1+n/m)) span instead of the APAS O(log n + log m).

### 3b. Implementation Fidelity

| # | Chap | File | Fidelity | Notes |
|---|------|------|----------|-------|
| 1 | 44 | DocumentIndex.rs | Good | All ADT 44.1 operations present; algorithms follow APAS structure |

Algorithm 44.2 (makeIndex) implementation:
- Step 1 (tagWords + flatten): Implemented as nested loops over documents, extracting words and creating (word, doc_id) pairs. Matches APAS semantics but uses sequential iteration rather than parallel flatten.
- Step 2 (Table.collect): Implemented as sort_unstable_by + sequential grouping pass. The sort provides the O(n log n) cost of collect. This is a valid implementation strategy.
- Step 3 (Set.fromSeq per entry): Each group of document IDs is converted to an AVLTreeSetStPer via AVLTreeSeqStPerS::from_vec + AVLTreeSetStPer::from_seq. Matches APAS.

Algorithm 44.3 (Index Functions): All six operations directly delegate to the underlying data structures (Table.find, Set.intersection, Set.union, Set.difference, Set.size, Set.toSeq). This matches APAS exactly.

The tokens() helper splits on non-alphabetic characters and lowercases all tokens. APAS does not define tokens() precisely, so any reasonable tokenizer is acceptable.

### 3c. Spec Fidelity

| # | Chap | File | Spec Strength | Notes |
|---|------|------|---------------|-------|
| 1 | 44 | DocumentIndex.rs | Weak/NoSpec | Most functions are outside verus! with no requires/ensures |

The DocumentIndex module is unusual in the APAS-VERUS codebase: nearly all exec functions are outside `verus!`. The module has:
- 2 exec fns with complete spec (50% of verus! fns)
- 0 exec fns missing spec
- 2 proof/spec fns clean
- 0 proof holes

The style review flags that `impl DocumentIndexTrait for DocumentIndex` and `impl QueryBuilderTrait for QueryBuilder` are outside verus!. This means the implementations have no verification -- they compile and run but Verus does not check them. The `to_seq` function in the trait has an `ensures` clause (docs@.finite(), set equality, containment), which is the strongest spec in the file.

The module achieves 0 proof holes by having almost no code inside verus! -- the verification placeholder `_document_index_verified()` is a no-op proof function. This means the "clean" status is vacuous: the code works (RTTs pass) but is not formally verified.

## Phase 4: Parallelism Review

| # | Chap | File | Parallel? | Notes |
|---|------|------|-----------|-------|
| 1 | 44 | DocumentIndex.rs | No | Single-threaded; sequential implementation |

APAS specifies O(log^2 n) span for makeIndex, implying parallel flatten and parallel collect. The implementation is entirely sequential. No Mt variant exists. This is an expected gap for an application-layer module that builds on the underlying ADTs rather than being an ADT itself.

## Phase 5: Runtime Test Review

| # | Chap | File | RTT File | Coverage |
|---|------|------|----------|----------|
| 1 | 44 | DocumentIndex.rs | TestDocumentIndex.rs | Comprehensive (16 tests) |
| 2 | 44 | Example44_1.rs | TestExample44_1.rs | Comprehensive (18 tests) |

RTT coverage is excellent. TestDocumentIndex.rs (451 lines) covers:
- DocumentCollectionLit! macro (empty, with data)
- make_index (empty, single document, multi-document)
- find (existing words, non-existent words)
- query_and, query_or, query_and_not (basic + complex combinations)
- size, to_seq
- tokens (basic, edge cases: empty, punctuation only, single word, multiple spaces)
- create_finder (staged computation pattern)
- QueryBuilder (find, and, complex_query)
- Large document collection (100 docs)
- Case-insensitive tokenization
- Duplicate words within a document
- Algorithmic complexity timing checks

TestExample44_1.rs (307 lines) covers:
- Tweet collection creation
- Index creation
- All textbook example queries (fun, club, food, chess)
- Complex queries from APAS (queryAnd + queryOr, size + queryAndNot)
- Staged computation pattern (create_tweet_finder)
- QueryBuilder pattern
- verify_textbook_examples() comprehensive check
- Performance comparison demo
- Tokenization demo
- Index statistics

All APAS textbook examples from Example 44.1, 44.2, and 44.3 are exercised in RTTs.

## Phase 6: PTT Review

| # | Chap | File | PTT File | Patterns |
|---|------|------|----------|----------|
| - | - | - | (none) | - |

No PTT files exist for Chapter 44. This is acceptable because:
- The module has no iterators (style review confirms missing Iterator/IntoIterator impls, which would be the primary PTT use case).
- The module has no complex requires clauses that would benefit from callability testing.
- The code is largely outside verus!, so there are no proof obligations to exercise.

## Phase 7: Gap Analysis

### Proof Gaps

| # | Chap | File | Gap Type | Description |
|---|------|------|----------|-------------|
| 1 | 44 | DocumentIndex.rs | outside_verus | impl DocumentIndexTrait for DocumentIndex is outside verus! |
| 2 | 44 | DocumentIndex.rs | outside_verus | impl QueryBuilderTrait for QueryBuilder is outside verus! |
| 3 | 44 | DocumentIndex.rs | no_iterator | No Iterator or IntoIterator impl |
| 4 | 44 | DocumentIndex.rs | no_mt_variant | No multi-threaded variant exists |

### Structural Gaps

The module is structurally complete but largely unverified:

1. **No formal verification of make_index**: The sort-based grouping algorithm is correct (RTTs confirm) but unproven. Moving the impl inside verus! would require proving the sort + grouping loop produces a valid table mapping.

2. **No formal verification of query operations**: query_and/or/and_not delegate to AVLTreeSetStPer operations. Moving these inside verus! would be straightforward since the delegates already have verified specs.

3. **Uses Box<dyn Fn> in Example44_1**: The TweetQueryExamples struct uses `Box<dyn Fn(&Word) -> DocumentSet>`, which is not verifiable in Verus. This is acceptable for an example file.

4. **tokens() is unverifiable**: Character-level iteration (`chars()`, `is_alphabetic()`, `push()`) uses Rust standard library functions that Verus does not model. This function would need to remain external_body if moved inside verus!.

### Style Warnings (from veracity-review-verus-style)

| # | Chap | File | Warning | Description |
|---|------|------|---------|-------------|
| 1 | 44 | DocumentIndex.rs | [13] | impl DocumentIndexTrait should be inside verus! |
| 2 | 44 | DocumentIndex.rs | [13] | impl QueryBuilderTrait should be inside verus! |
| 3 | 44 | DocumentIndex.rs | [17] | Missing Iterator impl |
| 4 | 44 | DocumentIndex.rs | [17] | Missing IntoIterator impl |
| 5 | 44 | DocumentIndex.rs | [17] | Missing PTT file |
| 6 | 44 | DocumentIndex.rs | [18] | struct DocumentIndex should come before proof fns |
| 7 | 44 | DocumentIndex.rs | [19] | clone return name 'result' could be more descriptive |

## Phase 8: TOC Review

| # | Chap | File | TOC Present? | Sections Correct? |
|---|------|------|-------------|-------------------|
| 1 | 44 | DocumentIndex.rs | No | No explicit TOC header |

The file does not have an explicit Table of Contents comment block. The structure is:
1. Module declaration + imports (lines 1-14)
2. verus! block with struct, Clone, PartialEq, Eq (lines 23-49)
3. Trait declaration outside verus! (lines 52-96)
4. Impl outside verus! (lines 98-197)
5. Free functions outside verus! (lines 199-233)
6. Display/Debug impls (lines 235-253)
7. Macro definition (lines 256-275)
8. Second verus! block with QueryBuilder struct (lines 277-282)
9. QueryBuilder trait + impl outside verus! (lines 284-339)

Style warning [18] correctly flags that the struct definition at line 28 appears after the proof fn at line 25. The proof placeholder should come after the struct definition.

## Summary

Chapter 44 implements the APAS Document Index ADT (Data Type 44.1), Algorithm 44.2 (makeIndex), and Algorithm 44.3 (index functions) in a single file (DocumentIndex.rs, 341 lines). The implementation is functionally complete and correct (all RTTs pass including exact reproduction of textbook examples).

Key findings:

1. **0 proof holes** -- the module is "clean" by the proof-holes metric, but this is vacuous because nearly all exec code is outside verus! and therefore not verified by Verus.
2. **Excellent RTT coverage**: 34 tests across DocumentIndex (16) and Example44_1 (18) cover all ADT operations, edge cases, textbook examples, and basic complexity checks.
3. **make_index cost deviation**: Implementation uses sequential sort-based grouping (O(n log n) work, O(n) span) vs. APAS parallel flatten + collect (O(n log n) work, O(log^2 n) span). Work matches; span does not due to sequential execution.
4. **Query operations work matches APAS**: query_and/or/and_not delegate to AVLTreeSetStPer balanced BST operations with O(m log(1+n/m)) work. Span is sequential rather than O(log n + log m).
5. **No Mt variant**: Chapter 44 is an application chapter, not an ADT chapter, so the absence of StEph/MtEph/MtPer variants is reasonable.
6. **Verification opportunity**: Moving the trivial delegating functions (find, query_and, query_or, query_and_not, size) inside verus! would be straightforward and would add real verification value. make_index and tokens() would require more effort due to string operations and sorting.
