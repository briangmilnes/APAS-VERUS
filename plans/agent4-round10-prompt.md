# Agent 4 — Round 10 Prompt

## Mission

Reduce Chap41 (64 holes — largest actionable chapter). Close Chap53 (1 hole).

## Your Files (no other agent touches these)

**Chap41** (64 real holes across 7 files):
- `ArraySetStEph.rs` — 3 holes (FIX FIRST — unblocks Chap42)
- `ArraySetEnumMtEph.rs` — 1 hole
- `AVLTreeSetStEph.rs` — 16 holes
- `AVLTreeSetStPer.rs` — 10 holes
- `AVLTreeSetMtEph.rs` — 19 holes
- `AVLTreeSetMtPer.rs` — 12 holes
- `Example41_3.rs` — 3 holes

**Chap53** (1 hole):
- `GraphSearchMtPer.rs` — 1 external_body

## Priority Order

1. **ArraySetStEph.rs** — 3 holes. Unblocks Chap42 (18 holes, Agent 2).
2. **Chap53 GraphSearchMtPer.rs** — 1 hole. Close the chapter.
3. **AVLTreeSetStEph.rs** — 16 holes. Largest single file.
4. **AVLTreeSetStPer.rs** — 10 holes.
5. **AVLTreeSetMtEph.rs** — 19 holes. You have context from Round 9.
6. **AVLTreeSetMtPer.rs** — 12 holes. You barely touched this in Round 9.
7. **Example41_3.rs** — 3 holes.
8. **ArraySetEnumMtEph.rs** — 1 hole.

## Specific Guidance

### ArraySetStEph.rs (3 holes) — HIGHEST PRIORITY

These 3 holes block all of Chap42 (18 holes). Fix them first. Check what the 3 holes are:
- Run `scripts/holes.sh src/Chap41/ArraySetStEph.rs`
- Read the file, understand each hole
- The ArraySet is a simple sorted-array-backed set. These should be provable.

### GraphSearchMtPer.rs (1 hole)

You said in Round 9: "Blocked by MtPer's lack of wf spec — AVLTreeSeqMtPerS::length()/nth()
require spec_avltreeseqmtper_wf() which can't be proved from outside the type."

If you can strengthen AVLTreeSetMtPer's specs this round to include wf, this becomes
provable. Try it after working on AVLTreeSetMtPer.

### AVLTreeSetStEph.rs (16 holes)

Agent 3 proved 1 hole in Round 9 (to_seq wf via clone_link strengthening). The remaining
16 include:
- feq assumes (closure requires that need to be lifted to function requires)
- set operation assumes (union, intersection, difference — need sorted invariant or
  direct set algebra proofs)
- clone/view assumes

**Strategy**:
- Read Agent 3's Round 9 report for what they proved and what technique they used.
- For feq: add `requires obeys_feq_full::<T>()` to trait functions. UPDATE ALL CALLERS
  in Chap43, 52, 53, 55 that use AVLTreeSetStEph.
- For sorted invariant: if spec_wf doesn't include sortedness, consider adding it.
  This unlocks size() (seq.len == set.len) and find() not-found case.

### AVLTreeSetStPer.rs (10 holes)

Agent 3 proved 2 in Round 9 (insert already-present, clone view equality). Remaining 10
are similar to StEph — feq, set ops, clone bridges.

### AVLTreeSetMtEph.rs (19 holes)

You proved 5 in Round 9 (wf-implies-inv pattern). Remaining:
- 7 external_body: spec_set_view, to_seq, filter, intersection, difference, union, iter.
  All blocked by Arc<RwLock> making inner state opaque.
- 11 assume: view-related, blocked by external_body spec_set_view.
- 1 trivial_wf: needs real wf predicate.

**Strategy**: Fix the trivial_wf first (give it a real invariant connecting the RwLock
contents to the set view). Then try to strengthen spec_set_view — if the RwLock predicate
includes enough info, spec_set_view can read from it.

### AVLTreeSetMtPer.rs (12 holes)

You got -1 in Round 9. This needs more work. Same Arc<RwLock> pattern as MtEph.
The Persistent variant uses Arc (no RwLock) for structural sharing. Check if the
patterns are different from MtEph.

### Example41_3.rs (3 holes)

Textbook example. Check what's in it — if it's just demo/exercise code, the holes
may be acceptable external_body on non-algorithmic logic.

## Techniques from Round 9

- **wf-implies-inv**: spec_wf() includes the invariant check. Replace assume(inv) with
  assert(inv).
- **Direct .elements access**: Bypass to_seq() when it doesn't ensure wf.
- **Recursive-to-loop conversion**: While loops with exec_allows_no_decreases_clause.
- **Closure requires propagation**: Lift into function requires per using_closures_standard.rs.

## CRITICAL RULES

- **NO accept()**. Do NOT use accept() anywhere. Do NOT convert assumes to accepts.
  If you can't prove something, leave the assume and explain what blocks it.
- Read `src/standards/using_closures_standard.rs` for closure patterns.
- Read `src/standards/partial_eq_eq_clone_standard.rs` for clone/eq patterns.
- When adding requires to trait functions, UPDATE ALL CALLERS across chapters.
- Run `scripts/validate.sh` after every change.
- Push to `agent4/ready`.
- Write `plans/agent4-round10-report.md`.

## Targets

- ArraySetStEph.rs: 0 holes (close it — unblocks Chap42)
- Chap41 total: ≤ 50 holes
- Chap53: closed (0 holes)
