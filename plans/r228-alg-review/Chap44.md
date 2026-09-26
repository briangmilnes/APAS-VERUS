# r228 Alg Analysis Review: Chap44

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap44.txt` defines the Document Index ADT (Data Type 44.1),
Algorithm 44.2 (makeIndex by `Table.collect` and `Set.fromSeq`), and
Algorithm 44.3 (index functions as table and set operations). The costs are
stated in the closing "Costs" paragraph.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 44.2 | makeIndex docs (tokens O(1) long) | n lg n | lg² n |
| 2 | Alg 44.3 | find I w (n words) | lg n | lg n |
| 3 | Alg 44.3 | queryAnd, queryOr, queryAndNot | m lg(1 + n/m) | lg n + lg m |
| 4 | Alg 44.3 | size, toSeq | via Set (CS 41.4) | not restated |

## 2. Representation and callee costs

`DocumentIndex` stores a `TableStPer<Word, AVLTreeSetStPer<DocumentId>>`.
The Chap42 review found that `TableStPer` is an unsorted array: `find_ref`
is a linear scan, and the persistent `insert` rebuilds the array by cloning
every entry. Cloning an entry deep-clones its `AVLTreeSetStPer` (O(set size)).
The Chap41 review lines in `src/Chap41/AVLTreeSetStPer.rs` give the set
costs used here: `intersection`, `union`, `difference` O(n h²) with
n = |A| + |B| and h the tree height (sequential `BSTParaStEph` with
deep-copy exposes); `to_seq` O(n h(T)); `size`, `singleton` O(1);
`clone`/`clone_wf` O(n).

Notation in the new lines: N = total tokens in the collection, D = number of
documents, w = number of distinct words (table size), d = size of one
document set, M = Σ over words of document-set sizes (M ≤ N), h = document-set
tree height.

## 3. Reviewed functions

### 3a. DocumentIndex.rs (34 lines)

T = trait site, I = impl site.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 44 | DocumentIndex.rs | make_index (T,I) | 44.2: n lg n, lg² n | T: n lg n; I: D·W·lg n | W N(N+D h²), S same | not textbook; old wrong [1] |
| 2 | 44 | DocumentIndex.rs | find (T,I) | 44.3: lg n | W lg n, S lg n | W w+d, S w+d | not textbook; old wrong [2] |
| 3 | 44 | DocumentIndex.rs | query_and (T,I) | 44.3 | T: m lg(1+n/m); I: n+m | W n h², S n h² | not textbook; old wrong [3] |
| 4 | 44 | DocumentIndex.rs | query_or (T,I) | 44.3 | T: m lg(1+n/m); I: n+m | W n h², S n h² | not textbook; old wrong [3] |
| 5 | 44 | DocumentIndex.rs | query_and_not (T,I) | 44.3 | T: m lg(1+n/m); I: n+m | W n h², S n h² | not textbook; old wrong [3] |
| 6 | 44 | DocumentIndex.rs | size (T,I) | "ref": 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 44 | DocumentIndex.rs | to_seq (T,I) | none | W n, S n | W n h, S n h | no cost; old wrong [4] |
| 8 | 44 | DocumentIndex.rs | empty (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 9 | 44 | DocumentIndex.rs | get_all_words (T,I) | none | W n, S n | W w+M, S w+M | no cost; old wrong [5] |
| 10 | 44 | DocumentIndex.rs | word_count (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 11 | 44 | DocumentIndex.rs | QB new (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 44 | DocumentIndex.rs | QB find (T,I) | T: 44.3 lg n | W lg n, S lg n | W w+d, S w+d | not textbook; old wrong [2] |
| 13 | 44 | DocumentIndex.rs | QB and, or, and_not (T,I) | none (44.3) | T: m lg(1+n/m); I: n+m | W n h², S n h² | not textbook; old wrong [3] |
| 14 | 44 | DocumentIndex.rs | QB complex_query (T) | none | no bound [6] | W w+n h², S same | no textbook cost |
| 15 | 44 | DocumentIndex.rs | QB complex_query (I) | none | W n, S n | W w+n h², S same | no cost; old wrong [6] |
| 16 | 44 | DocumentIndex.rs | tokens | none | W m, S m | W m, S m | no textbook cost |
| 17 | 44 | DocumentIndex.rs | create_finder | none | W 1, S 1 | W 1, S 1 | no textbook cost |

"none (44.3)": the QueryBuilder `and`/`or`/`and_not` have no APAS line, but
they are Algorithm 44.3's queryAnd/queryOr/queryAndNot, whose cost the prose
states; the verdict compares against the prose.

### 3b. Example44_1.rs (20 lines)

A textbook example file on a fixed five-tweet collection, so every function
is O(1) on its actual input. The new lines state the cost as a function of
the collection size, as the old lines did, because that is what the old
lines claim. None has a textbook cost.

| # | Chap | File | Function | Old review | New review | Verdict |
|---|---|---|---|---|---|---|
| 1 | 44 | Example44_1.rs | create_tweet_collection | Θ(n) | W D², S D² | old wrong [7] |
| 2 | 44 | Example44_1.rs | create_tweet_index, _finder | Θ(n²) | W D²+N(N+D h²) | old wrong [1] |
| 3 | 44 | Example44_1.rs | TweetQueryExamples new | Θ(n²) | W D²+N(N+D h²) | old wrong [1] |
| 4 | 44 | Example44_1.rs | search_fun/club/food/chess | Θ(log n) | W w+d, S w+d | old wrong [2] |
| 5 | 44 | Example44_1.rs | complex_query_fun_and_food_or_chess | Θ(m lg(1+n/m)) | W w+n h² | old wrong [3] |
| 6 | 44 | Example44_1.rs | count_fun_but_not_chess | Θ(m lg(1+n/m)) | W w+n h² | old wrong [3] |
| 7 | 44 | Example44_1.rs | search_food_or_fun, _party_and_food | Θ(m lg(1+n/m)) | W w+n h² | old wrong [3] |
| 8 | 44 | Example44_1.rs | get_all_words | Θ(n) | W w+M | old wrong [5] |
| 9 | 44 | Example44_1.rs | get_word_count | Θ(1) | W 1, S 1 | no textbook cost |
| 10 | 44 | Example44_1.rs | query_builder_example | no bound | W w+n h² | no textbook cost |
| 11 | 44 | Example44_1.rs | doc_set_to_sorted_vec | Θ(n lg n) | W n h+n lg n | old wrong [4] |
| 12 | 44 | Example44_1.rs | verify_textbook_examples | Θ(n²) | W D²+N(N+D h²) | old wrong [1] |
| 13 | 44 | Example44_1.rs | performance_comparison_demo | Θ(n²) | W D²+N(N+D h²) | old wrong [1] |
| 14 | 44 | Example44_1.rs | tokenization_demo | Θ(m) | W m, S m | no textbook cost |
| 15 | 44 | Example44_1.rs | index_statistics | Θ(n²) | W D²+N(N+D h²) | old wrong [1] |

Span equals Work in every row; nothing in the file is parallel.

### Footnotes

1. `make_index` is not Algorithm 44.2 (flatten, `Table.collect`, then
   `Set.fromSeq` per word). It loops over documents and tokens, and per token
   does a linear `TableStPer::find_ref` (O(w)), an `AVLTreeSetStPer` union of
   the existing set with a singleton (O(D h²) per Chap41), and a persistent
   `TableStPer::insert` that rebuilds the table and deep-clones every
   document set (O(w + M) ≤ O(N)). Total Work O(N·(N + D·h²)), all
   sequential. The old lines said O(n lg n) (trait) and O(D·W·lg n) (impl),
   treating the table as a BST.
2. `find` is a linear table scan plus a deep clone of the found set:
   O(w + d), not O(lg n).
3. The query functions delegate to `AVLTreeSetStPer` intersection, union, and
   difference, which the Chap41 review found to be O(n h²) sequential (deep-
   copy `expose` at every node). The old lines said O(m lg(1 + n/m)) (the
   textbook bound, with the span written equal to the work) or O(n + m).
4. `to_seq` calls `AVLTreeSetStPer::to_seq` (O(n h(T)) per Chap41), then
   copies through `AVLTreeSeqStPer::nth` (O(lg n) each). Old: O(n).
5. `get_all_words` calls `TableStPer::collect`, which clones the entry array,
   deep-cloning every document set, before extracting the keys: O(w + M).
6. The trait line for `complex_query` ("Work dominated by 4 finds + 3 set
   operations") states no bound; the Example44_1 `query_builder_example`
   line has the same text. Neither is counted as malformed (both are valid
   doc comments) or as an old mismatch.
7. `DocumentCollectionLit!` builds the collection with one
   `ArraySeqStPer::append` per document, each copying the prefix: O(D²).

## 4. Counts (per annotation site)

54 new lines were added, one per annotated site, in 2 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 2 |
| 2 | does not match textbook | 18 |
| 3 | does not match old analysis | 40 |
| 4 | no textbook cost | 34 |
| 5 | unannotated functions | 9 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 54 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 44 | DocumentIndex.rs | 34 | 2 | 18 | 23 | 14 |
| 2 | 44 | Example44_1.rs | 20 | 0 | 0 | 17 | 20 |

## 5. Unannotated functions (9)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 44 | DocumentIndex.rs | clone, eq, fmt x4 (DocumentIndex and QueryBuilder Display/Debug) |
| 2 | 44 | Example44_1.rs | default, fmt x2 |

## 6. Notable findings

1. **The document index inherits the unsorted-array table.** Because
   `TableStPer` is a linear array (Chap42 review), `find` is O(w + d) rather
   than O(lg n), and every insertion during `make_index` rebuilds the whole
   table with deep clones of every document set. `make_index` is
   Work O(N·(N + D·h²)) against the textbook's O(n lg n), a polynomial
   regression, and it does not follow Algorithm 44.2 (collect, then
   fromSeq); it inserts one token at a time.
2. **The set queries inherit the Chap41 set costs.** `query_and`,
   `query_or`, and `query_and_not` delegate to `AVLTreeSetStPer`, which the
   Chap41 review found to be O(n h²) and sequential; the old lines claimed
   the textbook's O(m lg(1 + n/m)) (trait) or O(n + m) (impl).
3. Nothing in Chap44 is parallel; the textbook's Span O(lg² n) for
   makeIndex and O(lg n + lg m) for the queries are not met anywhere.
4. The Example44_1 functions run on a constant five-document input; their
   old lines state asymptotic costs that the new lines correct in the same
   terms (17 of 20 differed).
