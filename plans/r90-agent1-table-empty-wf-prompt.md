# R90 Agent 1 — Table::empty ensures wf + cascade AdjTableGraph proofs, STEP 20

## Objective

Add `ensures wf` to `Table::empty()` in TableStEph and TableStPer, then use
that to prove as many AdjTableGraph holes as possible.

## Background

Agent1 in R89 added `find_ref` + `spec_stored_value` to Table and proved 3
AdjTableGraph holes. The report identified the next blocker: `Table::empty()`
doesn't ensure `spec_tablesteph_wf()`. This blocks `AdjTableGraph::empty()`
which blocks 4+ downstream functions (insert_vertex, delete_vertex, insert_edge,
delete_edge).

## Step 1: Strengthen Table::empty ensures

In `src/Chap42/TableStEph.rs`, the trait says:
```rust
fn empty() -> (empty: Self)
    ensures empty@ == Map::<K::V, V::V>::empty();
```

Add `empty.spec_tablesteph_wf()` to ensures. The impl creates
`TableStEph { entries: ArraySeqStEphS::empty() }` — proving wf means showing
`spec_keys_no_dups` on an empty sequence (trivial) and `obeys_feq_fulls`
(type-level, should be provable from bounds or assumed at the trait level).

Do the same for `src/Chap42/TableStPer.rs`.

## Step 2: Prove AdjTableGraph holes

With `empty` ensuring wf, work through AdjTableGraphStEph:
1. `empty` — should verify directly with Table::empty wf
2. `from_table` — may need caller to ensure stored values are wf
3. `insert_vertex` — Table::insert preserves wf? Check.
4. `delete_vertex` — Table iteration + nested set ops
5. `insert_edge` — Table find_ref + set insert + Table insert
6. `delete_edge` — Table find_ref + set delete + Table insert
7. `vertices` — domain iteration
8. `num_edges` — domain iteration + sum sizes
9. `out_neighbors` — clone wf gap (may remain blocked)

Prove as many as you can. Each one you prove in StEph can be ported to StPer
and MtPer.

## Read first

- `src/Chap42/TableStEph.rs` — Table trait + impl, your R89 `find_ref` additions
- `src/Chap42/TableStPer.rs` — same for persistent
- `src/Chap52/AdjTableGraphStEph.rs` — your primary target
- `src/Chap52/AdjTableGraphStPer.rs` — port target
- `src/Chap52/AdjTableGraphMtPer.rs` — Mt port target

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers. Check Chap43 still compiles:
  `scripts/validate.sh isolate Chap43`
- Do NOT add assume or accept.
- Do NOT weaken ensures.
- If Table::insert/delete need wf ensures too, add them.
- The goal is maximum Chap52 hole reduction. Every function proved in StEph
  is a template for StPer and MtPer.

## STEP 20

## Report

Write `plans/agent1-r90-table-wf-report.md`.
