<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Architecture: Coarse Lock + TSM + Parallel Inside

## Status: Design (2026-04-01)

This document describes the target Mt module architecture for APAS-VERUS.
It combines three layers to achieve thread safety, zero assumes, and parallel
computation simultaneously.

## 1. The Problem

APAS Mt modules need:
1. **Thread safety** — multiple threads call operations concurrently
2. **Zero assumes** — proof doesn't trust unverified ghost state
3. **Parallel computation** — map, reduce, filter, union use fork-join internally

Current architecture gets (1) via coarse RwLock but sacrifices (3). All computation
inside the lock is sequential on St types. The Mt types have parallel operations
(map_dc, reduce_dc, filter_dc, ParaPair union/intersect/difference) but they're
trapped behind their own lock boundaries.

## 2. Mt Trait Architecture: Locked and Unlocked

Each Mt module exposes two trait layers:

**Locked trait** — the public API. Takes `&self` / `&mut self`. Acquires the lock,
delegates to the unlocked trait on the owned interior, releases the lock. Returns
`Result` for capacity-bounded operations. This is what external callers use.

```rust
pub trait FooMtLockedTrait: Sized {
    fn insert(&mut self, x: T) -> Result<(), ()>
        requires old(self).wf(),
        ensures ...;

    fn size(&self) -> (n: usize)
        requires self.wf(),
        ensures ...;

    fn parallel_map(&mut self, f: &F) -> Result<(), ()>
        requires self.wf(), ...
        ensures ...;
}
```

