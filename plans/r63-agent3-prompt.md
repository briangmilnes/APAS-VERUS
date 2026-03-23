# Agent 3 — Round 63

You are Agent 3 working in `~/projects/APAS-VERUS-agent3`.
You are a **senior proof engineer**. Your job is to prove, not to catalog
reasons you can't. Read the code, read the error, write the proof.

## Baseline

- Main: 4504 verified, 0 errors, 6 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Target: Close PQMinStEph frontier capacity assume — 1 hole (Chap53, PQMinStEph.rs:269)

### The assume

Line 269: `assume(frontier_updated@.len() + 1 < usize::MAX as nat)`

This is inside the inner loop of `pq_explore` (line 241), which iterates over
neighbors and adds entries to `frontier_updated` via `union` with a singleton.

### What's already proven

Previous agents (R61) did excellent work on this file. The infrastructure is
largely in place:

- `pq_explore` requires `vertex_universe.finite()` and
  `vertex_universe.len() + 1 < usize::MAX` (lines 165-166)
- `visited@.subset_of(vertex_universe)` is maintained as a loop invariant (line 190)
- The visited capacity assume is already CLOSED (lines 210-212 use
  `lemma_len_subset`)
- The frontier vertex tracking invariant exists (lines 192-193, 254-255):
  every frontier entry's `.1` (vertex component) is in `vertex_universe`
- Neighbor vertices ∈ vertex_universe is established before the inner loop (lines 236-239)

### Why the frontier assume remains

The comment at lines 264-267 explains: "Proving injection needs priority_fn
view-determinism, which the generic Fn interface does not guarantee."

The frontier contains `Pair<Pair<P,V>,V>` entries. Multiple entries can map to
the same vertex if `priority_fn` returns different priorities at different times.
So you can't prove `frontier@.len() <= vertex_universe.len()` via simple subset
injection.

### Proof strategy: bound frontier via the loop iteration count

Forget injection. The inner loop iterates at most `nlen` times (neighbors count).
Each iteration adds at most 1 element to frontier_updated (union with singleton
on a set adds 0 or 1). So:

```
frontier_updated@.len() <= frontier_new@.len() + nlen
```

And `frontier_new` came from `frontier.difference(singleton)`, so:
```
frontier_new@.len() <= frontier@.len()
```

The outer loop invariant needs `frontier@.len() < usize::MAX` (or some bound).
Since frontier starts as `frontier_init` and the outer loop removes one entry
and adds at most `nlen` entries per iteration... this gets circular.

### Better strategy: use `lemma_len_union` bound

The key insight: `vstd::set_lib::lemma_len_union(a, b)` gives
`(a + b).len() <= a.len() + b.len()`. A singleton set has `.len() == 1`.
So after `frontier_updated.union(singleton)`:
```
frontier_updated_new@.len() <= frontier_updated@.len() + 1
```

You need `frontier_updated@.len() + 1 < usize::MAX` BEFORE the union.
That's the assume. To prove it, you need a bound on `frontier_updated@.len()`.

### Approach: add a graph edge count bound

The frontier can grow unboundedly unless we bound the total number of edges
the graph can produce. Consider adding a ghost parameter
`Ghost(max_frontier_size): Ghost<nat>` to `pq_explore` with:
```rust
requires
    max_frontier_size < usize::MAX,
    frontier_init@.len() <= max_frontier_size,
```

Then maintain as inner loop invariant:
```rust
invariant frontier_updated@.len() <= max_frontier_size,
```

This is provable if each iteration preserves the bound. Since union with a
set of size 1 adds at most 1 element, you need:
```
frontier_updated@.len() + 1 <= max_frontier_size
```

But this is exactly the assume again. The bound shifts up one level.

### Simplest approach: direct arithmetic on nlen + frontier_new

Actually, the simplest bound: at the START of the inner loop,
`frontier_updated == frontier_new` (line 230). The inner loop runs `nlen`
times, adding at most 1 each time. So:

```
// Before inner loop:
frontier_updated@.len() == frontier_new@.len()
// After i iterations:
frontier_updated@.len() <= frontier_new@.len() + i
// Need: frontier_new@.len() + nlen < usize::MAX
```

So add to the inner loop invariant:
```rust
frontier_updated@.len() <= frontier_new@.len() + i,
```

And prove BEFORE the inner loop:
```rust
frontier_new@.len() + nlen < usize::MAX
```

For this you need `frontier_new@.len() + nlen < usize::MAX`. You already have
`frontier_new@.len() <= frontier@.len()` (difference removes elements). And
`nlen` is the neighbor count from `graph(v)`, bounded by the `to_seq` length.

Use `lemma_wf_implies_len_bound_steph` on `neighbors_seq` to get
`nlen < usize::MAX`. Combined with `frontier_new@.len() < usize::MAX`
(from `frontier.spec_avltreesetsteph_wf()` + the len bound lemma), you get:

```
frontier_new@.len() + nlen < 2 * usize::MAX
```

That's not tight enough. You need a single `usize::MAX` bound, not a sum of two.

### The real insight

Look at line 190: `visited@.subset_of(vertex_universe)`. The visited set is
bounded: `visited@.len() <= vertex_universe.len() < usize::MAX`.

The frontier entries' vertices are all in `vertex_universe` (line 192-193).
The frontier is a SET (not a multiset). Two frontier entries with the same
`.1` (vertex) but different `.0` (priority) ARE different elements in the set.
So the frontier IS potentially larger than vertex_universe.

BUT: in practice, the algorithm only adds entries for vertices NOT in visited.
And each vertex gets at most one entry per neighbor-list encounter. The total
frontier size across all outer iterations is bounded by the total edge count.

**Practical fix**: Add `max_edges: Ghost<nat>` to `pq_explore` with:
```rust
requires max_edges < usize::MAX,
```

Thread it through. The caller (`pq_min_multi`) can compute or assume a bound.

### If you get stuck

If the full proof is too complex, try a partial approach:
1. Replace the `assume` with a requires-level bound on `pq_explore`
2. Push the assume up to `pq_min_multi` where it's more justified
3. Even lifting the assume one level up is progress — commit that

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent3-round63-report.md`. Push to `agent3/ready`.
