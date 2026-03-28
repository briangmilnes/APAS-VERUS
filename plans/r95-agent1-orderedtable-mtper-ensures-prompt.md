# R95 Agent 1 — Strengthen OrderedTableMtPer ensures (Chap43), STEP 20

## Objective

OrderedTableMtPer's `find`, `insert`, and `delete` have weak or missing ensures.
This blocks 16 assumes in AdjTableGraphMtPer. Strengthen the ensures to match
the StEph/StPer Table API.

## Current state

Read `src/Chap43/OrderedTableMtPer.rs`. The current ensures:

- **find**: NO ensures at all (returns `Option<V>` with no spec)
- **insert**: only `ensures updated@.dom().finite()` (no content spec)
- **delete**: only `ensures updated@.dom().finite()` (no content spec)

## Target ensures (match TableStEph pattern)

### find
```rust
fn find(&self, key: &K) -> (found: Option<V>)
    ensures
        match found {
            Some(v) => self@.contains_key(key@) && self@[key@] == v@,
            None => !self@.contains_key(key@),
        };
```

### insert
```rust
fn insert(&self, key: K, value: V) -> (updated: Self)
    ensures
        updated@.contains_key(key@),
        updated@.dom() =~= self@.dom().insert(key@),
        !self@.contains_key(key@) ==> updated@[key@] == value@,
        forall|k: K::V| k != key@ && self@.contains_key(k)
            ==> updated@.contains_key(k) && updated@[k] == self@[k];
```

### delete
```rust
fn delete(&self, key: &K) -> (updated: Self)
    ensures
        updated@ =~= self@.remove(key@);
```

## How to prove

OrderedTableMtPer wraps an AVLTreeSeqMtPer (or similar backing store) with
RwLock for thread safety. The inner operations already have specs — you need
to propagate them through the lock wrapper.

Read:
- `src/Chap43/OrderedTableMtPer.rs` — your file
- `src/Chap43/OrderedTableStPer.rs` — StPer version with full ensures (your template)
- `src/Chap43/OrderedTableStEph.rs` — StEph version with full ensures
- `src/Chap43/OrderedTableMtEph.rs` — MtEph version (may have stronger ensures than MtPer)

The MtPer version likely delegates to the StPer inner table while holding the
lock. The ensures should flow through from the inner operation's ensures.

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

Then check Chap52 callers:
```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers. Adding ensures is non-breaking (strengthening).
- Do NOT add assume or accept.
- Do NOT weaken existing ensures.
- If the lock wrapper makes it hard to propagate ensures, use external_body on
  individual functions with the STRONG ensures (not the weak ones). An
  external_body with strong ensures is better than a proved body with weak ensures.
- Even getting find's ensures right unblocks 5+ MtPer assumes in AdjTableGraph.

## STEP 20

## Report

Write `plans/agent1-r95-orderedtable-report.md`.
