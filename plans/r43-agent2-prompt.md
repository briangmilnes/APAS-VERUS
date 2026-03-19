# R43 Agent 2: Chap63 Connectivity + Chap64 SpanTree + TSPApprox (20 holes)

## Baseline

- Main at `100439a2`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4362 verified, 0 errors, 139 holes
- Chap05 SetStEph is clean (dependency satisfied)
- All 20 holes are `#[verifier::external_body]` on graph algorithm implementations

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert `assume()` to `accept()`.**
**DO NOT move code outside `verus!{}` or add `#[cfg(not(verus_keep_ghost))]` to
dodge verification.** All algorithm implementations belong inside `verus!{}`.
**DO NOT add `external_body` without explicit user approval.** If the proof is hard,
leave the existing `external_body` in place and report what you tried.
**NEVER sequentialize parallel files.** MtEph implementations must stay parallel.
Do not replace threaded code with sequential loops.

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.
Read `src/standards/using_closures_standard.rs` — all five files use closures.

## Files and Hole Counts

| # | Chap | File | Holes | Notes |
|---|------|------|-------|-------|
| 1 | 63 | ConnectivityStEph.rs | 5 | BFS-style star contraction, sequential |
| 2 | 63 | ConnectivityMtEph.rs | 7 | Parallel star contraction, ParaPair |
| 3 | 64 | SpanTreeStEph.rs | 2 | Delegates to star_contract |
| 4 | 64 | SpanTreeMtEph.rs | 1 | Parallel, verify_spanning_tree is OPAQUE_EXTERNAL |
| 5 | 64 | TSPApproxStEph.rs | 5 | DFS euler_tour, shortcut, tour_weight, TSP approx |

**Total: 20 holes + 1 fn_missing_ensures warning in TSPApproxStEph.rs**

## Dependency Context

Chap63 and Chap64 call into Chap62 (`star_contract`, `star_contract_mt`,
`sequential_star_partition`, `parallel_star_partition`). Those Chap62 callees are
themselves `external_body`. This means:

- You cannot prove the callers by opening up the callee bodies — they are opaque.
- What you CAN do: prove each function's own body is correct given the caller's
  preconditions, using the callee's `requires`/`ensures` as axioms.
- For functions whose entire body is a single call to an opaque external function,
  the proof work is: add real `ensures` to the trait method (if missing or weak),
  add real `requires` (if missing or incorrect), then remove `external_body` and
  let Verus check that the call satisfies the callee's requires.

Read each callee's trait signature carefully to know what ensures they export.
The callee ensures are the facts you can use.

## Part A: Chap63 ConnectivityStEph.rs (5 holes)

### File: `src/Chap63/ConnectivityStEph.rs`

Actual holes (from veracity analysis):

| # | Chap | Function | Line | Pattern |
|---|------|----------|------|---------|
| 1 | 63 | `count_components` | 80 | Recursive: base (no edges), then star partition + quotient graph + recurse |
| 2 | 63 | `connected_components` | 109 | Recursive: base (no edges), then star partition + compose maps |
| 3 | 63 | `build_quotient_edges` | 141 | Helper: iterate edges, remap via partition_map, collect non-self-loop edges |
| 4 | 63 | `count_components_hof` | 173 | Delegates to `star_contract` with base/expand closures |
| 5 | 63 | `connected_components_hof` | 189 | Delegates to `star_contract` with base/expand closures |

### What each function does

`build_quotient_edges`: Iterates over all edges `Edge(u, v)` in the graph. Looks up
`u` and `v` in `partition_map` (defaulting to themselves if absent). If `u_center !=
v_center`, adds a normalized `Edge(min, max)` to the quotient set. Body is simple
and self-contained — does not call any external_body function.

`count_components`: Recursive. Base: if no edges, return `graph.sizeV()`. Inductive:
call `sequential_star_partition` (external_body callee), call `build_quotient_edges`,
construct quotient graph, recurse.

`connected_components`: Recursive. Base: if no edges, build identity map and return
vertices. Inductive: same star partition, then recurse on quotient graph, compose maps.

