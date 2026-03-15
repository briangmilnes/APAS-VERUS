# Agent 2 — Round 19: Chap43 rank/select + Mt Propagation

## Mission

Two tasks you didn't finish in R18:

### Task A: rank/select ordering ensures (12 functions)

You added TotalOrder extremality to first/last/previous/next but skipped rank and select.
The excuse was "needs T::V: TotalOrder for Set::filter." Fix it.

**How to fix rank**: Don't use `Set::filter`. Count predecessors directly:

```rust
fn rank(&self, k: &T) -> (r: usize)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        r <= self@.len(),
        // r counts elements strictly less than k:
        r as int == self@.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
;
```

Or simpler — define a spec helper:
```rust
pub open spec fn spec_rank_count<T: StT + Ord + TotalOrder>(s: Set<T::V>, k: T) -> int {
    s.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, k) && t != k).len()
}
```

Or just use the T-quantifier pattern you used for first/last — write it as a count
over the abstract view. If the type system fights you, add `external_body` with the
correct spec. The spec is NOT blocked.

rank and select appear in:
- OrderedSetStEph.rs, OrderedSetStPer.rs (rank, select)
- OrderedTableStEph.rs, OrderedTableStPer.rs (rank_key, select_key)
- AugOrderedTableStEph.rs, AugOrderedTableStPer.rs (rank_key, select_key)

= 12 functions.

### Task B: Mt/MtPer ordering ensures propagation (~20 functions)

The Mt wrappers (OrderedSetMtEph, OrderedTableMtEph, OrderedTableMtPer,
AugOrderedTableMtEph) delegate to St methods through RwLock. Their specs should
match the St specs. For first/last/previous/next, copy the TotalOrder ensures
from the St trait verbatim.

Steps:
1. Read the St trait's ensures for first/last/previous/next.
2. Copy those ensures to the Mt trait declaration.
3. The Mt impl bodies are already `external_body` — no proof changes needed.
4. Just make sure the TotalOrder bound is on the Mt trait.

## Required Reading

- `src/standards/total_order_standard.rs` — ordering spec patterns.
- Your R18 work in OrderedSetStEph.rs — the first/last/previous/next ensures.

## Procedure

1. Write rank/select ordering ensures in all 6 St files.
2. Propagate first/last/previous/next/rank/select ensures to all Mt files.
3. `scripts/validate.sh` — 0 errors.

## Important

- Do NOT declare rank/select "blocked." Write the spec, add external_body.
- Mt propagation is trivial — copy ensures verbatim.
- Do NOT modify non-Chap43 files.

## Deliverables

- rank/select ensures in 6 St files.
- Ordering ensures propagated to Mt/MtPer files.
- `plans/agent2-round19-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
