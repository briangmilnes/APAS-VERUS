# Agent 3 — Round 11 Prompt

## Mission

Small-target sweep: close easy holes across Chap39, Chap42, and Chap53. You hit
structural blockers in Chap38/39 Mt files last round — pivot to the tractable ones.

## Your Files

**Chap39** (3 holes):
- `BSTSetTreapMtEph.rs` — 3 assume (wrapper assumes from inner specs)

**Chap42** (3 holes):
- `TableStEph.rs` — 1 assume (feq/clone bridge)
- `TableStPer.rs` — 1 assume + 1 external_body

**Chap53** (1 hole):
- `GraphSearchMtPer.rs` — 1 assume (AVLTreeSet wf predicate chain)

## Priority Order

1. **Chap53 GraphSearchMtPer.rs** (1 hole) — Close the chapter.
2. **Chap39 BSTSetTreapMtEph.rs** (3 holes) — Wrapper assumes.
3. **Chap42 TableStEph.rs + TableStPer.rs** (3 holes) — feq bridge assumes.

## Specific Guidance

### Chap53 GraphSearchMtPer.rs (1 hole)

The assume is on `frontier.elements.spec_avltreeseqmtper_wf()`. This is a wf predicate
that should be guaranteed by the constructor. Check:
1. Does `AVLTreeSeqMtPerS::new()` ensure wf in its postcondition?
2. Do mutations (insert, delete) preserve wf?
3. If yes, replace `assume(...)` with `assert(...)`.
4. If not, add `ensures spec_avltreeseqmtper_wf()` to the constructor and propagate.

### Chap39 BSTSetTreapMtEph.rs (3 holes)

You said: "All 3 blocked by upstream BSTParaTreapMtEph having `ensures true` on
insert/delete."

Try a different angle:
- Can the wrapper's spec be weakened to match what the inner type actually ensures?
- If the inner type ensures nothing useful, can you add ensures to the inner type's
  trait functions? You own the Chap39 files.
- The 3 assumes are about: singleton (len==1), insert (adds element), delete (removes).
  These are basic set properties — if the inner tree's view is correct, they should follow.

### Chap42 TableStEph.rs + TableStPer.rs (3 holes)

Check where the `assume(obeys_feq_clone::<Pair<K,V>>())` appears:
- If inside `eq()` or `clone()` body → acceptable, convert to accept with user approval
  (but for now leave as assume and note it in your report).
- If inside algorithmic code → add `requires obeys_feq_clone::<Pair<K,V>>()` to the
  function, then update callers. Check who calls these functions.

For TableStPer's external_body: read the function. If it's a simple delegation to an
inner type, write the real body.

## DO NOT TOUCH (other agents' files)

- Chap37, Chap45 — Agent 1
- Chap41, Chap43 — Agents 1 and 4
- Chap47 — Agent 2
- Chap38 — leave for now (structural blockers established in R10)

## Rules

- Read `src/standards/partial_eq_eq_clone_standard.rs` for feq/clone patterns.
- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent3/ready`. Write `plans/agent3-round11-report.md`.

## Targets

- Chap53: 0 holes (close it)
- BSTSetTreapMtEph.rs: ≤ 1
- TableStEph + TableStPer: ≤ 1
- Total: 7 → ≤ 2
