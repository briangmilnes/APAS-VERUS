<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Alternative Architecture: Coarse Lock + TSM + Parallel Inside

## Status: Alternative Design (2026-04-01)

This document describes an alternative Mt module architecture for APAS-VERUS
that would replace the current coarse RwLock pattern with RwLock+TSM. It
eliminates ~66% of the 183 lock-boundary assumes at the cost of ~68 lines
of TSM boilerplate per file.

For the current deployed architecture, see
`docs/current-architecture-coarse-lock-parallel.md`.

This design achieves four properties simultaneously: zero-assume locking,
caller-observable state, composable unlocked parallelism, and optimal split
cost.

## 1. The Four Properties

### 1.1. Zero-Assume Locking

M1's lock gives provable knowledge of the data's state without assumes. A TSM token lives inside the lock alongside the data. The RwLockPredicate ties `token.value() == data.abstract_state()`. After acquire, the predicate proves the relationship — no ghost field outside the lock, no assume bridge, no trust gap.

### 1.2. Caller-Observable State (View)

Callers can write specs against `self@` — the abstract state is observable from outside the lock without acquiring it. This requires a ghost field outside the lock that stays in sync with the locked data. With `&mut self` for writes, PCell Approach B maintains this: the ghost field is updated atomically with release_write, and `&mut self` guarantees sole ownership during the write — no other thread observes the gap.

### 1.3. Composable Unlocked Parallelism

M1 acquires its lock, owns M2 (an Mt type stored inside the locked interior), and calls M2's parallel operations directly without going through M2's lock. M1 provides thread safety. M2 provides parallel algorithms via its unlocked trait. No nested locks. No unsafe.

### 1.4. Optimal Split Cost

The inner data structures use O(1) split so that D&C parallelism achieves the textbook span bounds. Slice-backed sequences (`ArraySeqMtEphSliceS`) split via Arc::clone + window adjust — O(1). Trees split via unbox — O(1). With O(1) split, reduce and scan achieve O(lg n) span. Map and filter achieve O(lg n) span if the output rejoin is also O(1) (via pre-allocated shared output or rope-style deferred merge).

## 2. Two Concurrency Styles

There are two ways to achieve these four properties. Both use TSM for ghost-level proof. They differ in how they manage runtime concurrency.

### 2.1. RwLock + TSM

The data lives inside an `RwLock`. The TSM token lives alongside the data in the lock interior. A ghost field outside the lock provides View. `&mut self` for writes ensures the ghost stays in sync.

**How it works:**
- `acquire_write` → own interior (data + token). Predicate proves token == data.
- Mutate data, step token. Release. Update external ghost.
- `acquire_read` → borrow interior. Read from real data through the predicate.

**Proof gap:** 2 accepts per file. The external ghost field can't see through the lock. On acquire, we accept that ghost == inner. This is correct by induction (ghost set at previous release, `&mut self` prevents interleaving) but not machine-checked by Verus.

**Runtime cost:** RwLock acquire/release on every operation.

### 2.2. PCell + TSM + Atomics

The data lives in a `PCell` (Permissioned Cell) on the struct. The `PointsTo` proof token is managed by a TSM with `storage_map` or `storage_option` sharding. No RwLock — concurrency is controlled by atomic operations and TSM transitions that withdraw/deposit the PointsTo token.

