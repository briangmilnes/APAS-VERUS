# r228 Alg Analysis Review: Chap42

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap42.txt` defines the Table ADT (Data Type 42.1), Algorithm 42.3
(collect on tables, by `reduce (Table.union Sequence.append)`), and Cost
Specification 42.5. The book PDF table was checked against the prose; it has
the same rows. CS 42.5 has no row for `empty` or `tabulate`, and Algorithm
42.3 states no cost; the files' APAS lines for those three cite costs that
the book does not give (see section 5).

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | CS 42.5 | size a | 1 | 1 |
| 2 | CS 42.5 | singleton (k, v) | 1 | 1 |
| 3 | CS 42.5 | domain a | \|a\| | lg \|a\| |
| 4 | CS 42.5 | filter p a | Σ W(p(k, v)) | lg \|a\| + max S(p(k, v)) |
| 5 | CS 42.5 | map f a | Σ W(f(v)) | lg \|a\| + max S(f(v)) |
| 6 | CS 42.5 | find, delete, insert | lg \|a\| | lg \|a\| |
| 7 | CS 42.5 | intersection, difference, union | m lg(1 + n/m) | lg(n + m) |
| 8 | CS 42.5 | restrict a c, subtract a c | m lg(1 + n/m) | lg(n + m) |
| 9 | Alg 42.3 | collect a | none stated | none stated |

n = max(|a|, |b|) (or |c|), m = min. W(f) = S(f) = O(1) assumed for insert,
union, and intersection.

## 2. Representation

All three table files store an unsorted `Vec` of `Pair<K, V>` with the
invariant "no duplicate keys" (`spec_keys_no_dups`). There is no ordering and
no tree. Every point operation is a linear scan, every bulk operation is a
nested scan, and every update rebuilds the vector. Callee costs used below:
`ArraySetStEph` insert and find are O(|s|) and `to_seq` is O(|s|) (Chap41
review lines in `src/Chap41/ArraySetStEph.rs`); `ArraySeq*::from_vec` is O(1),
`subseq_copy` and `append` are sequential O(length) (Chap18 report).

## 3. Reviewed functions

T = trait declaration site, I = impl site; rows merge T and I when both
received the same line. a = self, b = other table, c = key set, s = key set
for tabulate. Σ = Σ W(f) and Σ S(f) over the entries.

### 3a. TableStEph.rs (40 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 42 | TableStEph.rs | size (T,I) | 42.5: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 42 | TableStEph.rs | empty (T,I) | "ref": 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 42 | TableStEph.rs | singleton (T,I) | 42.5: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 42 | TableStEph.rs | domain (T,I) | 42.5: a, lg a | W n, S n | W a², S a² | not textbook; old wrong [1] |
| 5 | 42 | TableStEph.rs | tabulate (T,I) | line only [5] | W s·W(f); I: n | W s+Σ, S s+Σ | not textbook: seq loop |
| 6 | 42 | TableStEph.rs | map (T,I) | 42.5 | W n·W(f); I: n | W a+Σ, S a+Σ | not textbook: seq loop |
| 7 | 42 | TableStEph.rs | filter (T,I) | 42.5 | W n+Σ; I: n | W a+Σ, S a+Σ | not textbook: seq loop |
| 8 | 42 | TableStEph.rs | intersection (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: nested scans |
| 9 | 42 | TableStEph.rs | union (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: nested scans |
| 10 | 42 | TableStEph.rs | difference (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: nested scans |
| 11 | 42 | TableStEph.rs | find (T,I) | 42.5: lg a | W n, S n | W a, S a | not textbook: linear scan |
| 12 | 42 | TableStEph.rs | find_ref (T,I) | none | W n, S n | W a, S a | no textbook cost |
| 13 | 42 | TableStEph.rs | delete (T,I) | 42.5: lg a | W n, S n | W a, S a | not textbook: rebuild |
| 14 | 42 | TableStEph.rs | insert (T,I) | 42.5: lg a | W n, S n | W a, S a | not textbook: scan+copy |
| 15 | 42 | TableStEph.rs | insert_wf (T,I) | none | W n, S n | W a, S a | no textbook cost |
| 16 | 42 | TableStEph.rs | delete_wf (T,I) | none | W n, S n | W a, S a | no textbook cost |
| 17 | 42 | TableStEph.rs | restrict (T,I) | 42.5 | W n·m, S n·m | W a·c, S a·c | not textbook: find per entry |
| 18 | 42 | TableStEph.rs | subtract (T,I) | 42.5 | W n·m, S n·m | W a·c, S a·c | not textbook: find per entry |
| 19 | 42 | TableStEph.rs | entries (T,I) | none | W n, S n | W a, S a | no textbook cost |
| 20 | 42 | TableStEph.rs | iter | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 21 | 42 | TableStEph.rs | from_sorted_entries | none | W n, S n | W 1, S 1 | no cost; old wrong [2] |

### 3b. TableStPer.rs (41 lines)

Same bodies as StEph except that updates return a new table, `union` is
built from `insert`, and the file adds `collect` and `collect_by_key`.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 42 | TableStPer.rs | size, singleton (T,I) | 42.5: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 42 | TableStPer.rs | empty (T,I) | "ref": 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 42 | TableStPer.rs | domain (T,I) | 42.5: a, lg a | W n, S n | W a², S a² | not textbook; old wrong [1] |
| 4 | 42 | TableStPer.rs | tabulate (T,I) | line only [5] | W s·W(f); I: n | W s+Σ, S s+Σ | not textbook: seq loop |
| 5 | 42 | TableStPer.rs | map, filter (T,I) | 42.5 | W n(+Σ), S same | W a+Σ, S a+Σ | not textbook: seq loop |
| 6 | 42 | TableStPer.rs | intersection (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: nested scans |
| 7 | 42 | TableStPer.rs | union (T,I) | 42.5 | W n·m, S n·m | W b(a+b), S b(a+b) | not textbook; old wrong [3] |
| 8 | 42 | TableStPer.rs | difference (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: nested scans |
| 9 | 42 | TableStPer.rs | find, delete, insert | 42.5: lg a | W n, S n | W a, S a | not textbook: linear |
| 10 | 42 | TableStPer.rs | find_ref (T,I) | none | W n, S n | W a, S a | no textbook cost |
| 11 | 42 | TableStPer.rs | insert_wf, delete_wf | none | W n, S n | W a, S a | no textbook cost |
| 12 | 42 | TableStPer.rs | restrict, subtract | 42.5 | W n·m, S n·m | W a·c, S a·c | not textbook: find per entry |
| 13 | 42 | TableStPer.rs | collect (T,I) | "ref 42.3" [6] | W n, S n | W a, S a | not textbook [6] |
| 14 | 42 | TableStPer.rs | iter | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 15 | 42 | TableStPer.rs | from_sorted_entries | none | W n, S n | W 1, S 1 | no cost; old wrong [2] |
| 16 | 42 | TableStPer.rs | collect_by_key | none (42.3) | W n², S n² | W a², S a² | no textbook cost [7] |

### 3c. TableMtEph.rs (37 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 42 | TableMtEph.rs | size, singleton (T,I) | 42.5: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 42 | TableMtEph.rs | empty (T,I) | "ref": 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 42 | TableMtEph.rs | domain (T,I) | 42.5: a, lg a | W n, S n | W a², S a² | not textbook; old wrong [1] |
| 4 | 42 | TableMtEph.rs | tabulate (T,I) | line only [5] | T: s·W(f), lg s; I: n, lg n | W s lg s+Σ, S s+maxS | not textbook; old wrong [4] |
| 5 | 42 | TableMtEph.rs | map (T,I) | 42.5 | T: n·W(f), lg n; I: n, lg n | W a lg a+Σ, S a+maxS | not textbook; old wrong [4] |
| 6 | 42 | TableMtEph.rs | filter (T,I) | 42.5 | W n+Σ; I: n | W a+Σ, S a+Σ | not textbook: seq in Mt |
| 7 | 42 | TableMtEph.rs | intersection (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: seq scans |
| 8 | 42 | TableMtEph.rs | union (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: seq scans |
| 9 | 42 | TableMtEph.rs | difference (T,I) | 42.5 | W n·m, S n·m | W a·b, S a·b | not textbook: seq scans |
| 10 | 42 | TableMtEph.rs | find, delete, insert | 42.5: lg a | W n, S n | W a, S a | not textbook: linear |
| 11 | 42 | TableMtEph.rs | restrict, subtract | 42.5 | W n·m, S n·m | W a·c, S a·c | not textbook: seq finds |
| 12 | 42 | TableMtEph.rs | entries (T,I) | none | W n, S n | W a, S a | no textbook cost |
| 13 | 42 | TableMtEph.rs | iter (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 14 | 42 | TableMtEph.rs | map_table_dc | none | W n, S lg n | W a lg a+Σ, S a+maxS | no cost; old wrong [4] |
| 15 | 42 | TableMtEph.rs | tabulate_table_dc | none | W n, S lg n | W s lg s+Σ, S s+maxS | no cost; old wrong [4] |
| 16 | 42 | TableMtEph.rs | from_sorted_entries | none | W n, S n | W 1, S 1 | no cost; old wrong [2] |

### Footnotes

1. `domain` inserts each key into an `ArraySetStEph`, and each
   `ArraySetStEph::insert` is a linear find plus a copy of the set. The i-th
   insert costs O(i), so the loop is O(|a|²) work and span, not O(n). This
   holds in all three files (6 lines).
2. `from_sorted_entries` wraps the given `Vec` with `from_vec`, which moves
   it: O(1). The old lines said O(n).
3. StPer `union` clones `a` (O(|a|)) and then calls the persistent `insert`
   once per entry of `b`; each `insert` scans and copies the whole growing
   table (O(|a| + j)). Total O(|b|·(|a| + |b|)), which is quadratic in |b|
   when b is the larger table; the old O(n·m) holds only when |a| ≥ |b|.
   StEph and MtEph `union` use the two-phase nested scan and are O(|a|·|b|).
4. MtEph `map` and `tabulate` call `map_table_dc` / `tabulate_table_dc`,
   which fork with `join` but split the input with two sequential
   `subseq_copy` calls and rejoin with the sequential `ArraySeqMtEphS::append`
   at every level. W(n) = 2W(n/2) + O(n) and S(n) = S(n/2) + O(n) give
   Work O(n lg n + Σ W(f)), Span O(n + max S(f)). The old lines claimed
   O(n) work and O(lg n) span (the same defect as the Chap18 Mt D&C helpers).
5. CS 42.5 has no `tabulate` row; the files' APAS line (Work |s|·W(f),
   Span lg |s| + S(f)) was used as the reference, and the verdict is against
   that line.
6. StPer `collect` returns a clone of the entry array. Its APAS line cites
   Algorithm 42.3 with Work O(|a|), Span O(lg |a|), but Algorithm 42.3 is a
   different function (sequence of pairs to table of sequences, which is
   `collect_by_key` here) and states no cost. The verdict compares with the
   line: the clone is sequential, Span O(|a|).
7. `collect_by_key` does, per input pair, a linear `find` that clones the
   key's value sequence, an `append`, a `delete`, and an `insert`; the
   delete and insert each clone every entry, including every value
   sequence, so step i costs O(i). Total O(|a|²) work and span. APAS's
   Algorithm 42.3 (reduce with union) states no cost.

## 4. Counts (per annotation site)

118 new lines were added, one per annotated site, in 3 files.
`TableSpecsAndLemmas.rs` and `Example42_1.rs` have no annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 18 |
| 2 | does not match textbook | 74 |
| 3 | does not match old analysis | 17 |
| 4 | no textbook cost | 26 |
| 5 | unannotated functions | 20 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 118 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 42 | TableStEph.rs | 40 | 6 | 24 | 3 | 10 |
| 2 | 42 | TableStPer.rs | 41 | 6 | 26 | 5 | 9 |
| 3 | 42 | TableMtEph.rs | 37 | 6 | 24 | 9 | 7 |

## 5. Unannotated functions (20)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 42 | TableStEph.rs | into_iter, default, eq, clone, fmt x2 |
| 2 | 42 | TableStPer.rs | into_iter, eq, clone, fmt x2 |
| 3 | 42 | TableMtEph.rs | into_iter, eq, clone, fmt x2 |
| 4 | 42 | Example42_1.rs | example_42_1, demonstrate_table_operations (T), example_42_1, performance_comparison |

`Example42_1.rs` is a textbook example file and was not reviewed beyond
listing. The `clone` and `eq` impls are O(|a|) sequential Vec operations.

No malformed annotations were found. Three APAS lines cite costs the book
does not give: `empty` ("Ch42 ref", O(1); harmless), `tabulate` (no CS 42.5
row), and StPer `collect` (cites Algorithm 42.3, which is a different
function and has no cost).

## 6. Notable findings

1. **The tables are unsorted arrays, not BSTs.** CS 42.5 assumes a balanced
   tree (O(lg |a|) point operations, O(m lg(1 + n/m)) bulk operations). All
   three files store an unsorted `Vec` with a no-duplicates invariant, so
   find/insert/delete are O(|a|) and every bulk operation is O(|a|·|b|). The
   old lines record this as ACCEPTED DIFFERENCE; 74 of 118 lines do not match
   the textbook, almost all for this reason. The trait comment on `entries`
   says "in key order", but nothing in the invariant orders the keys.
2. **`domain` is quadratic, not linear.** Every file builds the domain with
   one `ArraySetStEph::insert` per key, and that insert is O(set size), so
   `domain` is O(|a|²). The old lines said O(n) (6 sites).
3. **The MtEph "parallel" map and tabulate have linear span.** The D&C
   helpers copy both halves with `subseq_copy` and rejoin with `append`,
   both sequential Vec copies, giving Work O(n lg n), Span O(n) instead of
   the claimed O(n), O(lg n) (6 sites). MtEph `filter`, the set operations,
   and `domain` are plain sequential loops in an Mt module.
4. StPer `union` inserts b's entries one at a time into a persistent table
   that is copied on every insert, making it quadratic in |b|.
5. `from_sorted_entries` is O(1) (a `Vec` move), not O(n), in all three
   files.
