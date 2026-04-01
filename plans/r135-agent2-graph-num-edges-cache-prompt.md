# R135 Agent 2 — Fix num_edges: cache edge count in graph modules. AFK.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r135-agent2-graph-num-edges-report.md`

## Problem

Five graph Mt files implement `num_edges` by counting edges every call instead
of caching. APAS assumes O(1) cached edge count.

| # | File | Current impl | APAS cost |
|---|------|-------------|-----------|
| 1 | Chap52/AdjMatrixGraphMtEph.rs | counts trues in n×n matrix: O(n²) | O(1) |
| 2 | Chap52/AdjMatrixGraphMtPer.rs | counts trues in n×n matrix: O(n²) | O(1) |
| 3 | Chap52/AdjSeqGraphMtEph.rs | sums degrees over all vertices: O(n) | O(1) |
| 4 | Chap52/AdjSeqGraphMtPer.rs | sums degrees over all vertices: O(n) | O(1) |
| 5 | Chap52/AdjTableGraphMtPer.rs | sums degrees over all vertices: O(n) | O(1) |

Also check the St versions of these files — they likely have the same issue.

## What to do

For each file:

1. Read the struct definition. Add a `num_edges: usize` field (or `ghost num_edges`
   if the count is only needed for specs).

2. Read `new` / constructor — initialize `num_edges` to 0 (or compute from input).

3. Read `add_edge` / `insert_edge` / `set_edge` — increment `num_edges` when an
   edge is added. Be careful with duplicates: only increment if the edge didn't
   already exist.

4. Read `delete_edge` / `remove_edge` — decrement `num_edges` when an edge is
   removed. Only decrement if the edge existed.

5. Rewrite `num_edges(&self)` to return the cached field: O(1).

6. Update wf predicate if needed — the cached count should equal the actual count.
   Something like `self.num_edges as nat == actual_edge_count(self)`.

7. Update the alg analysis annotation from O(n²)/O(n) to O(1).

8. Check `from_vertices_and_edges` / `from_vertices_and_arcs` constructors — they
   should compute the initial edge count from the input.

## Also check set_edge

`AdjMatrixGraphMtPer.rs:197` and `AdjSeqGraphMtEph.rs:186` have `set_edge` that
DIFFERS from APAS. Read these — if the issue is that they rebuild instead of
updating in place, that may be a separate fix or may be inherent to the persistent
data structure.

## Validation

Run `scripts/validate.sh isolate Chap52`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- The cached count must be proved equal to the actual count in the wf predicate.
- Update all annotations that reference num_edges cost.

## When done

RCP.
