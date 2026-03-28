# R96 Agent 2 — Add clone_wf to Table::insert combine chain, STEP 20

## Objective

Table::insert uses a `combine: Fn(&V, &V) -> V` closure to merge values when
a key already exists. In AdjTableGraph, the combine is `|_old, new| new.clone()`.
The clone doesn't preserve wf — `new.clone()@ == new@` but NOT
`new.clone().spec_avltreesetsteph_wf()`.

This blocks ~8 stored-value-wf assumes across AdjTableGraphStEph and StPer.

## Strategy

Add a `insert_with_clone_wf` method (or modify the existing `insert`) that
ensures stored-value wf is preserved through the combine closure.

### Option A: New method `insert_wf`

Add to TableStEph trait:
```rust
fn insert_wf(&mut self, key: K, value: V)
    requires
        old(self).spec_tablesteph_wf(),
        value.spec_wf(),  // V: ClonePreservesWf
        // all existing stored values are wf
    ensures
        self.spec_tablesteph_wf(),
        self@.contains_key(key@),
        self@[key@] == value@,
        // all stored values still wf (old ones via clone_wf, new one from requires)
```

The impl replaces the combine path's `old.clone()` / `new.clone()` with
`old.clone_wf()` / `new.clone_wf()`. Since `V: ClonePreservesWf`, the wf
ensures propagates.

### Option B: Strengthen existing insert ensures

Add a stored-value-wf quantifier to the existing `insert` ensures:
```rust
ensures
    forall|k: K::V| self@.contains_key(k) && k != key@
        ==> self.spec_stored_value(k).spec_wf() == old(self).spec_stored_value(k).spec_wf()
```

This requires `V: ClonePreservesWf` as a bound on insert (or on the Table type).

### Option C: Require combine preserves wf

Add a requires clause saying the combine closure preserves wf:
```rust
requires
    forall|v1: &V, v2: &V| v1.spec_wf() && v2.spec_wf()
        ==> combine(v1, v2).spec_wf()
```

Pick whichever approach is cleanest. Option A is safest (no existing caller breakage).

## Read first

- `src/Chap42/TableStEph.rs` — insert trait + impl (line ~413)
- `src/Chap42/TableStPer.rs` — insert trait + impl
- `src/vstdplus/clone_view.rs` — ClonePreservesWf trait
- `src/Chap52/AdjTableGraphStEph.rs` — the caller (see insert_edge, insert_vertex)

## Isolation

```bash
scripts/validate.sh isolate Chap42
```

Then check callers:
```bash
scripts/validate.sh isolate Chap43
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers. Option A (new method) is safest.
- Do NOT add assume or accept.
- The goal is to provide a Table insert path that preserves stored-value wf.
  AdjTableGraph callers can switch to this path in a follow-up round.
- Do both StEph and StPer if time allows.

## STEP 20

## Report

Write `plans/agent2-r96-table-insert-wf-report.md`.
