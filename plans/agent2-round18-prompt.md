# Agent 2 — Round 18: Add TotalOrder to Chap43 Ordered Sets/Tables

## Mission

Add `T: TotalOrder` bound to Chap43 traits and write extremality ensures for first,
last, previous, next, rank, select. You declared this "blocked by StT lacking Ord" in
R17. It is not blocked — `TotalOrder` already exists in `src/vstdplus/total_order.rs`
and is used throughout the codebase (Chap37 BSTs, Chap45 PQs).

## Required Reading

**Before writing any code**, read `src/standards/total_order_standard.rs`. It defines
every pattern you need: extremality ensures, T vs T::V, proof techniques, import pattern.

## The Solution

### Step 1: Add TotalOrder bound to traits

Current:
```rust
pub trait OrderedSetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
```

Change to:
```rust
pub trait OrderedSetStEphTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Set<<T as View>::V>> {
```

Do this for ALL Chap43 traits:
- OrderedSetStEphTrait
- OrderedSetStPerTrait
- OrderedTableStEphTrait
- OrderedTableStPerTrait
- AugOrderedTableStEphTrait
- AugOrderedTableStPerTrait

And their impl blocks. And their Mt/MtPer wrappers.

### Step 2: Write extremality ensures

The textbook defines (ADT 43.1):
- `first(A) = min[|A|]` — the minimum element
- `last(A) = max[|A|]` — the maximum element
- `previous(A, k) = max{k' ∈ A | k' < k}` — largest element strictly less than k
- `next(A, k) = min{k' ∈ A | k' > k}` — smallest element strictly greater than k

`TotalOrder::le(x, y)` is a spec fn (no exec needed). Write:

```rust
/// ADT 43.1 first(A) = min[|A|].
fn first(&self) -> (first: Option<T>)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        self@.len() == 0 <==> first matches None,
        first matches Some(v) ==> self@.contains(v@),
        first matches Some(v) ==>
            forall|x: T::V| self@.contains(x) ==> #[trigger] TotalOrder::le(v@, x),
;
```

```rust
/// ADT 43.1 last(A) = max[|A|].
fn last(&self) -> (last: Option<T>)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        self@.len() == 0 <==> last matches None,
        last matches Some(v) ==> self@.contains(v@),
        last matches Some(v) ==>
            forall|x: T::V| self@.contains(x) ==> #[trigger] TotalOrder::le(x, v@),
;
```

```rust
/// ADT 43.1 previous(A, k) = max{k' ∈ A | k' < k}.
fn previous(&self, k: &T) -> (pred: Option<T>)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        pred matches Some(v) ==> self@.contains(v@),
        pred matches Some(v) ==> TotalOrder::le(v@, k@) && v@ != k@,
        pred matches Some(v) ==>
            forall|x: T::V| self@.contains(x) && TotalOrder::le(x, k@) && x != k@
                ==> #[trigger] TotalOrder::le(x, v@),
        pred matches None ==> forall|x: T::V| self@.contains(x)
            ==> TotalOrder::le(k@, x),
;
```

```rust
/// ADT 43.1 next(A, k) = min{k' ∈ A | k' > k}.
fn next(&self, k: &T) -> (succ: Option<T>)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        succ matches Some(v) ==> self@.contains(v@),
        succ matches Some(v) ==> TotalOrder::le(k@, v@) && v@ != k@,
        succ matches Some(v) ==>
            forall|x: T::V| self@.contains(x) && TotalOrder::le(k@, x) && x != k@
                ==> #[trigger] TotalOrder::le(v@, x),
        succ matches None ==> forall|x: T::V| self@.contains(x)
            ==> TotalOrder::le(x, k@),
;
```

For rank (ADT 43.1 `rank(A, k) = |{k' ∈ A | k' < k}|`):
```rust
fn rank(&self, k: &T) -> (r: usize)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        r as int == self@.filter(|x: T::V| TotalOrder::le(x, k@) && x != k@).len(),
        r <= self@.len(),
;
```

For select (inverse of rank):
```rust
fn select(&self, i: usize) -> (elem: Option<T>)
    requires self.spec_orderedsetsteph_wf(),
    ensures
        self@.finite(),
        i >= self@.len() ==> elem is None,
        elem matches Some(v) ==> self@.contains(v@),
        elem matches Some(v) ==>
            self@.filter(|x: T::V| TotalOrder::le(x, v@) && x != v@).len() == i as int,
;
```

### Step 3: TotalOrder::le operates on T::V

Note: `TotalOrder::le` is defined as `spec fn le(self, other: Self) -> bool`. The trait
bound is on `T`, but `self@` is a `Set<T::V>`. So the forall quantifies over `T::V` and
calls `TotalOrder::le` on `T::V` values. This requires `T::V: TotalOrder`.

