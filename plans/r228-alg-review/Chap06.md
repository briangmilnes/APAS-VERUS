# r228 Algorithmic-Analysis Review: Chap06 (Graph Theory)

Reviewer: Claude Opus 5.5, 2026-09-26. Review only: 512 comment lines added
under `src/Chap06/`, no code, spec, proof, or existing comment changed
(`git diff --numstat`: 512 insertions, 0 deletions; every added line is
`/// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): ...`).

## 1. Textbook cost specifications (`prompts/Chap06.txt`)

Chapter 6 is a definitions chapter. It defines graphs, neighborhoods,
degree, paths, reachability, weighted graphs, subgraphs, connectivity,
partitions, and trees (Definitions 6.1 to 6.25, Exercise 6.1). It states no
Algorithm, no Cost Specification, and no work or span bound for any
operation.

| # | Chap | Item | Topic | Cost stated |
|---|---|---|---|---|
| 1 | 06 | Def 6.1 | Directed graph (V, A) | none |
| 2 | 06 | Def 6.2 | Undirected graph (V, E) | none |
| 3 | 06 | Def 6.3 to 6.6 | Neighbors, N(v), incidence, degree | none |
| 4 | 06 | Def 6.7 to 6.13 | Paths, reachability, cycles, distance | none |
| 5 | 06 | Def 6.15 | n = \|V\|, m = \|E\|; sparse/dense | none |
| 6 | 06 | Def 6.17 | Weighted / edge-labeled graph | none |
| 7 | 06 | Def 6.18 to 6.25 | Subgraphs, partition, trees | none |
| 8 | 06 | Exercise 6.1 | Why cycles need length 3 | none |

Consequently every reviewed function's verdict is `no textbook cost`. The
302 existing `APAS (Ch06 Def 6.x)` lines in this chapter cite definitions
that carry no cost; the costs they state (for example `Span O(1)` for
`ng`, `Span O(log |A|)` for the Mt neighbor functions) are not in the
textbook (finding 1).

Cost model used for the new lines: `SetStEph` is a hash set, so `mem`,
`insert`, `size`, `choose` are expected O(1); `iter` is O(1); `clone`,
`split`, and `union` are sequential O(size) (`src/Chap05/SetStEph.rs`).
`clone_plus` on a graph calls its derived `Clone`, which deep-copies both
the vertex set and the edge set, O(|V| + |E|).

## 2. Per-function review

"decl" is the trait declaration's annotation, "impl" the implementation's.
Costs are written "Work, Span". "no tb cost" = `no textbook cost`;
"≠ old" = `does not match old analysis`.

Footnoted costs:

