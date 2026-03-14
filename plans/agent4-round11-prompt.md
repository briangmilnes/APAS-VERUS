# Agent 4 — Round 11 Prompt

## Mission

Continue Chap41 (58 holes). Close Chap53 (1 hole). Focus on AVLTreeSetStPer, MtPer,
and ArraySetStEph — you made good progress on MtEph and StEph in Round 10.

## Your Files

**Chap41** (58 holes across 7 files):
- `ArraySetStEph.rs` — 3 holes (feq assumes — still highest priority for unblocking Chap42)
- `ArraySetEnumMtEph.rs` — 1 hole
- `AVLTreeSetStEph.rs` — 7 holes (you got -1 in R10, 6 feq/clone + 1 sorted invariant)
- `AVLTreeSetStPer.rs` — 6 holes (same structural blockers as StEph)
- `AVLTreeSetMtEph.rs` — 10 holes (you got -9 in R10 with ghost field)
- `AVLTreeSetMtPer.rs` — 12 holes (untouched in R10)
- `Example41_3.rs` — SKIP

**Chap53** (1 hole):
- `GraphSearchMtPer.rs` — 1 assume (seq wf for nth/length)

## Priority Order

1. **ArraySetStEph.rs** (3 holes) — Still the key to unblocking Chap42 (16 holes).
2. **AVLTreeSetMtPer.rs** (12 holes) — You didn't attempt this in R10. Try the ghost
   field pattern that worked for MtEph.
3. **AVLTreeSetStPer.rs** (6 holes) — Same blockers as StEph, try in parallel.
4. **AVLTreeSetStEph.rs** (7 remaining) — Continue feq cascade work.
5. **AVLTreeSetMtEph.rs** (10 remaining) — External_body functions (to_seq, set ops,
   iter) + 2 unsafe impls + 2 view bridge assumes.
6. **Chap53 GraphSearchMtPer.rs** (1 hole) — seq wf assume.

## Specific Guidance

### ArraySetStEph.rs (3 holes)

You said: "assume(obeys_feq_full::<T>()) — type axioms, cascade through entire
table/graph hierarchy if changed to requires."

The cascade IS the work. Add `requires obeys_feq_full::<T>()` to the trait functions
that need it, then update callers in:
- Chap41 files (YOUR files — do it)
- Chap42 files (Agent 2's — DON'T touch, just note the cascade)
- Chap43 files (Agent 1's — DON'T touch, just note the cascade)

If the cascade is too broad to handle safely, try a different approach: can you prove
`obeys_feq_full::<T>()` from the existing trait bounds? If `T: StT` implies the feq
properties, you might not need it as a requires at all.

### AVLTreeSetMtPer.rs (12 holes) — NEW TARGET

You said: "No RwLock (plain struct wrapping seq), so ghost field approach doesn't apply."

If MtPer uses plain struct (not Arc<RwLock>), it should be EASIER than MtEph, not harder.
Read the file carefully:
- If it's a plain struct wrapping AVLTreeSeqMtPer, the view is just the inner view.
- Parallel operations that use fork-join: apply the HFScheduler standard pattern.
- The persistent variant creates new values — `ensures result@ == ...` should be
  provable from the inner seq's ensures.

### Ghost Field Pattern (from your R10 success)

You successfully added `ghost_set_view: Ghost<Set<V>>` to MtEph. Document this pattern:
1. Add ghost field to struct tracking the abstract view
2. wf spec uses the ghost field (not trivial `true`)
3. spec_set_view returns the ghost field value
4. Constructors (new, empty, singleton) initialize ghost field from StEph's view
5. Mutators (insert, delete) update ghost field
6. The 2 unsafe Send/Sync impls are the cost

Can this pattern apply to MtPer? Even if MtPer isn't Arc<RwLock>, a ghost field for
tracking the set view might still help prove operations.

### Chap53 GraphSearchMtPer (1 hole)

You replaced external_body with verified iterative code but 1 assume remains for seq wf.
If `frontier.elements` comes from a constructor that ensures wf, chain that through.
Check if `AVLTreeSeqMtPerS::new()` ensures wf.

## DO NOT TOUCH (other agents' files)

- Chap38, Chap39 — Agent 3
- Chap42, Chap47 — Agent 2
- Chap43, Chap37, Chap45 — Agent 1

## Rules

- Read `src/standards/using_closures_standard.rs` for closure requires patterns.
- Read `src/standards/partial_eq_eq_clone_standard.rs` for clone/eq patterns.
- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent4/ready`. Write `plans/agent4-round11-report.md`.

## Targets

- ArraySetStEph.rs: 0 holes
- AVLTreeSetMtPer.rs: ≤ 6
- Chap41 total: ≤ 45
- Chap53: 0 holes (close it)
