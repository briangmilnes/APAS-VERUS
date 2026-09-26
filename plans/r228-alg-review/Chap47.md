# r228 Alg Analysis Review: Chap47

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap47.txt` and `prompts/Chap47part2.txt` (Hash Tables) give
expected costs under simple uniform hashing, with load factor
alpha = n/m.

| # | Source | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Separate chaining analysis | insert, lookup, delete | 1 + alpha expected | 1 + alpha expected |
| 2 | Flat tables (uniform probing) | insert, unsuccessful lookup | 1/(1 − alpha) expected | 1/(1 − alpha) expected |
| 3 | Flat tables (uniform probing) | successful lookup | (1/alpha) ln(1/(1 − alpha)) | same |
| 4 | Ex 47.3 / resize text | resize (rehash) | n + m + m' | n + m + m' |
| 5 | Hash function | hash | 1 | 1 |

Chapter 47 states no costs for `createTable`, `loadAndSize`, `probe`, the
per-entry operations of `EntryTrait`, or `hash_index`. The files carry
legacy lines "APAS (Ch47 ref): ..." on them. Those lines are treated as
the textbook cost; a new review that agrees with such a line is counted
"matches textbook". Functions with no APAS line (clone helpers, the
recursive chain helpers, `metrics`, `call_hash_fn`) are "no textbook
cost".

Conventions:
- Old lines that say "O(1) expected" are counted as agreeing with
  O(1 + alpha) when alpha is held constant.
- Old lines that give the worst case "O(n)" for a chain operation are
  counted as consistent with the textbook, since the textbook's bound is
  an expectation.
- n = number of stored pairs, m = table size, m' = new table size.
- Every file is sequential: Span = Work.

## 2. Reviewed functions

Every trait function in `ParaHashTableStEph.rs`, `ChainedHashTable.rs`,
and `FlatHashTable.rs` has one annotation, on the trait declaration or on
the entry impl. Each concrete table file annotates its own impls. "old
wrong" means "does not match old analysis".

### 2a. ParaHashTableStEph.rs (14 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 47 | ParaHashTableStEph.rs | EntryTrait::new | none | N/A | W 1, S 1 | no cost; old wrong [1] |
| 2 | 47 | ParaHashTableStEph.rs | EntryTrait::insert | 1, 1 | 1 expected | W 1+alpha exp | matches textbook |
| 3 | 47 | ParaHashTableStEph.rs | EntryTrait::lookup | 1, 1 | 1 expected | W 1+alpha exp | matches textbook |
| 4 | 47 | ParaHashTableStEph.rs | EntryTrait::delete | 1+alpha | 1+alpha exp | W 1+alpha exp | matches textbook |
| 5 | 47 | ParaHashTableStEph.rs | clone_entry | none | W n, S n | W n, S n | no textbook cost |
| 6 | 47 | ParaHashTableStEph.rs | createTable | m, m | W m, S m | W m, S m | matches textbook |
| 7 | 47 | ParaHashTableStEph.rs | insert | 1, 1 | 1 expected | W 1+alpha exp [2] | matches textbook |
| 8 | 47 | ParaHashTableStEph.rs | lookup | 1, 1 | 1 expected | W 1+alpha exp [2] | matches textbook |
| 9 | 47 | ParaHashTableStEph.rs | delete | 1+alpha | 1+alpha exp | W 1+alpha exp [2] | matches textbook |
| 10 | 47 | ParaHashTableStEph.rs | metrics | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 11 | 47 | ParaHashTableStEph.rs | loadAndSize | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 12 | 47 | ParaHashTableStEph.rs | resize | n+m+m' | N/A | W n+m+m' | matches; old wrong [3] |
| 13 | 47 | ParaHashTableStEph.rs | clone_elem | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 14 | 47 | ParaHashTableStEph.rs | call_hash_fn | none | W 1, S 1 | W 1, S 1 | no textbook cost |

### 2b. ChainedHashTable.rs (1 line) and FlatHashTable.rs (7 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 47 | ChainedHashTable.rs | hash_index | 1, 1 | N/A | W 1, S 1 | matches; old wrong |
| 2 | 47 | FlatHashTable.rs | probe | 1, 1 | N/A | W 1, S 1 | matches; old wrong |
| 3 | 47 | FlatHashTable.rs | find_slot | 1/(1−alpha) | N/A | W 1/(1−alpha) exp | matches; old wrong |
| 4 | 47 | FlatHashTable.rs | FlatEntry new | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 5 | 47 | FlatHashTable.rs | FlatEntry insert | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 6 | 47 | FlatHashTable.rs | FlatEntry lookup | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 47 | FlatHashTable.rs | FlatEntry delete | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 47 | FlatHashTable.rs | clone_entry | none | W 1, S 1 | W 1, S 1 | no textbook cost |

### 2c. VecChainedHashTableStEph.rs (11 lines)

The first five rows are the `EntryTrait` impl for `Vec<(Key, Value)>`;
the rest are the table-level impls.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 47 | VecChainedHashTableStEph.rs | clone_vec_pairs | none | W n, S n | W n, S n | no textbook cost |
| 2 | 47 | VecChainedHashTableStEph.rs | Entry new | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 47 | VecChainedHashTableStEph.rs | Entry insert | 1+alpha | W n worst | W 1+alpha exp | matches textbook |
| 4 | 47 | VecChainedHashTableStEph.rs | Entry lookup | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 5 | 47 | VecChainedHashTableStEph.rs | Entry delete | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 6 | 47 | VecChainedHashTableStEph.rs | clone_entry | none | W n, S n | W n, S n | no textbook cost |
| 7 | 47 | VecChainedHashTableStEph.rs | insert | 1+alpha | W n worst | W 1+alpha exp [4] | matches textbook |
| 8 | 47 | VecChainedHashTableStEph.rs | lookup | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 9 | 47 | VecChainedHashTableStEph.rs | delete | 1+alpha | W n worst | W 1+alpha exp [4] | matches textbook |
| 10 | 47 | VecChainedHashTableStEph.rs | resize | n+m+m' | W n+m+m' | W n+m+m' exp | matches textbook |
| 11 | 47 | VecChainedHashTableStEph.rs | hash_index | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### 2d. LinkedListChainedHashTableStEph.rs (11 lines)

`LinkedListStEphS` (Chap18) is backed by a `Vec`, so the chain operations
have the same costs as the Vec-chained file.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 47 | LinkedListChained...StEph.rs | clone_linked_list_entry | none | W n, S n | W n, S n | no textbook cost |
| 2 | 47 | LinkedListChained...StEph.rs | Entry new | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 47 | LinkedListChained...StEph.rs | Entry insert | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 4 | 47 | LinkedListChained...StEph.rs | Entry lookup | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 5 | 47 | LinkedListChained...StEph.rs | Entry delete | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 6 | 47 | LinkedListChained...StEph.rs | clone_entry | none | W n, S n | W n, S n | no textbook cost |
| 7 | 47 | LinkedListChained...StEph.rs | insert | n worst | W n worst | W 1+alpha exp [4] | matches textbook |
| 8 | 47 | LinkedListChained...StEph.rs | lookup | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 9 | 47 | LinkedListChained...StEph.rs | delete | n worst | W n worst | W 1+alpha exp [4] | matches textbook |
| 10 | 47 | LinkedListChained...StEph.rs | resize | n+m+m' | W n+m+m' | W n+m+m' exp | matches textbook |
| 11 | 47 | LinkedListChained...StEph.rs | hash_index | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### 2e. StructChainedHashTable.rs (13 lines)

A hand-built singly linked `ChainList` of `Node`s with recursive helpers.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 47 | StructChainedHashTable.rs | chain_insert | none | W n, S n | W n, S n [5] | no textbook cost |
| 2 | 47 | StructChainedHashTable.rs | chain_lookup | none | W n, S n | W n, S n | no textbook cost |
| 3 | 47 | StructChainedHashTable.rs | chain_delete | none | W n, S n | W n, S n [5] | no textbook cost |
| 4 | 47 | StructChainedHashTable.rs | Entry new | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 47 | StructChainedHashTable.rs | Entry insert | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 6 | 47 | StructChainedHashTable.rs | Entry lookup | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 7 | 47 | StructChainedHashTable.rs | Entry delete | 1+alpha | W n, S n | W 1+alpha exp | matches textbook |
| 8 | 47 | StructChainedHashTable.rs | clone_entry | none | W n, S n | W n, S n | no textbook cost |
| 9 | 47 | StructChainedHashTable.rs | insert | n worst | W n worst | W 1+alpha exp [4] | matches textbook |
| 10 | 47 | StructChainedHashTable.rs | lookup | 1+alpha | W 1+alpha | W 1+alpha exp | matches textbook |
| 11 | 47 | StructChainedHashTable.rs | delete | n worst | W n worst | W 1+alpha exp [4] | matches textbook |
| 12 | 47 | StructChainedHashTable.rs | resize | n+m+m' | W n+m+m' | W n+m+m' exp | matches textbook |
| 13 | 47 | StructChainedHashTable.rs | hash_index | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### 2f. Flat (open-addressing) tables (19 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 47 | LinProbFlatHashTableStEph.rs | insert | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp [6] | matches textbook |
| 2 | 47 | LinProbFlatHashTableStEph.rs | lookup | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp [6] | matches textbook |
| 3 | 47 | LinProbFlatHashTableStEph.rs | delete | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 4 | 47 | LinProbFlatHashTableStEph.rs | resize | n+m+m' | W n+m+m' | W n+m+m' exp | matches textbook |
| 5 | 47 | LinProbFlatHashTableStEph.rs | probe | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 6 | 47 | LinProbFlatHashTableStEph.rs | find_slot | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 7 | 47 | QuadProbFlatHashTableStEph.rs | insert | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | lookup | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 9 | 47 | QuadProbFlatHashTableStEph.rs | delete | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 10 | 47 | QuadProbFlatHashTableStEph.rs | resize | n+m+m' | W n+m+m' | W n+m+m' exp | matches textbook |
| 11 | 47 | QuadProbFlatHashTableStEph.rs | probe | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 12 | 47 | QuadProbFlatHashTableStEph.rs | find_slot | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 13 | 47 | DoubleHashFlat...StEph.rs | second_hash | 1, 1 | W sizeof(Key) | W 1, S 1 | matches textbook |
| 14 | 47 | DoubleHashFlat...StEph.rs | insert | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 15 | 47 | DoubleHashFlat...StEph.rs | lookup | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 16 | 47 | DoubleHashFlat...StEph.rs | delete | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |
| 17 | 47 | DoubleHashFlat...StEph.rs | resize | n+m+m' | W n+m+m' | W n+m+m' exp | matches textbook |
| 18 | 47 | DoubleHashFlat...StEph.rs | probe | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 19 | 47 | DoubleHashFlat...StEph.rs | find_slot | 1/(1−alpha) | 1/(1−alpha) | W 1/(1−alpha) exp | matches textbook |

For every flat-table row the span equals the work (sequential probe loop).

### Footnotes

1. The trait declaration of `EntryTrait::new` says "N/A — abstract trait
   method". Every impl (Vec, LinkedListStEphS, ChainList, FlatEntry)
   builds an empty entry in O(1).
2. The table-level `insert`, `lookup`, `delete` in `ParaHashTableStEph`
   dispatch to the entry. Chained impls cost O(1 + alpha) expected; flat
   impls cost O(1/(1 − alpha)) expected.
3. The trait `resize` old line reads "N/A — abstract trait method". Every
   impl collects the n pairs from the m slots, allocates m' slots, and
   reinserts sequentially: O(n + m + m') expected. This site is also
   malformed (see section 5).
4. The table-level `insert` and `delete` in the chained files clone the
   selected bucket, edit the clone, and write it back. The clone is
   O(chain length), so the expected cost remains O(1 + alpha); the worst
   case is O(n). The legacy APAS line gives the worst case "O(n)".
5. `chain_insert` rebuilds every node up to the match (or the whole chain
   when the key is absent); `chain_delete` always walks the whole chain,
   rebuilding the kept nodes. Both are linear in the chain length and
   allocate O(chain length) nodes.
6. Linear probing has primary clustering, so its expected probe count is
   (1 + 1/(1 − alpha)²)/2 for insert, not the uniform-probing
   1/(1 − alpha) the textbook derives. The new lines follow the textbook's
   uniform-probing model and say so in the linear-probing file.

## 3. Counts (per annotation site)

76 new lines were added, one per annotated site, in 9 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 61 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 5 |
| 4 | no textbook cost | 15 |
| 5 | unannotated functions | 22 |
| 6 | malformed annotations | 17 |

Rows 1, 2, and 4 partition the 76 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 47 | ParaHashTableStEph.rs | 14 | 9 | 0 | 2 | 5 |
| 2 | 47 | ChainedHashTable.rs | 1 | 1 | 0 | 1 | 0 |
| 3 | 47 | FlatHashTable.rs | 7 | 6 | 0 | 2 | 1 |
| 4 | 47 | VecChainedHashTableStEph.rs | 11 | 9 | 0 | 0 | 2 |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 11 | 9 | 0 | 0 | 2 |
| 6 | 47 | StructChainedHashTable.rs | 13 | 8 | 0 | 0 | 5 |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 6 | 6 | 0 | 0 | 0 |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 6 | 6 | 0 | 0 | 0 |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 7 | 7 | 0 | 0 | 0 |

## 4. Unannotated functions (22)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 47 | ParaHashTableStEph.rs | fmt x4 (Debug/Display) |
| 2 | 47 | FlatHashTable.rs | clone (FlatEntry), fmt x2 |
| 3 | 47 | VecChainedHashTableStEph.rs | fmt |
| 4 | 47 | LinkedListChainedHashTableStEph.rs | fmt |
| 5 | 47 | StructChainedHashTable.rs | clone x2, eq x2, default |
| 6 | 47 | StructChainedHashTable.rs | fmt x5 |
| 7 | 47 | LinProbFlatHashTableStEph.rs | fmt |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | fmt |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | fmt |

All are derive-style trait impls (Clone, PartialEq, Default, Debug,
Display). No algorithmic function is unannotated.

## 5. Malformed annotations (17)

A new line was still added after the last Alg Analysis line of each of
these sites; the malformed text was left as is.

| # | Chap | File | Function | Defect |
|---|---|---|---|---|
| 1 | 47 | ParaHashTableStEph.rs | resize (T) | APAS line split; 2 old lines [7] |
| 2 | 47 | VecChainedHashTableStEph.rs | Entry insert | old line has no Span |
| 3 | 47 | VecChainedHashTableStEph.rs | insert | old line has no Span |
| 4 | 47 | VecChainedHashTableStEph.rs | delete | old line has no Span |
| 5 | 47 | LinkedListChained...StEph.rs | insert | old line has no Span |
| 6 | 47 | LinkedListChained...StEph.rs | delete | old line has no Span |
| 7 | 47 | StructChainedHashTable.rs | delete | Veracity comment inside block [8] |
| 8 | 47 | LinProbFlatHashTableStEph.rs | insert | old line has no Span |
| 9 | 47 | LinProbFlatHashTableStEph.rs | lookup | old line has no Span |
| 10 | 47 | LinProbFlatHashTableStEph.rs | delete | old line has no Span |
| 11 | 47 | LinProbFlatHashTableStEph.rs | find_slot | old line has no Span |
| 12 | 47 | QuadProbFlatHashTableStEph.rs | resize | no Span; Veracity comment [8] |
| 13 | 47 | DoubleHashFlat...StEph.rs | insert | old line has no Span |
| 14 | 47 | DoubleHashFlat...StEph.rs | lookup | no Span; Veracity comment [8] |
| 15 | 47 | DoubleHashFlat...StEph.rs | delete | old line has no Span |
| 16 | 47 | DoubleHashFlat...StEph.rs | resize | Veracity comment inside block [8] |
| 17 | 47 | DoubleHashFlat...StEph.rs | find_slot | old line has no Span |

7. The APAS line of the trait `resize` wraps onto a second `///` line
   ("... where n is number of elements," / continuation), and the block
   holds two old Code-review lines, the second "N/A — abstract trait
   method". This is the only site in the chapter where the count of old
   Code-review lines (15 in the file) exceeds the count of new lines (14).
