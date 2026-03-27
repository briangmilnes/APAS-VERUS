# R89 Agent 1 — Strengthen Table find ensures with value wf, STEP 20

## Objective

When `Table::find` returns `Some(v)`, callers need to know `v` is well-formed.
Currently find only ensures `self@[key@] == v@` — the abstract map equality. It
says nothing about the concrete value's well-formedness. This blocks AdjTableGraph
from proving anything after find (24 external_body holes).

Strengthen `find` ensures in TableStEph and TableStPer so that a well-formed table
returns well-formed values.

## The Problem

AdjTableGraph stores `Table<V, AVLTreeSetStEph<V>>` — vertices map to neighbor sets.
When `find` returns a neighbor set, the caller needs `neighbors.spec_avltreesetsteph_wf()`
to call insert/mem/etc. But `find` doesn't ensure this.

## Approach

The cleanest fix: add to `spec_tablesteph_wf` a clause that says all values in the
table are "well-formed" in some sense that propagates through `find`. Two options:

### Option A: Add value wf to table wf (preferred)

Add a clause to `spec_tablesteph_wf` that all values satisfy some predicate.
Since `V: StT` has no wf concept, the simplest approach is to strengthen the
`find` ensures to propagate what the table's wf already implies.

Actually, the real issue may be simpler: `find` returns a cloned/copied `V`, and
the View equality `self@[key@] == v@` IS the correct spec. The problem is that
`AVLTreeSetStEph` has internal invariants (sorted, no-dups) that aren't captured
by its View (`Set<V::V>`). So the returned value has the right abstract content
but might not be structurally well-formed.

### Option B: Add ensures wf on returned value

If the table is wf and all inserted values were wf, then found values should be wf.
Add to the `find` ensures:

```rust
fn find(&self, key: &K) -> (found: Option<V>)
    requires self.spec_tablesteph_wf(), obeys_view_eq::<K>()
    ensures
        match found {
            Some(v) => self@.contains_key(key@) && self@[key@] == v@
                && /* NEW: v is well-formed */,
            None => !self@.contains_key(key@),
        };
```

The challenge: what predicate? V is generic. You need a trait bound or a spec fn
that V provides.

## What to read

- `src/Chap42/TableStEph.rs` — trait (line 288), wf (line 454/462), find (line 399/1562)
- `src/Chap42/TableStPer.rs` — same pattern, find at line 498
- `src/Chap42/TableMtEph.rs` — Mt version for reference
- `src/Chap52/AdjTableGraphStEph.rs` — the caller that needs value wf from find
- `src/Chap52/AdjTableGraphMtPer.rs` — working Mt version (all external_body — same gap)
- `src/standards/spec_wf_standard.rs` — wf naming conventions

## Constraints

The fix must be **backwards compatible**. Existing callers of Table (OrderedTable,
OrderedSet, AdjTableGraph, etc.) must still compile. Don't break Chap43.

Check what other modules call `Table::find` and ensure the new ensures don't
break their proofs:
```bash
grep -r "\.find(" src/Chap42/ src/Chap43/ src/Chap52/ --include="*.rs" | grep -v "test\|example"
```

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

This pulls in Chap42 transitively. If you only change Chap42, you can use:
```bash
scripts/validate.sh isolate Chap42
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add assume or accept.
- Do NOT weaken existing ensures.
- Do NOT add external_body.
- If the fix is too complex (needs a new trait bound on V), document what's needed
  and what you tried. A partial fix that strengthens some functions is better than
  no fix.
- The goal is to unblock AdjTableGraph proofs — even one function provable through
  the stronger find spec is a win.

## STEP 20

## Report

Write `plans/agent1-r89-table-find-wf-report.md`.
