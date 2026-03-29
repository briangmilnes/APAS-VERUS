# R102 Agent 1 — Fix OrderedTableMtPer RwLock predicate + prove 9 holes, STEP 20

## Objective

The RwLock predicate on OrderedTableMtPer only carries `spec_orderedtablestper_wf()`.
It's missing `v@ == ghost_view` — the link between the locked inner table and the
ghost abstract view. This is why all 6 OrderedTableMtPer functions are external_body
and all 3 AdjTableGraphMtPer assumes are stuck.

Fix the predicate. Then remove the external_body from find/insert/delete/insert_wf/
delete_wf/map and prove them. Then prove the 3 Chap52 MtPer assumes.

## The Bug

In `src/Chap43/OrderedTableMtPer.rs` line 45-50:

```rust
pub struct OrderedTableMtPerInv;

impl RwLockPredicate<OrderedTableStPer<K, V>> for OrderedTableMtPerInv {
    open spec fn inv(self, v: OrderedTableStPer<K, V>) -> bool {
        v.spec_orderedtablestper_wf()  // MISSING: v@ == ghost_view
    }
}
```

## The Fix

Add a ghost field to the predicate struct:

```rust
pub struct OrderedTableMtPerInv<K: MtKey, V: StTInMtT> {
    pub ghost expected_view: Map<K::V, V::V>,
}

impl RwLockPredicate<OrderedTableStPer<K, V>> for OrderedTableMtPerInv<K, V> {
    open spec fn inv(self, v: OrderedTableStPer<K, V>) -> bool {
        v.spec_orderedtablestper_wf()
        && v@ == self.expected_view
    }
}
```

Then at construction (line 73-85), set `expected_view` to `inner@`. The RwLock
gets `new_arc_rwlock(inner, Ghost(OrderedTableMtPerInv { expected_view: inner@ }))`.

On lock acquire: `inv` gives `inner.spec_wf() && inner@ == expected_view`. Since
`self.ghost_locked_table@ == expected_view` (from construction), you get
`inner@ == self@`. The chain is complete.

## After fixing the predicate

Remove `#[verifier::external_body]` from all 6 functions. Each follows the pattern:
1. Acquire lock (read)
2. Get `inner` + proof that `inner@ == self@`
3. Call `inner.method()`
4. Method's ensures + view equality → MtPer ensures

For mutation (insert/delete): also update the ghost field and prove the new
predicate holds.

## Then fix Chap52

With strong MtPer ensures, prove the 3 AdjTableGraphMtPer assumes:
- num_edges overflow (use find with domain iteration)
- num_edges sum (use find + size coupling)
- delete_vertex wf (use map with value ensures)

## Read first

- `src/Chap43/OrderedTableMtPer.rs` — your primary file
- `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — the correct pattern
- `src/Chap05/SetMtEph.rs` — working example of RwLock with ghost view predicate
- `src/vstdplus/arc_rwlock.rs` — new_arc_rwlock, clone_arc_rwlock helpers
- `src/Chap52/AdjTableGraphMtPer.rs` — the 3 downstream assumes

## Isolation

```bash
scripts/validate.sh isolate Chap43
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- This is the biggest single fix remaining. 9 holes from 1 predicate change.
- The pattern exists in every other Mt module. Mirror it exactly.
- If mutation (insert/delete) is hard (ghost field update through RwLock),
  prove find first (read-only, simpler), then tackle mutations.
- Do NOT add assume or accept.

## STEP 20

## Report

Write `plans/agent1-r102-rwlock-fix-report.md`.