8. A `// Veracity: NEEDED proof block` comment sits between the APAS line
   and the old Code-review line, splitting the annotation block.

## 6. Notable findings

1. **The chapter's costs hold, under the textbook's model.** All 61
   textbook-backed sites match: chained insert, lookup, and delete are
   O(1 + alpha) expected and flat-table operations O(1/(1 − alpha))
   expected. No site disagrees with the textbook. The files are
   sequential, so span equals work everywhere; chapter 47 gives no
   parallel bound to miss.
2. **Tombstones inflate alpha in all three flat tables.** Deletes write a
   `Deleted` marker; `insert` probes past `Deleted` slots to an `Empty`
   slot or the key rather than reusing the first tombstone, and only
   `resize` clears tombstones. The alpha in 1/(1 − alpha) therefore counts
   live pairs plus tombstones, so a delete-heavy workload degrades until
   the next resize.
3. **Linear probing is analyzed under uniform probing.** The textbook's
   1/(1 − alpha) assumes each probe is independent. Linear probing's
   primary clustering gives (1 + 1/(1 − alpha)²)/2 expected probes for
   insert and unsuccessful lookup. The new LinProb lines state the model
   they assume.
4. **Chained insert and delete copy the bucket.** The table-level
   insert/delete in the Vec, LinkedList, and Struct chained files clone
   the bucket, modify the clone, and store it back. This keeps the
   expected bound but doubles the constant and allocates per operation.
   `LinkedListChainedHashTableStEph` is backed by a `Vec`, not a linked
   list, so it has no cost advantage over the Vec-chained file.
5. **Five old lines said "N/A".** The trait declarations of
   `EntryTrait::new`, `resize`, `hash_index`, `probe`, and `find_slot` had
   "N/A — abstract trait method" lines. Every impl has a definite cost,
   now recorded.
6. **17 malformed annotation blocks.** Most old Code-review lines in the
   concrete files end after "Work ... expected" and omit the Span. Four
   blocks are split by `// Veracity:` comments.
