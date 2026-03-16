# Agent 4 — Round 26: Chap41 AVLTreeSet Holes + Chap45 BinaryHeapPQ

## R25 Feedback

Excellent round — closed Chap42 completely (4→0). The 2-phase insert design and ghost
sources for filter are strong techniques. The trait strengthening cascade into
OrderedTableMtEph and AugOrderedTableMtEph was proactive and correct.

The Chap43 dependency chain is now: Chap41 AVLTreeSet files must be clean before
OrderedTable/OrderedSet can be fully proved. This round attacks Chap41 to unblock
that chain, plus the standalone BinaryHeapPQ find_min proof.

## Mission

1. Prove holes in Chap41 AVLTreeSetStEph and AVLTreeSetStPer (3 holes — unblocks Chap43)
2. Prove BinaryHeapPQ::find_min by strengthening wf with heap ordering (Chap45)
3. Fix Clone derive warnings in your files if any remain

## Current State

### Chap41 (25 holes total, but focus on StEph/StPer)

| # | Chap | File | Holes | Types |
|---|------|------|:-----:|-------|
| 1 | 41 | AVLTreeSetStEph.rs | 2 | 1 assume (vec len overflow), 1 external_body (filter) |
| 2 | 41 | AVLTreeSetStPer.rs | 1 | 1 external_body (filter) |
| 3 | 41 | AVLTreeSetMtEph.rs | 9 | 2 assume, 2 unsafe, 5 external_body |
| 4 | 41 | AVLTreeSetMtPer.rs | 9 | 1 assume, 8 external_body |
| 5 | 41 | Example41_3.rs | 4 | 4 external_body (skip — Example file) |

StEph (2 holes) and StPer (1 hole) have clean dependencies and are tractable.

### Chap45 (5 holes)

| # | Chap | File | Holes | Types |
|---|------|------|:-----:|-------|
| 1 | 45 | BinaryHeapPQ.rs | 2 | 1 assume (extract_all sortedness), 1 external_body (find_min) |
| 2 | 45 | BalancedTreePQ.rs | 2 | 1 external_body (insert), 1 external (trait impl) |
| 3 | 45 | Example45_2.rs | 1 | 1 external (skip — Example file) |

## Part 1: AVLTreeSetStEph — 2 Holes (Priority 1)

### Hole 1: `insert` assume (line 923)

```rust
proof { assume(new_vec@.len() < usize::MAX); }
```

This is an overflow guard. The fix: add `requires self@.len() < usize::MAX - 1` (or
similar bound) to the trait's `insert` spec. Insert adds at most 1 element, so the
set size must be < MAX - 1 to guarantee the result fits in usize. Then prove the
postcondition from the precondition.

Check if the trait declaration in AVLTreeSetStEph.rs already has a size bound on insert.
If not, strengthen it. This assume should be eliminable with a real precondition.

### Hole 2: `filter` external_body (line 314)

Filter takes a predicate and returns a subset. Read the implementation inside
external_body to understand the approach. The proof pattern:

1. Iterate over elements, apply predicate, collect matching elements
2. Build new AVLTreeSet from filtered collection
3. Prove: result is subset of original, result contains exactly the elements satisfying
   the predicate, result is a valid AVLTreeSet (wf preserved)

Your R25 filter proof in TableMtEph.rs (ghost sources tracking provenance) may transfer.
The key difference: AVLTreeSet is ordered, so the filtered collection must maintain
ordering. Since filtering preserves the relative order of elements, and the original
was sorted, the result is sorted.

## Part 2: AVLTreeSetStPer — 1 Hole (Priority 2)

### Hole: `filter` external_body (line 286)

Same operation as StEph but on the persistent (immutable) variant. The proof pattern
is the same: iterate, filter, rebuild. StPer returns a new value instead of mutating.

## Part 3: BinaryHeapPQ::find_min (Priority 3)

### Current State

`find_min` at line 662 of BinaryHeapPQ.rs returns `self.heap[0]` — the root of a
min-heap. It's `external_body` because:

1. `spec_binaryheappq_wf` doesn't include the heap ordering property
2. `spec_leq_view` is disconnected from `TotalOrder::le`

### The Fix: Spec Strengthening

**Step 1: Define heap ordering predicate**

```rust
pub open spec fn spec_is_min_heap<T: TotalOrder + View>(seq: Seq<T>) -> bool {
    forall|i: int| 0 < i < seq.len() ==>
        TotalOrder::le(seq[spec_parent(i)], seq[i])
}
```

Where `spec_parent(i) = (i - 1) / 2`.

**Step 2: Strengthen `spec_binaryheappq_wf`**

Add `spec_is_min_heap(self.heap@)` to the wf predicate. This says the heap array
satisfies the min-heap property: every parent ≤ its children.

**Step 3: Prove find_min**

With the strengthened wf:
- `self.heap[0]` is the root
- By `spec_is_min_heap`: root ≤ all elements (transitively: root ≤ parent of any node,
  parent ≤ node, so root ≤ all)
- A transitivity lemma may be needed: `lemma_heap_root_is_min`

The transitivity proof: for any element at index `i`, trace the parent chain
`i → parent(i) → parent(parent(i)) → ... → 0`. Each step, the parent ≤ child.
By transitivity, root ≤ element at `i`. This is induction on index `i`.

**Step 4: Connect spec_leq_view**

Read how `spec_leq_view` is used in the file. It may need to be connected to
`TotalOrder::le` via a requires clause or a lemma. If `spec_leq_view(a, b)` is
meant to equal `TotalOrder::le(a, b)`, add that as a spec axiom or derive it.

### Impact of Strengthening wf

Adding heap property to wf means: `insert`, `delete_min`, `meld`, `from_seq` must all
maintain the heap property. These are currently `external_body` or have weak specs.
Adding the heap property to wf does NOT break them — `external_body` functions accept
any spec. But when you later prove those bodies, you'll need to show heap property
is maintained.

For this round: just prove find_min. The cascade to other functions is future work.

### What This Unblocks

BinaryHeapPQ being proved unblocks Chap57 (Dijkstra, 5 holes) and Chap65 (Prim).

## Part 4: Clone Derive Warnings (Priority 4)

Check all files you touch for `#[derive(Clone)]` on non-Copy types. If found, replace
with manual Clone impl per `src/standards/partial_eq_eq_clone_standard.rs`.

From the project scan, these Chap52 graph files have `derive(Clone)` on non-Copy types:
- EdgeSetGraphStEph.rs, EdgeSetGraphStPer.rs, EdgeSetGraphMtPer.rs
- AdjTableGraphStEph.rs, AdjTableGraphStPer.rs, AdjTableGraphMtPer.rs

These are not in your primary scope but if you have time, fix them.

## Important

- You MAY strengthen wf predicates — that's the goal.
- You MAY add requires/ensures to trait methods and impls.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT add `requires true`. Omit requires if no precondition needed.
- Read `src/standards/using_closures_standard.rs` if filter uses closures.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- AVLTreeSetStEph: 2 holes closed (insert assume, filter external_body)
- AVLTreeSetStPer: 1 hole closed (filter external_body)
- BinaryHeapPQ::find_min proved (or specific blocker documented)
- Clone derive warnings fixed where found
- `plans/agent4-round26-report.md`
- 0 errors on validate.
- Commit + push to `agent4/ready`.