`count_components_hof`, `connected_components_hof`: Each calls `star_contract` with
inline closures for `base` and `expand`. The closures capture no state in
`count_components_hof`. `connected_components_hof` has more complex closures.

### Proof strategy

1. **`build_quotient_edges`**: Simplest hole. The body is a loop over graph edges with
   no external_body callee. Add real `ensures` to the trait if missing, then remove
   `external_body`. Likely ensures: result contains only edges where endpoints differ,
   all result edges are normalized (`u <= v`). Use `SetStEph` membership reasoning.

2. **`count_components` and `connected_components`**: These call external_body callees
   (`sequential_star_partition`, `build_quotient_edges`). Check whether the callee's
   `ensures` export enough information to prove the caller's postconditions. If the
   callees' specs are too weak, you may only be able to strengthen the caller's `ensures`
   rather than remove `external_body`. Do the stronger spec work first; the `external_body`
   removal can follow if the callee specs support it.

3. **`count_components_hof` and `connected_components_hof`**: These call `star_contract`,
   which is external_body. Read `star_contract`'s trait method signature in
   `src/Chap62/StarContractionStEph.rs` to understand what ensures it exports. The
   closures need their `requires`/`ensures` propagated per the closures standard. Read
   `src/standards/using_closures_standard.rs` before touching these.

### TOC check

Verify that the file's `Table of Contents` section ordering matches CLAUDE.md standard.
The `ConnectivityStEphTrait` impl is not currently present (trait methods are declared
but the impl block appears to be using free functions). If there is a bare `impl` block
missing the trait, that is a violation of the trait-impl pattern. Read the file before
drawing conclusions.

## Part B: Chap63 ConnectivityMtEph.rs (7 holes)

### File: `src/Chap63/ConnectivityMtEph.rs`

Actual holes:

| # | Chap | Function | Line | Pattern |
|---|------|----------|------|---------|
| 1 | 63 | `count_components_mt` | 90 | Recursive parallel, calls `parallel_star_partition` |
| 2 | 63 | `connected_components_mt` | 119 | Recursive parallel, calls `parallel_star_partition` |
| 3 | 63 | `build_quotient_edges_parallel` | 149 | Collects edges into ArraySeqStEphS, calls `route_edges_parallel` |
| 4 | 63 | `route_edges_parallel` | 168 | Divide-and-conquer with `ParaPair!` macro |
| 5 | 63 | `compose_maps_parallel` | 221 | Sequential map composition (currently named "parallel" but is sequential) |
| 6 | 63 | `count_components_hof` | 239 | Delegates to `star_contract_mt` |
| 7 | 63 | `connected_components_hof` | 253 | Delegates to `star_contract_mt` |

### Proof strategy

Same general approach as Part A, but with parallel patterns:

- `compose_maps_parallel`: Body is a sequential loop over `partition_map`. No external_body
  callee. This is the best candidate for removing `external_body` first — the logic is
  self-contained. Add real `ensures` (output maps every key in `partition_map` to its
  composed component), then prove the loop maintains the invariant.

- `route_edges_parallel`: Uses `ParaPair!` macro. Read
  `src/standards/using_closures_standard.rs` before modifying. Closures must have named
  variables with explicit `ensures`. The `ParaPair!` macro wraps `join()`.

- `build_quotient_edges_parallel`: Calls `route_edges_parallel`. If `route_edges_parallel`
  stays external_body, this cannot be easily proven either.

- `count_components_mt`, `connected_components_mt`, the two `_hof` variants: Same
  dependency analysis as Part A — these call external_body callees in Chap62.

**NEVER sequentialize the Mt file.** The `ParaPair!` and `join()` calls must stay.
Do not replace them with sequential loops to make proofs easier.

## Part C: Chap64 SpanTreeStEph.rs (2 holes)

### File: `src/Chap64/SpanTreeStEph.rs`

| # | Chap | Function | Line | Notes |
|---|------|----------|------|-------|
| 1 | 64 | `spanning_tree_star_contraction` | 53 | Uses `star_contract` with expand closure; expand is a complex nested loop |
| 2 | 64 | `verify_spanning_tree` | 110 | Checks edge count and edge membership; no external_body callee |

