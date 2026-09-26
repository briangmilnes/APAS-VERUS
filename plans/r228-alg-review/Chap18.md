# r228 Alg Analysis Review: Chap18

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap18.txt` defines the sequence ADT (Data Type 18.1, Definitions
18.3-18.22) but states costs only qualitatively: iterate has linear span,
reduce has logarithmic span, scan does "essentially the same work and span
as reduce", and ninject "may be implemented more efficiently and in lower
span". The cost tables that the existing APAS lines cite are in Chapter 20
("Cost of Sequences") and Chapter 22 of the book; they were read from
`prompts/algorithms-parallel-and-sequential.pdf`.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | CS 20.2 array | length, nth, empty, singleton, isEmpty, isSingleton, subseq | 1 | 1 |
| 2 | CS 20.2 array | tabulate f n | 1 + Σ W(f(i)) | 1 + max S(f(i)) |
| 3 | CS 20.2 array | map f a | 1 + Σ W(f(x)) | 1 + max S(f(x)) |
| 4 | CS 20.2 array | filter f a | 1 + Σ W(f(x)) | lg\|a\| + max S(f(x)) |
| 5 | CS 20.2 array | append a b | 1 + \|a\| + \|b\| | 1 |
| 6 | CS 20.2 array | flatten a | 1 + \|a\| + Σ\|a[i]\| | 1 + lg\|a\| |
| 7 | CS 20.2 array | update a (i,x) | 1 + \|a\| | 1 |
| 8 | CS 20.2 array | inject a b | 1 + \|a\| + \|b\| | lg(degree(b)) |
| 9 | CS 20.2 array | ninject a b | 1 + \|a\| + \|b\| | 1 |
| 10 | CS 20.2 array | collect f a | 1 + W(f)\|a\| lg\|a\| | 1 + S(f) lg²\|a\| |
| 11 | CS 20.3 | iterate f x a | 1 + Σ W(f) over trace | 1 + Σ S(f) over trace |
| 12 | CS 20.4 | reduce f x a | 1 + Σ W(f) over trace | lg\|a\| · max S(f) |
| 13 | CS 20.5 | scan f x a (f O(1)) | \|a\| | lg\|a\| |
| 14 | CS 20.6 tree | collect (same as array) | 1 + W(f)\|a\| lg\|a\| | 1 + S(f) lg²\|a\| |
| 15 | CS 20.7 list | length, singleton, isEmpty, isSingleton | 1 | 1 |
| 16 | CS 20.7 list | nth a i | i | i |
| 17 | CS 20.7 list | tabulate, map, filter | 1 + Σ W(f) | 1 + Σ S(f) |
| 18 | CS 20.7 list | subseq a (i,j) | 1 + i | 1 + i |
| 19 | CS 20.7 list | append a b | 1 + \|a\| | 1 + \|a\| |
| 20 | CS 20.7 list | flatten, update, inject, ninject | linear in inputs | same as work |
| 21 | CS 20.7 list | iterate, reduce | 1 + Σ W(f) | 1 + Σ S(f) |
| 22 | CS 20.7 list | scan | \|a\| | \|a\| |
| 23 | CS 22.2 stseq | nth, update | 1 | 1 |
| 24 | CS 22.2 stseq | inject a b | \|b\| | lg(degree(b)) |

## 2. Reviewed functions

Every file in Chap18 except `ArraySeqMtEphSlice.rs` is `Vec`-backed, and
most trait functions carry an annotation on the trait declaration (T) and
another on the impl (I); each received its own line. Rows below merge T and
I when both lines have the same content; a footnote marks rows where only one
side disagreed with its old analysis. W = Work, S = Span, n = |a|, m = |b|,
Σ = Σ W(f) or Σ S(f), L = length argument.

### 2a. ArraySeq.rs (Vec-backed base sequence)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 18 | ArraySeq.rs | new (T,I) | none | T: W L, S 1; I: n,n | W L, S L | no cost; T old wrong [1] |
| 2 | 18 | ArraySeq.rs | set (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 18 | ArraySeq.rs | length (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 18 | ArraySeq.rs | nth (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 5 | 18 | ArraySeq.rs | empty (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 6 | 18 | ArraySeq.rs | singleton (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 18 | ArraySeq.rs | subseq (T,I) | 20.2: 1,1 | W j, S j | W L, S L | not textbook: Vec copy |
| 8 | 18 | ArraySeq.rs | append (T,I) | 20.2: n+m, 1 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 9 | 18 | ArraySeq.rs | filter (T,I) | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop |
| 10 | 18 | ArraySeq.rs | update (T,I) | 20.2 n,1; 22.2 1,1 | W n, S n | W n, S n | not textbook: seq copy |
| 11 | 18 | ArraySeq.rs | is_empty (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 12 | 18 | ArraySeq.rs | is_singleton (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 13 | 18 | ArraySeq.rs | iterate (T,I) | 20.3 | W n, S n | W 1+Σ, S 1+Σ | matches textbook |
| 14 | 18 | ArraySeq.rs | reduce (T,I) | 20.4 | W n, S n | W 1+Σ, S 1+Σ | not textbook: seq fold |
| 15 | 18 | ArraySeq.rs | scan (T,I) | 20.5: n, lg n | W n, S n | W n, S n | not textbook: seq loop |
| 16 | 18 | ArraySeq.rs | inject (T,I) | 20.2; 22.2 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 17 | 18 | ArraySeq.rs | scan_inclusive (T,I) | none (20.5) | T: W n, S 1; I: n,n | W n, S n | not textbook; T old wrong [2] |
| 18 | 18 | ArraySeq.rs | subseq_copy (T,I) | none | T: W L, S 1; I: j,j | W L, S L | no cost; T old wrong [2] |
| 19 | 18 | ArraySeq.rs | remove (T) | none | W n, S 1 | W n, S n | no cost; old wrong [3] |
| 20 | 18 | ArraySeq.rs | insert (T,I) | none | T: W n, S 1; I: n,n | W n, S n | no cost; T old wrong [3] |
| 21 | 18 | ArraySeq.rs | from_vec (T,I) | none | T: W n worst; I: 1,1 | W 1, S 1 | no cost; T old wrong [4] |
| 22 | 18 | ArraySeq.rs | find_key (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 23 | 18 | ArraySeq.rs | collect (T,I) | 20.6 | T: n², n²; I: n, n | W n², S n² | not textbook; I old wrong [5] |
| 24 | 18 | ArraySeq.rs | map | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop |
| 25 | 18 | ArraySeq.rs | tabulate | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop |
| 26 | 18 | ArraySeq.rs | flatten | 20.2 | W Σ\|ai\|, same S | W n+Σ\|ai\|, same S | not textbook: seq loops |
| 27 | 18 | ArraySeq.rs | iterate_prefixes | none | W n, S n | W n+Σ, S n+Σ | no textbook cost |

The impl `remove` (line 1193) is malformed and was not given a line (see
section 5).

### 2b. ArraySeqStEph.rs and ArraySeqStPer.rs

The two files have identical executable bodies apart from `set`, which only
StEph has; every verdict below holds in both files (StPer rows 2 omitted).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 18 | ArraySeqStEph/StPer.rs | new (T,I) | none | W L, S L | W L, S L | no textbook cost |
| 2 | 18 | ArraySeqStEph.rs | set (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 18 | ArraySeqStEph/StPer.rs | length, nth (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 18 | ArraySeqStEph/StPer.rs | subseq_copy (T,I) | none | W L, S L | W L, S L | no textbook cost |
| 5 | 18 | ArraySeqStEph/StPer.rs | subseq (T,I) | 20.2: 1,1 | W j, S j | W L, S L | not textbook: Vec copy |
| 6 | 18 | ArraySeqStEph/StPer.rs | from_vec (T,I) | none | T: W n worst; I: 1,1 | W 1, S 1 | no cost; T old wrong [4] |
| 7 | 18 | ArraySeqStEph/StPer.rs | empty, singleton (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 18 | ArraySeqStEph/StPer.rs | append (T,I) | 20.2: n+m, 1 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 9 | 18 | ArraySeqStEph/StPer.rs | filter (T,I) | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop |
| 10 | 18 | ArraySeqStEph/StPer.rs | update (T,I) | 20.2; 22.2 | W n, S n | W n, S n | not textbook: seq copy |
| 11 | 18 | ArraySeqStEph/StPer.rs | inject (T,I) | 20.2; 22.2 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 12 | 18 | ArraySeqStEph/StPer.rs | is_empty, is_singleton | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 13 | 18 | ArraySeqStEph/StPer.rs | iterate (T,I) | 20.3 | W n, S n | W 1+Σ, S 1+Σ | matches textbook |
| 14 | 18 | ArraySeqStEph/StPer.rs | reduce (T,I) | 20.4 | W n, S n | W 1+Σ, S 1+Σ | not textbook: seq fold |
| 15 | 18 | ArraySeqStEph/StPer.rs | scan (T,I) | 20.5 | W n, S n | W n, S n | not textbook: seq loop |
| 16 | 18 | ArraySeqStEph/StPer.rs | map, tabulate (T,I) | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop |
| 17 | 18 | ArraySeqStEph/StPer.rs | flatten (T,I) | 20.2 | W Σ\|ai\|, same S | W n+Σ\|ai\|, same S | not textbook: seq loops |

St files are sequential by design, so every span mismatch in 2b is the
"St sequential span" category of the notation standard.

### 2c. ArraySeqMtEph.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 18 | ArraySeqMtEph.rs | new (T,I) | none | T: W L, S lg L; I: n,n | W L, S L | no cost; T old wrong [1] |
| 2 | 18 | ArraySeqMtEph.rs | set (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 18 | ArraySeqMtEph.rs | length, nth (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 18 | ArraySeqMtEph.rs | subseq_copy (T,I) | none | T: W L, S lg L; I: L,L | W L, S L | no cost; T old wrong [2] |
| 5 | 18 | ArraySeqMtEph.rs | subseq (T,I) | 20.2: 1,1 | W j, S j | W L, S L | not textbook: Vec copy |
| 6 | 18 | ArraySeqMtEph.rs | from_vec (T,I) | none | T: W n worst; I: 1,1 | W 1, S 1 | no cost; T old wrong [4] |
| 7 | 18 | ArraySeqMtEph.rs | empty, singleton (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 18 | ArraySeqMtEph.rs | append (T,I) | 20.2: n+m, 1 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 9 | 18 | ArraySeqMtEph.rs | filter (T,I) | 20.2 | T: n, lg n; I: n, n | W n lg n+Σ, S n | not textbook; old wrong [6] |
| 10 | 18 | ArraySeqMtEph.rs | update (T,I) | 20.2; 22.2 | W n, S n | W n, S n | not textbook: seq copy |
| 11 | 18 | ArraySeqMtEph.rs | inject (T,I) | 20.2; 22.2 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 12 | 18 | ArraySeqMtEph.rs | ninject (T,I) | 20.2: n+m, 1 | W n+m, S n+m | W n+m, S n+m | not textbook: calls inject [7] |
| 13 | 18 | ArraySeqMtEph.rs | is_empty, is_singleton | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 18 | ArraySeqMtEph.rs | iterate (T,I) | 20.3 | W n, S n | W 1+Σ, S 1+Σ | matches textbook |
| 15 | 18 | ArraySeqMtEph.rs | reduce (T,I) | 20.4 | T: n, lg n; I: n, n | W n lg n+Σ, S n+lg n·maxS | not textbook; old wrong [6] |
| 16 | 18 | ArraySeqMtEph.rs | scan (T,I) | 20.5 | W n, S n | W n, S n | not textbook: seq loop |
| 17 | 18 | ArraySeqMtEph.rs | map (T,I) | 20.2 | T: n, lg n+S(f); I: n,n | W n lg n+Σ, S n+maxS | not textbook; old wrong [6] |
| 18 | 18 | ArraySeqMtEph.rs | tabulate (T,I) | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop |
| 19 | 18 | ArraySeqMtEph.rs | flatten (T,I) | 20.2 | W Σ\|ai\|, same S | W n+Σ\|ai\|, same S | not textbook: seq loops |
| 20 | 18 | ArraySeqMtEph.rs | map_par | none | W n, S lg n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 21 | 18 | ArraySeqMtEph.rs | filter_par | none | W n, S lg n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 22 | 18 | ArraySeqMtEph.rs | reduce_par | none | W n, S lg n | W n lg n+Σ, S n+lg n·maxS | no cost; old wrong [6] |
| 23 | 18 | ArraySeqMtEph.rs | reduce_dc | none | W n, S lg n | W n lg n+Σ, S n+lg n·maxS | no cost; old wrong [6] |
| 24 | 18 | ArraySeqMtEph.rs | map_dc | none | W n, S n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 25 | 18 | ArraySeqMtEph.rs | filter_dc | none | W n, S n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 26 | 18 | ArraySeqMtEph.rs | ninject_par | none | W n+m, S m | W n+m, S n+m | not textbook; old wrong [8] |
| 27 | 18 | ArraySeqMtEph.rs | apply_ninject_updates | none | W m, S m | W m, S m | no textbook cost |

### 2d. ArraySeqMtPer.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 18 | ArraySeqMtPer.rs | new (T,I) | none | T: W L, S lg L; I: n,n | W L, S L | no cost; T old wrong [1] |
| 2 | 18 | ArraySeqMtPer.rs | length, nth (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 18 | ArraySeqMtPer.rs | subseq_copy (T,I) | none | T: W L, S lg L; I: L,L | W L, S L | no cost; T old wrong [2] |
| 4 | 18 | ArraySeqMtPer.rs | subseq (T,I) | 20.2: 1,1 | W j, S j | W L, S L | not textbook: Vec copy |
| 5 | 18 | ArraySeqMtPer.rs | from_vec (T,I) | none | T: W n worst; I: 1,1 | W 1, S 1 | no cost; T old wrong [4] |
| 6 | 18 | ArraySeqMtPer.rs | empty, singleton (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 18 | ArraySeqMtPer.rs | append (T,I) | 20.2: n+m, 1 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 8 | 18 | ArraySeqMtPer.rs | filter (T,I) | 20.2 | T: n, lg n; I: n, n | W n lg n+Σ, S n | not textbook; old wrong [6] |
| 9 | 18 | ArraySeqMtPer.rs | update (T,I) | 20.2; 22.2 | W n, S n | W n, S n | not textbook: seq copy |
| 10 | 18 | ArraySeqMtPer.rs | inject (T,I) | 20.2; 22.2 | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 11 | 18 | ArraySeqMtPer.rs | is_empty, is_singleton | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 12 | 18 | ArraySeqMtPer.rs | iterate (T,I) | 20.3 | W n, S n | W 1+Σ, S 1+Σ | matches textbook |
| 13 | 18 | ArraySeqMtPer.rs | reduce (T,I) | 20.4 | W n, S n | W 1+Σ, S 1+Σ | not textbook: seq fold [9] |
| 14 | 18 | ArraySeqMtPer.rs | scan (T,I) | 20.5 | W n, S n | W n, S n | not textbook: seq loop |
| 15 | 18 | ArraySeqMtPer.rs | map, tabulate (T,I) | 20.2 | W n, S n | W n+Σ, S n+Σ | not textbook: seq loop [9] |
| 16 | 18 | ArraySeqMtPer.rs | flatten (T,I) | 20.2 | W Σ\|ai\|, same S | W n+Σ\|ai\|, same S | not textbook: seq loops |
| 17 | 18 | ArraySeqMtPer.rs | map_par | none | W n, S lg n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 18 | 18 | ArraySeqMtPer.rs | filter_dc | none | W n, S n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 19 | 18 | ArraySeqMtPer.rs | filter_par | none | W n, S lg n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 20 | 18 | ArraySeqMtPer.rs | map_inner | none | W n, S n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 21 | 18 | ArraySeqMtPer.rs | filter_inner | none | W n, S n | W n lg n+Σ, S n+maxS | no cost; old wrong [6] |
| 22 | 18 | ArraySeqMtPer.rs | reduce_inner | none | W n, S lg n | W n lg n+Σ, S n+lg n·maxS | no cost; old wrong [6] |
| 23 | 18 | ArraySeqMtPer.rs | tabulate_inner | none | W n, S n | W n lg n+Σ, S n+maxS | no cost; old wrong [10] |

`reduce_par` (line 1289) is malformed and was not given a line (section 5).

### 2e. ArraySeqMtEphSlice.rs (Arc<Vec> with O(1) slices)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 18 | ArraySeqMtEphSlice.rs | length (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 18 | ArraySeqMtEphSlice.rs | nth_cloned (T,I) | none (nth) | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 18 | ArraySeqMtEphSlice.rs | slice (T,I) | none (subseq) | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 18 | ArraySeqMtEphSlice.rs | from_vec (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 18 | ArraySeqMtEphSlice.rs | empty, singleton (T,I) | 20.2: 1,1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 6 | 18 | ArraySeqMtEphSlice.rs | new (T,I) | none | W n, S n | W L, S L | no textbook cost |
| 7 | 18 | ArraySeqMtEphSlice.rs | to_vec (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 18 | ArraySeqMtEphSlice.rs | reduce (T) | none (20.4) | W n, S lg n | W 1+Σ, S lg n·maxS | matches textbook |
| 9 | 18 | ArraySeqMtEphSlice.rs | map (T) | none (20.2) | W n, S n | W n lg n+Σ, S n+maxS | not textbook; old wrong [11] |
| 10 | 18 | ArraySeqMtEphSlice.rs | filter (T) | none (20.2) | W n, S n | W n lg n+Σ, S n+maxS | not textbook; old wrong [11] |
| 11 | 18 | ArraySeqMtEphSlice.rs | tabulate (T) | 20.2 | W n·W(f), S lg n·S(f) | W n lg n+Σ, S n+maxS | not textbook; old wrong [11] |
| 12 | 18 | ArraySeqMtEphSlice.rs | scan (T) | 20.5 | W n lg n, S n | W n lg n, S n | not textbook: seq rejoin |
| 13 | 18 | ArraySeqMtEphSlice.rs | is_empty, is_singleton | none (20.2) | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 18 | ArraySeqMtEphSlice.rs | append (T,I) | none (20.2) | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 15 | 18 | ArraySeqMtEphSlice.rs | update (T,I) | none (20.2) | W n, S n | W n, S n | not textbook: to_vec copy |
| 16 | 18 | ArraySeqMtEphSlice.rs | inject (T,I) | none (20.2) | W n+m, S n+m | W n+m, S n+m | not textbook: seq loops |
| 17 | 18 | ArraySeqMtEphSlice.rs | ninject (T,I) | none (20.2) | W n+m, S n+m | W n+m, S n+m | not textbook: calls inject |
| 18 | 18 | ArraySeqMtEphSlice.rs | flatten | 20.2 | W Σ\|ai\|, S lg²n+max | W n+lg n·Σ\|ai\|, S Σ\|ai\| | not textbook; old wrong [12] |

"none (20.x)" means the file has no APAS line, but the textbook states a cost
for the operation; the verdict compares against that textbook cost.

### 2f. LinkedListStEph.rs and LinkedListStPer.rs

Both "linked lists" are `Vec`-backed. The executable bodies are identical
apart from `set` (StEph only); every verdict holds in both files.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 18 | LinkedListStEph/StPer.rs | new (T,I) | none | W n, S n | W L, S L | no textbook cost |
| 2 | 18 | LinkedListStEph.rs | set (T,I) | none | T: index, index; I: 1,1 | W 1, S 1 | no cost; T old wrong [13] |
| 3 | 18 | LinkedListStEph/StPer.rs | length (T,I) | none (20.7) | W 1, S 1 | W 1, S 1 | matches textbook |
| 4 | 18 | LinkedListStEph/StPer.rs | nth (T,I) | 20.7: i,i; 22.2 | W 1, S 1 | W 1, S 1 | not textbook: Vec index |
| 5 | 18 | LinkedListStEph/StPer.rs | subseq_copy (T,I) | 20.7: i,i | W j, S j | W L, S L | not textbook: Vec copy |
| 6 | 18 | LinkedListStEph/StPer.rs | from_vec (T,I) | none | T: n, n; I: 1,1 | W 1, S 1 | no cost; T old wrong [4] |
| 7 | 18 | LinkedListStEph/StPer.rs | empty, singleton (T,I) | none (20.7) | W 1, S 1 | W 1, S 1 | matches textbook |
| 8 | 18 | LinkedListStEph/StPer.rs | tabulate, map, filter | 20.7 | W n, S n | W n+Σ, S n+Σ | matches textbook |
| 9 | 18 | LinkedListStEph/StPer.rs | append (T,I) | 20.7: n, n | W n+m, S n+m | W n+m, S n+m | not textbook: copies b |
| 10 | 18 | LinkedListStEph/StPer.rs | flatten (T,I) | none (20.7) | W Σ\|ai\|, same S | W n+Σ\|ai\|, same S | matches textbook |
| 11 | 18 | LinkedListStEph/StPer.rs | update (T,I) | 22.2: 1,1 | W n, S n | W n, S n | not textbook [14] |
| 12 | 18 | LinkedListStEph/StPer.rs | is_empty, is_singleton | none (20.7) | W 1, S 1 | W 1, S 1 | matches textbook |
| 13 | 18 | LinkedListStEph/StPer.rs | iterate, reduce (T,I) | 20.7 | W n, S n | W 1+Σ, S 1+Σ | matches textbook |
| 14 | 18 | LinkedListStEph/StPer.rs | scan (T,I) | 20.7: n, n | W n, S n | W n, S n | matches textbook |
| 15 | 18 | LinkedListStEph/StPer.rs | iter, into_iter (x2) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 16 | 18 | LinkedListStEph/StPer.rs | clone | none | W n, S 1 | W n, S n | no cost; old wrong [15] |
| 17 | 18 | LinkedListStEph/StPer.rs | eq | none | W n, S 1 | W n, S n | no cost; old wrong [15] |

### Footnotes

1. `new` calls `std::vec::from_elem`, a sequential clone fill: Span O(L),
   not O(1) (ArraySeq trait) or O(lg L) (MtEph/MtPer traits).
2. `subseq_copy` and `scan_inclusive` are sequential loops; the trait lines
   claimed Span O(1) (ArraySeq) or O(lg L) (MtEph/MtPer).
3. `Vec::remove` / `Vec::insert` shift the tail sequentially: Span O(n),
   not O(1).
4. `from_vec` moves the `Vec` into the struct: O(1). The trait lines claimed
   O(n) worst case.
5. `collect` impl line said O(n); each pair does a linear `find_key` over
   the groups plus `Vec::remove`/`Vec::insert`, O(|a|²) total, as the trait
   line says.
6. Every Mt divide-and-conquer helper (`map_dc`, `filter_dc`, `reduce_dc`,
   `map_par`, `filter_par`, `reduce_par`, `map_inner`, `filter_inner`,
   `reduce_inner`) splits with two sequential `subseq_copy` calls, O(n) at
   each level, and (except reduce) rejoins with the sequential `append`,
   another O(n). The recurrences W(n) = 2W(n/2) + O(n) and
   S(n) = S(n/2) + O(n) give Work O(n lg n) and Span O(n). The old lines
   claimed O(n) work and O(lg n) span.
7. The trait `ninject` delegates to the sequential `inject`; the parallel
   `ninject_par` exists but the trait does not use it.
8. `ninject_par` clones `a` twice sequentially (O(n) each), partitions the
   updates sequentially, and the two writers serialize on one write lock:
   Span O(n + m), not O(m).
9. MtPer `reduce`, `map`, and `tabulate` are sequential loops in an Mt
   module; the parallel `*_inner` versions exist but are not called.
10. `tabulate_inner` has no split copy but rejoins with sequential `append`
    at every level: Work O(n lg n), Span O(n).
11. The Slice helpers split in O(1) (`slice`) but rejoin by pushing the right
    result into the left vector sequentially: W(n) = 2W(n/2) + O(n),
    S(n) = S(n/2) + O(n). The old `tabulate` line also wrote span as a
    product, lg n · S(f), where the fork-join bound is additive.
12. `flatten_dc_vec` rejoins by sequential push, so the top level alone
    copies about half the output: Span O(Σ|a[i]|), Work O(|a| + lg|a| · Σ|a[i]|).
13. `LinkedListStEph::set` trait line claimed O(index) list traversal; the
    body is `Vec::set`, O(1).
14. The only APAS line on list `update` cites CS 22.2 (O(1)); the code is a
    persistent copy, O(|a|), which equals the CS 20.7 list cost.
15. `Vec::clone` and `Vec` equality touch every element sequentially:
    Span O(n), not O(1).

## 3. Counts (per annotation site)

348 new lines were added: one per annotated function site (trait and impl
counted separately), in 8 files. `ArraySeqSpecsAndLemmas.rs` has no
annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 133 |
| 2 | does not match textbook | 133 |
| 3 | does not match old analysis | 48 |
| 4 | no textbook cost | 82 |
| 5 | unannotated functions | 61 |
| 6 | malformed annotations | 2 |

Rows 1, 2, and 4 partition the 348 lines; row 3 overlaps them (48 lines
carry both a textbook verdict and "does not match old analysis").

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 18 | ArraySeq.rs | 49 | 14 | 21 | 7 | 14 |
| 2 | 18 | ArraySeqStEph.rs | 42 | 14 | 20 | 1 | 8 |
| 3 | 18 | ArraySeqStPer.rs | 40 | 14 | 20 | 1 | 6 |
| 4 | 18 | ArraySeqMtEph.rs | 52 | 14 | 23 | 16 | 15 |
| 5 | 18 | ArraySeqMtPer.rs | 47 | 14 | 20 | 12 | 13 |
| 6 | 18 | ArraySeqMtEphSlice.rs | 34 | 15 | 13 | 4 | 6 |
| 7 | 18 | LinkedListStEph.rs | 43 | 24 | 8 | 4 | 11 |
| 8 | 18 | LinkedListStPer.rs | 41 | 24 | 8 | 3 | 9 |

## 4. Unannotated functions (61)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 18 | ArraySeq.rs | iter, iter_mut, into_iter x3, clone, eq, fmt x2 |
| 2 | 18 | ArraySeqStEph.rs | iter, into_iter x2, clone, eq, fmt x2 |
| 3 | 18 | ArraySeqStPer.rs | iter, into_iter x2, clone, eq, fmt x2 |
| 4 | 18 | ArraySeqMtEph.rs | iter, into_iter x2, clone, eq, fmt x4 |
| 5 | 18 | ArraySeqMtPer.rs | iter, into_iter x2, clone, eq, fmt x2 |
| 6 | 18 | ArraySeqMtEphSlice.rs | iter x2, into_iter, clone, eq, fmt x2 |
| 7 | 18 | ArraySeqMtEphSlice.rs | impl reduce, map, filter, tabulate, scan |
| 8 | 18 | ArraySeqMtEphSlice.rs | reduce_dc, map_dc_vec, filter_dc_vec |
| 9 | 18 | ArraySeqMtEphSlice.rs | tabulate_dc_vec, scan_dc_vec, flatten_dc_vec |
| 10 | 18 | LinkedListStEph.rs | fmt x2 |
| 11 | 18 | LinkedListStPer.rs | fmt x2 |

The Slice D&C helpers (rows 8-9) carry the algorithmic cost of the Slice
file and have no annotation of their own; their costs are stated in the
trait lines of `reduce`, `map`, `filter`, `tabulate`, `scan`, and `flatten`.

## 5. Malformed annotations (2)

| # | Chap | File | Line | Function | Problem |
|---|---|---|---|---|---|
| 1 | 18 | ArraySeq.rs | 1192 | remove (impl) | glued onto `// Veracity: UNNEEDED` |
| 2 | 18 | ArraySeqMtPer.rs | 1288 | reduce_par | glued onto `// Veracity: UNNEEDED` |

