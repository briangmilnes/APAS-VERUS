# r228 Alg Analysis Review: Chap05

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap05.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Def 5.1 union, intersection | none |
| 2 | Def 5.2 Cartesian product | none |
| 3 | Def 5.3 set partition | none |
| 4 | Def 5.4 Kleene star and plus | none |
| 5 | Def 5.5 relation, domain, range | none |
| 6 | Def 5.6 function (mapping) | none |
| 7 | Ex 5.1 Kleene closure under concat | proof exercise, no cost |

The chapter is a review of definitions and states no cost for any operation.
The existing `APAS (Ch05 Def 5.x)` lines in `SetStEph.rs`, `SetMtEph.rs`,
`RelationStEph.rs`, and `MappingStEph.rs` carry costs (for example "Work
O(|a| + |b|), Span O(1)" for union) that do not appear in the prose. Per the
plan, where an APAS line exists this review compares against it; where it
does not, the verdict is "no textbook cost". The main finding for the chapter
is that those APAS lines are not sourced from the text.

## 2. Reviewed functions

Each function has an annotation on its trait declaration (T) and on its impl
(I); both sites received a line and both are counted. Where the two sites
carry the same verdict they share a row. W = Work, S = Span.

### KleeneStPer.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 05 | KleeneStPer.rs | new (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 2 | 05 | KleeneStPer.rs | mem_star (T,I) | none | W s, S s | W s, S s | no textbook cost |
| 3 | 05 | KleeneStPer.rs | mem_plus (T,I) | none | W s, S s | W s, S s | no textbook cost |
| 4 | 05 | KleeneStPer.rs | alphabet (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |

### SetStEph.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 5 | 05 | SetStEph.rs | from_vec (T,I) | W v, S 1 | W v, S v | W v, S v | not tb: sequential |
| 6 | 05 | SetStEph.rs | iter (T) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 7 | 05 | SetStEph.rs | to_seq (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 05 | SetStEph.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 9 | 05 | SetStEph.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 10 | 05 | SetStEph.rs | size (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 11 | 05 | SetStEph.rs | mem (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |
| 12 | 05 | SetStEph.rs | insert (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp am, S 1 | matches textbook |
| 13 | 05 | SetStEph.rs | union (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 14 | 05 | SetStEph.rs | disjoint_union (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential |
| 15 | 05 | SetStEph.rs | intersection (T,I) | W a+b, S 1 | W a, S a | W a, S a | not tb: sequential [1] |
| 16 | 05 | SetStEph.rs | elt_cross_set (T,I) | none | W b, S b | W b, S b | no textbook cost |
| 17 | 05 | SetStEph.rs | cartesian_product (T,I) | W ab, S 1 | W ab, S ab | W a^2 b, S a^2 b | not tb; not old [2] |
| 18 | 05 | SetStEph.rs | all_nonempty (T,I) | none | W p, S p | W p, S p | no textbook cost |
| 19 | 05 | SetStEph.rs | partition_on_elt (T,I) | none | W p, S p | W p, S p | no textbook cost |
| 20 | 05 | SetStEph.rs | partition (T,I) | W a p, S 1 | W a p, S a p | W a p, S a p | not tb: sequential |
| 21 | 05 | SetStEph.rs | split (T,I) | W n, S 1 | W n, S n | W n, S n | not tb: sequential |
| 22 | 05 | SetStEph.rs | choose (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |

### RelationStEph.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 23 | 05 | RelationStEph.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 24 | 05 | RelationStEph.rs | from_set (T) | W r, S 1 | W r, S r | W 1, S 1 | not tb; not old [3] |
| 25 | 05 | RelationStEph.rs | from_set (I) | W r, S 1 | W 1, S 1 | W 1, S 1 | not tb: move [3] |
| 26 | 05 | RelationStEph.rs | from_vec (T,I) | W r, S 1 | W r, S r | W v, S v | not tb: sequential |
| 27 | 05 | RelationStEph.rs | size (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 28 | 05 | RelationStEph.rs | domain (T,I) | W R, S 1 | W R, S R | W R, S R | not tb: sequential |
| 29 | 05 | RelationStEph.rs | range (T,I) | W R, S 1 | W R, S R | W R, S R | not tb: sequential |
| 30 | 05 | RelationStEph.rs | mem (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |
| 31 | 05 | RelationStEph.rs | relates (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |
| 32 | 05 | RelationStEph.rs | iter (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### MappingStEph.rs

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 33 | 05 | MappingStEph.rs | is_functional_vec (T,I) | no cost | W v^2, S v^2 | W v^2, S v^2 | no textbook cost |
| 34 | 05 | MappingStEph.rs | is_functional_vec_at (T,I) | no cost | W v, S v | W v, S v | no textbook cost |
| 35 | 05 | MappingStEph.rs | is_functional_SetStEph_at (T,I) | no cost | W s, S s | W s, S s | no textbook cost |
| 36 | 05 | MappingStEph.rs | is_functional_SetStEph (T,I) | no cost | W s^2, S s^2 | W s^2, S s^2 | no textbook cost |
| 37 | 05 | MappingStEph.rs | is_functional_RelationStEph (T,I) | no cost | W r^2, S r^2 | W r^2, S r^2 | no textbook cost |
| 38 | 05 | MappingStEph.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 39 | 05 | MappingStEph.rs | from_vec (T,I) | W v, S 1 | W v, S v | W v, S v | not tb: sequential |
| 40 | 05 | MappingStEph.rs | from_relation (T,I) | W r, S 1 | W r, S r | W r, S r | not tb: sequential clone |
| 41 | 05 | MappingStEph.rs | size (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 42 | 05 | MappingStEph.rs | domain (T,I) | W m, S 1 | W m, S m | W m, S m | not tb: sequential |
| 43 | 05 | MappingStEph.rs | range (T,I) | W m, S 1 | W m, S m | W m, S m | not tb: sequential |
| 44 | 05 | MappingStEph.rs | mem (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |
| 45 | 05 | MappingStEph.rs | iter (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### SetMtEph.rs

The `SetMtEph` impl bodies are line-for-line the `SetStEph` bodies (same
sequential loops) except `cartesian_product`.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 46 | 05 | SetMtEph.rs | from_vec (T,I) | W v, S 1 | W v, S v | W v, S v | not tb: sequential in Mt |
| 47 | 05 | SetMtEph.rs | iter (T) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 48 | 05 | SetMtEph.rs | to_seq (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 49 | 05 | SetMtEph.rs | empty (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 50 | 05 | SetMtEph.rs | singleton (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 51 | 05 | SetMtEph.rs | size (T,I) | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 52 | 05 | SetMtEph.rs | mem (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |
| 53 | 05 | SetMtEph.rs | insert (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp am, S 1 | matches textbook |
| 54 | 05 | SetMtEph.rs | union (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential in Mt |
| 55 | 05 | SetMtEph.rs | disjoint_union (T,I) | W a+b, S 1 | W a+b, S a+b | W a+b, S a+b | not tb: sequential in Mt |
| 56 | 05 | SetMtEph.rs | intersection (T,I) | W a+b, S 1 | W a, S a | W a, S a | not tb: sequential [1] |
| 57 | 05 | SetMtEph.rs | elt_cross_set (T,I) | none | W b, S b | W b, S b | no textbook cost |
| 58 | 05 | SetMtEph.rs | cartesian_product (T,I) | W ab, S b | W ab, S ab | W a^2 b, S a^2 b | not tb; not old [4] |
| 59 | 05 | SetMtEph.rs | all_nonempty (T,I) | none | W p, S p | W p, S p | no textbook cost |
| 60 | 05 | SetMtEph.rs | partition_on_elt (T,I) | none | W p, S p | W p, S p | no textbook cost |
| 61 | 05 | SetMtEph.rs | partition (T,I) | W a p, S 1 | W a p, S a p | W a p, S a p | not tb: sequential in Mt |
| 62 | 05 | SetMtEph.rs | split (T,I) | W n, S 1 | W n, S n | W n, S n | not tb: sequential in Mt |
| 63 | 05 | SetMtEph.rs | choose (T,I) | W 1, S 1 | W 1, S 1 | W 1 exp, S 1 | matches textbook |
| 64 | 05 | SetMtEph.rs | Locked empty (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 65 | 05 | SetMtEph.rs | Locked size (T,I) | none | W 1, S 1 | W 1, S 1 + lock | no textbook cost |
| 66 | 05 | SetMtEph.rs | Locked mem (T,I) | none | W 1, S 1 | W 1 exp, S 1 + lock | no textbook cost |
| 67 | 05 | SetMtEph.rs | Locked insert (T,I) | none | W 1, S 1 | W 1 exp am, S 1 + lock | no textbook cost |
| 68 | 05 | SetMtEph.rs | Locked choose (T,I) | none | W 1, S 1 | W 1 exp, S 1 + lock | no textbook cost |

Footnotes:

1. `intersection` iterates only `self` and probes `s2` in O(1) expected
   time, so its work O(|a|) is below the APAS line's O(|a| + |b|); the span
   is sequential.
2. `SetStEph::cartesian_product` calls `product = product.union(&a_cross)`
   once per element of `self`. `union` clones its receiver, so step i copies
   i x |b| pairs; the total is O(|a|^2 x |b|). The old analysis counted only
   the O(|b|) insert work per step.
3. `RelationStEph::from_set` moves its argument into the struct, O(1). The
   trait-level old line said O(|pairs|); the impl-level old line already said
   O(1). The APAS line's O(|pairs|) is not from the prose.
4. `SetMtEph::cartesian_product` spawns one task per element of `self`, but
   (a) the spawn loop clones `s2` in the parent thread before each spawn,
   which is O(|a| x |b|) sequential work and span, and (b) the join loop calls
   `product.disjoint_union(&thread_result)`, and `disjoint_union` rebuilds a
   fresh hash set from both inputs, so step i costs O(i x |b|). Work and span
   are both O(|a|^2 x |b|); the APAS line's Span O(|b|) needs a parallel
   union or a flatten of the per-element results.

## 3. Counts (per annotation site)

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 44 |
| 2 | does not match textbook | 44 |
| 3 | does not match old analysis | 5 |
| 4 | no textbook cost | 44 |
| 5 | unannotated functions | 31 |
| 6 | malformed annotations | 0 |

Per file: KleeneStPer 8 sites (8 no tb); SetStEph 35 (13 match, 14 not tb,
8 no tb, 2 not old); RelationStEph 18 (10 match, 8 not tb, 1 not old);
MappingStEph 26 (8 match, 8 not tb, 10 no tb); SetMtEph 45 (13 match, 14 not
tb, 18 no tb, 2 not old).

Unannotated exec functions:

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 05 | KleeneStPer.rs | Debug::fmt, Display::fmt |
| 2 | 05 | SetStEph.rs | iter (impl), into_iter, clone, hash, eq, 2 fmt |
| 3 | 05 | RelationStEph.rs | into_iter, clone, hash, eq, 2 fmt |
| 4 | 05 | MappingStEph.rs | into_iter, clone, hash, eq, 2 fmt |
| 5 | 05 | SetMtEph.rs | iter (impl), into_iter, clone, hash, eq, 5 fmt |

Malformed: none. One placement note: in `MappingStEph.rs` the impl of
`is_functional_vec_at` has `#[verifier::loop_isolation(false)]` above its
doc comment rather than below it; the annotation itself is well formed.

## 4. Notable findings

- Cost regression: both `cartesian_product` implementations are
  O(|a|^2 x |b|), not O(|a| x |b|), because each step copies the whole
  accumulated product (`union` clones its receiver; `disjoint_union` rebuilds
  from both inputs). The old analyses missed this.
- Missing parallelism: `SetMtEph.rs` is an Mt module, but every bulk
  operation except `cartesian_product` is the St sequential loop, so its span
  equals its work. In `cartesian_product` the parallel phase is dominated by
  the sequential `s2` clones and the sequential join.
- The `APAS (Ch05 Def 5.x)` cost lines state costs the chapter does not
  give; they are the reviewer's own targets, not textbook costs.
- The old trait-level annotation of `RelationStEph::from_set` (O(|pairs|))
  disagreed with its own impl-level annotation (O(1)); the impl is a move.