**Unlocked trait** — the full implementation on owned data (not behind a lock).
Contains all operations: sequential (insert, find, size, nth) and parallel
(map, reduce, filter, union). Called by the locked trait after `acquire_write`,
or called directly when the data is already owned (e.g., inside another
module's lock).

```rust
pub trait FooMtUnlockedTrait: Sized {
    fn insert(self, x: T) -> Self
        requires self.wf(), ...
        ensures ...;

    fn find(&self, x: &T) -> Option<&T>
        requires self.wf(), ...
        ensures ...;

    fn size(&self) -> usize
        requires self.wf(), ...
        ensures ...;

    fn map(self, f: &F) -> Self          // parallel internally where applicable
        requires self.wf(), ...
        ensures ...;

    fn reduce(&self, f: &F, id: T) -> T  // parallel internally where applicable
        requires self.wf(), ...
        ensures ...;
}
```

The locked trait acquires the lock, calls the unlocked trait, releases:

```rust
fn map(&mut self, f: &F) -> Result<(), ()> {
    let (interior, write_handle) = self.lock.acquire_write();
    // capacity check...
    let new_data = interior.data.map(f);  // unlocked trait — parallel internally
    // step TSM...
    write_handle.release_write(new_interior);
    Ok(())
}

fn size(&self) -> usize {
    let read_handle = self.lock.acquire_read();
    let n = read_handle.borrow().data.size();  // unlocked trait — sequential
    read_handle.release_read();
    n
}
```

When M1 stores M2 inside its lock, M1's locked trait calls M2's **unlocked** trait
directly on the owned M2 data — bypassing M2's lock since M1 already has exclusive
access. This works for all operations, sequential or parallel.

## 3. Mt Inside Mt: Composable Parallelism

If module M1 stores module M2 (an Mt type) inside M1's locked interior, then
after M1's `acquire_write`, you own M2. You call M2's unlocked parallel operations
directly — not through M2's lock, but through M2's unlocked trait on owned data.
M1's lock already gives exclusive access to M2.

This is composition: M1 provides the lock boundary. M2 provides the parallel
algorithms via its unlocked trait. No nested locks. No unsafe.

## 4. How Rust and Verus Enable This

### 4.1. What Rust gives us

Rust's ownership and borrowing system is the foundation. When `RwLock::acquire_write`
returns the inner value by move, Rust's type system guarantees:

- **Exclusive ownership**: no other thread has a reference to the data. The compiler
  enforces this statically — there's no runtime check, no flag, no possibility of a
  data race. The write guard proves exclusivity.

- **Move semantics**: destructuring a struct into its fields transfers ownership of
  each field. `let FooInterior { sequences, sets, token } = interior;` gives you
  three independent owned values. Each can be passed to a different join arm.

- **Send + 'static for join**: `join()` requires its closures to be `Send + 'static`.
  Owned values that are `Send` can move into join closures. `Arc<Vec<T>>` is `Send`
  when `T: Send`. `Box<Tree<T>>` is `Send` when `T: Send`. The slice-backed
  `ArraySeqMtEphSliceS<T>` is `Send` because it holds `Arc<Vec<T>>`.

- **No aliasing during mutation**: after splitting into two owned slices, each arm
  owns its slice exclusively. Even though both slices may share the same backing
  `Arc<Vec<T>>` (for reads), Rust prevents either arm from mutating through the
  shared Arc. New output must be allocated — the type system forces this.

- **Lifetime erasure in closures**: `move` closures own their captured data. The
  data's lifetime is the closure's lifetime, which is `'static` for `join()`. No
  dangling references, no use-after-free, enforced at compile time.

The locked/unlocked trait split maps directly onto Rust's borrow checker: the locked
trait takes `&self` / `&mut self` (borrows), while the unlocked trait takes `self` /
`&self` (owned or borrowed, no lock). The compiler ensures you can't call the unlocked
trait without owning the data — which means you either acquired the lock or you're
inside another module that acquired its lock.

### 4.2. What Verus verification adds on top

Rust guarantees memory safety and data-race freedom. Verus adds functional correctness:

- **RwLockPredicate**: the lock invariant is a spec-level predicate over the locked
  data. On every `acquire_read` / `acquire_write`, Verus proves the predicate holds.
  On every `release_write`, Verus checks the predicate still holds for the new data.
  This is a machine-checked proof, not a runtime assertion.

- **TSM (tokenized state machine)**: ghost tokens inside the lock track abstract
  state. The predicate ties `token.value() == concrete_data.abstract_view()`. After
  acquire, this equality is proved — not assumed. Every operation that modifies the
  data must step the token, and the predicate re-checks on release. The proof chain
  is: `predicate → token == data → operation preserves relation → new predicate`. No
  assumes anywhere.

- **Closure specs**: Verus closures carry `requires` and `ensures`. When a map closure
  goes into a `join()` arm, its ensures travel with it. The proof that the output
  sequence has the right values chains through: `closure.ensures(input[i], output[i])`
  for each element. `join()` preserves the ensures of both arms.

- **Ghost erasure**: tracked tokens, Ghost values, and spec functions are erased at
  compile time. The TSM token, the predicate check, the closure specs — none of these
  exist at runtime. The generated machine code is the same as unverified Rust. The
  proof is static, not dynamic.

- **Quantifier-based invariants**: loop invariants and function ensures use `forall`
  and `exists` quantifiers with explicit triggers. When two join arms each prove a
  quantified property over their half, the combined property over the full range
  follows by disjunction on the index. Verus + Z3 handle this automatically when
  triggers are set correctly.

Together: Rust guarantees that the parallel operations can't corrupt memory or race.
Verus guarantees that the parallel operations compute the right answer. The coarse lock
ensures thread safety. The TSM ensures proof integrity. The fork-join ensures parallel
performance. All three layers compose because Rust's ownership model and Verus's proof
model agree on who owns what and when.

## 5. Slice-Backed Sequences: O(1) Split

`ArraySeqMtEphSliceS` (Chap19) stores `Arc<Vec<T>>` with offset+length window.
`slice()` and `subseq_copy()` are O(1) — Arc::clone + adjust window. Both halves
share the same backing storage. They're owned, `Send + 'static`, and move directly
into `join()` closures.

This makes splitting data for fork-join free.

## 5. Slice Mutation and Rejoining

### 5.1. The problem: Arc sharing prevents mutation

After `slice()`, two slices share the same `Arc<Vec<T>>`. The Arc refcount is > 1.
Rust prevents mutation through a shared Arc — you'd need `Arc::make_mut` (which
clones the backing Vec if refcount > 1, O(n)) or unsafe interior mutability.

This means: **you cannot mutate the input slices in place after splitting.** All
operations that produce modified output must create new data.

### 5.2. Operation categories by output structure

**Category A — Scalar output (reduce, scan final value, size, find):**
Both join arms read from input slices (shared, immutable). Results are scalars.
No merge needed. O(1) combine.

**Category B — Same-size output (map, tabulate):**
Each arm produces a new sequence of known size (same as input for map, specified
for tabulate). Total output size is known before forking.

**Category C — Variable-size output (filter, flatten):**
Each arm produces a subset of unknown size. Total output size not known until
both arms complete.

### 5.3. Strategies for rejoining

**Strategy 1: Pre-allocated shared output (Categories B, C with max bound)**

Allocate one output `Vec<T>` of the known (or max possible) size before forking.
Each join arm writes to its disjoint region: left writes `[0..mid]`, right writes
`[mid..n]`. After join, the output is already contiguous in a single Vec.

For map: output size = input size. Pre-allocate exactly.
For tabulate: output size = length parameter. Pre-allocate exactly.
For filter: output size ≤ input size. Pre-allocate input size, track actual count.

Cost: O(n) work for allocation (O(1) span), O(1) rejoin.

Rust/Verus challenge: two join arms writing to the same Vec requires disjoint
mutable access. Options:
- **PCell per element**: each slot is `PCell<T>`, each arm gets `PointsTo` tokens
  for its region. Verus's FIFO example does this with `storage_map` sharding.
- **new-mut-ref**: Verus's upcoming disjoint mutable borrow support. `&mut [0..mid]`
  and `&mut [mid..n]` coexist in separate join arms.
- **Unsafe slice::split_at_mut**: works in Rust, but we don't use unsafe.

**Strategy 2: Two independent outputs + adjacent merge (all categories)**

Each join arm creates its own `Vec<T>` independently. After join, merge into a
single slice-backed sequence.

If both output Vecs can be placed in a single backing allocation (i.e., we
allocate one large Vec, let left fill the first half and right fill the second
half), the result is one `ArraySeqMtEphSliceS` with the full window. But this
is really Strategy 1 with extra steps.

More practically: each arm returns its own `ArraySeqMtEphSliceS` (separate
Arc backing). To merge, we must copy both into a new Vec — O(n) concat.

**Adjacent merge** (O(1)) is only possible when both slices share the same
`Arc<Vec<T>>` backing AND are contiguous (`left.start + left.len == right.start`).
This happens when:
- Both arms read from (not write to) the original input — the input slices are
  already adjacent in the same backing
- Or both arms write to a pre-allocated shared output (Strategy 1)

It does NOT happen when each arm independently allocates its own output Vec.

**Strategy 3: Return slice pairs (defer merge)**

Don't merge at all. Return a pair of slices (or a small tree of slices) and let
the consumer iterate over them logically. This is the rope/segmented approach.
We don't have this data structure yet, but it would give O(1) "merge" for all
categories by deferring the contiguous-memory requirement.

### 5.4. Summary: what works today

| Strategy | Cost | Works in current Verus? | Categories |
|----------|------|------------------------|------------|
| Pre-allocated + PCell per element | O(1) rejoin | Yes (FIFO pattern) | B, C |
| Pre-allocated + new-mut-ref | O(1) rejoin | Not yet (experimental) | B, C |
| Independent Vecs + O(n) concat | O(n) rejoin | Yes (current D&C) | B, C |
| Adjacent merge of read-only slices | O(1) rejoin | Yes (already works) | A input |
| Rope/segmented (deferred merge) | O(1) rejoin | Not implemented | All |

For the near term, the PCell-per-element approach (Strategy 1) is the path to
O(1) rejoin for map and tabulate. It requires a TSM with `storage_map` sharding
to manage the per-element PointsTo tokens, similar to the FIFO queue example.

For filter (variable-size), Strategy 1 with a max-bound pre-allocation works but
wastes space. Strategy 2 (independent Vecs + concat) is simpler and only O(n).

## 6. The Three Layers

### 6.1. Layer 1 — Thread Safety

One coarse `RwLock` on the outer module. `&mut self` for writes, `&self` for
reads. The lock serializes concurrent access to the whole module.

### 6.2. Layer 2 — Zero Assumes

TSM token inside the lock alongside the data. `RwLockPredicate` ties
`token.value() == data.abstract_state()`. After acquire, the predicate proves
the relationship. No ghost field outside the lock, no assume bridge.

### 6.3. Layer 3 — Parallel Computation

After `acquire_write`, you own the interior including its Mt data structures.
Call their unlocked parallel operations directly. `join()` arms take owned slices
or subtrees. Pre-allocate output for O(1) rejoin. Step TSM token. Release.

## 7. Interior Structure

```
FooMtEph {
    lock: RwLock<FooInterior, FooInv>,
    inst: Tracked<FooSM::Instance>,
}

FooInterior {
    sequences: ArraySeqMtEphSliceS<T>,     // O(1) slice, parallel reduce/map
    sets: AVLTreeSetMtEph<T>,              // parallel union/intersect/difference
    tables: OrderedTableMtEph<K, V>,       // parallel tabulate/map
    token: Tracked<FooSM::state>,          // TSM token
}
```

## 8. Operation Lifecycle

### 8.1. Write (parallel)

```
acquire_write → own (interior, write_handle)
  ↓
exec-time capacity check → Err if full
  ↓
call Mt data structure's unlocked parallel operation on owned data
  (internally: O(1) slice split, join, pre-allocated output)
  ↓
step TSM token (proof block)
  ↓
release_write(updated interior)
```

### 8.2. Read

```
acquire_read → borrow interior
  ↓
predicate proves data wf
  ↓
compute from real data
  ↓
release_read → return value
```

## 9. View

Since writes take `&mut self`, PCell Approach B gives View + zero assumes.
Ghost field outside the lock updated atomically with release_write. `&mut self`
guarantees sole ownership during write — no other thread observes the gap.

## 10. Cost Analysis vs APAS

| Operation  | APAS Span              | Current Span    | New Span               | Notes                          |
|------------|------------------------|-----------------|------------------------|--------------------------------|
| length     | O(1)                   | O(1)            | O(1)                   | Match                          |
| nth        | O(1)                   | O(1)            | O(1)                   | Match                          |
| subseq     | O(1)                   | O(j) copy       | O(1) slice             | Fixed by slice-backed          |
| append     | O(1)                   | O(n)            | O(1) adjacent merge    | Fixed if slices are adjacent   |
| filter     | O(lg n + max S(f))     | O(lg n) D&C     | O(lg n + max S(f))     | Match with pre-alloc output    |
| update     | O(1)                   | O(n) clone      | O(1) slice + write     | Fixed by slice-backed          |
| inject     | O(lg degree)           | O(n+m)          | O(lg degree)           | Needs parallel inject impl     |
| reduce     | O(lg n * max S(f))     | O(lg n) D&C     | O(lg n * max S(f))     | Match                          |
| scan       | O(lg n)                | O(n)            | O(lg n)                | Needs Blelloch impl            |
| map        | O(1 + max S(f))        | O(lg n) D&C     | O(lg n + max S(f))     | lg n from D&C depth, not PRAM  |
| tabulate   | O(1 + max S(f))        | O(n)            | O(lg n + max S(f))     | lg n from D&C depth            |
| flatten    | O(lg n)                | O(Σ a_i)        | O(lg n)                | With adjacent slice merge      |
| union      | O(lg n)                | O(lg² n)        | O(lg² n)               | Fork-join vs PRAM gap          |
| intersect  | O(lg n)                | O(lg² n)        | O(lg² n)               | Fork-join vs PRAM gap          |
| difference | O(lg n)                | O(lg² n)        | O(lg² n)               | Fork-join vs PRAM gap          |

### 10.1. Remaining gaps

**O(lg n) vs O(1) for map/tabulate**: APAS assumes PRAM O(1) fork. Fork-join D&C
has O(lg n) recursion depth. This is the fundamental PRAM-vs-fork-join gap —
unavoidable without true PRAM.

**O(lg² n) vs O(lg n) for set operations**: Fork-join ParaPair does O(lg n) work
per recursive level sequentially (the split). PRAM does the split concurrently.
Standard fork-join gap.

**scan**: Needs Blelloch parallel prefix (up-sweep/down-sweep). Not implemented
but architecturally possible with O(1) slice split.

**inject**: Needs parallel inject implementation. Deterministic ordering
constraint makes this harder than map/filter.

## 11. Disjoint Parallel Writes

The pre-allocated output pattern requires two `join()` arms to write to disjoint
regions of the same Vec. Options:

1. **PCell per element**: Each output slot is a `PCell<T>`. Each join arm gets
   `PointsTo` tokens for its slice region. The FIFO Verus example demonstrates
   this pattern for a ring buffer with `storage_map` sharding in a TSM.

2. **Split mutable slices**: Verus's `new-mut-ref` work enables mutable references
   to disjoint sub-places. Once stable, `&mut output[0..mid]` and
   `&mut output[mid..n]` could coexist in separate join arms.

3. **Two separate Vecs + adjacent merge**: Each arm writes to its own Vec. If the
   output is slice-backed and both Vecs are placed adjacent in a single backing
   allocation, the merge is O(1) window adjustment. This avoids the disjoint-write
   problem but requires allocation coordination.

Option 3 is simplest with current Verus. Options 1-2 give true O(1) span for
the output construction.

## 12. What's Needed

### 12.1. Experiments

1. **coarse_lock_parallel_tsm.rs**: Rewrite with `ArraySeqMtEphSliceS` as one
   inner Mt type and an AVLTreeSet/OrderedTable as another. Demonstrate parallel
   reduce (O(1) split, O(lg n) span) and parallel union (ParaPair) both inside
   a single `acquire_write`.

2. **disjoint_write_pcell.rs**: Experiment with PCell-per-slot output array.
   Two join arms write to disjoint PointsTo regions. Proves O(1) rejoin for
   map/tabulate.

### 12.2. Migration

1. Verify experiments
2. Migrate one real module (graph module from Chap52+ — has both sequences and sets)
3. Update `toplevel_coarse_rwlocks_for_mt_modules.rs` standard
4. Systematize across all Mt modules

### 12.3. Verus Dependencies

- `make-ghost-send-sync` branch: fixes `Ghost<T>` Send/Sync, eliminates need for
  `unsafe impl Send/Sync` on types containing Ghost fields
- `new-mut-ref`: enables disjoint mutable borrows, potentially simplifying the
  parallel write pattern
- Clone on closures: still unrecognized by Verus; `clone_fn` workaround remains
  necessary