Both are `//` comments, not doc comments, so no review line was added.
Review results for the record: impl `remove` is Work O(|a|), Span O(|a|)
(`Vec::remove`), which matches its old text; `reduce_par` is
Work O(|a| lg |a| + Σ W(f)), Span O(|a| + lg |a| · max S(f)) (footnote 6),
which does not match its old text (Work O(|a|), Span O(log |a|)).
A minor formatting oddity: the `iterate_prefixes` annotation in `ArraySeq.rs`
is indented eight spaces while its doc block is indented four; it parses,
and the new line follows its indentation.

## 6. Notable findings

1. **The Mt "parallel" D&C functions are not work-efficient and have linear
   span.** In `ArraySeqMtEph.rs` and `ArraySeqMtPer.rs`, every D&C helper
   splits its input with two sequential `subseq_copy` calls and rejoins with
   the sequential `append`. That makes Work O(n lg n) and Span O(n) for map,
   filter, and tabulate, and Work O(n lg n), Span O(n + lg n · max S(f)) for
   reduce. The old lines claimed O(n) work and O(lg n) span for 12 of these
   sites. The fork-join structure is right; the Vec copies at each level
   remove the parallel benefit. `ArraySeqMtEphSlice.rs` fixes the split
   (O(1) `slice`), so its `reduce` matches CS 20.4, but its map, filter,
   tabulate, scan, and flatten still rejoin by sequential push and remain
   Work O(n lg n), Span O(n).
