# R99 Agent 1 — Test if the Verus ICE still exists, STEP 20

## Objective

11 assumes in Chap52 AdjTableGraph are classified as "blocked by Verus ICE."
But we've never tested this on the current Verus revision. The standalone
reproducer in `bugs/ice-set-type-projection/` passed clean. The ICE may have
been fixed.

Test each ICE-blocked assume by removing it and running validate. If it
verifies — prove it. If it crashes — capture the error and move on to the next.

## Method

For each ICE-blocked assume, one at a time:

1. Comment out the assume
2. Run `scripts/validate.sh isolate Chap52`
3. Three outcomes:
   - **Verifies**: The ICE is fixed. Now prove the assertion properly.
   - **Verification error** (not crash): The ICE is fixed but the proof needs work. Try to prove it. If too hard, put the assume back.
   - **Verus panic/crash**: The ICE still exists. Put the assume back, note the exact error and stack trace in your report, move to the next assume.

## Start with the simplest

**AdjTableGraphStEph.rs line 473** — `insert_vertex` stored-value-wf:
```rust
assume(forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) ==>
    self.spec_stored_value(k).spec_avltreesetsteph_wf());
```

This is the simplest ICE-blocked assume because insert_vertex just adds an
empty set — the stored-value-wf should follow from insert_wf ensures.

If that works, try the others in order:
- StEph insert_edge (line 612)
- StEph delete_edge (line 653)
- StEph delete_vertex (lines 507, 521)
- StPer insert_vertex (line 426)
- StPer delete_vertex (lines 461, 473)
- StPer delete_edge (line 559)
- MtPer insert_vertex graph wf (line 323 — rwlock:predicate)

## If the ICE is gone

If NONE of the assumes crash Verus, then the ICE was fixed in a recent Verus
update. Prove as many as you can within 20 steps. The stored-value-wf quantifier
should follow from insert_wf/delete_wf ensures. The graph-closure quantifier
needs the subset proof (agent1 R96 technique).

## If the ICE persists

For each crashing assume, capture:
- The exact line and assume text
- The Verus error message (first 5 lines of the panic)
- Whether it's the same "abstract datatype should be boxed" error

Then try option 2: write a **generic lemma** that quantifies over `Map<A, Set<A>>`
with plain type parameter `A` instead of `V::V`:

```rust
proof fn lemma_stored_values_wf<A>(
    adj: Map<A, Set<A>>,
    entries: Seq<(A, Set<A>)>,
)
    requires /* each entry value is finite */
    ensures forall|k: A| adj.dom().contains(k) ==> adj[k].finite()
```

Call it with `A = <V as View>::V`. If the ICE is on the projection inside the
quantifier, this dodges it.

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — find the ICE-blocked assumes
- `bugs/ice-set-type-projection/reproducer.rs` — standalone test that passed
- `bugs/ice-set-type-projection/README.md` — context

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- ALWAYS put the assume back if the proof doesn't work. Never leave code broken.
- Capture crash output for the bug report.
- Even proving 1 ICE-blocked assume is a breakthrough — it means they're ALL provable.

## STEP 20

## Report

Write `plans/agent1-r99-ice-test-report.md`.
