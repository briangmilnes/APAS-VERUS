# R87 Agent 1 — Fix Prim PQEntry total order + mst_weight overflow (Prim + Kruskal), STEP 20

## Objective

Fix 4 assumes and 2 missing spec warnings in PrimStEph.rs and KruskalStEph.rs.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent1/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Fix 1: PQEntry lexicographic total order

PrimStEph.rs has `TotalOrder for PQEntry<V>` with assumes on `antisymmetric` and
the `cmp` equal case. The problem: `le` compares only priority, so two entries with
same priority but different vertices satisfy `le(a,b) && le(b,a)` without `a == b`.

**Fix**: make `le` lexicographic on (priority, vertex):

```rust
open spec fn le(self, other: Self) -> bool {
    self.priority < other.priority
    || (self.priority == other.priority && TotalOrder::le(self.vertex, other.vertex))
}
```

Then:
- `reflexive`: priority == priority && le(v, v) by V::reflexive
- `transitive`: case split on priority <, ==; chain with V::transitive
- `antisymmetric`: priority == && le(v1,v2) && le(v2,v1) → v1 == v2 by V::antisymmetric → entries equal
- `total`: priority < or > gives one direction; priority == delegates to V::total
- `cmp`: compare priority first, if equal compare vertex

The vertex type V has `TotalOrder` (it's `HashOrd` which implies `Ord`).
Read `src/vstdplus/total_order.rs` for the TotalOrder trait and integer impls.

All 4 proof methods should have non-empty but straightforward bodies — call the
corresponding V proof method in the equal-priority case. No assumes needed.

For `cmp`: compare priority with `std::cmp::Ord::cmp`, if Equal then compare vertex.

## Fix 2: mst_weight overflow

Both PrimStEph and KrustalStEph have `mst_weight` functions that sum edge weights
using u64 addition with `assume(total + edge.weight <= u64::MAX)`.

**Fix**: add `requires` bounding the total weight:

```rust
fn mst_weight(mst: &SetStEph<LabEdge<V, u64>>) -> (total: u64)
    requires
        mst.spec_setsteph_wf(),
        spec_total_weight(mst) <= u64::MAX as int,
    ensures
        total as int == spec_total_weight(mst),
```

Add a `spec fn spec_total_weight` that computes the sum. The loop invariant
carries `total as int == partial_sum` and the requires guarantees no overflow.

Alternatively, simpler: just add `requires` stating that the sum fits, and
keep the loop simple. The caller proves the sum fits.

## Fix 3: Missing requires/ensures warnings

- `sort_edges_by_weight` (KrustalStEph): add real `requires` (it already has
  one about finiteness — may just need the veracity annotation)
- `pq_entry_new` (PrimStEph): add `requires` for the priority/vertex validity
- Both `mst_weight` functions: add `ensures` relating result to spec

## Important

- Do NOT modify UnionFindStEph.rs — another agent works on it.
- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.

## STEP 20

## Report

Write `plans/agent1-round87-report.md`.
