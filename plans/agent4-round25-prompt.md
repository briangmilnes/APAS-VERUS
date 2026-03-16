# Agent 4 — Round 25: TableMtEph Closures Per Standard + Chap58/59

## Mission

Prove the 4 remaining holes in TableMtEph.rs by restructuring the closure pattern to
avoid nested closures in join arms. Then prove Chap58 and Chap59 holes.

User feedback: "It should put in the closures per standard."

## Current State (4 holes in Chap42)

| # | Chap | File | Function | Current Blocker |
|---|------|------|----------|-----------------|
| 1 | 42 | TableMtEph.rs | tabulate | join() + Arc + nested closures |
| 2 | 42 | TableMtEph.rs | map | join() + Arc + nested closures |
| 3 | 42 | TableMtEph.rs | filter | join() + Arc + nested closures |
| 4 | 42 | TableMtEph.rs | insert | join() + Arc + nested closures |

## Why Nested Closures Are Blocked

The current pattern creates closures-within-closures:
1. Join arm captures `Arc<F>` (user's closure)
2. Inside the join arm, creates an inner closure to pass to `ArraySeqMtEphS::tabulate`
3. Verus cannot reason about the inner closure's requires/ensures depending on the Arc'd
   closure's spec

## The Fix: Helper Functions Instead of Nested Closures

Successful Mt files in the project avoid nested closures by calling helper functions
directly from join arms. Examples:

- `SubsetSumMtEph.rs` — calls `subset_sum_rec()` directly from join arms
- `MergeSortMtPer.rs` — calls `merge_sort_rec()` directly from join arms
- `BFSMtPer.rs` — calls processing functions directly

### Pattern

Instead of:
```rust
let f1 = move || -> (r: Result) {
    // NESTED CLOSURE — blocked
    ArraySeqMtEphS::tabulate(&|i| { f_arc(entries[i]) }, len)
};
```

Write a **trait method** that does the sequential work:
```rust
// In TableMtEphTrait:
fn tabulate_range<F: Fn(&K) -> V>(
    keys: &ArraySeqMtEphS<K>,
    f: &F,
    start: usize,
    len: usize,
) -> (result: Self)
    requires
        forall|k: &K| f.requires((k,)),
        start + len <= keys@.len(),
    ensures
        result.spec_tablemteph_wf(),
        // ... result specs matching tabulate contract for the subrange ...
```

Then each join arm calls the trait method directly:
```rust
let f1 = move || -> (r: TableMtEph<K, V>)
    requires /* f_arc requires propagation */
    ensures /* result specs */
{
    TableMtEph::tabulate_range(&left_keys, &*f_arc1, 0, mid)
};
```

This eliminates the nested closure. The join arm has a single closure whose
requires/ensures reference the Arc'd function. The trait method does the real work with
a simple loop.

### Where Helpers Go

**In the trait**: Helpers that operate on `Self` or produce `Self` — methods like
`tabulate_range`, `map_range`, `filter_range`. These are the natural home since they
use the table's own invariants and data.

**Top-level free functions**: Only for helpers that don't involve `Self` — pure proof
lemmas on `Seq`/`Map`, or utility functions on raw types like sequences of pairs.

Most of the helpers for TableMtEph should be trait methods.

## Step-by-Step

### Step 1: Read the Standards

Read `src/standards/using_closures_standard.rs`. Understand Patterns A, B, C.

Read `src/standards/helper_function_placement_standard.rs`. This defines where helpers
go: trait methods when they need `&self` or produce `Self`; `pub(crate)` free functions
when they operate on bare data (Link, arrays, sequences). Fork-join helpers are covered
explicitly. No nested closures in join arms.

### Step 2: Read TableStEph.rs Proofs

Your R24 proved all 6 functions in TableStEph.rs. Read your own proofs. The StEph versions
use the same loop structures but without Arc wrapping. Your loop invariants with
f.ensures/combine.ensures tracking are directly transferable.

### Step 3: Write Helper Methods for Each Operation

Add trait methods (one per operation):
- `tabulate_range` — sequential tabulate over a key subrange
- `map_range` — sequential map over an entry subrange
- `filter_range` — sequential filter over an entry subrange
- `insert_single` — sequential insert processing (or similar)

These go in the trait and impl, not as top-level free functions, because they operate on
the table type. Each helper takes the user closure by reference (`&F`), not by Arc. The
Arc unwrapping happens in the join arm (`&*f_arc`).

### Step 4: Restructure Join Arms

For each of the 4 functions:
1. Clone the Arc for each join arm: `let f_arc1 = clone_arc_rwlock(f_arc);`
   (or `Arc::clone` if not RwLock-wrapped)
2. Create join closures that call the helper functions directly
3. Thread the closure's `f.requires`/`f.ensures` through the join arm's requires/ensures
4. After join, concatenate results and prove the combined postcondition

### Step 5: Strengthen MtEph Specs to Match StEph

Your R24 research noted that TableMtEph's ensures are weaker than TableStEph's — they
don't propagate `f.ensures`/`combine.ensures` through results. When you remove the
`external_body`, strengthen the ensures to match TableStEph.

## Part 2: Chap58 BellmanFord + Chap59 Johnson (Priority 2)

### Chap58 (2 holes, clean deps)

| # | Chap | File | Holes |
|---|------|------|:-----:|
| 1 | 58 | BellmanFordStEphI64.rs | ? |
| 2 | 58 | (second file) | ? |

Read the files and assess. These may be graph algorithm holes (relaxation step, negative
cycle detection). Report what you find and prove what you can.

### Chap59 (1 hole, clean deps)

| # | Chap | File | Holes |
|---|------|------|:-----:|
| 1 | 59 | JohnsonStEphI64.rs | 1 |

Read and assess. Johnson's algorithm composes Bellman-Ford + Dijkstra.

## Important

- Read `src/standards/using_closures_standard.rs` FIRST.
- Read `src/standards/helper_function_placement_standard.rs` for helper placement rules.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT add `requires true`. Omit requires if genuinely no precondition needed.
- Do NOT modify `requires`/`ensures` unless strengthening.
- The closure pattern must match the standard — named closures with explicit ensures.
- `scripts/validate.sh` after each function — 0 errors.

## Deliverables

- 4 holes closed in TableMtEph.rs (tabulate, map, filter, insert)
- Chap58/59 holes assessed and proved where possible
- `plans/agent4-round25-report.md`
- 0 errors on validate.
- Commit + push to `agent4/ready`.
