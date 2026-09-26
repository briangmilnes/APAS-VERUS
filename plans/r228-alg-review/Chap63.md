# r228 Alg Analysis Review: Chap63

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap63.txt` (Graph connectivity) gives the algorithms and leaves
their costs as exercises.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 63.2, Ex 63.3 | countComponents by star contraction | (n + m) lg n | lg² n |
| 2 | Alg 63.3, Ex 63.4 | connectedComponents (expand = C ∘ P) | (n + m) lg n | lg² n |

The exercise answers follow from Thm 62.3 (Chap62): the count's `base` and
`expand` are O(1); the components' expand composes two maps, O(n) work and
O(lg n) span per round. The St files' APAS lines give span O((n + m) lg n)
(sequential).

Notation: n = |V|, m = |E|, c = centers per round, |P| = size of the
partition map. Callee costs come from this batch's Chap62 review:

- St `star_contract`: O(n² m) worst case (greedy partition, up to n rounds
  of O(n + c m)).
- Mt `star_contract_mt`: Work O((n + m) lg² n), Span O(n lg n + m lg² n),
  both expected.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 63 | ConnectivityStEph.rs | count_components (T,I) | (n+m)lg n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [1] |
| 2 | 63 | ConnectivityStEph.rs | connected_components (T,I) | (n+m)lg n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [1] |
| 3 | 63 | ConnectivityStEph.rs | count_components_hof (T,I) | (n+m)lg n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [1] |
| 4 | 63 | ConnectivityStEph.rs | connected_components_hof (T,I) | (n+m)lg n | W (n+m) lg n | W n² m, S n² m | not textbook; old wrong [1] |
| 5 | 63 | ConnectivityStEph.rs | build_quotient_edges | none | W m, S m | W m, S m | no textbook cost |
| 6 | 63 | ConnectivityMtEph.rs | count_components_mt (T,I) | (n+m)lg n, lg² n | S lg² n (T), m (I) | [2] | not textbook; old wrong [2] |
| 7 | 63 | ConnectivityMtEph.rs | connected_components_mt (T,I) | (n+m)lg n, lg² n | S lg² n (T), n lg n (I) | [2] | not textbook; old wrong [2] |
| 8 | 63 | ConnectivityMtEph.rs | count_components_hof (T,I) | (n+m)lg n, lg² n | S lg² n (T), m (I) | [2] | not textbook; old wrong [2] |
| 9 | 63 | ConnectivityMtEph.rs | connected_components_hof (T,I) | (n+m)lg n, lg² n | S lg² n (T), n lg n (I) | [2] | not textbook; old wrong [2] |
| 10 | 63 | ConnectivityMtEph.rs | compose_maps_parallel | none | W \|P\|, S \|P\| | W \|P\|, S \|P\| | no textbook cost |

### Footnotes

1. The St functions are `star_contract` with `base`/`expand` closures
   (`count_components` and `connected_components` delegate to the `_hof`
   versions). The count's closures are O(1); the components' `base` builds
   an identity map, O(n), and its `expand` composes P with C in a
   sequential loop, O(n) per round. The cost is set by `star_contract`:
   O(n² m) worst case.
2. The Mt functions delegate to `star_contract_mt`: Work O((n + m) lg² n)
   expected, Span O(n lg n + m lg² n) expected. The components' `expand`
   is a sequential O(n) loop per round, which adds O(n lg n) to the span,
   already within the bound. The old impl lines (span O(m) or O(n lg n))
   identified a bottleneck but missed the lg-factor losses in the star
   partition and the sequential quotient build.

## 3. Counts (per annotation site)

18 new lines were added in 2 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 16 |
| 3 | does not match old analysis | 16 |
| 4 | no textbook cost | 2 |
| 5 | unannotated functions | 2 |
| 6 | malformed annotations | 0 |

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 63 | ConnectivityStEph.rs | 9 | 0 | 8 | 8 | 1 |
| 2 | 63 | ConnectivityMtEph.rs | 9 | 0 | 8 | 8 | 1 |

## 4. Unannotated functions (2)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 63 | ConnectivityStEph.rs | fmt (Display) |
| 2 | 63 | ConnectivityMtEph.rs | fmt (Display) |

The `base` and `expand` closures inside the `_hof` functions are not
separate functions and were not counted.

## 5. Malformed annotations (0)

None.

## 6. Notable findings

1. **Connectivity inherits Chap62's contraction costs.** Every Chap63
   entry point is `star_contract` or `star_contract_mt` with small
   closures, so the St versions are O(n² m) worst case and the Mt versions
   O((n + m) lg² n) work with near-linear span, not the textbook's
   O((n + m) lg n) work and O(lg² n) span.
2. **The components' expand step is sequential.** Both St and Mt compose
   the partition map with the component map in a sequential loop; the
   textbook's C ∘ P is a parallel map with O(lg n) span.
3. `compose_maps_parallel` is sequential, as its old line says, and is not
   called by the `_hof` functions, which inline their own composition.
