# Agent 4 — Round 24: Chap42 Tables — Investigate and Prove

## Mission

Prove holes in Chap42's 3 Table files (15 `external_body` holes). All holes are on
closure-using functions: tabulate, map, filter, insert, intersection, union.

## Current State

| # | Chap | File | Holes | Functions |
|---|------|------|:-----:|-----------|
| 1 | 42 | TableStEph.rs | 6 | tabulate, map, filter, intersection, union, insert |
| 2 | 42 | TableStPer.rs | 5 | tabulate, map, filter, union, insert |
| 3 | 42 | TableMtEph.rs | 4 | tabulate, map, filter, insert |

All 15 holes are `external_body` on functions that take closure arguments (`Fn`).

## Dependencies

Chap42 was previously blocked by `Chap41::ArraySetStEph` — that's now clean (filter
proved in R23). Check if any other external deps are holed.

## Approach

### Step 1: Read and understand TableStEph.rs

Read the file thoroughly. Understand:
- The data model (View type — likely `Map<K::V, V::V>`)
- How closures are used (tabulate builds from keys, map transforms values, filter selects,
  insert with combine function for duplicates)
- What the existing ensures say
- Read `src/standards/using_closures_standard.rs` for closure verification patterns

### Step 2: Start with `insert` (simplest closure use)

`insert` takes a `combine: Fn(&V, &V) -> V` for handling duplicate keys. The body
likely does a lookup + insert-or-combine. This is the simplest closure pattern — single
call site.

Prove:
```rust
fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
    ensures
        self@.dom() == old(self)@.dom().insert(key@),
        // key's value is either combined or fresh
```

### Step 3: Prove `map`

`map` transforms all values: `self@[k] == f(old(self)@[k])` for all k. This is a
loop over entries with a closure call per entry. Loop invariant tracks the map state.

### Step 4: Prove `filter`

`filter` keeps entries satisfying a predicate. You proved ArraySet::filter in R23 —
similar pattern but on key-value pairs.

### Step 5: Prove `tabulate`

`tabulate` builds a table from a set of keys and a function: `result@[k] == f(k)`.
Loop over keys, call f, insert each.

### Step 6: Prove `intersection` and `union` (TableStEph only)

These combine two tables with a `combine` function for shared keys. More complex —
requires iterating both tables.

### Step 7: Apply to StPer and MtEph

StPer should follow the same pattern as StEph (persistent Arc-based variant).
MtEph has the threading layer — check if the closures need `Send + Sync + 'static`
handling. For Mt functions, verify the parallel structure (Phase 4 from reviews).

## Closure Verification Pattern

From `using_closures_standard.rs`, the key pattern:

```rust
fn map<F: Fn(&V) -> V>(&mut self, f: F, Ghost(spec_f): Ghost<spec_fn(V::V) -> V::V>)
    requires
        forall|v: &V| f.requires((v,)),
        forall|v: V, r: V| f.ensures((&v,), r) ==> r@ == spec_f(v@),
    ensures
        forall|k| self@.dom().contains(k) ==>
            self@[k] == spec_f(old(self)@[k]),
```

The closure's `requires`/`ensures` must be threaded through the loop invariant.
Read the standard before writing proofs.

## Important

- Read `src/standards/using_closures_standard.rs` FIRST.
- Do NOT add `assume` or `accept` in algorithmic code.
- Do NOT modify `requires`/`ensures` unless strengthening.
- The closure verification pattern is precise — follow it exactly.
- `scripts/validate.sh` after each function — 0 errors.

## Deliverables

- Proven holes in Chap42 Table files (as many as achievable).
- `plans/agent4-round24-report.md` with clear accounting per function.
- 0 errors on validate.
- Commit + push to `agent4/ready`.
