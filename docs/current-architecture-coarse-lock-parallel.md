<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Current Architecture: Coarse RwLock with Ghost View

## Status: Deployed (2026-04-02)

This document describes the Mt module architecture as currently deployed across
APAS-VERUS. It covers the concurrency pattern, assume taxonomy, what TSM
migration would change, and what it would not.

For the target architecture (RwLock+TSM+parallel inside), see
`docs/architecture-coarse-lock-parallel-mt.md`.

## 1. Desired Properties

We want four properties from every Mt module:

| # | Property | Description |
|---|----------|-------------|
| a | Zero-Assume Locking | Lock acquire/release proves data identity without assumes |
| b | Caller-Observable State | Callers write specs against `self@` without holding the lock |
| c | Composable Unlocked Parallelism | M1 owns M2 inside its lock, calls M2's parallel ops directly |
| d | Optimal Split Cost | O(1) split for D&C parallelism to hit textbook span bounds |

The current architecture achieves (b), (c), and (d) but not (a). Every lock
boundary needs assumes to bridge the ghost field to the locked data. The
alternative architectures (RwLock+TSM, PCell+TSM+Atomics) achieve (a) at
the cost of additional boilerplate or proof complexity.

### 1.1. Per-interface assume cost (single Mt module)

What a typical Mt module with 10 operations looks like today:

| # | Operation | Assumes | Pattern |
|---|-----------|---------|---------|
| 1 | insert (write) | 1 | ghost-lock bridge |
| 2 | delete (write) | 1 | ghost-lock bridge |
| 3 | find (read) | 2 | ghost-lock bridge + result correctness |
| 4 | contains (read) | 2 | ghost-lock bridge + result correctness |
| 5 | size (read) | 2 | ghost-lock bridge + return value |
| 6 | is_empty (read) | 2 | ghost-lock bridge + return value |
| 7 | height (read) | 2 | ghost-lock bridge + return value |
| 8 | min (read) | 3 | ghost-lock bridge + exists + contains |
| 9 | max (read) | 3 | ghost-lock bridge + exists + contains |
| 10 | union (write) | 1 | ghost-lock bridge |
| | **Subtotal (lock)** | **19** | |
| 11 | clone | 1 | Verus limitation |
| 12 | eq | 1 | Verus limitation |
| 13 | iterator next | 1 | Verus limitation |
| | **Subtotal (Verus)** | **3** | |
| | **Total per module** | **~22** | |

