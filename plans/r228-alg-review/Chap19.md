# r228 Alg Analysis Review: Chap19

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap19.txt`)

Chapter 19 gives a parametric implementation of the sequence ADT in terms of
the primitives nth, length, subseq, tabulate, flatten, inject, and ninject. The
prose states no cost of its own; the costs are those of the array-sequence cost
specification, which the existing APAS lines cite as `Ch20 CS 20.2` (general),
`CS 20.3` (iterate), `CS 20.4` (reduce), `CS 20.5` (scan), and `Ch22 CS 22.2`
(single-threaded array sequences). This review compares against those lines.

| # | Item | Definition | Cost used (from APAS lines) |
|---|---|---|---|
| 1 | Alg 19.1 empty | tabulate (λi.i) 0 | W 1, S 1 |
| 2 | Alg 19.2 singleton | tabulate (λi.x) 1 | W 1, S 1 |
| 3 | Alg 19.3 map | tabulate (λi.f(a[i])) \|a\| | W 1 + ΣW(f), S 1 + max S(f) |
| 4 | Alg 19.4 append | flatten or tabulate select | W \|a\|+\|b\|, S 1 |
| 5 | Alg 19.5 filter (deflate) | flatten (map (deflate f) a) | W 1 + ΣW(f), S lg\|a\| + max S(f) |
| 6 | Alg 19.6 update | tabulate | W \|a\|, S 1 (CS 22.2: W 1) |
| 7 | Alg 19.7 isEmpty, isSingleton | length test | W 1, S 1 |
| 8 | Alg 19.8 iterate | left-to-right recursion | W 1 + ΣW(f), S 1 + ΣS(f) |
| 9 | Alg 19.9 reduce | divide and conquer | W 1 + ΣW(f), S lg\|a\| · max S(f) |
| 10 | Alg 19.10 scan | contraction | W \|a\|, S lg\|a\| |
| 11 | primitives nth, length, subseq | — | W 1, S 1 |
| 12 | primitive flatten | — | W \|a\| + Σ\|a[i]\|, S lg\|a\| |
| 13 | primitive inject | — | W \|a\|+\|b\|, S lg(degree(b)) |
| 14 | primitive ninject | — | W \|a\|+\|b\|, S 1 |

## 2. Reviewed functions

Most functions carry an annotation on the trait declaration (T) and on the impl
(I). Both sites received a line and both are counted. Where the two sites carry
the same verdict they share a row. W = Work, S = Span, n = |a|.

### ArraySeqStEph.rs (48 sites)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 19 | ArraySeqStEph.rs | new (T,I) | none | W len, S len | W len, S len | no textbook cost |
| 2 | 19 | ArraySeqStEph.rs | set (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 19 | ArraySeqStEph.rs | length (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 19 | ArraySeqStEph.rs | nth (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 5 | 19 | ArraySeqStEph.rs | subseq_copy (T,I) | none | W len, S len | W len, S len | no textbook cost |
| 6 | 19 | ArraySeqStEph.rs | subseq (T,I) | W 1, S 1 | W len, S len | W len, S len | not tb: Vec copy |
| 7 | 19 | ArraySeqStEph.rs | from_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 8 | 19 | ArraySeqStEph.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 9 | 19 | ArraySeqStEph.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 10 | 19 | ArraySeqStEph.rs | append (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 11 | 19 | ArraySeqStEph.rs | filter (T,I) | W ΣW, S lg n | W n, S n | W n+ΣW, S n+ΣS | not tb: sequential |
| 12 | 19 | ArraySeqStEph.rs | update (T,I) | W n, S 1 [1] | W n, S n | W n, S n | not tb: Vec clone |
| 13 | 19 | ArraySeqStEph.rs | inject (T,I) | W a+b, S lg d | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 14 | 19 | ArraySeqStEph.rs | is_empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 15 | 19 | ArraySeqStEph.rs | is_singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 16 | 19 | ArraySeqStEph.rs | iterate_iter (T,I) | no cost [2] | W n, S n | W n+ΣW, S n+ΣS | matches textbook |
| 17 | 19 | ArraySeqStEph.rs | iterate (T) | W ΣW, S ΣS | W ΣW, no S | W n+ΣW, S n+ΣS | match; not old [3] |
| 18 | 19 | ArraySeqStEph.rs | iterate (I) | W ΣW, S ΣS | W n, S n | W n+ΣW, S n+ΣS | matches textbook |
| 19 | 19 | ArraySeqStEph.rs | reduce_iter (T,I) | no cost [2] | W n, S n | W n+ΣW, S n+ΣS | not tb: fold |
| 20 | 19 | ArraySeqStEph.rs | reduce (T,I) | W ΣW, S lg n·S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: fold |
| 21 | 19 | ArraySeqStEph.rs | scan (T,I) | W n, S lg n | W n, S n | W n, S n | not tb: sequential |
| 22 | 19 | ArraySeqStEph.rs | map (T,I) | W ΣW, S max S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: sequential |
| 23 | 19 | ArraySeqStEph.rs | tabulate (T,I) | W ΣW, S max S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: sequential |
| 24 | 19 | ArraySeqStEph.rs | flatten (T,I) | W a+Σ, S lg a | W a+Σ, S a+Σ | W a+Σ, S a+Σ | not tb: sequential |
| 25 | 19 | ArraySeqStEph.rs | deflate (T,I) | no cost [2] | W 1, S 1 | W 1+W(f), S 1+S(f) | no textbook cost |

### ArraySeqStPer.rs (46 sites)

Same bodies as `ArraySeqStEph.rs` except: no `set`, and `update` is a
tabulate over all |a| positions instead of clone + set.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 26 | 19 | ArraySeqStPer.rs | new (T,I) | none | W len, S len | W len, S len | no textbook cost |
| 27 | 19 | ArraySeqStPer.rs | length (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 28 | 19 | ArraySeqStPer.rs | nth (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 29 | 19 | ArraySeqStPer.rs | subseq_copy (T,I) | none | W len, S len | W len, S len | no textbook cost |
| 30 | 19 | ArraySeqStPer.rs | subseq (T,I) | W 1, S 1 | W len, S len | W len, S len | not tb: Vec copy |
| 31 | 19 | ArraySeqStPer.rs | from_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 32 | 19 | ArraySeqStPer.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 33 | 19 | ArraySeqStPer.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 34 | 19 | ArraySeqStPer.rs | append (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 35 | 19 | ArraySeqStPer.rs | filter (T,I) | W ΣW, S lg n | W n, S n | W n+ΣW, S n+ΣS | not tb: sequential |
| 36 | 19 | ArraySeqStPer.rs | update (T,I) | W n, S 1 [1] | W n, S n | W n, S n | not tb: seq tabulate [4] |
| 37 | 19 | ArraySeqStPer.rs | inject (T,I) | W a+b, S lg d | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 38 | 19 | ArraySeqStPer.rs | is_empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 39 | 19 | ArraySeqStPer.rs | is_singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 40 | 19 | ArraySeqStPer.rs | iterate_iter (T,I) | no cost [2] | W n, S n | W n+ΣW, S n+ΣS | matches textbook |
| 41 | 19 | ArraySeqStPer.rs | iterate (T) | W ΣW, S ΣS | W ΣW, no S | W n+ΣW, S n+ΣS | match; not old [3] |
| 42 | 19 | ArraySeqStPer.rs | iterate (I) | W ΣW, S ΣS | W n, S n | W n+ΣW, S n+ΣS | matches textbook |
| 43 | 19 | ArraySeqStPer.rs | reduce_iter (T,I) | no cost [2] | W n, S n | W n+ΣW, S n+ΣS | not tb: fold |
| 44 | 19 | ArraySeqStPer.rs | reduce (T,I) | W ΣW, S lg n·S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: fold |
| 45 | 19 | ArraySeqStPer.rs | scan (T,I) | W n, S lg n | W n, S n | W n, S n | not tb: sequential |
| 46 | 19 | ArraySeqStPer.rs | map (T,I) | W ΣW, S max S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: sequential |
| 47 | 19 | ArraySeqStPer.rs | tabulate (T,I) | W ΣW, S max S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: sequential |
| 48 | 19 | ArraySeqStPer.rs | flatten (T,I) | W a+Σ, S lg a | W a+Σ, S a+Σ | W a+Σ, S a+Σ | not tb: sequential |
| 49 | 19 | ArraySeqStPer.rs | deflate (T,I) | no cost [2] | W 1, S 1 | W 1+W(f), S 1+S(f) | no textbook cost |

### ArraySeqMtEph.rs (56 sites)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 50 | 19 | ArraySeqMtEph.rs | new (T,I) | none | W len, S len | W len, S len | no textbook cost |
| 51 | 19 | ArraySeqMtEph.rs | set (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 52 | 19 | ArraySeqMtEph.rs | length (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 53 | 19 | ArraySeqMtEph.rs | nth (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 54 | 19 | ArraySeqMtEph.rs | subseq_copy (T,I) | none | W len, S len | W len, S len | no textbook cost |
| 55 | 19 | ArraySeqMtEph.rs | subseq (T,I) | W 1, S 1 | W len, S len | W len, S len | not tb: Vec copy |
| 56 | 19 | ArraySeqMtEph.rs | from_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 57 | 19 | ArraySeqMtEph.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 58 | 19 | ArraySeqMtEph.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 59 | 19 | ArraySeqMtEph.rs | append (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential in Mt |
| 60 | 19 | ArraySeqMtEph.rs | filter (T,I) | W ΣW, S lg n | W n, S n | W n lg n, S n | not tb; not old [5] |
| 61 | 19 | ArraySeqMtEph.rs | update (T,I) | W n, S 1 [1] | W n, S n | W n, S n | not tb: Vec clone |
| 62 | 19 | ArraySeqMtEph.rs | inject (T,I) | W a+b, S lg d | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 63 | 19 | ArraySeqMtEph.rs | ninject (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: seq inject |
| 64 | 19 | ArraySeqMtEph.rs | is_empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 65 | 19 | ArraySeqMtEph.rs | is_singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 66 | 19 | ArraySeqMtEph.rs | iterate_iter (T,I) | no cost [2] | W n, S n | W n+ΣW, S n+ΣS | matches textbook |
| 67 | 19 | ArraySeqMtEph.rs | iterate (T,I) | W ΣW, S ΣS | W ΣW, S ΣS | W n+ΣW, S n+ΣS | matches textbook |
| 68 | 19 | ArraySeqMtEph.rs | reduce_iter (T,I) | no cost [2] | W n, S n | W n+ΣW, S n+ΣS | not tb: fold |
| 69 | 19 | ArraySeqMtEph.rs | reduce (T,I) | W ΣW, S lg n·S | W n, S lg n | W n lg n, S n | not tb; not old [6] |
| 70 | 19 | ArraySeqMtEph.rs | scan (T,I) | W n, S lg n | W n, S n | W n, S n | not tb: sequential in Mt |
| 71 | 19 | ArraySeqMtEph.rs | map (T,I) | W ΣW, S max S | W ΣW/n, S n | W n lg n, S n | not tb; not old [5] |
| 72 | 19 | ArraySeqMtEph.rs | tabulate (T,I) | W ΣW, S max S | W ΣW, S ΣS | W n+ΣW, S n+ΣS | not tb: sequential in Mt |
| 73 | 19 | ArraySeqMtEph.rs | flatten (T,I) | W a+Σ, S lg a | W a+Σ, S a+Σ | W a+Σ, S a+Σ | not tb: sequential in Mt |
| 74 | 19 | ArraySeqMtEph.rs | deflate (T,I) | no cost [2] | W 1, S 1 | W 1+W(f), S 1+S(f) | no textbook cost |
| 75 | 19 | ArraySeqMtEph.rs | map_par | no cost [2] | W n, S n | W n lg n, S n | not tb; not old [5] |
| 76 | 19 | ArraySeqMtEph.rs | filter_dc | none | W n, S n | W n lg n, S n | no tb; not old [5] |
| 77 | 19 | ArraySeqMtEph.rs | filter_par | no cost [2] | W n, S n | W n lg n, S n | not tb; not old [5] |
| 78 | 19 | ArraySeqMtEph.rs | reduce_par | no cost [2] | W n, S lg n | W n lg n, S n | not tb; not old [6] |
| 79 | 19 | ArraySeqMtEph.rs | map_dc | none | W n, S n | W n lg n, S n | no tb; not old [5] |
| 80 | 19 | ArraySeqMtEph.rs | concat_seqs | none | W b, S b | W b am, S b | no textbook cost |

### ArraySeqMtEphSlice.rs (38 sites)

Slices share an `Arc<Vec<T>>` backing, so `slice`/`subseq_copy` are O(1).
Only `reduce`, `map`, `filter`, `tabulate`, `scan` carry annotations on the
trait only; their impls and D&C helpers are unannotated.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 81 | 19 | ArraySeqMtEphSlice.rs | length (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 82 | 19 | ArraySeqMtEphSlice.rs | nth_cloned (T,I) | CS nth | W 1, S 1 | W 1, S 1 | matches textbook |
| 83 | 19 | ArraySeqMtEphSlice.rs | slice (T,I) | CS subseq | W 1, S 1 | W 1, S 1 | matches textbook |
| 84 | 19 | ArraySeqMtEphSlice.rs | subseq_copy (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 85 | 19 | ArraySeqMtEphSlice.rs | from_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 86 | 19 | ArraySeqMtEphSlice.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 87 | 19 | ArraySeqMtEphSlice.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 88 | 19 | ArraySeqMtEphSlice.rs | new (T,I) | none | W n, S n | W len, S len | no textbook cost |
| 89 | 19 | ArraySeqMtEphSlice.rs | to_vec (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 90 | 19 | ArraySeqMtEphSlice.rs | is_empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 91 | 19 | ArraySeqMtEphSlice.rs | is_singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 92 | 19 | ArraySeqMtEphSlice.rs | set (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 93 | 19 | ArraySeqMtEphSlice.rs | append (T,I) | CS a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 94 | 19 | ArraySeqMtEphSlice.rs | update (T,I) | CS n, S 1 | W n, S n | W n, S n | not tb: to_vec copy |
| 95 | 19 | ArraySeqMtEphSlice.rs | inject (T,I) | CS a+b, lg d | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 96 | 19 | ArraySeqMtEphSlice.rs | ninject (T,I) | CS a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: seq inject |
| 97 | 19 | ArraySeqMtEphSlice.rs | reduce (T) | CS lg n·S | W n, S lg n | W n+ΣW, S lg n·S | matches textbook |
| 98 | 19 | ArraySeqMtEphSlice.rs | map (T) | CS S 1 | W n, S n | W n lg n, S n | not tb; not old [7] |
| 99 | 19 | ArraySeqMtEphSlice.rs | filter (T) | CS S lg n | W n, S n | W n lg n, S n | not tb; not old [7] |
| 100 | 19 | ArraySeqMtEphSlice.rs | tabulate (T) | W nW, S lg n+S | W nW, S lg n·S | W n lg n, S n | not tb; not old [7] |
| 101 | 19 | ArraySeqMtEphSlice.rs | scan (T) | W n, S lg n | W n lg n, S n | W n lg n, S n | not tb: seq rejoin |
| 102 | 19 | ArraySeqMtEphSlice.rs | flatten (free fn) | W Σ, S lg a+max | W Σ, S lg^2 a+max | W Σ lg a, S Σ | not tb; not old [8] |

"CS" in the APAS column means the file has no APAS line and the comparison uses
the array-sequence cost specification (CS 20.2) for the same ADT operation.

Footnotes:

1. `update` has two APAS lines: CS 20.2 Work O(|a|), Span O(1), and CS 22.2
   Work O(1), Span O(1). The implementations copy the whole Vec sequentially,
   so neither holds.
2. The APAS line names an algorithm but states no cost (for example
   `APAS (Ch19 Alg 19.8): iterate (iterative form)`, `... deflate (part of
   filter)`, `APAS (Ch19 Alg 19.9): parallel variant — reduce`). The review
   used the cost of the named ADT operation.
3. The trait-level `iterate` old line gives only Work, and calls APAS parallel
   ("ACCEPTED DIFFERENCE: St sequential, APAS parallel"). APAS iterate (CS
   20.3) is sequential with Span O(1 + ΣS(f)), so the sequential loop matches
   the textbook.
4. `ArraySeqStPer::update` old line says "clone entire Vec + set", but the
   body is a sequential tabulate over all |a| positions. The cost is the same;
   the stated reason is wrong.
5. The Mt D&C map/filter helpers (`map_par`, `filter_par`, `filter_dc`,
   `map_dc`, and the trait `map`/`filter` that call them) split with two
   `subseq_copy` calls (sequential O(n) copies) and recombine with a
   sequential `append` or `concat_seqs` at every level. The recurrences are
   W(n) = 2W(n/2) + O(n) = O(n lg n) and S(n) = S(n/2) + O(n) = O(n). The old
   lines state Work O(n).
6. `reduce_par` (and the trait `reduce` that calls it) also copies both halves
   with `subseq_copy` before each `join`. Work is O(|a| lg |a| + ΣW(f)) and
   Span O(|a| + lg |a| · max S(f)). The old lines state Span O(lg |a|), which
   holds only for the Slice version, where splits are O(1).
7. The Slice `map_dc_vec`, `filter_dc_vec`, and `tabulate_dc_vec` split in
   O(1) but rejoin by pushing every element of the right result onto the left
   Vec in a sequential loop, so W O(n lg n), S O(n). The old `tabulate` line
   (Span O(lg n · S(f))) omits the O(n) rejoin; the old `map`/`filter` lines
   state Work O(n).
8. `flatten_dc_vec` rejoins by a sequential push of the right half's
   elements, up to Σ|a[i]| per level. Work O(|a| + Σ|a[i]| lg |a|), Span
   O(lg |a| + Σ|a[i]|). The old line (Span O(lg² |a| + max |a[i]|)) treats the
   rejoin as logarithmic.

## 3. Counts (per annotation site = lines added)

| # | Measure | Count |
|---|---|---|
| 1 | lines added | 188 |
| 2 | matches textbook | 63 |
| 3 | does not match textbook | 84 |
| 4 | does not match old analysis | 17 |
| 5 | no textbook cost | 41 |
| 6 | unannotated functions | 39 |
| 7 | malformed annotations | 0 |

Rows 2, 3, 5 partition the 188 sites; row 4 overlaps them (2 of the 17 carry
"no textbook cost", 13 carry "does not match textbook", 2 carry "matches
textbook").

Per file: StEph 48 (16 match, 22 not tb, 10 no tb, 1 not old); StPer 46 (16,
22, 8, 1); MtEph 56 (16, 27, 13, 11); MtEphSlice 38 (15, 13, 10, 4).

Unannotated exec functions:

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 19 | ArraySeqStEph.rs | iter, 2 into_iter, clone, eq, 2 fmt |
| 2 | 19 | ArraySeqStPer.rs | iter, 2 into_iter, clone, eq, 2 fmt |
| 3 | 19 | ArraySeqMtEph.rs | iter, 2 into_iter, clone, eq, 2 fmt |
| 4 | 19 | ArraySeqMtEphSlice.rs | iter (T,I), impl reduce/map/filter/tabulate/scan |
| 5 | 19 | ArraySeqMtEphSlice.rs | reduce_dc, map_dc_vec, filter_dc_vec |
| 6 | 19 | ArraySeqMtEphSlice.rs | tabulate_dc_vec, scan_dc_vec, flatten_dc_vec |
| 7 | 19 | ArraySeqMtEphSlice.rs | into_iter, clone, eq, 2 fmt |

Malformed: none. Notes:

- Several APAS lines carry no cost (footnote 2).
- `APAS (Ch19 Alg 19.15)` on flatten and "Algorithm 19.11 (Function nth)"
  cite algorithm numbers that do not appear in `prompts/Chap19.txt` (the
  chapter ends at Alg 19.10).
- In `ArraySeqStPer.rs` (impl `iterate_iter`, `reduce_iter`, `scan`) and
  `ArraySeqMtEph.rs` (impl `reduce_iter`, `scan`, `filter`, `filter_dc`,
  `reduce_par`), a `// Veracity:` line sits between the doc block and the
  `fn`, or inside the doc block. The new line was placed directly after the
  last `Alg Analysis` line in each case.