Check if `T::V` has `TotalOrder`. The `StT` trait requires `View`, so `T: View<V = ...>`.
If `T: TotalOrder` but `T::V` doesn't implement `TotalOrder`, you need to bridge.

Looking at `src/vstdplus/total_order.rs`, there are impls for `u64`, `i64`, `String`.
The `View` of `u64` is `u64` (self), so `TotalOrder::le` works directly on the view.

For generic `T: StT + Ord + TotalOrder` where `T: View<V = T::V>`, you need to express
`TotalOrder::le` on `T::V` values. Since the View maps `T -> T::V`, and `TotalOrder`
is on `T`, use the **spec bridge**:

```rust
spec fn spec_le_view(a: T::V, b: T::V) -> bool;
```

This is potentially uninterpreted. The cleaner approach: if `T::V == T` (which is common
for primitive types), `TotalOrder::le` works directly. For wrapped types where `T::V != T`,
you may need an axiom or a spec fn.

**Simplest approach**: Write the ensures using `TotalOrder::le` on `T::V` values directly.
If Verus complains that `T::V` doesn't implement `TotalOrder`, add the bound:
`T: StT + Ord + TotalOrder` where `T::V: TotalOrder`. Or use a helper spec fn:

```rust
pub open spec fn spec_view_le<T: StT + TotalOrder>(a: T, b: T) -> bool {
    TotalOrder::le(a, b)
}
```

And in ensures, quantify over `T` values that happen to be in the set (via `contains`).
The set is `Set<T::V>`, so `forall|x: T::V|` is correct.

Look at how LeftistHeapPQ does it (line 279):
```rust
forall|e: T| self@.count(e) > 0 ==> #[trigger] TotalOrder::le(*min_elem.unwrap(), e)
```

The view is `Multiset<T>` (not `Multiset<T::V>`). The set view is `Set<T::V>`. So
there may be a type mismatch. If `self@: Set<T::V>` and `TotalOrder::le` operates on `T`,
you need to work with the underlying `T` values somehow.

**Look at how Chap37 BSTSplayStEph does it** (Agent 3 fixed this in R17):
```rust
fn minimum(&self) -> (min: Option<&T>)
    ensures
        min.is_some() ==> forall|x: T| self.spec_root().tree_contains(x)
            ==> #[trigger] TotalOrder::le(*min.unwrap(), x),
```

The BST operates on `T` directly (not `T::V`). The OrderedSet view is `Set<T::V>`.

**Resolution**: The simplest fix is to quantify over `T` values. But the set `self@` is
`Set<T::V>`. You need:
```rust
forall|x: T::V| self@.contains(x) ==> TotalOrder::le(v@, x)
```
This requires `TotalOrder` on `T::V`. Add the bound where needed. For common types
like `u64` where `View<V = u64>`, this just works because `u64: TotalOrder`.

If a fully generic solution is blocked, add `external_body` with the strong spec.
The ensures is correct; the proof can come later.

## Files to Modify

| # | File | Changes |
|---|------|---------|
| 1 | OrderedSetStEph.rs | Add TotalOrder, extremality ensures |
| 2 | OrderedSetStPer.rs | Same |
| 3 | OrderedTableStEph.rs | Add TotalOrder, table-level ordering ensures |
| 4 | OrderedTableStPer.rs | Same |
| 5 | AugOrderedTableStEph.rs | Add TotalOrder, ordering ensures |
| 6 | AugOrderedTableStPer.rs | Same |
| 7+ | Mt/MtPer wrappers | Update trait bounds |

## Procedure

1. **Read** `src/vstdplus/total_order.rs` — understand the trait.
2. **Read** `src/Chap37/BSTSplayStEph.rs` — see how R17 Agent 3 used TotalOrder in ensures.
3. **Read** `src/Chap45/LeftistHeapPQ.rs` — see how find_min ensures minimality.
4. For each Chap43 file:
   a. Add `+ TotalOrder` to the trait bound.
   b. Write extremality ensures for first, last, previous, next.
   c. Write rank/select ensures using `TotalOrder::le`.
   d. Fix all impl blocks and call sites for the new bound.
   e. Add `external_body` where proof breaks.
5. `scripts/validate.sh` — 0 errors.

## Important

- The TotalOrder trait ALREADY EXISTS. You do not need to create it.
- "StT lacks ordering" was wrong — TotalOrder provides exactly the spec-level ordering.
- Add `external_body` if proofs break. Strong spec > weak spec.
- Do NOT weaken the ensures to avoid proof work.
- Do NOT modify Chap38/41/42 files (Agent 1's scope).

## Deliverables

- Extremality ensures on all first/last/previous/next/rank/select across 6+ Chap43 files.
- `plans/agent2-round18-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
