<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 44: Document Index — Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap44.txt` (Data Type 44.1, Algorithms 44.2–44.3, Examples 44.1–44.2)

## Phase 1: Inventory

Source files: 2 (`DocumentIndex.rs`, `Example44_1.rs`). All plain Rust, no `verus!` blocks. The module is gated with `#[cfg(all(not(any(feature = "experiments_only", feature = "dev_only")), not(verus_keep_ghost)))]`, explicitly excluding it from Verus compilation.

| # | File | Functions | Traits | Trait Impls | Bare Impls | V! | -V! | NoSpec |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | `DocumentIndex.rs` | 15 | 1 (9 methods) | 1 (9 methods) | 1 (6 methods: QueryBuilder) | 0 | 15 | 15 |
| 2 | `Example44_1.rs` | 7 | 1 (2 methods) | 1 (2 methods) | 0 | 0 | 7 | 7 |
| | **Total** | **22** | | | | **0** | **22** | **22** |

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|---|---|
| 1 | Data Type 44.1 (Document Index) | ADT with operations: `makeIndex`, `find`, `queryAnd`, `queryOr`, `queryAndNot`, `size`, `toSeq` |
| 2 | Types | `Word = String`, `DocumentId = usize`, `DocumentSet = Set<DocumentId>`, `DocumentCollection = Seq<(String, Seq<Word>)>` |

### Algorithms

| # | Item | Description |
|---|---|---|
| 1 | Algorithm 44.2 (Make Index) | `makeIndex docs = Table.collect (tagWords docs)` where `tagWords` flattens `(docId, word)` pairs. Uses Table.collect to group by word. |
| 2 | Algorithm 44.3 (Index Functions) | `find idx w = if Table.find idx w then s else ∅`; `queryAnd = Set.intersection`; `queryOr = Set.union`; `queryAndNot = Set.difference` |

### Cost Specs (from prose)

| # | Operation | APAS Work | APAS Span |
|---|---|---|---|
| 1 | `makeIndex docs` | O(n lg n) | O(n lg n) |
| 2 | `find idx w` | O(lg n) | O(lg n) |
| 3 | `queryAnd s₁ s₂` | O(m lg(1+n/m)) | O(lg(n+m)) |
| 4 | `queryOr s₁ s₂` | O(m lg(1+n/m)) | O(lg(n+m)) |
| 5 | `queryAndNot s₁ s₂` | O(m lg(1+n/m)) | O(lg(n+m)) |
| 6 | `size s` | O(1) | O(1) |
| 7 | `toSeq s` | O(n) | O(lg n) |

### Examples

| # | Item | Description |
|---|---|---|
| 1 | Example 44.1 | Tweet collection: 3 tweets from @potus/@flotus, build index, query "go AND green", "go OR green", "go AND NOT green" |
| 2 | Example 44.2 | Staged computation: build index once, query many times |

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

