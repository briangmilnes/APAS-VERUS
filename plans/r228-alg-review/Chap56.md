# r228 Alg Analysis Review: Chap56

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap56.txt` (Shortest Paths: introduction).

| # | Source | Content | Work | Span |
|---|---|---|---|---|
| 1 | Def 56.1 | Path weight (sum of edge weights) | none given | none given |
| 2 | Def 56.2 | Shortest path, distance δ | none given | none given |
| 3 | Problems 56.1–56.4 | SPSP, SSSP, SSSP+, APSP statements | none given | none given |
| 4 | Sub-paths property | Sub-path of a shortest path is shortest | none given | none given |

Chap56 states definitions and problems only; the algorithms and their
costs (Dijkstra, Bellman-Ford, Johnson) are in Chap57–59. Every Chap56
function is therefore reviewed as "no textbook cost".

Notation: n = number of vertices stored in the result (|V| in the impl
annotations); k = |path|. All files are St, so Span = Work.

## 2. Cost base

- `Vec` index read / in-place set, `ArraySeqStEph::nth`/`set`: O(1).
- `ArraySeqStEph::new`, `ArraySeqStPer::tabulate`: O(n) sequential.
- `Vec::clone` of a row of length n: O(n).
- `extract_path`: predecessor walk bounded by n steps (loop guard), then an
  O(n) reversal.

## 3. Reviewed functions

Each row merges the sites of one function in one file (trait declaration
and impl body); the count in parentheses is the number of review lines
added for that row. Total sites: 133.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 56 | AllPairsResultStEphF64.rs | new (×2) | — | W O(n^2), S O(n^2) [1] | W O(n^2), S O(n^2) | no textbook cost |
| 2 | 56 | AllPairsResultStEphF64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 3 | 56 | AllPairsResultStEphF64.rs | set_distance (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 4 | 56 | AllPairsResultStEphF64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 5 | 56 | AllPairsResultStEphF64.rs | set_predecessor (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 6 | 56 | AllPairsResultStEphF64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 7 | 56 | AllPairsResultStEphF64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 8 | 56 | AllPairsResultStEphI64.rs | new (×2) | — | W O(n^2), S O(n^2) [1] | W O(n^2), S O(n^2) | no textbook cost |
| 9 | 56 | AllPairsResultStEphI64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 10 | 56 | AllPairsResultStEphI64.rs | set_distance (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 11 | 56 | AllPairsResultStEphI64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 12 | 56 | AllPairsResultStEphI64.rs | set_predecessor (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 13 | 56 | AllPairsResultStEphI64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 14 | 56 | AllPairsResultStEphI64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 15 | 56 | AllPairsResultStPerF64.rs | new (×2) | — | W O(n^2), S O(n^2) [1] | W O(n^2), S O(n^2) | no textbook cost |
| 16 | 56 | AllPairsResultStPerF64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 17 | 56 | AllPairsResultStPerF64.rs | set_distance (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 18 | 56 | AllPairsResultStPerF64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 19 | 56 | AllPairsResultStPerF64.rs | set_predecessor (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 20 | 56 | AllPairsResultStPerF64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 21 | 56 | AllPairsResultStPerF64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 22 | 56 | AllPairsResultStPerI64.rs | new (×2) | — | W O(n^2), S O(n^2) [1] | W O(n^2), S O(n^2) | no textbook cost |
| 23 | 56 | AllPairsResultStPerI64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 24 | 56 | AllPairsResultStPerI64.rs | set_distance (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 25 | 56 | AllPairsResultStPerI64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 26 | 56 | AllPairsResultStPerI64.rs | set_predecessor (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 27 | 56 | AllPairsResultStPerI64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 28 | 56 | AllPairsResultStPerI64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 29 | 56 | Example56_1.rs | example_path_weight_int (×1) | — | W O(1), S O(1) | W O(1), S O(1) | no textbook cost |
| 30 | 56 | Example56_1.rs | example_path_weight_i64 (×1) | — | W O(1), S O(1) | W O(1), S O(1) | no textbook cost |
| 31 | 56 | Example56_1.rs | example_negative_weights (×1) | — | W O(1), S O(1) | W O(1), S O(1) | no textbook cost |
| 32 | 56 | Example56_3.rs | example_negative_cycle (×1) | — | W O(1), S O(1) | W O(1), S O(1) | no textbook cost |
| 33 | 56 | Example56_3.rs | example_undefined_shortest_path (×1) | — | W O(1), S O(1) | W O(1), S O(1) | no textbook cost |
| 34 | 56 | PathWeightUtilsStEph.rs | path_weight_int (×2) | Def 56.1 [2] | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 35 | 56 | PathWeightUtilsStEph.rs | path_weight_float (×2) | Def 56.1 [2] | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 36 | 56 | PathWeightUtilsStEph.rs | validate_subpath_property_int (×2) | — | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 37 | 56 | PathWeightUtilsStEph.rs | validate_subpath_property_float (×2) | — | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 38 | 56 | PathWeightUtilsStPer.rs | path_weight_int (×2) | Def 56.1 [2] | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 39 | 56 | PathWeightUtilsStPer.rs | path_weight_float (×2) | Def 56.1 [2] | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 40 | 56 | PathWeightUtilsStPer.rs | validate_subpath_property_int (×2) | — | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 41 | 56 | PathWeightUtilsStPer.rs | validate_subpath_property_float (×2) | — | W O(k), S O(k) | W O(k), S O(k) | no textbook cost |
| 42 | 56 | SSSPResultStEphF64.rs | new (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 43 | 56 | SSSPResultStEphF64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 44 | 56 | SSSPResultStEphF64.rs | set_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 45 | 56 | SSSPResultStEphF64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 46 | 56 | SSSPResultStEphF64.rs | set_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 47 | 56 | SSSPResultStEphF64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 48 | 56 | SSSPResultStEphF64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 49 | 56 | SSSPResultStEphI64.rs | new (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 50 | 56 | SSSPResultStEphI64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 51 | 56 | SSSPResultStEphI64.rs | set_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 52 | 56 | SSSPResultStEphI64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 53 | 56 | SSSPResultStEphI64.rs | set_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 54 | 56 | SSSPResultStEphI64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 55 | 56 | SSSPResultStEphI64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 56 | 56 | SSSPResultStPerF64.rs | new (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 57 | 56 | SSSPResultStPerF64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 58 | 56 | SSSPResultStPerF64.rs | set_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 59 | 56 | SSSPResultStPerF64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 60 | 56 | SSSPResultStPerF64.rs | set_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 61 | 56 | SSSPResultStPerF64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 62 | 56 | SSSPResultStPerF64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 63 | 56 | SSSPResultStPerI64.rs | new (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |
| 64 | 56 | SSSPResultStPerI64.rs | get_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 65 | 56 | SSSPResultStPerI64.rs | set_distance (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 66 | 56 | SSSPResultStPerI64.rs | get_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 67 | 56 | SSSPResultStPerI64.rs | set_predecessor (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 68 | 56 | SSSPResultStPerI64.rs | is_reachable (×2) | — | W O(1), S O(1) [1] | W O(1), S O(1) | no textbook cost |
| 69 | 56 | SSSPResultStPerI64.rs | extract_path (×2) | — | W O(n), S O(n) [1] | W O(n), S O(n) | no textbook cost |

### Footnotes

[1] On the trait-declaration sites of the SSSPResult* and AllPairsResult*
files the old Code-review line reads only "matches APAS", with no Work or
Span (56 lines). The impl-body sites carry the costs shown, and those
agree with the new review.
[2] The APAS lines on `path_weight_int`/`path_weight_float` cite Def 56.1
("computes path weight") but give no cost (4 lines).

## 4. Counts (per annotation site)

| # | Category | Count |
|---|---|---|
| 1 | Review lines added | 133 |
| 2 | matches textbook | 0 |
| 3 | does not match textbook | 0 |
| 4 | does not match old analysis | 0 |
| 5 | no textbook cost | 133 |
| 6 | Unannotated functions | 22 |
| 7 | Malformed annotations | 0 |

Per file: SSSPResult{StEphI64, StEphF64, StPerI64, StPerF64} 14 each;
AllPairsResult{StEphI64, StEphF64, StPerI64, StPerF64} 14 each;
PathWeightUtils{StEph, StPer} 8 each; Example56_1 3; Example56_3 2.

## 5. Unannotated functions (22)

All are `fmt` bodies of Debug/Display impls outside `verus!`:

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 56 | SSSPResultStEphI64.rs | Debug::fmt, Display::fmt |
| 2 | 56 | SSSPResultStEphF64.rs | Debug::fmt, Display::fmt |
| 3 | 56 | SSSPResultStPerI64.rs | Debug::fmt, Display::fmt |
| 4 | 56 | SSSPResultStPerF64.rs | Debug::fmt, Display::fmt |
| 5 | 56 | AllPairsResultStEphI64.rs | Debug::fmt, Display::fmt |
| 6 | 56 | AllPairsResultStEphF64.rs | Debug::fmt, Display::fmt |
| 7 | 56 | AllPairsResultStPerI64.rs | Debug::fmt, Display::fmt |
| 8 | 56 | AllPairsResultStPerF64.rs | Debug::fmt, Display::fmt |
| 9 | 56 | PathWeightUtilsStEph.rs | Debug::fmt |
| 10 | 56 | PathWeightUtilsStPer.rs | Debug::fmt |
| 11 | 56 | Example56_1.rs | Debug::fmt, Display::fmt |
| 12 | 56 | Example56_3.rs | Debug::fmt, Display::fmt |

## 6. Malformed annotations (0)

No annotation line is malformed in form. Two non-standard patterns are
recorded above: 56 old Code-review lines that say only "matches APAS"
(footnote [1]) and 4 APAS lines citing Def 56.1 without a cost
(footnote [2]).

## 7. Notable findings

1. The old analysis is correct throughout: every SSSP/AllPairs accessor,
   `new`, `extract_path`, and path-weight function has the cost the old
   impl-site line states. No verdict in this chapter is "does not match".
2. `set_distance`/`set_predecessor` in all four AllPairsResult files cost
   O(n): they clone row u, set one cell, and store the row back. An
   in-place update of the row would be O(1), as it is in the SSSPResult
   files (including the StPer ones, which consume `self` and set in place).
3. `SSSPResultStPer*::set_*` are O(1) despite the persistent interface,
   because they take `self` by value and mutate the owned Vec; no copy.
4. Example56_1 and Example56_3 are `#[verifier::external]` println
   demonstrations on fixed graphs (O(1)); CLAUDE.md normally excludes
   Example files, but their existing annotations were reviewed for
   completeness.
