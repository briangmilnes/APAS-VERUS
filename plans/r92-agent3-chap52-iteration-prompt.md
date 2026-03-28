# R92 Agent 3 — Prove Chap52 iteration holes (num_edges, vertices), STEP 20

## Objective

Remove external_body from the iteration-based functions in AdjTableGraph.
These iterate over the Table domain and are blocked by loop invariant
complexity, not the Verus ICE.

Target functions (same in all 3 files):
- `num_edges` — iterate domain, sum neighbor set sizes
- `vertices` — iterate domain, build set from keys

## Strategy

### num_edges

```rust
fn num_edges(&self) -> usize {
    let domain = self.adj.domain();
    let seq = domain.to_seq();
    let mut count = 0;
    for i in 0..seq.len() {
        let neighbors = self.adj.find_ref(&seq[i]);
        count += neighbors.size();
    }
    count
}
```

The loop invariant needs:
- `count == spec_sum_adj_sizes_partial(self@, seq@, i)` — partial sum up to index i
- `seq@` is the domain sequence (from domain.to_seq ensures)
- Each `find_ref` returns a reference to the stored neighbor set (wf guaranteed)

You may need a helper `spec_sum_adj_sizes_partial` that sums over a prefix.

### vertices

```rust
fn vertices(&self) -> SetStEph<V> {
    let domain = self.adj.domain();
    let seq = domain.to_seq();
    let mut result = SetStEph::empty();
    for i in 0..seq.len() {
        result.insert(seq[i].clone());
    }
    result
}
```

Invariant: `result@ == seq@.subrange(0, i).to_set()` or similar.

### IMPORTANT: Avoid the ICE

Do NOT write `assert forall` or quantifiers over `Map<V::V, Set<V::V>>` in
proof bodies — this crashes Verus. Work with the domain sequence (`Seq<V>`)
and `find_ref` to access individual values. Never quantify over the map directly.

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — start here (has find_ref from R89)
- `src/Chap42/TableStEph.rs` — domain(), find_ref(), spec_stored_value
- `src/Chap41/ArraySetStEph.rs` — domain return type API

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Start with StEph only. If you prove both functions in StEph, port to StPer.
- Do NOT trigger the Verus ICE.
- Do NOT add assume or accept.
- These functions have `ensures true` currently — that's fine, focus on removing
  external_body. Strengthening ensures is a bonus.

## STEP 20

## Report

Write `plans/agent3-r92-iteration-report.md`.
