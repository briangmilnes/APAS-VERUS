# R102 Agent 2 — Prove StarPartitionMtEph (value vs view no_duplicates), STEP 20

## Objective

The last Chap62 hole: `parallel_star_partition` in StarPartitionMtEph.rs.
External_body because `SetStEph::to_seq()` gives value-level `no_duplicates`
but the proof needs view-level `no_duplicates`.

## Key discovery: the Verus ICE is GONE

The ICE on `Set<V::V>` quantifiers in proof mode was fixed. You CAN now
write `assert forall` over `Map<V::V, Set<V::V>>` and similar. This was
confirmed in R99 agent1.

## The problem

`SetStEph::to_seq()` ensures `seq@.no_duplicates()` — meaning
`i != j ==> seq[i] != seq[j]` (value-level). The proof needs
`i != j ==> seq[i]@ != seq[j]@` (view-level).

## Strategy: Bridge lemma

Write a proof lemma that for types satisfying `StT + Hash + Eq`, if elements
come from a `SetStEph` (which uses `HashSetWithViewPlus` internally), then
view-level no_duplicates holds.

The argument: `HashSetWithViewPlus` membership is view-based (`contains` checks
`@`). Two elements with the same view can't both be in the set. Therefore
`to_seq()` produces view-unique elements.

```rust
proof fn lemma_to_seq_view_no_duplicates<V: StT + Hash>(set: &SetStEph<V>)
    requires set.spec_setsteph_wf()
    ensures
        forall|i: int, j: int|
            0 <= i < j < set.to_seq()@.len()
            ==> set.to_seq()@[i]@ != set.to_seq()@[j]@
```

The proof: by contradiction. If `seq[i]@ == seq[j]@` with `i != j`, then
`set@.contains(seq[i]@)` and `set@.contains(seq[j]@)` — but `seq[i]@ == seq[j]@`
means they're the same view-level element. The set can only contain it once,
so the seq can only map to it once... but `to_seq` maps the set bijectively.

Actually, the simpler approach: `to_seq` ensures `to_seq()@.to_set() == set@`.
If `seq[i]@ == seq[j]@`, then `to_set` maps both to the same element. But
`no_duplicates` + `to_set` preserves length. Contradiction with pigeonhole.

## After the bridge lemma

With view-level no_duplicates established, the 6 loops in parallel_star_partition
can use `seq[i]@ != seq[j]@` to derive contradictions where the proof currently
fails. Remove the `external_body` and fix the proof body (which is already
written inside the external_body — it was preserved).

## Read first

- `src/Chap62/StarPartitionMtEph.rs` — your file (proof body inside external_body)
- `src/Chap62/StarPartitionStEph.rs` — proved StEph version (reference)
- `src/Chap05/SetStEph.rs` — to_seq ensures, HashSetWithViewPlus backing
- Agent4 R89 report — previous attempt and analysis of the gap

## Isolation

```bash
scripts/validate.sh isolate Chap64
```

(Pulls in Chap62 transitively.)

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- The ICE is gone. Don't be afraid of quantifiers over `Set<V::V>`.
- The proof body is already inside the external_body — 200+ lines of loop
  invariants and intermediate assertions. Start by removing external_body
  and seeing what fails.
- Do NOT add assume or accept.
- Even proving with a few remaining assumes is progress over external_body.

## STEP 20

## Report

Write `plans/agent2-r102-starpartition-report.md`.
