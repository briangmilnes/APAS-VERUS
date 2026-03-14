# Agent 3 — Round 12 Prompt

## Mission

Continue Chap38 (32 holes) where you left off in Round 10. Also try Chap37 easy
targets (3 holes) and apply your feq broadcast trick more broadly.

## Your Files

**Chap38** (32 holes across 2 files):
- `BSTParaStEph.rs` — 15 assume (you got -4 in R10 with enriched predicate)
- `BSTParaMtEph.rs` — 17 external_body

**Chap37** (3 easy holes):
- `AVLTreeSeq.rs` — 1 external_body (iterator next)
- `BSTSplayStEph.rs` — 1 trivial_wf (wf returns true, needs real invariant)
- `AVLTreeSeqStPer.rs` — 1 assume (slice indexing lemma)

## Priority Order

1. **Chap37 easy targets** (3 holes) — Quick wins before the hard stuff.
2. **BSTParaStEph.rs** (15 holes) — You know this file. Continue from R10.
3. **BSTParaMtEph.rs** (17 holes) — If you make progress on StEph, try Mt.

## Specific Guidance

### Your feq Broadcast Trick

In Round 11 you created `Pair_feq_trigger + group_Pair_axioms` to eliminate feq
assumes in TableMtEph. Can you apply this same technique to BSTParaStEph's
remaining assumes? The 11 union/intersect/difference assumes are about T-vs-T::V
bridges — a broadcast proof might help here too.

### BSTParaStEph.rs (15 holes) — Your R10 Blockers

You identified these blockers:
1. `expose: k@ == node.key@` — T::clone has no verified ensures
2. `insert/delete overflow` — needs `self@.len() < usize::MAX` precondition
3. `union/intersect/difference (11)` — T-vs-T::V bridge, can't get exec witnesses
4. `clone external_body` — RwLock clone boundary

For #1: Can you add a `requires` for clone correctness? Or use the eq/clone
standard pattern from `src/standards/partial_eq_eq_clone_standard.rs`?

For #2: Adding `requires self@.len() < usize::MAX` is an API change but legitimate.
Add it to the trait function signature and update callers within your files.

For #3: This is the hard one. Your broadcast trick from R11 might be the key.

### Chap37 Easy Targets

- **AVLTreeSeq.rs iterator next**: Standard pattern from Chap18/ArraySeqStEph.rs.
  Read that file's iterator section, copy the pattern.
- **BSTSplayStEph.rs trivial_wf**: `spec_bstsplaysteph_wf` returns `true`. Add the
  real BST invariant (sorted keys, valid tree structure).
- **AVLTreeSeqStPer.rs assume**: Slice indexing proof, needs seq_to_slice reasoning.

### BSTParaMtEph.rs (17 external_body) — Stretch

You said in R10: "fake view, no ghost state, code outside verus!, needs complete
rewrite." If that's still true, skip it. But if you can add ghost fields and a
real view (like you did for BSTParaStEph's predicate in R10), try one function
as a proof of concept.

## DO NOT TOUCH

- Chap41, Chap53 — Agent 4
- Chap42, Chap47 — Agent 2
- Chap43, Chap45 — Agent 1
- Chap39 — Leave (structural blockers established)

## Rules

- Read standards before writing code.
- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent3/ready`. Write `plans/agent3-round12-report.md`.

## Targets

- Chap37: 3 → ≤ 1
- BSTParaStEph.rs: 15 → ≤ 10
- Total: 18 → ≤ 11 (-7)
