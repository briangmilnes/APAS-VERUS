# R79 Agent 5 — PrimStEph TotalOrder: explore tie-breaking fix (Chap65, 2 holes)

## Objective

The 2 PrimStEph holes are `assume` + `external_body` in the TotalOrder impl for PQEntry.
Agent 4 R78 confirmed these are structural: the TotalOrder trait requires antisymmetric
(`le(x,y) && le(y,x) ==> x == y`) but PQEntry is a preorder on priority — equal priority
does not imply equal entry.

**Explore whether adding a tie-breaking field to PQEntry's `le` spec fixes this.**

## Current state

```rust
impl TotalOrder for PQEntry<V> {
    open spec fn le(self, other: Self) -> bool {
        self.priority.spec_le(other.priority)
    }
}
```

This makes `le` a preorder, not a total order. Antisymmetric fails because
`le(x,y) && le(y,x)` means equal priorities, not equal entries.

## Proposed fix

Add vertex comparison as tie-breaker:
```rust
open spec fn le(self, other: Self) -> bool {
    self.priority.spec_lt(other.priority)
    || (self.priority.spec_eq(other.priority) && self.vertex@ <= other.vertex@)
}
```

This makes `le` a total order (assuming vertex@ values are unique, which they are in
Prim's algorithm — each vertex appears at most once in the PQ).

**Challenges**:
- Need `spec_lt` and `spec_eq` on WrappedF64 — check if `vstdplus/float.rs` provides these
- Need `<=` on vertex view type — check what `V: HashOrd` provides
- May need to change `cmp` implementation to match the new spec
- The 4 proof functions (reflexive, transitive, antisymmetric, total) need proofs using
  float axioms + vertex ordering axioms

If this approach doesn't work due to missing float axioms, document what's needed and
leave the holes.

## Key resources

- `src/Chap65/PrimStEph.rs` — Read fully
- `src/vstdplus/float.rs` — Float axioms and specs
- `src/vstdplus/total_order.rs` — TotalOrder trait

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent5/ready`.

## Report

Write `plans/agent5-round79-report.md` with holes before/after (table with Chap column).
