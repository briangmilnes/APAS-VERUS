# Algorithmic analysis issues: annotations that disagree with the code

Date: 2026-09-21. Source: `docs/HashMigration.md` §3 (812 `/// - Alg Analysis`
annotations in the r209 write set, every one read against the body it
annotates) and §5 item 1. Status: OPEN. No annotation and no algorithm has
been changed; each row is a claim in a comment that the code as written does
not support.

The rule that governs every row: the algorithms stay as they are. The issue in
each row is that a `/// - Alg Analysis` line states a cost the body does not
have. Two resolutions exist per row, and both are the user's call: correct the
annotation to what the code does, or change the algorithm to what the
annotation promises. The second is a textbook-fidelity question (APAS gives
the parallel cost), not a proof question, and is out of scope for the Verus
upgrade rounds.

## 1. Summary

| # | Measurement | Value |
|---|-------------|------:|
| 1 | annotations read | 812 |
| 2 | confirmed | 604 |
| 3 | textbook quotes (state the book's bound, not a claim about the body) | 136 |
| 4 | disputed | 72 |
| 5 | disputed lines explained by one cause: a sequential merge after a `ParaPair!` fork | 48 (D5–D10, D14, D15) |
| 6 | files with a disputed line | 17 |

## 2. The systematic finding: fork, then merge sequentially

Every divide-and-conquer helper in the Mt files of Chap61, 62, and 66 splits
its input, forks the two halves with `ParaPair!`, and then combines the two
results with a sequential loop: a `for kv in right.iter()` insert loop, a
`while i < right.len()` push loop, or a `left.union(&right)`. The recurrences
are therefore

    Span  S(n) = S(n/2) + Θ(n)      = Θ(n)
    Work  W(n) = 2 W(n/2) + Θ(n)    = Θ(n lg n)

and not the annotated Span O(lg n), Work O(n). Every caller that adds O(lg n)
levels on top of these helpers inherits a factor of n in its span. The textbook
bound needs a parallel merge (a `Set::union` or map merge with O(lg n) span),
which is an algorithm change and is not proposed here.

## 3. Disputed annotations by code

Work and span in the "code says" column are read from the bodies. Line numbers
are in the working tree at the end of r209.

| # | Code | Chap | File | Lines | Fn | Annotation says | Code says |
|---|------|------|------|-------|----|-----------------|-----------|
| 1 | D1 | 17 | MathSeq.rs | 230, 617 | `iter`, `iter_mut` | "returns iterator wrapper" | returns `slice::Iter` directly since r208; O(1) stands, wording stale |
| 2 | D2 | 50 | OptBinSearchTreeStEph.rs | 124, 131, 142, 215, 221, 231 | `set_key_prob`, `update_prob`, `clear_memo` | Work O(1), Span O(1) | each calls `memo.clear()`: Θ(\|memo\|) ≤ O(n²), keys are `(i, l)` pairs |
| 3 | D3 | 50 | MatrixChainStEph.rs | 215, 223, 232, 421, 427, 433 | `set_dimension`, `update_dimension`, `clear_memo` | Work O(n) "clears memo HashMap" | `memo.clear()` is Θ(\|memo\|) ≤ O(n²) for n matrices |
| 4 | D4 | 61 | EdgeContractionMtEph.rs | 59, 68, 86, 234 | trait lines and first review line | Span O(lg \|V\|), O(1), or O(degree) | sequential loops at 112, 118, 131: Span Θ(\|V\|+\|E\|); `should_select_edge` scans all edges: Work Θ(\|E\|²) |
| 5 | D4 | 61 | VertexMatchingMtEph.rs | 59, 75, 113, 146, 229 | same pattern | same | loops at 130, 163, 254; the second review line under each fn already states the code's cost |
| 6 | D5 | 61 | EdgeContractionMtEph.rs | 148 | `build_edges_parallel` | Work Θ(k), Span Θ(lg k) | `ParaPair!` then sequential `left.union(&right)`: Work Θ(k lg k), Span Θ(k) |
| 7 | D5 | 62 | StarContractionMtEph.rs | 260, 328 | `build_quotient_graph_parallel`, `route_edges_parallel` | same | same |
| 8 | D6 | 62 | StarPartitionMtEph.rs | 115, 1518 | `parallel_star_partition` | Span O(lg(n+m)) | six D&C loops with linear-span merges: Span Θ(n+m); Work O((n+m) lg(n+m)) stands |
| 9 | D7 | 62 | StarPartitionMtEph.rs | 144, 268, 441, 948, 1419 | `hash_coin_flips_mt`, `build_th_edges_mt`, `build_p_vec_mt`, `build_p_vec_with_inject_mt`, `build_centers_mt` | Work O(n), Span O(lg n) "D&C fork-join" | sequential merge after the fork (inserts at 207, 253, 457, 537, 709; pushes at 413, 499, 1118): Work Θ(n lg n), Span Θ(n) |
| 10 | D7 | 66 | BoruvkaMtEph.rs | 79, 204, 266, 334, 402, 473, 474, 639, 833 | `vertex_bridges_mt`, `hash_coin_flips_mt`, `compute_remaining_mt`, `collect_mst_labels_mt`, `build_partition_map_mt`, `filter_tail_to_head_mt`, `reroute_edges_mt` | same | pushes at 317, 384, 906; `union` at 1492: same |
| 11 | D8 | 62 | StarPartitionMtEph.rs | 548, 729, 1211 | `build_vertex_to_index_mt`, `build_satellite_map_mt`, `build_partition_map_mt` | Work O(n lg n), Span O(lg n) "sequential merge" | work right; the root merge alone is Θ(n): Span Θ(n) |
| 12 | D9 | 62 | StarContractionMtEph.rs | 93, 116, 127, 208, 438 | `star_contract_mt` and callers | Span O(lg² n) | linear span per level (D5, D6, sequential `compose_maps_parallel`): Span Θ((n+m) lg n); sibling review lines 209, 439 already say so |
| 13 | D9 | 63 | ConnectivityMtEph.rs | 77, 83, 92, 98, 115, 139, 203, 224 | connectivity via star contraction | Span O(lg² n) | Θ((n+m) lg n); lines 116, 140, 204, 225 already say so |
| 14 | D9 | 64 | SpanTreeMtEph.rs | 66 | spanning tree via star contraction | Span O(lg² n) | Θ((n+m) lg n); line 86 already says so |
| 15 | D10 | 62 | StarContractionMtEph.rs | 209 | `star_contract_mt` | "quotient build O(lg m) via ParaPair" | ends in `union` (D5): Θ(m) span; headline Span O(n lg n) should read O((n+m) lg n) |
| 16 | D11 | 64 | SpanTreeStEph.rs, SpanTreeMtEph.rs | 82, 85 | headline lines | `Work O((n+m)` and stops | truncated mid-formula; the next line carries the full analysis |
| 17 | D12 | 65 | PrimStEph.rs | 91, 269 | `prim_mst` | Work O(m lg n) | `LabUnDirGraphStEph` keeps edges in a flat set, so `ng()` and `get_edge_label()` are O(m) per call: Work O(m² lg n), Span the same; line 270 already says so |
| 18 | D13 | 66 | BoruvkaStEph.rs | 172 | `boruvka_mst_with_seed` | Span O(lg³ n) | St file, sequential: Span = Work = O(m lg n); line 174 says so |
| 19 | D14 | 66 | BoruvkaMtEph.rs | 94, 580, 581 | `bridge_star_partition_mt` | Span O(lg \|V\|) | calls D7 helpers (Θ(n)) and sequential loops at 614, 626: Span Θ(n) |
| 20 | D15 | 66 | BoruvkaMtEph.rs | 109, 127, 726, 923 | `boruvka_mst_mt`, `boruvka_mst_mt_with_seed` | Work O(m lg n), Span O(lg² n) | O(lg n) rounds, each Θ(m) span and Θ(m lg m) work: Span Θ(m lg n), Work Θ(m lg m lg n) |

Line counts per code: D1 2, D2 6, D3 6, D4 9, D5 3, D6 2, D7 14, D8 3, D9 14,
D10 1, D11 2, D12 2, D13 1, D14 3, D15 4; total 72.

## 4. Kinds of disagreement, and what resolving each means

| # | Kind | Codes | Lines | If the annotation is corrected | If the algorithm is changed |
|---|------|-------|------:|-------------------------------|-----------------------------|
| 1 | sequential merge after a parallel fork | D5, D6, D7, D8, D10, D14, D15 | 30 | replace Span O(lg n) by Θ(n) and Work O(n) by Θ(n lg n) in the helpers; propagate to callers | write a parallel merge for `SetStEph`/`HashMap` results with O(lg n) span; then the annotations hold as written |
| 2 | callers of kind 1 | D9 | 14 | Span O(lg² n) becomes Θ((n+m) lg n); the sibling review lines already state it, so the fix is to delete the contradicted line | follows from kind 1 |
| 3 | trait line contradicted by the impl's own second review line | D4, D12, D13 | 12 | delete the contradicted line; the correct one is already present | change `ng()` and `get_edge_label()` to adjacency lists (D12); parallelise the Chap61 loops (D4) |
| 4 | memo `clear()` cost | D2, D3 | 12 | Work O(1) or O(n) becomes O(\|memo\|) ≤ O(n²) | none: `HashMap::clear` is linear in entries |
| 5 | stale wording | D1 | 2 | say "returns `slice::Iter`" | none |
| 6 | truncated line | D11 | 2 | delete the fragment | none |

Kinds 4, 5, and 6 are annotation errors with nothing to argue. Kinds 1 to 3 are
where the textbook's parallel bound and the implementation differ, and the
user decides which side moves.

## 5. Not in this document

Files outside the r209 write set were not read for this purpose. The
migration rounds check every annotation in every file they edit
(`plans/r208-agent1-chap02-06.md` rule 9, `plans/r209-agent1-hash-migration.md`
rule 2), so this list grows as chapters are migrated. r208 (Chap02–06)
reported no disputed line.

## 6. r211 (the manual-loop rewrite in Chap05 and Chap06)

r211 (`docs/RunTimeTestsRestored.md`) read every `Alg Analysis` line in the
26 source files it edited (1067 lines) and edited none. Its one exec change,
the manual-loop form on the iterator's own `next()`, keeps every statement
of each loop body and one `next()` per iteration, so no Code review line's
cost is disputed by it. One format observation:

| # | Code | Chap | File | Lines | Function(s) | Observation |
|---|------|------|------|-------|-------------|-------------|
| 21 | F1 | 05 | MappingStEph.rs | 185, 191, 197, 203, 209 | the five `is_functional_*` fns | APAS "no cost" line; standard forbids it |

Row 21: the five lines read `Alg Analysis: APAS (Ch05 Def 5.6): definitional
predicate, no cost specified.`; `src/standards/alg_analysis_notation_standard.rs`
says a function with no APAS cost specification carries only its Code review
line, not an APAS line saying there is none. The Code review lines under
them are right. Not edited by r211.
