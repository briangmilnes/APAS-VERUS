# R110 Agent 4 — Remove opaque from spec_key_unique_pairs_set. AFK. PBOGH.

## Objective

Remove `#[verifier::opaque]` from `spec_key_unique_pairs_set` in both:
- `src/Chap43/OrderedTableStEph.rs` line 79
- `src/Chap43/OrderedTableStPer.rs` line 81

These are 2 of the 7 remaining holes in the entire project.

## Background

`spec_key_unique_pairs_set` was made opaque in R105 (commit `0318daef0`) to kill
91K instantiations from its symmetric triggers:

```rust
pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
    forall|k: KV, v1: VV, v2: VV|
        s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2
}
```

The quantifier has `s.contains((k, v1))` and `s.contains((k, v2))` — each
instantiation produces new `contains` terms that trigger more instantiations.
Classic symmetric trigger problem.

The opaque fix was a blunt instrument: 21 `reveal(spec_key_unique_pairs_set)`
calls were added across the two files (12 in StEph, 9 in StPer) inside proof
lemmas. The reveals work in proof contexts because lemmas have bounded scope.
But the predicate is still counted as a hole because opaque hides the body from
callers who might need it.

## The real fix

The goal is NOT to remove opaque and let the matching loop return. The goal is
to restructure so the predicate doesn't need to be opaque.

### Strategy 1: Broadcast lemma approach

Keep opaque. Add a broadcast group that automatically proves key uniqueness is
preserved by the operations (insert, remove, subset). Then callers get the facts
they need without revealing the quantifier body. If every caller site that currently
calls `reveal` can instead get the fact from a broadcast lemma, the `reveal` calls
can be removed and the opaque is no longer a hole — it's a controlled abstraction.

Wait — opaque IS currently counted as a hole by veracity. So this doesn't help
unless veracity stops counting it.

### Strategy 2: Targeted trigger fix

The matching loop is caused by `s.contains((k, v1))` and `s.contains((k, v2))`
being symmetric in the variables v1/v2. If we can make the triggers asymmetric,
the loop breaks.

Try restructuring the quantifier:
```rust
pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
    forall|k: KV, v: VV|
        #[trigger] s.contains((k, v)) ==>
        forall|v2: VV| s.contains((k, v2)) ==> v == v2
}
```

This nests the quantifiers. The outer trigger is `s.contains((k, v))` — one term,
no symmetry. The inner quantifier only fires when the outer one matches. This
should prevent the self-feeding loop while preserving the same semantics.

BUT: nested quantifiers may confuse Z3 differently. And all 21 reveal sites +
all callers that pattern-match on the old shape will need updating.

### Strategy 3: Replace with Map-based uniqueness

Key uniqueness of a set of pairs is equivalent to: the set can be viewed as a
Map (each key maps to exactly one value). If the OrderedTable's View is already
`Map<K::V, V::V>`, then key uniqueness is automatic — Maps enforce it by
construction. Check whether `spec_key_unique_pairs_set` is actually needed or
whether it's redundant with the Map View.

## What to do

1. Read both files thoroughly. Understand every use of `spec_key_unique_pairs_set`.
2. Read the wf predicate — is key uniqueness a conjunct of wf?
3. Check if the Map View already guarantees uniqueness, making the predicate redundant.
4. Try Strategy 2 (nested quantifiers) first — it's the most direct fix.
5. If that causes too many cascading changes, try Strategy 3.
6. If nothing works after 15 steps, report what you tried and what blocked you.
   Do NOT just slap opaque back on and call it done.

## Read first

- `src/Chap43/OrderedTableStEph.rs` — full file, especially:
  - `spec_key_unique_pairs_set` (line 79)
  - All `reveal(spec_key_unique_pairs_set)` sites (12 of them)
  - `spec_orderedtablesteph_wf` — the wf predicate
  - How the View type relates to the pair set
- `src/Chap43/OrderedTableStPer.rs` — same predicate, same pattern
- `plans/agent2-r105-opaque-key-unique-report.md` — R105 report on why it was made opaque
- `src/Chap43/analyses/veracity-review-verus-proof-holes.log` — current hole status

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

Then check downstream: Chap44, Chap52 import from Chap43.

## Rules

- Do NOT add assume or accept.
- Do NOT add external_body.
- Do NOT weaken any ensures.
- Do NOT leave the predicate opaque and call it done. The goal is removing the
  opaque annotation. If you can't remove it, explain specifically why not and
  what you tried.
- The matching loop is the enemy. Understand it. Fix the triggers or restructure
  the predicate. Don't just throw rlimit at it.
- Commit working intermediate states.
- No subagents.
- PBOGH. This is 2 of 7 holes. Do the proof work.

## STEP 20

## Report

Write `plans/agent4-r110-key-unique-report.md`.