2. **MtPer trait methods are sequential.** `ArraySeqMtPer.rs` `reduce`,
   `map`, `tabulate`, and `scan` are sequential loops; the parallel
   `*_inner` versions exist but the trait does not call them. Likewise the
   MtEph trait `ninject` calls the sequential `inject` instead of
   `ninject_par`, and `ninject_par` itself has Span O(n + m) because of two
   sequential Vec clones and a single write lock that serializes both
   writers.
3. **The "linked lists" are Vec-backed and misnamed.** `LinkedListStEph.rs`
   and `LinkedListStPer.rs` store a `Vec`, so `nth` is O(1) rather than the
   list cost O(i), `append` copies b (O(|a| + |b|) rather than O(|a|)), and
   `subseq_copy` costs O(length) rather than the shared-suffix O(1 + i).
   Several old lines on these files reasoned as if the structure were a list
   (`set` claimed O(index)), and `clone`/`eq` claimed Span O(1) for
   sequential O(n) element loops.
4. Every array-sequence file makes `subseq`, `append`, `update`, `inject`,
   `flatten`, and `scan` sequential copies; these are the Vec-backed category
   of the notation standard, stated consistently by the old lines.
5. Nine old trait lines claimed a smaller span than their own impl line in
   the same file (`new`, `subseq_copy`, `scan_inclusive`, `remove`,
   `insert` in `ArraySeq.rs`; `new`, `subseq_copy` in MtEph/MtPer), and four
   trait lines claimed `from_vec` is O(n) worst case when it is a move.
