# R119 Agent 2 — Fill proof holes in Chap43 + Chap52 + Chap53. AFK. DOT.

## Problem

19 proof holes across 5 chapters. Agent1 is handling Chap37 (5 files, all
RwLock-boundary assumes). You handle the remaining 3 files with holes.

## Target files (3 files, 3 holes)

### 1. Chap52/AdjTableGraphMtPer.rs (1 hole)

**Line 440:**
```rust
move |neighbors: &AVLTreeSetMtPer<V>| -> (r: AVLTreeSetMtPer<V>)
    ensures r@ == neighbors@.remove(v_clone@)
{
    proof {
        assume(neighbors.spec_avltreesetmtper_wf());
    }
    neighbors.delete(&v_clone)
},
```

This is inside a closure passed to `ordered_table.map()`. The assume bridges
the fact that `delete` requires `spec_avltreesetmtper_wf()` but the map
callback has no way to express that the table's values are well-formed.

**Fix strategy**: The ordered table's stored values (AVLTreeSetMtPer) are
always wf because they were inserted via wf-preserving operations. Check if
the table's wf predicate can be strengthened to include value wf, or if
the map callback's requires can carry this. If neither works, this assume
may be genuinely structural — document why and leave it.

### 2. Chap53/GraphSearchMtPer.rs (1 hole)

**Line 179:**
```rust
assume(neighbors.spec_avltreesetmtper_wf());
```

Same pattern — graph closure returns an AVLTreeSetMtPer and the caller
needs wf. Check if the graph's accessor (`out_neighbors`) ensures wf on
the returned set. If it does, propagate that ensures into the closure.
If it doesn't, check if it should.

### 3. Chap43/OrderedSetMtEph.rs (10 assumes, 1 hole counted)

The file has ~10 assumes but veracity counts it as 1 hole. These are
RwLock-boundary assumes. The assumes fall into categories:

**Category A — result bridging (5 assumes):**
```rust
assume(inner@ =~= self@);  // lines 404, 461, 473, 485, 497
```
These bridge the fact that after acquiring the RwLock read guard, the
inner value's view equals self's view. This should follow from the
RwLockPredicate invariant. Check if `self@` is defined as reading
through the lock, and if so, these may be provable.

**Category B — operation results (2 assumes):**
```rust
assume(count == self@.len());   // line 290
assume(found == self@.contains(x@));  // line 320
```
These bridge inner St operation ensures through the lock boundary.

**Category C — capacity bounds (3 assumes):**
```rust
assume(locked_val@.len() + 1 < usize::MAX as nat);  // lines 330, 504
assume(locked_val@.len() + other_ref@.len() < usize::MAX as nat);  // line 378
```
These are capacity preconditions the outer function should require.
Check if the outer trait functions have `requires self@.len() < usize::MAX`
or similar. If not, add the requires and remove the assumes.

## Work order

1. Read `src/Chap43/OrderedSetMtEph.rs` and `src/Chap43/OrderedSetStEph.rs`.
2. Tackle Category C first (capacity assumes) — most likely fixable by
   adding requires to the outer trait functions.
3. Tackle Category A (inner@ =~= self@) — check if provable from RwLock inv.
4. Tackle Category B (operation results) — check if provable from inner ensures.
5. Read `src/Chap52/AdjTableGraphMtPer.rs` — the map closure context.
6. Read `src/Chap53/GraphSearchMtPer.rs` — the graph closure context.
7. Try to fix the wf assumes in Chap52/53.
8. Validate each: `scripts/validate.sh isolate Chap43`, then Chap52, then Chap53.
9. RTTs: `scripts/rtt.sh`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add accept. If an assume can't be removed, leave it as assume.
- Do NOT convert assume to accept.
- If an assume is genuinely structural (RwLock boundary, no path to prove it),
  document exactly why and leave it.
- Adding requires to remove capacity assumes is good — but fix callers.
- No subagents.

## STEP 25

## Report

Write `plans/agent2-r119-hole-fills-report.md`. For each assume, report:
fixed (removed), or structural (documented why it can't be removed).