### Proof strategy

`verify_spanning_tree`: Body iterates tree edges, calls `graph.Neighbor(u, v)`. No
external_body callees. This is the simplest Chap64 hole. Add real `ensures` to the
trait method (currently has none beyond the function signature), then remove
`external_body`. The function should ensure: result is true iff the tree has `n-1`
edges and all edges are in the graph. Use `LabUnDirGraphStEph` membership specs.
Note: the current trait signature has `ensures` for some things — read it first.

`spanning_tree_star_contraction`: Calls `star_contract` with a complex `expand` closure
that has two nested loops. The closure captures `partition_map` and searches `original_edges`
for each quotient edge. This is the hardest Chap64 StEph hole. The proof requires:
- The expand closure has correct `requires`/`ensures` per the closures standard.
- `star_contract`'s `ensures` exports enough structure to prove the outer function's
  postconditions.

Read `star_contract`'s spec in `src/Chap62/StarContractionStEph.rs` first.

## Part D: Chap64 SpanTreeMtEph.rs (1 hole)

### File: `src/Chap64/SpanTreeMtEph.rs`

| # | Chap | Function | Line | Notes |
|---|------|----------|------|-------|
| 1 | 64 | `spanning_tree_star_contraction_mt` | 76 | Parallel, uses `star_contract_mt` with join() in expand closure |

Note: `verify_spanning_tree` in SpanTreeMtEph.rs is tagged OPAQUE_EXTERNAL by veracity
(structural false positive). It is not a proof target this round.

### Proof strategy

The `expand` closure in `spanning_tree_star_contraction_mt` uses an `Arc<RwLock<...>>`
for shared mutable state and calls `join()` twice (once for star edges, once for quotient
edges). The Arc write pattern is established in `SpanTreeMtEphEdgesInv` at the top of
the file.

Before touching: read `src/standards/using_closures_standard.rs` and
`src/standards/arc_usage_standard.rs`. The closure's `ensures` must express what the
Arc contains after both `join()` calls.

If the expand closure cannot be verified directly (too complex), consider whether the
function-level spec can be strengthened instead of removing `external_body`.

## Part E: Chap64 TSPApproxStEph.rs (5 holes + 1 warning)

### File: `src/Chap64/TSPApproxStEph.rs`

| # | Chap | Function | Line | Notes |
|---|------|----------|------|-------|
| 1 | 64 | `euler_tour` | 93 | Calls `euler_tour_dfs`; uses `HashSetWithViewPlus` |
| 2 | 64 | `euler_tour_dfs` | 111 | Recursive DFS with `&mut Vec<V>` and `&mut HashSetWithViewPlus` |
| 3 | 64 | `shortcut_tour` | 175 | Loop with `HashSetWithViewPlus` visited set, no external callee |
| 4 | 64 | `tour_weight` | 206 | Loop summing `WrappedF64` edge weights, calls `get_edge_weight` |
| 5 | 64 | `approx_metric_tsp` | 269 | Sequences euler_tour + shortcut_tour + tour_weight |
| W | 64 | `get_edge_weight` | 238 | `fn_missing_ensures` warning — add ensures clause |

Also note: `euler_tour_dfs` has `&mut` parameters. Read `src/standards/mut_standard.rs`
before modifying it.

### Proof strategy

**Fix `get_edge_weight` warning first** (easiest): Add an `ensures` clause. The function
calls `graph.get_edge_label(u, v)` and wraps the result. The ensures should express what
`Some(w)` means in terms of the graph spec. Read `LabUnDirGraphStEph`'s `get_edge_label`
spec to find the right postcondition.

`shortcut_tour`: Body is a loop over `euler_tour` slice with a `HashSetWithViewPlus`
visited set. No external_body callees. Good candidate for early removal. The loop
invariant: `shortcut` contains each first-seen vertex from `euler_tour[0..i]` in order,
`visited` tracks which vertices have been seen. Add real `ensures` (each vertex appears
at most once in `shortcut`, except the start vertex appended at end).