- [a] Work O(\|S\|·\|A\|·(\|V\|+\|A\|)), Span O((\|V\|+\|A\|)·(lg\|S\|+lg\|A\|)), S = u_set / verts.
- [b] Work O(\|S\|·\|E\|·(\|V\|+\|E\|)), Span O((\|V\|+\|E\|)·(lg\|S\|+lg\|E\|)).
- [c] Work O(\|arcs\|·(\|V\|+\|A\|)), Span O((\|V\|+\|A\|)·lg\|arcs\|).
- [d] Work O(\|edges\|·(\|V\|+\|E\|)), Span O((\|V\|+\|E\|)·lg\|edges\|).
- [e] Work O(\|A\|·(\|V\|+\|A\|)), Span O((\|V\|+\|A\|)·lg\|A\|).
- [f] Work O(\|E\|·(\|V\|+\|E\|)), Span O((\|V\|+\|E\|)·lg\|E\|).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 06 | DirGraphStEph.rs | empty (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 2 | 06 | DirGraphStEph.rs | from_sets (decl) | Def 6.1: O(\|V\|+\|A\|), O(1) | O(\|V\|+\|A\|), O(\|V\|+\|A\|) | O(1), O(1) | no tb cost; ≠ old |
| 3 | 06 | DirGraphStEph.rs | vertices (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 4 | 06 | DirGraphStEph.rs | arcs (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 5 | 06 | DirGraphStEph.rs | sizeV (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 6 | 06 | DirGraphStEph.rs | sizeA (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 7 | 06 | DirGraphStEph.rs | neighbor (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 8 | 06 | DirGraphStEph.rs | ng (decl) | Def 6.1: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 9 | 06 | DirGraphStEph.rs | ng_of_vertices (decl) | Def 6.1: O(\|S\|·\|A\|), O(1) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | no tb cost |
| 10 | 06 | DirGraphStEph.rs | n_plus (decl) | Def 6.1: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 11 | 06 | DirGraphStEph.rs | n_minus (decl) | Def 6.1: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 12 | 06 | DirGraphStEph.rs | n_plus_of_vertices (decl) | Def 6.1: O(\|S\|·\|A\|), O(1) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | no tb cost |
| 13 | 06 | DirGraphStEph.rs | n_minus_of_vertices (decl) | Def 6.1: O(\|S\|·\|A\|), O(1) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | no tb cost |
| 14 | 06 | DirGraphStEph.rs | incident (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 15 | 06 | DirGraphStEph.rs | degree (decl) | Def 6.1: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 16 | 06 | DirGraphStEph.rs | in_degree (decl) | Def 6.1: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 17 | 06 | DirGraphStEph.rs | out_degree (decl) | Def 6.1: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 18 | 06 | DirGraphStEph.rs | iter_vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 19 | 06 | DirGraphStEph.rs | iter_arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 20 | 06 | DirGraphStEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 21 | 06 | DirGraphStEph.rs | from_sets (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 22 | 06 | DirGraphStEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 23 | 06 | DirGraphStEph.rs | arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 24 | 06 | DirGraphStEph.rs | sizeV (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 25 | 06 | DirGraphStEph.rs | sizeA (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 26 | 06 | DirGraphStEph.rs | neighbor (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 27 | 06 | DirGraphStEph.rs | ng (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 28 | 06 | DirGraphStEph.rs | ng_of_vertices (impl) | — | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | no tb cost |
| 29 | 06 | DirGraphStEph.rs | n_plus (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 30 | 06 | DirGraphStEph.rs | n_minus (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 31 | 06 | DirGraphStEph.rs | n_plus_of_vertices (impl) | — | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | no tb cost |
| 32 | 06 | DirGraphStEph.rs | n_minus_of_vertices (impl) | — | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | O(\|S\|·\|A\|), O(\|S\|·\|A\|) | no tb cost |
| 33 | 06 | DirGraphStEph.rs | incident (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 34 | 06 | DirGraphStEph.rs | degree (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 35 | 06 | DirGraphStEph.rs | in_degree (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 36 | 06 | DirGraphStEph.rs | out_degree (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 37 | 06 | DirGraphMtEph.rs | empty (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 38 | 06 | DirGraphMtEph.rs | from_sets (decl) | Def 6.1: O(\|V\|+\|A\|), O(1) | O(\|V\|+\|A\|), O(1) | O(1), O(1) | no tb cost; ≠ old |
| 39 | 06 | DirGraphMtEph.rs | vertices (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 40 | 06 | DirGraphMtEph.rs | arcs (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 41 | 06 | DirGraphMtEph.rs | sizeV (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 42 | 06 | DirGraphMtEph.rs | sizeA (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 43 | 06 | DirGraphMtEph.rs | neighbor (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 44 | 06 | DirGraphMtEph.rs | incident (decl) | Def 6.1: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 45 | 06 | DirGraphMtEph.rs | n_plus (decl) | Def 6.1: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 46 | 06 | DirGraphMtEph.rs | out_degree (decl) | Def 6.1: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 47 | 06 | DirGraphMtEph.rs | n_minus (decl) | Def 6.1: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 48 | 06 | DirGraphMtEph.rs | in_degree (decl) | Def 6.1: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 49 | 06 | DirGraphMtEph.rs | ng (decl) | Def 6.1: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 50 | 06 | DirGraphMtEph.rs | degree (decl) | Def 6.1: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 51 | 06 | DirGraphMtEph.rs | n_plus_of_vertices (decl) | Def 6.1: O(\|U\|·\|A\|), O(lg\|U\|+lg\|A\|) | O(\|U\|·\|A\|), O(lg\|U\|+lg\|A\|) | [a] | no tb cost; ≠ old |
| 52 | 06 | DirGraphMtEph.rs | n_minus_of_vertices (decl) | Def 6.1: O(\|U\|·\|A\|), O(lg\|U\|+lg\|A\|) | O(\|U\|·\|A\|), O(lg\|U\|+lg\|A\|) | [a] | no tb cost; ≠ old |
| 53 | 06 | DirGraphMtEph.rs | ng_of_vertices (decl) | Def 6.1: O(\|U\|·\|A\|), O(lg\|U\|+lg\|A\|) | O(\|U\|·\|A\|), O(lg\|U\|+lg\|A\|) | [a] | no tb cost; ≠ old |
| 54 | 06 | DirGraphMtEph.rs | n_plus_par (decl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 55 | 06 | DirGraphMtEph.rs | n_minus_par (decl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 56 | 06 | DirGraphMtEph.rs | n_plus_of_vertices_par (decl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 57 | 06 | DirGraphMtEph.rs | n_minus_of_vertices_par (decl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 58 | 06 | DirGraphMtEph.rs | ng_of_vertices_par (decl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 59 | 06 | DirGraphMtEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 60 | 06 | DirGraphMtEph.rs | from_sets (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 61 | 06 | DirGraphMtEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 62 | 06 | DirGraphMtEph.rs | arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 63 | 06 | DirGraphMtEph.rs | sizeV (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 64 | 06 | DirGraphMtEph.rs | sizeA (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 65 | 06 | DirGraphMtEph.rs | neighbor (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 66 | 06 | DirGraphMtEph.rs | incident (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 67 | 06 | DirGraphMtEph.rs | n_plus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 68 | 06 | DirGraphMtEph.rs | out_degree (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 69 | 06 | DirGraphMtEph.rs | n_minus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 70 | 06 | DirGraphMtEph.rs | in_degree (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 71 | 06 | DirGraphMtEph.rs | ng (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 72 | 06 | DirGraphMtEph.rs | degree (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 73 | 06 | DirGraphMtEph.rs | n_plus_of_vertices (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 74 | 06 | DirGraphMtEph.rs | n_minus_of_vertices (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 75 | 06 | DirGraphMtEph.rs | ng_of_vertices (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 76 | 06 | DirGraphMtEph.rs | n_plus_par (impl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 77 | 06 | DirGraphMtEph.rs | n_minus_par (impl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 78 | 06 | DirGraphMtEph.rs | n_plus_of_vertices_par (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 79 | 06 | DirGraphMtEph.rs | n_minus_of_vertices_par (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 80 | 06 | DirGraphMtEph.rs | ng_of_vertices_par (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 81 | 06 | DirGraphMtEph.rs | new (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 82 | 06 | DirGraphMtEph.rs | vertices (decl) | — | O(\|V\|), O(\|V\|) | O(\|V\|), O(\|V\|) | no tb cost |
| 83 | 06 | DirGraphMtEph.rs | arcs (decl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 84 | 06 | DirGraphMtEph.rs | sizeV (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 85 | 06 | DirGraphMtEph.rs | sizeA (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 86 | 06 | DirGraphMtEph.rs | neighbor (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 87 | 06 | DirGraphMtEph.rs | n_plus (decl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 88 | 06 | DirGraphMtEph.rs | n_minus (decl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 89 | 06 | DirGraphMtEph.rs | ng (decl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 90 | 06 | DirGraphMtEph.rs | n_plus_of_vertices (decl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 91 | 06 | DirGraphMtEph.rs | n_minus_of_vertices (decl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 92 | 06 | DirGraphMtEph.rs | ng_of_vertices (decl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 93 | 06 | DirGraphMtEph.rs | new (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 94 | 06 | DirGraphMtEph.rs | vertices (impl) | — | O(\|V\|), O(\|V\|) | O(\|V\|), O(\|V\|) | no tb cost |
| 95 | 06 | DirGraphMtEph.rs | arcs (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 96 | 06 | DirGraphMtEph.rs | sizeV (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 97 | 06 | DirGraphMtEph.rs | sizeA (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 98 | 06 | DirGraphMtEph.rs | neighbor (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 99 | 06 | DirGraphMtEph.rs | n_plus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 100 | 06 | DirGraphMtEph.rs | n_minus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 101 | 06 | DirGraphMtEph.rs | ng (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 102 | 06 | DirGraphMtEph.rs | n_plus_of_vertices (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 103 | 06 | DirGraphMtEph.rs | n_minus_of_vertices (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 104 | 06 | DirGraphMtEph.rs | ng_of_vertices (impl) | — | O(\|S\|·\|A\|), O(lg\|S\|·lg\|A\|) | [a] | no tb cost; ≠ old |
| 105 | 06 | UnDirGraphStEph.rs | empty (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 106 | 06 | UnDirGraphStEph.rs | from_sets (decl) | Def 6.2: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(1), O(1) | no tb cost; ≠ old |
| 107 | 06 | UnDirGraphStEph.rs | vertices (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 108 | 06 | UnDirGraphStEph.rs | edges (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 109 | 06 | UnDirGraphStEph.rs | sizeV (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 110 | 06 | UnDirGraphStEph.rs | sizeE (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 111 | 06 | UnDirGraphStEph.rs | neighbor (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 112 | 06 | UnDirGraphStEph.rs | ng (decl) | Def 6.2: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 113 | 06 | UnDirGraphStEph.rs | ng_of_vertices (decl) | Def 6.2: O(\|U\|·\|E\|), O(1) | O(\|U\|·\|E\|), O(\|U\|·\|E\|) | O(\|U\|·\|E\|), O(\|U\|·\|E\|) | no tb cost |
| 114 | 06 | UnDirGraphStEph.rs | incident (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 115 | 06 | UnDirGraphStEph.rs | degree (decl) | Def 6.2: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 116 | 06 | UnDirGraphStEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 117 | 06 | UnDirGraphStEph.rs | from_sets (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 118 | 06 | UnDirGraphStEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 119 | 06 | UnDirGraphStEph.rs | edges (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 120 | 06 | UnDirGraphStEph.rs | sizeV (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 121 | 06 | UnDirGraphStEph.rs | sizeE (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 122 | 06 | UnDirGraphStEph.rs | neighbor (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 123 | 06 | UnDirGraphStEph.rs | ng (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 124 | 06 | UnDirGraphStEph.rs | ng_of_vertices (impl) | — | O(\|S\|·\|E\|), O(\|S\|·\|E\|) | O(\|S\|·\|E\|), O(\|S\|·\|E\|) | no tb cost |
| 125 | 06 | UnDirGraphStEph.rs | incident (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 126 | 06 | UnDirGraphStEph.rs | degree (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 127 | 06 | UnDirGraphMtEph.rs | empty (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 128 | 06 | UnDirGraphMtEph.rs | from_sets (decl) | Def 6.2: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(1) | O(1), O(1) | no tb cost; ≠ old |
| 129 | 06 | UnDirGraphMtEph.rs | vertices (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 130 | 06 | UnDirGraphMtEph.rs | edges (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 131 | 06 | UnDirGraphMtEph.rs | sizeV (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 132 | 06 | UnDirGraphMtEph.rs | sizeE (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 133 | 06 | UnDirGraphMtEph.rs | neighbor (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 134 | 06 | UnDirGraphMtEph.rs | ng (decl) | Def 6.2: O(\|E\|), O(lg\|E\|) | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 135 | 06 | UnDirGraphMtEph.rs | ng_of_vertices (decl) | Def 6.2: O(\|U\|·\|E\|), O(lg\|U\|+lg\|E\|) | O(\|U\|·\|E\|), O(lg\|U\|+lg\|E\|) | [b] | no tb cost; ≠ old |
| 136 | 06 | UnDirGraphMtEph.rs | incident (decl) | Def 6.2: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 137 | 06 | UnDirGraphMtEph.rs | degree (decl) | Def 6.2: O(\|E\|), O(lg\|E\|) | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 138 | 06 | UnDirGraphMtEph.rs | ng_par (decl) | — | O(\|E\|), O(lg\|E\|) | [d] | no tb cost; ≠ old |
| 139 | 06 | UnDirGraphMtEph.rs | ng_of_vertices_par (decl) | — | O(\|S\|·\|E\|), O(lg\|S\|·lg\|E\|) | [b] | no tb cost; ≠ old |
| 140 | 06 | UnDirGraphMtEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 141 | 06 | UnDirGraphMtEph.rs | from_sets (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 142 | 06 | UnDirGraphMtEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 143 | 06 | UnDirGraphMtEph.rs | edges (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 144 | 06 | UnDirGraphMtEph.rs | sizeV (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 145 | 06 | UnDirGraphMtEph.rs | sizeE (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 146 | 06 | UnDirGraphMtEph.rs | neighbor (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 147 | 06 | UnDirGraphMtEph.rs | ng (impl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 148 | 06 | UnDirGraphMtEph.rs | ng_of_vertices (impl) | — | O(\|S\|·\|E\|), O(lg\|S\|·lg\|E\|) | [b] | no tb cost; ≠ old |
| 149 | 06 | UnDirGraphMtEph.rs | incident (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 150 | 06 | UnDirGraphMtEph.rs | degree (impl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 151 | 06 | UnDirGraphMtEph.rs | ng_par (impl) | — | O(\|E\|), O(lg\|E\|) | [d] | no tb cost; ≠ old |
| 152 | 06 | UnDirGraphMtEph.rs | ng_of_vertices_par (impl) | — | O(\|S\|·\|E\|), O(lg\|S\|·lg\|E\|) | [b] | no tb cost; ≠ old |
| 153 | 06 | UnDirGraphMtEph.rs | new (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 154 | 06 | UnDirGraphMtEph.rs | vertices (decl) | — | O(\|V\|), O(\|V\|) | O(\|V\|), O(\|V\|) | no tb cost |
| 155 | 06 | UnDirGraphMtEph.rs | edges (decl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 156 | 06 | UnDirGraphMtEph.rs | sizeV (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 157 | 06 | UnDirGraphMtEph.rs | sizeE (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 158 | 06 | UnDirGraphMtEph.rs | neighbor (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 159 | 06 | UnDirGraphMtEph.rs | ng (decl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 160 | 06 | UnDirGraphMtEph.rs | ng_of_vertices (decl) | — | O(\|S\|·\|E\|), O(lg\|S\|·lg\|E\|) | [b] | no tb cost; ≠ old |
| 161 | 06 | UnDirGraphMtEph.rs | new (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 162 | 06 | UnDirGraphMtEph.rs | vertices (impl) | — | O(\|V\|), O(\|V\|) | O(\|V\|), O(\|V\|) | no tb cost |
| 163 | 06 | UnDirGraphMtEph.rs | edges (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 164 | 06 | UnDirGraphMtEph.rs | sizeV (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 165 | 06 | UnDirGraphMtEph.rs | sizeE (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 166 | 06 | UnDirGraphMtEph.rs | neighbor (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 167 | 06 | UnDirGraphMtEph.rs | ng (impl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 168 | 06 | UnDirGraphMtEph.rs | ng_of_vertices (impl) | — | O(\|S\|·\|E\|), O(lg\|S\|·lg\|E\|) | [b] | no tb cost; ≠ old |
| 169 | 06 | LabDirGraphStEph.rs | empty (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 170 | 06 | LabDirGraphStEph.rs | from_vertices_and_labeled_arcs (decl) | Def 6.17: O(\|V\|+\|A\|), O(1) | O(\|V\|+\|A\|), O(\|V\|+\|A\|) | O(1), O(1) | no tb cost; ≠ old |
| 171 | 06 | LabDirGraphStEph.rs | vertices (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 172 | 06 | LabDirGraphStEph.rs | labeled_arcs (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 173 | 06 | LabDirGraphStEph.rs | arcs (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 174 | 06 | LabDirGraphStEph.rs | add_vertex (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 175 | 06 | LabDirGraphStEph.rs | add_labeled_arc (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 176 | 06 | LabDirGraphStEph.rs | get_arc_label (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 177 | 06 | LabDirGraphStEph.rs | has_arc (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 178 | 06 | LabDirGraphStEph.rs | n_plus (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 179 | 06 | LabDirGraphStEph.rs | n_minus (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 180 | 06 | LabDirGraphStEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 181 | 06 | LabDirGraphStEph.rs | from_vertices_and_labeled_arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 182 | 06 | LabDirGraphStEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 183 | 06 | LabDirGraphStEph.rs | labeled_arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 184 | 06 | LabDirGraphStEph.rs | arcs (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 185 | 06 | LabDirGraphStEph.rs | add_vertex (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 186 | 06 | LabDirGraphStEph.rs | add_labeled_arc (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 187 | 06 | LabDirGraphStEph.rs | get_arc_label (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 188 | 06 | LabDirGraphStEph.rs | has_arc (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 189 | 06 | LabDirGraphStEph.rs | n_plus (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 190 | 06 | LabDirGraphStEph.rs | n_minus (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 191 | 06 | LabDirGraphMtEph.rs | empty (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 192 | 06 | LabDirGraphMtEph.rs | from_vertices_and_labeled_arcs (decl) | Def 6.17: O(\|V\|+\|A\|), O(1) | O(\|V\|+\|A\|), O(1) | O(1), O(1) | no tb cost; ≠ old |
| 193 | 06 | LabDirGraphMtEph.rs | vertices (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 194 | 06 | LabDirGraphMtEph.rs | labeled_arcs (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 195 | 06 | LabDirGraphMtEph.rs | arcs (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 196 | 06 | LabDirGraphMtEph.rs | add_vertex (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 197 | 06 | LabDirGraphMtEph.rs | add_labeled_arc (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 198 | 06 | LabDirGraphMtEph.rs | get_arc_label (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 199 | 06 | LabDirGraphMtEph.rs | has_arc (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 200 | 06 | LabDirGraphMtEph.rs | n_plus (decl) | Def 6.17: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 201 | 06 | LabDirGraphMtEph.rs | n_minus (decl) | Def 6.17: O(\|A\|), O(lg\|A\|) | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 202 | 06 | LabDirGraphMtEph.rs | n_plus_par (decl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 203 | 06 | LabDirGraphMtEph.rs | n_minus_par (decl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 204 | 06 | LabDirGraphMtEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 205 | 06 | LabDirGraphMtEph.rs | from_vertices_and_labeled_arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 206 | 06 | LabDirGraphMtEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 207 | 06 | LabDirGraphMtEph.rs | labeled_arcs (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 208 | 06 | LabDirGraphMtEph.rs | arcs (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 209 | 06 | LabDirGraphMtEph.rs | add_vertex (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 210 | 06 | LabDirGraphMtEph.rs | add_labeled_arc (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 211 | 06 | LabDirGraphMtEph.rs | get_arc_label (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 212 | 06 | LabDirGraphMtEph.rs | has_arc (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 213 | 06 | LabDirGraphMtEph.rs | n_plus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 214 | 06 | LabDirGraphMtEph.rs | n_minus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 215 | 06 | LabDirGraphMtEph.rs | n_plus_par (impl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 216 | 06 | LabDirGraphMtEph.rs | n_minus_par (impl) | — | O(\|A\|), O(lg\|A\|) | [c] | no tb cost; ≠ old |
| 217 | 06 | LabDirGraphMtEph.rs | new (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 218 | 06 | LabDirGraphMtEph.rs | add_vertex (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 219 | 06 | LabDirGraphMtEph.rs | add_labeled_arc (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 220 | 06 | LabDirGraphMtEph.rs | n_plus (decl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 221 | 06 | LabDirGraphMtEph.rs | n_minus (decl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 222 | 06 | LabDirGraphMtEph.rs | new (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 223 | 06 | LabDirGraphMtEph.rs | add_vertex (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 224 | 06 | LabDirGraphMtEph.rs | add_labeled_arc (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 225 | 06 | LabDirGraphMtEph.rs | n_plus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 226 | 06 | LabDirGraphMtEph.rs | n_minus (impl) | — | O(\|A\|), O(lg\|A\|) | [e] | no tb cost; ≠ old |
| 227 | 06 | LabUnDirGraphStEph.rs | empty (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 228 | 06 | LabUnDirGraphStEph.rs | from_vertices_and_labeled_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(1), O(1) | no tb cost; ≠ old |
| 229 | 06 | LabUnDirGraphStEph.rs | vertices (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 230 | 06 | LabUnDirGraphStEph.rs | labeled_edges (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 231 | 06 | LabUnDirGraphStEph.rs | edges (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 232 | 06 | LabUnDirGraphStEph.rs | add_vertex (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 233 | 06 | LabUnDirGraphStEph.rs | add_labeled_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 234 | 06 | LabUnDirGraphStEph.rs | get_edge_label (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 235 | 06 | LabUnDirGraphStEph.rs | has_edge (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 236 | 06 | LabUnDirGraphStEph.rs | ng (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 237 | 06 | LabUnDirGraphStEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 238 | 06 | LabUnDirGraphStEph.rs | from_vertices_and_labeled_edges (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 239 | 06 | LabUnDirGraphStEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 240 | 06 | LabUnDirGraphStEph.rs | labeled_edges (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 241 | 06 | LabUnDirGraphStEph.rs | edges (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 242 | 06 | LabUnDirGraphStEph.rs | add_vertex (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 243 | 06 | LabUnDirGraphStEph.rs | add_labeled_edge (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 244 | 06 | LabUnDirGraphStEph.rs | get_edge_label (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 245 | 06 | LabUnDirGraphStEph.rs | has_edge (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 246 | 06 | LabUnDirGraphStEph.rs | ng (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 247 | 06 | LabUnDirGraphMtEph.rs | empty (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 248 | 06 | LabUnDirGraphMtEph.rs | from_vertices_and_labeled_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(1) | O(1), O(1) | no tb cost; ≠ old |
| 249 | 06 | LabUnDirGraphMtEph.rs | vertices (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 250 | 06 | LabUnDirGraphMtEph.rs | labeled_edges (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 251 | 06 | LabUnDirGraphMtEph.rs | edges (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 252 | 06 | LabUnDirGraphMtEph.rs | add_vertex (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 253 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 254 | 06 | LabUnDirGraphMtEph.rs | get_edge_label (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 255 | 06 | LabUnDirGraphMtEph.rs | has_edge (decl) | Def 6.17: O(\|E\|), O(1) | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 256 | 06 | LabUnDirGraphMtEph.rs | ng (decl) | Def 6.17: O(\|E\|), O(lg\|E\|) | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 257 | 06 | LabUnDirGraphMtEph.rs | ng_par (decl) | — | O(\|E\|), O(lg\|E\|) | [d] | no tb cost; ≠ old |
| 258 | 06 | LabUnDirGraphMtEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 259 | 06 | LabUnDirGraphMtEph.rs | from_vertices_and_labeled_edges (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 260 | 06 | LabUnDirGraphMtEph.rs | vertices (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 261 | 06 | LabUnDirGraphMtEph.rs | labeled_edges (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 262 | 06 | LabUnDirGraphMtEph.rs | edges (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 263 | 06 | LabUnDirGraphMtEph.rs | add_vertex (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 264 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 265 | 06 | LabUnDirGraphMtEph.rs | get_edge_label (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 266 | 06 | LabUnDirGraphMtEph.rs | has_edge (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 267 | 06 | LabUnDirGraphMtEph.rs | ng (impl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 268 | 06 | LabUnDirGraphMtEph.rs | ng_par (impl) | — | O(\|E\|), O(lg\|E\|) | [d] | no tb cost; ≠ old |
| 269 | 06 | LabUnDirGraphMtEph.rs | empty (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 270 | 06 | LabUnDirGraphMtEph.rs | vertices (decl) | — | O(\|V\|), O(\|V\|) | O(\|V\|), O(\|V\|) | no tb cost |
| 271 | 06 | LabUnDirGraphMtEph.rs | labeled_edges (decl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 272 | 06 | LabUnDirGraphMtEph.rs | edges (decl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 273 | 06 | LabUnDirGraphMtEph.rs | has_edge (decl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 274 | 06 | LabUnDirGraphMtEph.rs | ng (decl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 275 | 06 | LabUnDirGraphMtEph.rs | add_vertex (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 276 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge (decl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 277 | 06 | LabUnDirGraphMtEph.rs | empty (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 278 | 06 | LabUnDirGraphMtEph.rs | vertices (impl) | — | O(\|V\|), O(\|V\|) | O(\|V\|), O(\|V\|) | no tb cost |
| 279 | 06 | LabUnDirGraphMtEph.rs | labeled_edges (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 280 | 06 | LabUnDirGraphMtEph.rs | edges (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 281 | 06 | LabUnDirGraphMtEph.rs | has_edge (impl) | — | O(\|E\|), O(\|E\|) | O(\|E\|), O(\|E\|) | no tb cost |
| 282 | 06 | LabUnDirGraphMtEph.rs | ng (impl) | — | O(\|E\|), O(lg\|E\|) | [f] | no tb cost; ≠ old |
| 283 | 06 | LabUnDirGraphMtEph.rs | add_vertex (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 284 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge (impl) | — | O(1), O(1) | O(1), O(1) | no tb cost |
| 285 | 06 | WeightedDirGraphStEphF64.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 286 | 06 | WeightedDirGraphStEphF64.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 287 | 06 | WeightedDirGraphStEphF64.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 288 | 06 | WeightedDirGraphStEphF64.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 289 | 06 | WeightedDirGraphStEphF64.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 290 | 06 | WeightedDirGraphStEphF64.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 291 | 06 | WeightedDirGraphStEphF64.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 292 | 06 | WeightedDirGraphStEphF64.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 293 | 06 | WeightedDirGraphStEphF64.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 294 | 06 | WeightedDirGraphStEphF64.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 295 | 06 | WeightedDirGraphStEphF64.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 296 | 06 | WeightedDirGraphStEphF64.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 297 | 06 | WeightedDirGraphStEphI8.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 298 | 06 | WeightedDirGraphStEphI8.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 299 | 06 | WeightedDirGraphStEphI8.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 300 | 06 | WeightedDirGraphStEphI8.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 301 | 06 | WeightedDirGraphStEphI8.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 302 | 06 | WeightedDirGraphStEphI8.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 303 | 06 | WeightedDirGraphStEphI8.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 304 | 06 | WeightedDirGraphStEphI8.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 305 | 06 | WeightedDirGraphStEphI8.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 306 | 06 | WeightedDirGraphStEphI8.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 307 | 06 | WeightedDirGraphStEphI8.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 308 | 06 | WeightedDirGraphStEphI8.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 309 | 06 | WeightedDirGraphStEphI8.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 310 | 06 | WeightedDirGraphStEphI8.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 311 | 06 | WeightedDirGraphStEphI8.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 312 | 06 | WeightedDirGraphStEphI8.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 313 | 06 | WeightedDirGraphStEphI8.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 314 | 06 | WeightedDirGraphStEphI8.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 315 | 06 | WeightedDirGraphStEphI16.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 316 | 06 | WeightedDirGraphStEphI16.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 317 | 06 | WeightedDirGraphStEphI16.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 318 | 06 | WeightedDirGraphStEphI16.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 319 | 06 | WeightedDirGraphStEphI16.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 320 | 06 | WeightedDirGraphStEphI16.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 321 | 06 | WeightedDirGraphStEphI16.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 322 | 06 | WeightedDirGraphStEphI16.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 323 | 06 | WeightedDirGraphStEphI16.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 324 | 06 | WeightedDirGraphStEphI16.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 325 | 06 | WeightedDirGraphStEphI16.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 326 | 06 | WeightedDirGraphStEphI16.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 327 | 06 | WeightedDirGraphStEphI16.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 328 | 06 | WeightedDirGraphStEphI16.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 329 | 06 | WeightedDirGraphStEphI16.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 330 | 06 | WeightedDirGraphStEphI16.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 331 | 06 | WeightedDirGraphStEphI16.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 332 | 06 | WeightedDirGraphStEphI16.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 333 | 06 | WeightedDirGraphStEphI32.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 334 | 06 | WeightedDirGraphStEphI32.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 335 | 06 | WeightedDirGraphStEphI32.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 336 | 06 | WeightedDirGraphStEphI32.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 337 | 06 | WeightedDirGraphStEphI32.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 338 | 06 | WeightedDirGraphStEphI32.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 339 | 06 | WeightedDirGraphStEphI32.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 340 | 06 | WeightedDirGraphStEphI32.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 341 | 06 | WeightedDirGraphStEphI32.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 342 | 06 | WeightedDirGraphStEphI32.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 343 | 06 | WeightedDirGraphStEphI32.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 344 | 06 | WeightedDirGraphStEphI32.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 345 | 06 | WeightedDirGraphStEphI32.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 346 | 06 | WeightedDirGraphStEphI32.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 347 | 06 | WeightedDirGraphStEphI32.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 348 | 06 | WeightedDirGraphStEphI32.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 349 | 06 | WeightedDirGraphStEphI32.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 350 | 06 | WeightedDirGraphStEphI32.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 351 | 06 | WeightedDirGraphStEphI64.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 352 | 06 | WeightedDirGraphStEphI64.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 353 | 06 | WeightedDirGraphStEphI64.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 354 | 06 | WeightedDirGraphStEphI64.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 355 | 06 | WeightedDirGraphStEphI64.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 356 | 06 | WeightedDirGraphStEphI64.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 357 | 06 | WeightedDirGraphStEphI64.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 358 | 06 | WeightedDirGraphStEphI64.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 359 | 06 | WeightedDirGraphStEphI64.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 360 | 06 | WeightedDirGraphStEphI64.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 361 | 06 | WeightedDirGraphStEphI64.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 362 | 06 | WeightedDirGraphStEphI64.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 363 | 06 | WeightedDirGraphStEphI64.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 364 | 06 | WeightedDirGraphStEphI64.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 365 | 06 | WeightedDirGraphStEphI64.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 366 | 06 | WeightedDirGraphStEphI64.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 367 | 06 | WeightedDirGraphStEphI64.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 368 | 06 | WeightedDirGraphStEphI64.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 369 | 06 | WeightedDirGraphStEphI128.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 370 | 06 | WeightedDirGraphStEphI128.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 371 | 06 | WeightedDirGraphStEphI128.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 372 | 06 | WeightedDirGraphStEphI128.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 373 | 06 | WeightedDirGraphStEphI128.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 374 | 06 | WeightedDirGraphStEphI128.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 375 | 06 | WeightedDirGraphStEphI128.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 376 | 06 | WeightedDirGraphStEphI128.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 377 | 06 | WeightedDirGraphStEphI128.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 378 | 06 | WeightedDirGraphStEphI128.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 379 | 06 | WeightedDirGraphStEphI128.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 380 | 06 | WeightedDirGraphStEphI128.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 381 | 06 | WeightedDirGraphStEphI128.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 382 | 06 | WeightedDirGraphStEphI128.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 383 | 06 | WeightedDirGraphStEphI128.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 384 | 06 | WeightedDirGraphStEphI128.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 385 | 06 | WeightedDirGraphStEphI128.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 386 | 06 | WeightedDirGraphStEphI128.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 387 | 06 | WeightedDirGraphStEphIsize.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 388 | 06 | WeightedDirGraphStEphIsize.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 389 | 06 | WeightedDirGraphStEphIsize.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 390 | 06 | WeightedDirGraphStEphIsize.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 391 | 06 | WeightedDirGraphStEphIsize.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 392 | 06 | WeightedDirGraphStEphIsize.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 393 | 06 | WeightedDirGraphStEphIsize.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 394 | 06 | WeightedDirGraphStEphIsize.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 395 | 06 | WeightedDirGraphStEphIsize.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 396 | 06 | WeightedDirGraphStEphIsize.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 397 | 06 | WeightedDirGraphStEphIsize.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 398 | 06 | WeightedDirGraphStEphIsize.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 399 | 06 | WeightedDirGraphStEphIsize.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 400 | 06 | WeightedDirGraphStEphIsize.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 401 | 06 | WeightedDirGraphStEphIsize.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 402 | 06 | WeightedDirGraphStEphIsize.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 403 | 06 | WeightedDirGraphStEphIsize.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 404 | 06 | WeightedDirGraphStEphIsize.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 405 | 06 | WeightedDirGraphStEphU8.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 406 | 06 | WeightedDirGraphStEphU8.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 407 | 06 | WeightedDirGraphStEphU8.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 408 | 06 | WeightedDirGraphStEphU8.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 409 | 06 | WeightedDirGraphStEphU8.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 410 | 06 | WeightedDirGraphStEphU8.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 411 | 06 | WeightedDirGraphStEphU8.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 412 | 06 | WeightedDirGraphStEphU8.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 413 | 06 | WeightedDirGraphStEphU8.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 414 | 06 | WeightedDirGraphStEphU8.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 415 | 06 | WeightedDirGraphStEphU8.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 416 | 06 | WeightedDirGraphStEphU8.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 417 | 06 | WeightedDirGraphStEphU8.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 418 | 06 | WeightedDirGraphStEphU8.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 419 | 06 | WeightedDirGraphStEphU8.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 420 | 06 | WeightedDirGraphStEphU8.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 421 | 06 | WeightedDirGraphStEphU8.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 422 | 06 | WeightedDirGraphStEphU8.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 423 | 06 | WeightedDirGraphStEphU16.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 424 | 06 | WeightedDirGraphStEphU16.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 425 | 06 | WeightedDirGraphStEphU16.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 426 | 06 | WeightedDirGraphStEphU16.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 427 | 06 | WeightedDirGraphStEphU16.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 428 | 06 | WeightedDirGraphStEphU16.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 429 | 06 | WeightedDirGraphStEphU16.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 430 | 06 | WeightedDirGraphStEphU16.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 431 | 06 | WeightedDirGraphStEphU16.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 432 | 06 | WeightedDirGraphStEphU16.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 433 | 06 | WeightedDirGraphStEphU16.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 434 | 06 | WeightedDirGraphStEphU16.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 435 | 06 | WeightedDirGraphStEphU16.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 436 | 06 | WeightedDirGraphStEphU16.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 437 | 06 | WeightedDirGraphStEphU16.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 438 | 06 | WeightedDirGraphStEphU16.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 439 | 06 | WeightedDirGraphStEphU16.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 440 | 06 | WeightedDirGraphStEphU16.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 441 | 06 | WeightedDirGraphStEphU32.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 442 | 06 | WeightedDirGraphStEphU32.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 443 | 06 | WeightedDirGraphStEphU32.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 444 | 06 | WeightedDirGraphStEphU32.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 445 | 06 | WeightedDirGraphStEphU32.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 446 | 06 | WeightedDirGraphStEphU32.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 447 | 06 | WeightedDirGraphStEphU32.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 448 | 06 | WeightedDirGraphStEphU32.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 449 | 06 | WeightedDirGraphStEphU32.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 450 | 06 | WeightedDirGraphStEphU32.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 451 | 06 | WeightedDirGraphStEphU32.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 452 | 06 | WeightedDirGraphStEphU32.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 453 | 06 | WeightedDirGraphStEphU32.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 454 | 06 | WeightedDirGraphStEphU32.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 455 | 06 | WeightedDirGraphStEphU32.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 456 | 06 | WeightedDirGraphStEphU32.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 457 | 06 | WeightedDirGraphStEphU32.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 458 | 06 | WeightedDirGraphStEphU32.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 459 | 06 | WeightedDirGraphStEphU64.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 460 | 06 | WeightedDirGraphStEphU64.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 461 | 06 | WeightedDirGraphStEphU64.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 462 | 06 | WeightedDirGraphStEphU64.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 463 | 06 | WeightedDirGraphStEphU64.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 464 | 06 | WeightedDirGraphStEphU64.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 465 | 06 | WeightedDirGraphStEphU64.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 466 | 06 | WeightedDirGraphStEphU64.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 467 | 06 | WeightedDirGraphStEphU64.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 468 | 06 | WeightedDirGraphStEphU64.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 469 | 06 | WeightedDirGraphStEphU64.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 470 | 06 | WeightedDirGraphStEphU64.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 471 | 06 | WeightedDirGraphStEphU64.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 472 | 06 | WeightedDirGraphStEphU64.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 473 | 06 | WeightedDirGraphStEphU64.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 474 | 06 | WeightedDirGraphStEphU64.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 475 | 06 | WeightedDirGraphStEphU64.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 476 | 06 | WeightedDirGraphStEphU64.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 477 | 06 | WeightedDirGraphStEphU128.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 478 | 06 | WeightedDirGraphStEphU128.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 479 | 06 | WeightedDirGraphStEphU128.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 480 | 06 | WeightedDirGraphStEphU128.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 481 | 06 | WeightedDirGraphStEphU128.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 482 | 06 | WeightedDirGraphStEphU128.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 483 | 06 | WeightedDirGraphStEphU128.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 484 | 06 | WeightedDirGraphStEphU128.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 485 | 06 | WeightedDirGraphStEphU128.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 486 | 06 | WeightedDirGraphStEphU128.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 487 | 06 | WeightedDirGraphStEphU128.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 488 | 06 | WeightedDirGraphStEphU128.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 489 | 06 | WeightedDirGraphStEphU128.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 490 | 06 | WeightedDirGraphStEphU128.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 491 | 06 | WeightedDirGraphStEphU128.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 492 | 06 | WeightedDirGraphStEphU128.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 493 | 06 | WeightedDirGraphStEphU128.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 494 | 06 | WeightedDirGraphStEphU128.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 495 | 06 | WeightedDirGraphStEphUsize.rs | from_weighed_edges (decl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 496 | 06 | WeightedDirGraphStEphUsize.rs | add_weighed_edge (decl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 497 | 06 | WeightedDirGraphStEphUsize.rs | get_edge_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 498 | 06 | WeightedDirGraphStEphUsize.rs | weighed_edges (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 499 | 06 | WeightedDirGraphStEphUsize.rs | out_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 500 | 06 | WeightedDirGraphStEphUsize.rs | in_neighbors_weighed (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 501 | 06 | WeightedDirGraphStEphUsize.rs | total_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 502 | 06 | WeightedDirGraphStEphUsize.rs | edges_above_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 503 | 06 | WeightedDirGraphStEphUsize.rs | edges_below_weight (decl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 504 | 06 | WeightedDirGraphStEphUsize.rs | from_weighed_edges (impl) | Def 6.17: O(\|V\|+\|E\|), O(1) | O(\|V\|+\|E\|), O(\|V\|+\|E\|) | O(\|E\|), O(\|E\|) | no tb cost; ≠ old |
| 505 | 06 | WeightedDirGraphStEphUsize.rs | add_weighed_edge (impl) | Def 6.17: O(1), O(1) | O(1), O(1) | O(1), O(1) | no tb cost |
| 506 | 06 | WeightedDirGraphStEphUsize.rs | get_edge_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 507 | 06 | WeightedDirGraphStEphUsize.rs | weighed_edges (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 508 | 06 | WeightedDirGraphStEphUsize.rs | out_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 509 | 06 | WeightedDirGraphStEphUsize.rs | in_neighbors_weighed (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 510 | 06 | WeightedDirGraphStEphUsize.rs | total_weight (impl) | Def 6.17: O(\|A\|), O(1) | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 511 | 06 | WeightedDirGraphStEphUsize.rs | edges_above_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |
| 512 | 06 | WeightedDirGraphStEphUsize.rs | edges_below_weight (impl) | — | O(\|A\|), O(\|A\|) | O(\|A\|), O(\|A\|) | no tb cost |

## 3. Counts

| # | Chap | Category | Count |
|---|---|---|---|
| 1 | 06 | Reviewed annotation sites (new lines) | 512 |
| 2 | 06 | matches textbook | 0 |
| 3 | 06 | does not match textbook | 0 |
| 4 | 06 | does not match old analysis | 106 |
| 5 | 06 | no textbook cost | 512 |
| 6 | 06 | no textbook cost, agrees with old analysis | 406 |
| 7 | 06 | unannotated functions | 48 |
| 8 | 06 | malformed annotations | 1 |

The 106 "does not match old analysis" lines break down as:

| # | Chap | Cause | Files | Lines |
|---|---|---|---|---|
| 1 | 06 | Mt neighbor ops clone graph per node | 4 Mt files | 72 |
| 2 | 06 | from_weighed_edges: V moved, not copied | 13 Weighted | 26 |
| 3 | 06 | from_sets / from_vertices_*: O(1) move | 8 files (decl) | 8 |

Unannotated functions (no Alg Analysis line; none added):

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 06 | DirGraphStEph.rs | into_iter, clone, eq, fmt (×2) |
| 2 | 06 | DirGraphMtEph.rs | into_iter, clone, eq, fmt (×5) |
| 3 | 06 | UnDirGraphStEph.rs | into_iter, clone, eq, fmt (×2) |
| 4 | 06 | UnDirGraphMtEph.rs | into_iter, clone, eq, fmt (×5) |
| 5 | 06 | LabDirGraphStEph.rs | into_iter, clone, fmt (×2) |
| 6 | 06 | LabDirGraphMtEph.rs | into_iter, clone, fmt (×5) |
| 7 | 06 | LabUnDirGraphStEph.rs | into_iter, clone, fmt (×2) |
| 8 | 06 | LabUnDirGraphMtEph.rs | into_iter, clone, fmt (×5) |

These are `IntoIterator`, `Clone`, `PartialEq`, `Debug`, and `Display`
impls: 48 function definitions in total, each `fmt` counted separately.
The 13 Weighted files have none.

Malformed annotations:

| # | Chap | File | Function | Problem |
|---|---|---|---|---|
| 1 | 06 | DirGraphMtEph.rs | LockedDirGraphMtEph::arcs (impl) | `// Veracity: NEEDED proof block` sits between the `///` block and `fn` |

The new line was placed inside the `///` block, above that `//` line. It is
still attached to `arcs` (a plain `//` comment does not detach a doc
comment), so this is recorded, not fixed. Several other functions have a
stray `// Veracity:` line directly above their `///` block
(`LabUnDirGraphMtEph.rs` locked `ng`, `add_vertex`; `DirGraphMtEph.rs`
locked `sizeV`, `sizeA`); those do not split the block.

## 4. Notable findings

1. **Unsupported APAS lines (302 lines).** Chapter 6 states no costs, yet
   every trait declaration carries an `APAS (Ch06 Def 6.1/6.2/6.17)` line
   with a work and span bound. Several claim bounds no implementation
   could meet on this representation, for example `ng` with Span O(1) and
   `n_plus_of_vertices` with Span O(1). The APAS lines should be removed
   (standard 25: functions with no APAS cost get only a Code review line)
   or re-sourced to the chapter that actually states the cost (Chap52
   graph representations).

2. **Mt neighbor functions: old Work O(|A|), Span O(log |A|) is wrong
   (72 lines, 4 files).** `n_plus_par`, `n_minus_par`, `ng_par`, and the
   `*_of_vertices_par` recursions in `DirGraphMtEph.rs`,
   `UnDirGraphMtEph.rs`, `LabDirGraphMtEph.rs`, and `LabUnDirGraphMtEph.rs`
   execute, at every recursion node:
   `let g_left = self.clone_plus(); let g_right = self.clone_plus();`.
   `clone_plus` calls the graph's derived `Clone`, which deep-copies both
   hash sets, O(|V| + |A|). With ~2|A| recursion nodes the work is
   O(|A|·(|V| + |A|)) instead of O(|A|), a quadratic regression for a
   neighbor query. The clones and the `SetStEph::split` and `union` calls
   are sequential and precede or follow the fork, so each level of the
   recursion costs O(|V| + |A|) span and the total span is
   O((|V| + |A|)·log |A|), not O(log |A|). Even without the clones, the
   sequential O(k) `split` and `union` would make the span O(|A|). The
   `*_of_vertices_par` functions multiply this by |u_set| and also clone
   the arc set at each leaf. The locked (`RwLock`) wrappers inherit these
   costs. The graph is only read in the recursion, so passing `&self`
   (or an `Arc`) into the closures instead of a deep clone would restore
   O(|A|) work; logarithmic span would additionally need a tree-backed or
   parallel `split`/`union`.

3. **Construction costs overstated (34 lines).** The `from_sets` /
   `from_vertices_and_labeled_arcs` / `from_vertices_and_labeled_edges`
   trait annotations claim O(|V| + |A|), but every implementation moves
   the two `SetStEph` values into the struct: O(1). The weighted
   `from_weighed_edges` (13 files) iterates and copies only the edge set
   and moves the vertex set: O(|E|), not O(|V| + |E|).

Other observations (costs agree with the old lines, no change needed):

- All St neighbor, degree, label, and weight queries are sequential
  O(|A|) scans. `has_arc` / `get_arc_label` / `get_edge_weight` scan
  because the labeled-edge hash set is keyed by the full (u, v, label)
  triple; a lookup without the label cannot use the hash.
- The 12 integer `WeightedDirGraphStEph*` files and the F64 file are
  cost-identical copies differing only in the weight type.
- `ng_of_vertices` in the St files unions into an accumulator whose size
  is bounded by 2|A|, so O(|S|·|A|) holds.