## 4. Notable findings

- Wrong old analysis, cost regression: `ArraySeqMtEph::reduce_par` (and
  `reduce`) is annotated Span O(lg |a|), but every level copies both halves
  with the sequential `subseq_copy`, so Span is O(|a|) and Work O(|a| lg |a|).
  The Slice version (`reduce_dc`) avoids the copy with O(1) slices and does
  meet the textbook bound; `ArraySeqMtEph` should split the same way.
- Wrong old analysis: every D&C map/filter/tabulate in `ArraySeqMtEph.rs` and
  `ArraySeqMtEphSlice.rs` is O(n lg n) work, not O(n), because the split or
  the rejoin copies O(n) elements per level. The Slice `flatten` line claims
  Span O(lg² |a| + max |a[i]|); the sequential rejoin makes it O(Σ|a[i]|).
- Missing parallelism: in `ArraySeqMtEph.rs`, `append`, `tabulate`, `flatten`,
  `scan`, `inject`, and `ninject` are the St sequential loops. In all four
  files `subseq`, `update`, and `append` copy element by element, so the
  O(1)-span APAS costs hold for none of them except the Slice `slice`.
- The trait-level `iterate` old lines in StEph/StPer claim "APAS parallel";
  APAS iterate is sequential, so the sequential loop matches the textbook.
- `scan` in the St/MtEph files calls `f` twice per element (once for the
  pushed prefix, once for the accumulator); asymptotically harmless but it
  doubles the combine work.
