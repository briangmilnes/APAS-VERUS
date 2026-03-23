# Agent 1 — Round 63

You are Agent 1 working in `~/projects/APAS-VERUS-agent1`.
You are a **senior proof engineer**. Your job is to prove, not to catalog
reasons you can't. Read the code, read the error, write the proof.

## Baseline

- Main: 4504 verified, 0 errors, 6 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Target: Close `call_hash_fn` external_body — 1 hole (Chap47, ParaHashTableStEph.rs:341)

`call_hash_fn` wraps a hash closure `H: Fn(&Key, usize) -> usize` with an
`external_body` because "Verus cannot reason about opaque Fn closures." But Verus
CAN reason about closures — via `f.requires` and `f.ensures`. The external_body
is unnecessary.

### Before you write any code

**Read `src/standards/using_closures_standard.rs`** in full. It shows three
patterns for propagating closure specs. You need Pattern A/C here.

### Current code (lines 339-349)

```rust
#[verifier::external_body]
pub fn call_hash_fn<Key, H: Fn(&Key, usize) -> usize>(
    hash_fn: &H, key: &Key, table_size: usize,
    spec_hash: Ghost<spec_fn(Key) -> nat>,
) -> (index: usize)
    requires table_size > 0,
    ensures
        index < table_size,
        index as nat == (spec_hash@)(*key) % (table_size as nat),
{
    (hash_fn)(key, table_size)
}
```

The `spec_hash` ghost parameter is the Pattern C companion — it's the spec-level
version of what the hash closure computes.

### Fix path

1. **Remove `#[verifier::external_body]`**.

2. **Add closure specs to requires**:
   ```rust
   requires
       table_size > 0,
       hash_fn.requires((&key, table_size)),
       forall|k: &Key, ts: usize, idx: usize|
           hash_fn.ensures((k, ts), idx) ==> idx < ts && idx as nat == (spec_hash@)(*k) % (ts as nat),
   ```

   The `forall` over `hash_fn.ensures` ties the opaque closure to `spec_hash`.
   This is the Pattern C bridge: the ghost `spec_fn` tells specs what the closure
   computes; the `forall` over ensures proves the connection.

3. **The body `(hash_fn)(key, table_size)` should now verify** because:
   - `hash_fn.requires` is satisfied (in our requires)
   - Verus knows the result satisfies `hash_fn.ensures`
   - Our `forall` converts that to `index < table_size && index as nat == spec_hash(key) % table_size`

4. **Propagate to callers.** Every call site of `call_hash_fn` must now prove:
   - `hash_fn.requires((&key, table_size))` — the hash fn accepts these args
   - The `forall` over `hash_fn.ensures` — the hash fn matches `spec_hash`

   Trace upward: `call_hash_fn` is called from functions in `ParaHashTableStEphTrait`
   impls (insert, lookup, delete, resize). Those take `H: Fn(...)` as a type param
   on the struct. The struct's callers (RTT tests, other modules) provide concrete
   closures.

   At each layer, lift the closure requires into the function's own requires clause.
   The closure standard says: **never assume a closure's requires — propagate it.**

5. **Check the trait signature** `ParaHashTableStEphTrait`. If `call_hash_fn`'s new
   requires need closure specs, the trait methods that call it must also carry those
   specs. Update the trait signatures, then update all impls.

### Watch out for

- The `forall` quantifier needs a trigger. Use `#[trigger] hash_fn.ensures((k, ts), idx)`.
- If callers construct `HashTable` with a concrete closure, they must prove that closure
  satisfies the `forall` over ensures. Look at how `new`/`createTable` constructs tables.
- The `Ghost(spec_hash)` is already threaded everywhere. The new requires just add
  `hash_fn.requires` and the ensures bridge — the spec_hash plumbing is done.

### Scope

This is a single-hole target but it touches the trait and all impls. Budget for
cascading requires updates across 3-4 files in Chap47. If the cascade gets large,
focus on getting `call_hash_fn` itself verified first, then fix callers one by one.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent1-round63-report.md`. Push to `agent1/ready`.