Every read has at least 2 assumes (bridge + result). Writes have 1 (bridge
only, because the StEph operation's ensures chain through). Reads with rich
return values (min/max with exists/contains guarantees) have 3.

The alternative architectures eliminate the lock column (~19) but not the
Verus column (~3). See `docs/architecture-coarse-lock-parallel-mt.md`.

## 2. How It Works

Each Mt module wraps an inner StEph type with an RwLock and a ghost field:

```rust
struct BSTPlainMtEph<T> {
    lock: RwLock<ParamBST<T>, BSTPlainMtEphInv>,
    ghost ghost_root: Ghost<ParamBSTView<T>>,
}
```

The ghost field provides caller-observable state (`View`) without holding
the lock. Operations follow this pattern:

```
acquire_write → locked_data
assume(self.ghost_field@ == locked_data@)    // bridge
result = locked_data.st_operation()
self.ghost_field = Ghost(new_abstract_state)
release_write(locked_data)
```

Every read and write needs an assume to bridge the ghost field (outside the
lock, visible to callers) to the locked data (inside the lock, known only
after acquire).

Reads are similar but acquire_read and don't update the ghost.

## 3. Assume Taxonomy

There are 183 assumes across 30+ Mt files. They fall into well-defined
categories.

### 4.1. Categories

| # | Category | Count | Example | Necessary? |
|---|----------|-------|---------|------------|
| 1 | Ghost-lock bridge | ~25 | `assume(self.ghost_root@ == tree)` | TSM eliminates |
| 2 | View propagation | ~15 | `assume(inner@ =~= self@)` | TSM eliminates |
| 3 | Read result correctness | ~30 | `assume(found == tree_contains(target))` | TSM eliminates |
| 4 | Return value (size, height, isEmpty) | ~12 | `assume(n as nat == spec_size())` | TSM eliminates |
| 5 | Extrema (min, max) | ~10 | `assume(min.is_some() ==> tree_contains(min))` | TSM eliminates |
| 6 | Inner type wf | ~16 | `assume(spec_orderedtablesteph_wf())` | TSM partially eliminates |
| 7 | Find value correspondence | 6 | `accept(found.unwrap() == *target)` | TSM eliminates (now accepts) |
| 8 | Clone bridge | ~12 | `assume(cloned@ == self@)` | Verus limitation, stays |
| 9 | PartialEq bridge | ~12 | `assume(equal == (self@ == other@))` | Verus limitation, stays |
| 10 | Unreachable diverge | 7 | `assume(false); diverge()` | Thread join error arm, stays |
| 11 | Capacity | ~6 | `assume(len < usize::MAX)` | Exec guard, stays |
| 12 | Iterator invariant | ~5 | `assume(iter_invariant(self))` | Verus limitation, stays |
| 13 | Domain finiteness | ~3 | `assume(self@.dom().finite())` | TSM eliminates |
| 14 | Miscellaneous | ~24 | Various | Mixed |

### 4.2. The ghost-lock bridge in detail

The single most common assume pattern. After acquiring the lock, the code
needs to assert that the ghost field (which callers used to write specs)
matches the actual locked data. The ghost field was set on the last release,
but Verus can't prove it still holds because another thread could have
modified the data. In practice, the RwLock guarantees this — but Verus
doesn't know that.

```rust
fn insert(&mut self, x: T) {
    let (mut tree, write_handle) = self.lock.acquire_write();
    proof { assume(self.ghost_root@ == tree@); }  // THE BRIDGE
    let new_tree = tree.insert(x);
    proof { self.ghost_root = Ghost(new_tree@); }
    write_handle.release_write(new_tree);
}
```

TSM eliminates this by putting a token inside the lock. The RwLockPredicate
ties the token to the data: `inv(data, token) == (data@ == token.value)`.
After acquire, the predicate is known true — no assume needed.

### 4.3. The read-result pattern

After a read operation (find, size, contains, min, max), the code assumes
the result from the inner StEph operation matches the abstract state:

```rust
fn find(&self, target: &T) -> Option<T> {
    let tree = self.lock.acquire_read();
    let found = tree.find(target);
    proof { assume(found == self@.tree_contains(*target)); }  // read result
    self.lock.release_read(tree);
    found
}
```

TSM eliminates this because the token proves `tree@ == self@`, so the
StEph operation's ensures (which are proved from `tree@`) chain through
the token to prove the result in terms of `self@`.

## 4. What TSM Migration Changes

### 4.1. Structure

Replace the ghost field with a TSM instance:

```rust
tokenized_state_machine! { BSTPlainTSM {
    fields { #[sharding(variable)] pub root: ParamBSTView<T> }
    init!{ initialize(r: ParamBSTView<T>) { init root = r; } }
    transition!{ do_insert(old_root: ParamBSTView<T>, new_root: ParamBSTView<T>) {
        require pre.root == old_root;
        update root = new_root;
    }}
    // ... one transition per operation
}}

struct BSTPlainMtEph<T> {
    lock: RwLock<(ParamBST<T>, BSTPlainTSM::Instance), BSTPlainMtEphInv<T>>,
}

impl RwLockPredicate<(ParamBST<T>, BSTPlainTSM::Instance)> for BSTPlainMtEphInv<T> {
    open spec fn inv(self, v: (ParamBST<T>, BSTPlainTSM::Instance)) -> bool {
        v.1.root() == v.0@  // token matches data
    }
}
```

### 4.2. Operation pattern with TSM

```rust
fn insert(&mut self, x: T) {
    let ((mut tree, token), write_handle) = self.lock.acquire_write();
    // predicate gives: token.root() == tree@  -- NO ASSUME
    let new_tree = tree.insert(x);
    let new_token = token.do_insert(tree@, new_tree@);
    write_handle.release_write((new_tree, new_token));
}
```

### 4.3. Assume reduction

| Metric | Current | After TSM |
|--------|---------|-----------|
| Total assumes | 183 | ~62 |
| Assumes eliminated | — | ~121 (66%) |
| Assumes remaining | 183 | ~62 |
| Categories eliminated | — | Ghost-lock bridge, view propagation, read results, return values, extrema, finiteness |
| Categories unchanged | — | Clone, PartialEq, diverge, capacity, iterator |
| TSM boilerplate per file | 0 lines | ~68 lines |
| Accepts per file (View) | 2 | 2 |

### 4.4. What TSM does NOT fix

**Clone bridge (~12 assumes).** Verus does not recognize Clone on closures
or generic types. The `assume(cloned@ == self@)` inside `Clone::clone` is a
Verus workaround, not a concurrency issue. TSM doesn't help.

**PartialEq bridge (~12 assumes).** Same situation. `assume(equal == (self@
== other@))` inside `PartialEq::eq` is a Verus workaround for opaque equality.

**assume(false); diverge() (7).** Thread join can return Err (thread panicked).
The error arm is unreachable in correct code but Verus can't prove it. This
is inherent to Rust's threading model.

**Capacity (~6 assumes).** Integer overflow guards. These need exec-time
checks regardless of the locking strategy. TSM doesn't change arithmetic.

**Iterator invariant (~5).** Verus doesn't allow `requires` on external
trait impls (`std::iter::Iterator::next`). Hand-rolled iterators need the
assume. This is a Verus limitation, not a concurrency issue.

**View accepts (2 per file).** The final bridge from TSM token to `self@`
(the View) still needs an accept. The token proves `data@ == token.value`.
The accept says `self@ == token.value`. This is the irreducible gap:
callers see `self@` without holding the lock, so the connection between
the lock-protected token and the externally-visible view requires trust.

## 5. The Accepts: Why 2 Per File Is the Floor

Both the current pattern and TSM need exactly 2 accepts per file for
the View bridge:

1. **Write accept**: After release_write, the ghost View must equal the
   new token/data state. The caller can't verify this because it just
   released the lock.

2. **Read accept**: After release_read, the return value must correspond
   to the View the caller had before the read. The lock guarantees this
   but Verus can't prove it across the acquire/release boundary.

These 2 accepts are architecturally irreducible with Verus's current
type system. PCell+`&mut self` can eliminate the write accept (ownership
guarantees sole access), but the read accept remains.

## 6. Current Proof Hole Status (2026-04-02)

10 holes on main after R138-R140 merges:

| # | Chap | File | Hole | Blocked on |
|---|------|------|------|------------|
| 1 | 19 | ArraySeqMtEphSlice.rs | bare_impl | Being fixed (R141 agent3) |
| 2 | 19 | ArraySeqMtEphSlice.rs | fn_missing_ensures | Being fixed (R141 agent3) |
| 3 | 41 | AVLTreeSetMtEph.rs | unsafe impl Send | Verus: Ghost not Send |
| 4 | 41 | AVLTreeSetMtEph.rs | unsafe impl Sync | Verus: Ghost not Sync |
| 5 | 41 | AVLTreeSetMtPer.rs | unsafe impl Send | Verus: Ghost not Send |
| 6 | 41 | AVLTreeSetMtPer.rs | unsafe impl Sync | Verus: Ghost not Sync |
| 7-10 | 57 | DijkstraStEphU64.rs | 4× assume/proof_fn | Verus: OrdSpecImpl panic |

### 6.1. Blocked holes by Verus issue

| Issue | Holes | Verus fix |
|-------|-------|-----------|
| Ghost not Send/Sync | 4 | make-ghost-send-sync (Elanor Tang, UMich) |
| OrdSpecImpl panic on user types | 4 | vir/ast_util.rs:734 fix needed |

### 6.2. Verification metrics

- 5608 verified, 0 errors
- 3627 runtime tests pass
- 221 proof-time tests pass

## 7. Migration Path

TSM migration is a separate project phase. The steps:

1. Finish current algorithmic work (OrderedTable bugs, parallel ops, DIFFERS).
2. Wait for Verus make-ghost-send-sync (eliminates 4 unsafe holes).
3. Pick one Mt module as pilot (BSTPlainMtEph — simplest, 13 assumes).
4. Define TSM, add predicate, convert operations one at a time.
5. Validate each operation individually (isolate mode).
6. Measure: verify assumes eliminated vs boilerplate added.
7. If successful, migrate remaining 30+ Mt files.

Expected result: 183 assumes → ~62. Plus ~2000 lines of TSM boilerplate across
the codebase. The remaining 62 are Verus language limitations that will decrease
as Verus evolves (Clone on closures, iterator requires, OrdSpecImpl for user types).