`tour_weight`: Calls `get_edge_weight` (which has a real body). After fixing the
`get_edge_weight` ensures, this function's loop sums `WrappedF64` values. Floating-point
arithmetic axioms in `vstdplus/float.rs` are limited — check what is available there
before writing ensures. The ensures can conservatively state that the result equals the
sum of weights for consecutive vertex pairs.

`euler_tour` and `euler_tour_dfs`: DFS with mutable state. `euler_tour_dfs` is recursive
with `&mut Vec<V>` and `&mut HashSetWithViewPlus<(V, V)>`. Read `mut_standard.rs` for
the `*final(x)` pattern (`-V new-mut-ref` mode is enabled in this project). The
recursion needs a decreases clause (decreasing number of unvisited tree edges).

`approx_metric_tsp`: Simple sequencing of the three sub-algorithms. Once the callee
`ensures` are proven, this function's body is straightforward to verify.

## Execution Order

Work files in this order — from lowest to highest complexity:

1. `TSPApproxStEph.rs` — fix `get_edge_weight` warning first (1 ensures clause)
2. `TSPApproxStEph.rs` — `shortcut_tour` (self-contained loop, no external callee)
3. `SpanTreeStEph.rs` — `verify_spanning_tree` (self-contained loop, no external callee)
4. `ConnectivityStEph.rs` — `build_quotient_edges` (self-contained loop, no external callee)
5. `ConnectivityMtEph.rs` — `compose_maps_parallel` (self-contained loop, no external callee)
6. `TSPApproxStEph.rs` — `tour_weight` (depends on get_edge_weight ensures)
7. `TSPApproxStEph.rs` — `euler_tour_dfs` then `euler_tour` (recursive DFS with mut)
8. `TSPApproxStEph.rs` — `approx_metric_tsp` (depends on above)
9. `ConnectivityStEph.rs` — remaining 4 holes (depend on star_contract callee specs)
10. `ConnectivityMtEph.rs` — remaining 6 holes (depend on star_contract_mt callee specs)
11. `SpanTreeStEph.rs` — `spanning_tree_star_contraction` (complex expand closure)
12. `SpanTreeMtEph.rs` — `spanning_tree_star_contraction_mt` (complex parallel expand)

For steps 9-12: read the callee specs in Chap62 before deciding whether to prove or
leave. If callee specs are too weak to support the caller's postconditions, document
what spec the callee would need, add that spec work to the report, and leave the holes.

## Validation Protocol

Run `scripts/validate.sh` after each file or pair of files. Do NOT run rtt or ptt
at the same time as validate. Sequential only.

If validate fails: read the error, fix it, run again. Do not proceed to the next
file until the current file verifies clean.

After all files that you verified: run `scripts/rtt.sh`, then `scripts/ptt.sh`.
All must pass before you push.

**Never pipe or filter `scripts/validate.sh` output.** Show the full output.

## Search Before Writing

Before writing a new lemma or proof step, search for existing vstd or APAS-VERUS
support:

```bash
veracity-search 'proof fn .*finite'
veracity-search 'fn _ types HashSetWithViewPlus'
veracity-search 'lemma.*seq.*len'
```

Check `~/projects/verus/source/vstd/` for relevant lemmas. Custom lemmas create
proof islands — prefer ecosystem lemmas.

## Commit and Push Protocol

Commit after each file that verifies clean. Use `git add -A` to stage everything.
Push after each commit. This protects work and gives the orchestrator visibility.

Commit message format:
```
R43: Chap63/64 — <brief description of what was proved>
```

Example: `R43: Chap64 TSPApprox — shortcut_tour + verify_spanning_tree`

## Report

When done, write your summary to `plans/agent2-r43-report.md`.

Include:

1. Holes before/after per file (table with # / Chap / File / Before / After columns).
2. Techniques used per function proved.
3. Remaining holes with a precise description of what blocked each.
4. Callee spec gaps discovered (if Chap62 callees needed stronger ensures to
   enable Chap63/64 proofs, document the specific missing postcondition).
5. Final verified count and error count from `scripts/validate.sh`.

Every table in the report that references source files MUST include a Chap column.