#### DocumentIndex.rs

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match |
|---|---|---|---|---|---|:---:|
| 1 | `make_index` | O(n lg n) | O(n lg n) | **O(n²)** | **O(n²)** | **No** |
| 2 | `find` | O(lg n) | O(lg n) | O(lg n) | O(lg n) | Yes |
| 3 | `query_and` | O(m lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | Partial |
| 4 | `query_or` | O(m lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | Partial |
| 5 | `query_and_not` | O(m lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | Partial |
| 6 | `size` | O(1) | O(1) | O(1) | O(1) | Yes |
| 7 | `to_seq` | O(n) | O(lg n) | **O(n²)** | **O(n²)** | **No** |

**Cost match: 2/7** (find, size). Query ops are partially matching — O(m+n) merge sort vs O(m lg(1+n/m)) tree-based, but same asymptotic class when m ≈ n.

### make_index Cost Analysis

The implementation of `make_index` has two nested loops:

1. **Outer loop:** iterates over all documents and all words in each document, building `(word, docId)` pairs into a sorted `ArraySeqStPerS<Pair<Word, DocumentId>>`. Each pair is inserted via `table.insert(...)` which is O(n) for the sorted-array backing. Total: O(n²) where n = total word-document pairs.

2. **Grouping:** For each unique word, scans the entire sorted pairs array to collect document IDs. This is another O(n × unique_words) pass.

The prose Algorithm 44.2 uses `Table.collect` which groups in O(n lg n) via a tree-based table. The sorted-array implementation cannot achieve this.

### to_seq Cost Analysis

`to_seq` builds a sequence by repeated `singleton` + `append` on `ArraySeqStPerS`, leading to O(n²) total from O(n) copies per append.

### Phase 3b: Implementation Fidelity

| # | Operation | Prose | Implementation | Fidelity |
|---|---|---|---|---|
| 1 | `make_index` | `Table.collect(tagWords docs)` — groups by word | Nested loops: inserts `(word, docId)` pairs into sorted array, then groups | **Low** — O(n²) instead of O(n lg n); intermediate `table.insert` discards prior docIds for same word |
| 2 | `find` | `if Table.find idx w then s else ∅` | Binary search on sorted array, returns `Option<DocumentSet>`, maps None to empty set | High |
| 3 | `query_and` | `Set.intersection s₁ s₂` | Delegates to `AVLTreeSetStPer::intersection` | High |
| 4 | `query_or` | `Set.union s₁ s₂` | Delegates to `AVLTreeSetStPer::union` | High |
| 5 | `query_and_not` | `Set.difference s₁ s₂` | Delegates to `AVLTreeSetStPer::difference` | High |
| 6 | `size` | `\|a\|` | Returns table size | High |
| 7 | `to_seq` | Sequence of key-value pairs | Repeated singleton + append (O(n²)) | Correct but slow |
| 8 | `tokens` | Prose: `tagWords` — flatten docs to `(docId, word)` pairs | Splits string on whitespace, lowercases | High (utility) |

**Key bug in make_index:** The intermediate step inserts each `(word, docId)` pair as a separate entry. Because `table.insert` replaces existing entries for the same key (without a combine function), only the last document for each word survives in the intermediate table. The grouping loop then re-scans the original pairs to build the document set, mitigating this partially, but the algorithm is not faithful to `Table.collect`.

### Phase 3c: Spec Fidelity

No Verus specs exist. All functions have `spec_strength = none`.

## Phase 4: Parallelism Review

No Mt variant exists for Chapter 44. All code is single-threaded.

| # | Observation |
|---|---|
| 1 | No `DocumentIndexMtEph` or `DocumentIndexMtPer` variants |
| 2 | APAS defines parallel costs (span) for make_index and query operations |
| 3 | `make_index` could benefit from parallelism (embarrassingly parallel tokenization + parallel collect) |
| 4 | Query operations inherit parallelism potential from underlying set operations |

## Phase 5: Runtime Test Review

| # | Test File | Tests | Operations Covered |
|---|---|:---:|---|
| 1 | `TestDocumentIndex.rs` | ~8 | make_index, find, query_and, query_or, query_and_not, size, to_seq, tokens |
| 2 | `TestExample44_1.rs` | ~5 | Example 44.1 tweet queries, Example 44.2 staged computation, textbook value verification |
| | **Total RTT** | **~13** | |

### Test Quality

TestExample44_1.rs is notably thorough — it verifies exact textbook example values:
- Builds the tweet collection from APAS Example 44.1
- Queries "go AND green" → expects `{1}` (only tweet 1 contains both)
- Queries "go OR green" → expects `{0, 1, 2}` (all tweets contain at least one)
- Queries "go AND NOT green" → expects `{0, 2}`
- Demonstrates staged computation (Example 44.2 pattern)

### Test Gaps

| # | Gap | Severity |
|---|---|---|
| 1 | No tests for empty index edge cases | Low |
| 2 | No tests for words appearing in zero documents after index construction | Low |
| 3 | No performance/cost tests | Low |
| 4 | No tests for `create_finder` / `QueryBuilder` | Medium |

## Phase 6: PTT Review

No PTTs exist. No Verus code to test.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Prose Item | Status | Notes |
|---|---|---|---|
| 1 | Algorithm 44.2 (collect-based makeIndex) | **Wrong algorithm** | Uses nested loops instead of `Table.collect`. O(n²) instead of O(n lg n). |
| 2 | Parallel variants | **Missing** | No Mt (multi-threaded) document index |
| 3 | `tagWords` as named function | **Inlined** | Tokenization is done inline in `make_index`, not as a separate named function matching the prose |

### Code with No Prose Counterpart

| # | Item | Kind | Notes |
|---|---|---|---|
| 1 | `create_finder` | Utility | Returns a closure for repeated lookups |
| 2 | `QueryBuilder` struct | Builder pattern | Fluent API for composing queries (`and`, `or`, `and_not`) |
| 3 | `DocumentCollectionLit!` macro | Convenience | Literal syntax for document collections |
| 4 | `Example44_1` module | Example | Good — matches prose examples 44.1 and 44.2 |

## Phase 8: TOC Review

| # | File | Has TOC | Has Module Header | Copyright |
|---|---|:---:|:---:|:---:|
| 1 | `DocumentIndex.rs` | No | Yes | Yes |
| 2 | `Example44_1.rs` | No | Yes | Yes |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Debug | Display | Macro |
|---|---|:---:|:---:|:---:|:---:|:---:|
| 1 | `DocumentIndex.rs` | ❌ out (derive) | ❌ out (derive) | ❌ out | - | ✅ out |
| 2 | `Example44_1.rs` | - | - | - | - | - |

No verus! blocks exist, so the "should be inside" classification is aspirational.

## Proof Holes Summary

```
Modules:   2 clean (no holes), 0 holed
Holes Found: 0 total
Note: Module gated with #[cfg(not(verus_keep_ghost))] — explicitly excluded from Verus
```

Zero proof holes — vacuously clean since no Verus code exists.

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 22 |

All functions lack Verus specifications.

## Overall Assessment

**Chapter 44 implements the Document Index ADT (Data Type 44.1) with 7 operations plus utilities, backed by `TableStPer` and `AVLTreeSetStPer`. No Verus verification exists. The critical `make_index` algorithm is O(n²) instead of the prose-specified O(n lg n). No parallel variant exists. Test coverage is good, with exact textbook value verification.**

### Strengths

1. **Excellent test coverage for textbook examples** — TestExample44_1 verifies exact APAS Example 44.1 query results.
2. All 7 ADT operations semantically correct (find, query_and, query_or, query_and_not, size, to_seq, make_index).
3. QueryBuilder provides a nice composable query API beyond the prose.
4. `create_finder` enables the "staged computation" pattern from Example 44.2.
5. `tokens` utility correctly handles whitespace splitting and lowercasing.

### Weaknesses

1. **No Verus verification** — zero functions inside `verus!`, zero specs. Explicitly excluded via `cfg(not(verus_keep_ghost))`.
2. **`make_index` is O(n²)** instead of O(n lg n) — uses nested loops and sorted-array table.insert instead of `Table.collect`.
3. **`to_seq` is O(n²)** — repeated singleton + append on arrays.
4. **No parallel variant** — APAS defines span costs, implying parallel implementation is expected.
5. **No TOC headers** in source files.
6. **`tagWords` not a separate named function** — inlined in `make_index`, less modular than prose suggests.
7. Query operations (`query_and`, `query_or`, `query_and_not`) delegate to set operations that are O(m+n) merge-based rather than O(m lg(1+n/m)) tree-based.
