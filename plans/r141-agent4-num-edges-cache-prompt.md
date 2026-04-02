# R141 Agent 4 — Cache num_edges in AdjTableGraphMtPer. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap52/AdjTableGraphMtPer.rs` — the file to modify.
Read `src/Chap52/AdjSeqGraphStEph.rs` — example of cached num_edges field.
Read `src/Chap52/AdjMatrixGraphMtPer.rs` — another example.

Report file: `plans/r141-agent4-num-edges-cache-report.md`

## Problem

`AdjTableGraphMtPer::num_edges` sums adjacency set sizes in a loop — O(n+m)
work. APAS CS 52.3 says O(1). Other graph types (AdjMatrixGraphMtPer,
AdjSeqGraphStEph) already cache num_edges as a field.

## What to do

1. Add `num_edges: usize` field to the `AdjTableGraphMtPer` struct.

2. Add `self.num_edges as nat == spec_sum_adj_sizes(self.adj@)` (or equivalent)
   to the wf predicate so the cached value is always correct.

3. Update `num_edges(&self)` to return `self.num_edges` — O(1).

4. Update `add_edge` to increment: `num_edges: self.num_edges + 1` (when the
   edge didn't exist).

5. Update `delete_edge` to decrement: `num_edges: self.num_edges - 1` (when
   the edge existed).

6. Update `add_vertex` — doesn't change edge count.

7. Update `delete_vertex` — decrements by the degree of the deleted vertex
   (both in-edges and out-edges). Check the current implementation to see how
   edges are cleaned up.

8. Update `new`/`empty` — `num_edges: 0`.

9. Update `from_adjacency` if it exists — compute initial count.

10. Update Clone, Debug, Display, PartialEq if they reference struct fields.

11. Update the annotation from DIFFERS to matches APAS.

## Proof

The wf invariant `num_edges == spec_sum_adj_sizes(adj@)` must be maintained
through all operations. The key proofs:
- add_edge: if !has_edge(u,v), new sum = old sum + 1
- delete_edge: if has_edge(u,v), new sum = old sum - 1
- delete_vertex: new sum = old sum - out_degree(v) - in_degree_from_others(v)

Look at how AdjSeqGraphStEph maintains its cached count for reference.

## Validation

Run `scripts/validate.sh isolate Chap52`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- The cached value must be proved correct from wf, not assumed.
- O(1) num_edges after the fix.

## When done

RCP.
