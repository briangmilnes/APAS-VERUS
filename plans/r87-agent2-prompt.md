# R87 Agent 2 — Fix UnionFind rank overflow assume, STEP 20

## Objective

Replace the `admit()` at line 982 of UnionFindStEph.rs with a real proof that
`rank_u + 1 <= usize::MAX`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent2/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## The problem

In the equal-rank union case, rank is incremented: `rank_u + 1`. The admit
assumes this doesn't overflow usize. The textbook guarantee: rank ≤ log₂(n)
where n is the component size. Since n ≤ elements.len() ≤ usize::MAX, rank < 64.

## Approach: add rank bound to wf

Add a new wf conjunct:
```rust
forall|v: V::V| #[trigger] self.rank@.contains_key(v) ==>
    self.rank@[v] < 64
```

(64 = number of bits in usize, so rank < 64 means rank + 1 ≤ 63 < usize::MAX.)

This is a conservative bound — the real bound is log₂(component_size), but
64 is sufficient and doesn't require tracking component sizes.

### What needs to change

1. **Add the conjunct** to `spec_uf_wf` (or as a new named sub-predicate
   `spec_rank_bounded_64`).

2. **Prove preservation in `new()`**: empty UF has no ranks, vacuously true.

3. **Prove preservation in `insert()`**: new element gets rank 0 < 64.

4. **Prove preservation in `union_merge`** (inside the proof lemma
   `lemma_union_merge_wf`): in the equal-rank case, winner rank was r,
   new rank is r + 1. Need r < 63 (so r + 1 < 64). This follows from:
   - Both components had rank r
   - Rank r means each component has ≥ 2^r elements (union by rank property)
   - Two components of size ≥ 2^r means total ≥ 2^(r+1) elements
   - Total elements ≤ usize::MAX = 2^64 - 1
   - So 2^(r+1) ≤ 2^64 - 1, meaning r + 1 ≤ 63, meaning r ≤ 62

   This requires proving the 2^rank ≤ component_size property. You may need
   a ghost field tracking component sizes, or an inductive argument.

   **Simpler alternative**: just bound rank by elements.len() directly:
   ```
   self.rank@[v] < self.elements@.len()
   ```
   This is weaker than log₂ but still prevents overflow (since
   elements.len() ≤ usize::MAX, rank < usize::MAX, so rank + 1 ≤ usize::MAX).
   Preservation: union doesn't change elements, and rank only increases in the
   equal case where rank_u < elements.len() (because at least 2 elements exist
   in the component).

### WARNING about closed spec_uf_wf

`spec_uf_wf` is now a `closed spec fn`. Adding a new conjunct means:
- Add it inside `spec_uf_wf`'s body
- Update `lemma_decompose_wf` and `lemma_assemble_wf` if they enumerate conjuncts
- May need a new `reveal` in functions that check the new bound

Read the existing architecture carefully before editing. The sub-predicate
decomposition from R84/R85 is intricate.

## Important

- Do NOT modify KrustalStEph.rs or PrimStEph.rs — another agent works on them.
- Do NOT add new `assume` or `accept`.
- Do NOT weaken ensures clauses.
- Leave the union_merge and union `external_body` in place — Z3 &mut encoding
  issue is separate.

## STEP 20

## Report

Write `plans/agent2-round87-report.md`.