**How it works:**
- PCell holds the data. PCell is always Send+Sync (it's just bytes without the proof token).
- PointsTo is the ghost proof of what's in the PCell. Whoever holds it can access the data.
- TSM transitions `withdraw` the PointsTo (lend it to a thread) and `deposit` it back.
- Atomic variables (head/tail indices, flags) coordinate which thread gets the token.
- Thread acquires PointsTo via TSM transition, accesses PCell, modifies data, deposits updated PointsTo back.

**Proof gap:** 0 accepts. The PointsTo IS the proof of what's in the cell. `borrow` returns `v === perm.value()` — the value is what the token says, by construction. No ghost field, no bridge, no trust gap.

**Runtime cost:** Atomic operations only. No lock acquire/release.

### 2.3. Comparison

| | RwLock + TSM | PCell + TSM + Atomics |
|---|---|---|
| **Zero-Assume Locking** | 0 assumes | 0 assumes |
| **Caller-Observable State** | 2 accepts per file | 0 assumes |
| **Composable Parallelism** | 0 assumes | 0 assumes |
| **Optimal Split Cost** | 0 assumes | 0 assumes |
| **Capacity exec checks** | 1 per mutating op | 1 per mutating op |
| **Total accepts/assumes** | 2 per file | 0 |
| **Ghost protocol complexity** | Low (TSM tracks count/state) | High (storage_map, withdraw/deposit) |
| **Runtime cost** | RwLock acquire/release | Atomic ops only |
| **Proven in experiment** | Yes | Partially |
| **Migration effort from current** | Low | High |

### 2.4. What the accepts mean in RwLock + TSM

If an accept is wrong (ghost != inner), every ensures referencing `self@` is a lie. Callers act on false postconditions.

The argument for correctness is inductive:
1. `new()` sets ghost = inner (provably, no accept)
2. Every write: acquire, accept ghost == inner, mutate, update ghost = new inner, release
3. `&mut self` on writes means no interleaving between release and next acquire

If `&mut self` holds (sole ownership), the ghost set at release is still valid at the next acquire. The accept is correct by induction. But this induction is not machine-checked — it's the 2-accept trust gap.

**The constraint:** if the Mt struct is wrapped in `Arc` for sharing across threads, the `&mut self` guarantee breaks. Other Arc clones can acquire the lock independently, changing the inner data while our ghost is stale. The architectural rule: Mt structs with View must not be shared via Arc. Thread sharing goes through the locked trait (`&self` reads are fine — they don't use the ghost for computation, only for caller specs).

### 2.5. Proof consequences

**RwLock + TSM proof profile:**
- Every operation's functional correctness (algorithmic logic, data structure invariants, cost bounds) is fully machine-checked by Verus.
- The TSM predicate proves token == data on every lock acquire — machine-checked.
- The 2 accepts per file are the ONLY unverified claims. They assert ghost_view == locked_data. If wrong, caller specs (ensures referencing `self@`) are unsound, but the internal algorithmic proofs remain valid — the data structure still works correctly, callers just can't observe its state through View.
- Proof effort: low. The TSM is simple (tracks count or abstract state). The RwLock predicate is a conjunction. The accepts are boilerplate.

**PCell + TSM + Atomics proof profile:**
- Every operation's functional correctness is fully machine-checked.
- The PointsTo token proves cell contents on every access — machine-checked. No accepts anywhere.
- The TSM protocol (storage_map, withdraw/deposit, invariants) must prove that tokens are never duplicated, never lost, and always deposited in a consistent state. This is the bulk of the proof work — and it's ALL machine-checked. No trust gaps.
- Proof effort: high. The TSM must model the full concurrency protocol. Each operation needs a corresponding transition. The invariants must capture the relationship between multiple threads' tokens and the shared state. The FIFO example is ~766 lines for a 4-operation queue.

**The tradeoff:** RwLock + TSM gets 98% of the proof for 20% of the effort. PCell + TSM + Atomics gets 100% for 5x the effort. The 2% gap (the 2 accepts) is well-understood, inductive, and constrained by the `&mut self` ownership rule.

### 2.6. Current choice

APAS-VERUS uses **RwLock + TSM** for the foreseeable future. The 2 accepts per file are well-understood, documented in the standard (`toplevel_coarse_rwlocks_for_mt_modules.rs`), and cheap. The PCell + TSM + Atomics path eliminates them but adds significant ghost protocol complexity and is only partially proven. Large data structures (sequences, trees, tables) don't naturally map to atomic-only concurrency.

PCell + TSM + Atomics remains a future option for fine-grained concurrent data structures (lock-free queues, concurrent hash maps) where the complexity is inherent to the algorithm.

## 3. Current Status

| Property | Status | Evidence |
|----------|--------|----------|
| Zero-Assume Locking | **Proven** | `bst_plain_mt_tsm.rs`: 10 ops, zero assumes. `coarse_lock_parallel_tsm.rs`: TSM + lock, zero assumes. |
| Caller-Observable State | **Partially proven** | `bst_plain_mt_pcell.rs` Approach B: View works with `&mut self` writes for a simple nat count. Not yet demonstrated with a rich type or composed with TSM + parallelism. |
| Composable Unlocked Parallelism | **Proven** | `coarse_lock_parallel_tsm.rs` R135: acquire lock, call inner Mt type's reduce/map (D&C + join internally), release. No nested locks. |
| Optimal Split Cost | **In progress** | `ArraySeqMtEphSliceS` has O(1) slice but no map/reduce/filter yet. Agent 3 R136 adding them. Reduce will get true O(lg n) span. Map/filter get O(1) split but O(n) rejoin — O(1) rejoin needs pre-allocated output or ropes. |

| Property | Status | Evidence |
|----------|--------|----------|
| Zero-Assume Locking | **Proven** | `bst_plain_mt_tsm.rs`: 10 ops, zero assumes. `coarse_lock_parallel_tsm.rs`: TSM + lock, zero assumes. |
| Caller-Observable State | **Partially proven** | `bst_plain_mt_pcell.rs` Approach B: View works with `&mut self` writes for a simple nat count. Not yet demonstrated with a rich type or composed with TSM + parallelism. |
| Composable Unlocked Parallelism | **Proven** | `coarse_lock_parallel_tsm.rs` R135: acquire lock, call inner Mt type's reduce/map (D&C + join internally), release. No nested locks. |
| Optimal Split Cost | **In progress** | `ArraySeqMtEphSliceS` has O(1) slice but no map/reduce/filter yet. Agent 3 R136 adding them. Reduce will get true O(lg n) span. Map/filter get O(1) split but O(n) rejoin — O(1) rejoin needs pre-allocated output or ropes. |

## 3. Mt Trait Architecture: Locked and Unlocked

Each Mt module exposes two trait layers:

**Locked trait** — the public API. Takes `&self` / `&mut self`. Acquires the lock, delegates to the unlocked trait on the owned interior, releases the lock. Returns `Result` for capacity-bounded operations. This is what external callers use.

```rust
pub trait FooMtLockedTrait: Sized {
    fn insert(&mut self, x: T) -> Result<(), ()>
        requires old(self).wf(),
        ensures ...;

    fn size(&self) -> (n: usize)
        requires self.wf(),
        ensures ...;

    fn map(&mut self, f: &F) -> Result<(), ()>
        requires self.wf(), ...
        ensures ...;
}
```

**Unlocked trait** — the full implementation on owned data (not behind a lock). Contains all operations: sequential (insert, find, size, nth) and parallel (map, reduce, filter, union). Called by the locked trait after `acquire_write`, or called directly when the data is already owned (e.g., inside another module's lock).

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

When M1 stores M2 inside its lock, M1's locked trait calls M2's **unlocked** trait directly on the owned M2 data — bypassing M2's lock since M1 already has exclusive access. This works for all operations, sequential or parallel.

## 4. How Rust and Verus Enable This

### 4.1. What Rust gives us

Rust's ownership and borrowing system is the foundation. When `RwLock::acquire_write` returns the inner value by move, Rust's type system guarantees:

- **Exclusive ownership**: no other thread has a reference to the data. The compiler enforces this statically — there's no runtime check, no flag, no possibility of a data race. The write guard proves exclusivity.

- **Move semantics**: destructuring a struct into its fields transfers ownership of each field. `let FooInterior { sequences, sets, token } = interior;` gives you three independent owned values. Each can be passed to a different join arm.

- **Send + 'static for join**: `join()` requires its closures to be `Send + 'static`. Owned values that are `Send` can move into join closures. `Arc<Vec<T>>` is `Send` when `T: Send`. `Box<Tree<T>>` is `Send` when `T: Send`. The slice-backed `ArraySeqMtEphSliceS<T>` is `Send` because it holds `Arc<Vec<T>>`.

- **No aliasing during mutation**: after splitting into two owned slices, each arm owns its slice exclusively. Even though both slices may share the same backing `Arc<Vec<T>>` (for reads), Rust prevents either arm from mutating through the shared Arc. New output must be allocated — the type system forces this.

- **Lifetime erasure in closures**: `move` closures own their captured data. The data's lifetime is the closure's lifetime, which is `'static` for `join()`. No dangling references, no use-after-free, enforced at compile time.

The locked/unlocked trait split maps directly onto Rust's borrow checker: the locked trait takes `&self` / `&mut self` (borrows), while the unlocked trait takes `self` / `&self` (owned or borrowed, no lock). The compiler ensures you can't call the unlocked trait without owning the data — which means you either acquired the lock or you're inside another module that acquired its lock.

### 4.2. What Verus verification adds on top

Rust guarantees memory safety and data-race freedom. Verus adds functional correctness:

- **RwLockPredicate**: the lock invariant is a spec-level predicate over the locked data. On every `acquire_read` / `acquire_write`, Verus proves the predicate holds. On every `release_write`, Verus checks the predicate still holds for the new data. This is a machine-checked proof, not a runtime assertion.

- **TSM (tokenized state machine)**: ghost tokens inside the lock track abstract state. The predicate ties `token.value() == concrete_data.abstract_view()`. After acquire, this equality is proved — not assumed. Every operation that modifies the data must step the token, and the predicate re-checks on release. The proof chain is: `predicate → token == data → operation preserves relation → new predicate`. No assumes anywhere.

- **Closure specs**: Verus closures carry `requires` and `ensures`. When a map closure goes into a `join()` arm, its ensures travel with it. The proof that the output sequence has the right values chains through: `closure.ensures(input[i], output[i])` for each element. `join()` preserves the ensures of both arms.

- **Ghost erasure**: tracked tokens, Ghost values, and spec functions are erased at compile time. The TSM token, the predicate check, the closure specs — none of these exist at runtime. The generated machine code is the same as unverified Rust. The proof is static, not dynamic.

- **Quantifier-based invariants**: loop invariants and function ensures use `forall` and `exists` quantifiers with explicit triggers. When two join arms each prove a quantified property over their half, the combined property over the full range follows by disjunction on the index. Verus + Z3 handle this automatically when triggers are set correctly.

Together: Rust guarantees that the parallel operations can't corrupt memory or race. Verus guarantees that the parallel operations compute the right answer. The coarse lock ensures thread safety. The TSM ensures proof integrity. The fork-join ensures parallel performance. All three layers compose because Rust's ownership model and Verus's proof model agree on who owns what and when.

## 5. Slice-Backed Sequences: O(1) Split

`ArraySeqMtEphSliceS` (Chap19) stores `Arc<Vec<T>>` with offset+length window. `slice()` and `subseq_copy()` are O(1) — Arc::clone + adjust window. Both halves share the same backing storage. They're owned, `Send + 'static`, and move directly into `join()` closures.

This makes splitting data for fork-join free.

## 6. Slice Mutation and Rejoining

### 6.1. The problem: Arc sharing prevents mutation

After `slice()`, two slices share the same `Arc<Vec<T>>`. The Arc refcount is > 1. Rust prevents mutation through a shared Arc — you'd need `Arc::make_mut` (which clones the backing Vec if refcount > 1, O(n)) or unsafe interior mutability.

This means: **you cannot mutate the input slices in place after splitting.** All operations that produce modified output must create new data.

### 6.2. Operation categories by output structure

**Category A — Scalar output (reduce, scan final value, size, find):** Both join arms read from input slices (shared, immutable). Results are scalars. No merge needed. O(1) combine.

**Category B — Same-size output (map, tabulate):** Each arm produces a new sequence of known size (same as input for map, specified for tabulate). Total output size is known before forking.

**Category C — Variable-size output (filter, flatten):** Each arm produces a subset of unknown size. Total output size not known until both arms complete.

### 6.3. Strategies for rejoining

**Strategy 1: Pre-allocated shared output (Categories B, C with max bound)**

Allocate one output `Vec<T>` of the known (or max possible) size before forking. Each join arm writes to its disjoint region: left writes `[0..mid]`, right writes `[mid..n]`. After join, the output is already contiguous in a single Vec.

For map: output size = input size. Pre-allocate exactly.
For tabulate: output size = length parameter. Pre-allocate exactly.
For filter: output size ≤ input size. Pre-allocate input size, track actual count.

Cost: O(n) work for allocation (O(1) span), O(1) rejoin.

Rust/Verus challenge: two join arms writing to the same Vec requires disjoint mutable access. Options:
- **PCell per element**: each slot is `PCell<T>`, each arm gets `PointsTo` tokens for its region. Verus's FIFO example does this with `storage_map` sharding.
- **new-mut-ref**: Verus's upcoming disjoint mutable borrow support. `&mut [0..mid]` and `&mut [mid..n]` coexist in separate join arms.
- **Unsafe slice::split_at_mut**: works in Rust, but we don't use unsafe.

**Strategy 2: Two independent outputs + adjacent merge (all categories)**

Each join arm creates its own `Vec<T>` independently. After join, merge into a single slice-backed sequence.

Adjacent merge (O(1)) is only possible when both slices share the same `Arc<Vec<T>>` backing AND are contiguous (`left.start + left.len == right.start`). This happens when both arms read from (not write to) the original input, or both arms write to a pre-allocated shared output (Strategy 1). It does NOT happen when each arm independently allocates its own output Vec.

When adjacent merge isn't possible, combine by copying both into a new Vec — O(n) concat.

**Strategy 3: Return slice pairs (defer merge)**

Don't merge at all. Return a pair of slices (or a small tree of slices) and let the consumer iterate over them logically. This is the rope/segmented approach. We don't have this data structure yet, but it would give O(1) "merge" for all categories by deferring the contiguous-memory requirement.

### 6.4. Summary: what works today

| Strategy | Cost | Works in current Verus? | Categories |
|----------|------|------------------------|------------|
| Pre-allocated + PCell per element | O(1) rejoin | Yes (FIFO pattern) | B, C |
| Pre-allocated + new-mut-ref | O(1) rejoin | Not yet (experimental) | B, C |
| Independent Vecs + O(n) concat | O(n) rejoin | Yes (current D&C) | B, C |
| Adjacent merge of read-only slices | O(1) rejoin | Yes (already works) | A input |
| Rope/segmented (deferred merge) | O(1) rejoin | Not implemented | All |

For the near term, the PCell-per-element approach (Strategy 1) is the path to O(1) rejoin for map and tabulate. It requires a TSM with `storage_map` sharding to manage the per-element PointsTo tokens, similar to the FIFO queue example.

For filter (variable-size), Strategy 1 with a max-bound pre-allocation works but wastes space. Strategy 2 (independent Vecs + concat) is simpler and only O(n).

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
  (internally: O(1) slice split, join, reassemble)
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
compute from real data (via unlocked trait)
  ↓
release_read → return value
```

## 9. View

Since writes take `&mut self`, PCell Approach B gives View + zero assumes. Ghost field outside the lock updated atomically with release_write. `&mut self` guarantees sole ownership during write — no other thread observes the gap.

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

**O(lg n) vs O(1) for map/tabulate**: APAS assumes PRAM O(1) fork. Fork-join D&C has O(lg n) recursion depth. This is the fundamental PRAM-vs-fork-join gap — unavoidable without true PRAM.

**O(lg² n) vs O(lg n) for set operations**: Fork-join ParaPair does O(lg n) work per recursive level sequentially (the split). PRAM does the split concurrently. Standard fork-join gap.

**scan**: Needs Blelloch parallel prefix (up-sweep/down-sweep). Not implemented but architecturally possible with O(1) slice split.

**inject**: Needs parallel inject implementation. Deterministic ordering constraint makes this harder than map/filter.

## 11. Disjoint Parallel Writes

The pre-allocated output pattern requires two `join()` arms to write to disjoint regions of the same Vec. Options:

1. **PCell per element**: Each output slot is a `PCell<T>`. Each join arm gets `PointsTo` tokens for its slice region. The FIFO Verus example demonstrates this pattern for a ring buffer with `storage_map` sharding in a TSM.

2. **Split mutable slices**: Verus's `new-mut-ref` work enables mutable references to disjoint sub-places. Once stable, `&mut output[0..mid]` and `&mut output[mid..n]` could coexist in separate join arms.

3. **Two separate Vecs + adjacent merge**: Each arm writes to its own Vec. If the output is slice-backed and both Vecs are placed adjacent in a single backing allocation, the merge is O(1) window adjustment. This avoids the disjoint-write problem but requires allocation coordination.

Option 3 is simplest with current Verus. Options 1-2 give true O(1) span for the output construction.

## 12. What's Needed

### 12.1. Experiments

1. **coarse_lock_parallel_tsm.rs**: Currently uses `ArraySeqMtEphS` (Vec-backed). Switch to `ArraySeqMtEphSliceS` once R136 adds map/reduce/filter to the slice type. Also add a second inner Mt type (set or table) to demonstrate composability.

2. **disjoint_write_pcell.rs**: Experiment with PCell-per-slot output array. Two join arms write to disjoint PointsTo regions. Proves O(1) rejoin for map/tabulate.

3. **View + TSM + parallelism composed**: Demonstrate all four properties in a single experiment. PCell Approach B for View, TSM for zero assumes, slice-backed for O(1) split, unlocked trait for composable parallelism.

### 12.2. Migration

1. Verify experiments
2. Migrate one real module (graph module from Chap52+ — has both sequences and sets)
3. Update `toplevel_coarse_rwlocks_for_mt_modules.rs` standard
4. Systematize across all Mt modules

### 12.3. Verus Dependencies

- `make-ghost-send-sync` branch: fixes `Ghost<T>` Send/Sync, eliminates need for `unsafe impl Send/Sync` on types containing Ghost fields
- `new-mut-ref`: enables disjoint mutable borrows, potentially simplifying the parallel write pattern
- Clone on closures: still unrecognized by Verus; `clone_fn` workaround remains necessary
