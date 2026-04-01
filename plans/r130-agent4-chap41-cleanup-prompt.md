# R130 Agent 4 — Cleanup Chap41 AVLTreeSet Mt from R128. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- Standard 4 (`spec_wf_standard.rs`) — wf predicates
- Standard 7 (`partial_eq_eq_clone_standard.rs`) — PartialEq/Clone patterns
- Standard 22 (`capacity_bounds_standard.rs`) — size bounds propagation
- Standard 23 (`mt_type_bounds_standard.rs`) — use trait aliases

Report file: `plans/r130-agent4-chap41-cleanup-report.md`

## Problem

Your R128 restructuring of Chap41 (AVLTreeSetMtEph/MtPer) has 17 assumes remaining.
Many are provable. You also deleted APAS cost annotations. This round fixes both.

## Fix 1: Add size bound to wf (eliminates ~5 assumes)

Both `spec_avltreesetmteph_wf` and `spec_avltreesetmtper_wf` are currently:
```rust
self.tree@.finite()
```

Change to:
```rust
self.tree@.finite() && self.tree@.len() <= usize::MAX
```

This propagates through requires/ensures and eliminates every
`assume(self.tree@.len() < usize::MAX)` and similar size-bound assumes in insert,
delete, union, to_seq, from_seq.

## Fix 2: Restore deleted APAS annotations

You deleted `CS 41.3`, `CS 41.4`, and `claude-4-sonet` annotation lines from trait
declarations. The APAS lines (`/// - APAS Cost Spec 41.3: ...` or
`/// - Alg Analysis: APAS (Ch41 CS 41.3): ...`) are textbook references that must
NEVER be deleted. Restore them.

Check `git diff HEAD~2 -- src/Chap41/AVLTreeSetMtEph.rs src/Chap41/AVLTreeSetMtPer.rs`
to see exactly what was deleted. Restore all APAS lines. You may update Code review
lines but never delete APAS lines.

## Fix 3: PartialEq finiteness (eliminates 2 assumes in MtPer)

`assume(self.tree@.finite())` and `assume(other.tree@.finite())` in PartialEq::eq
are provable from wf. After Fix 1, `spec_wf` includes finiteness. Add
`requires self.spec_avltreesetmtper_wf(), other.spec_avltreesetmtper_wf()` to eq
if not already present, or prove finiteness from the type invariant.

## Fix 4: PartialEq early return (eliminates 1 assume in MtPer)

`assume(false == (self@ == other@))` when sizes differ is provable:
for finite sets, `self@.len() != other@.len() ==> self@ != other@`.
vstd has `lemma_len_eq` or similar. Prove it instead of assuming.

## Fix 5: clone-preserves-view (improve 1 assume)

`assume(elem@ == seq@[i])` in from_seq should use
`axiom_cloned_implies_eq_owned` from `crate::vstdplus::feq::feq` instead of a bare
assume. Read standard 7 for the pattern.

## Fix 6: to_seq assumes (try to prove)

`assume(seq@.to_set() =~= self@)` and `assume(forall contains)` in to_seq.
These should follow from `collect_in_order`'s ensures. Read the ensures of
`BSTParaMtEph::collect_in_order` (or whatever in-order traversal method you used).
If the ensures are too weak, strengthen them. If you can't prove these, leave the
assumes and report what blocks them.

## Validation

Run `scripts/validate.sh isolate Chap41`. Then `scripts/rtt.sh`.
Run `scripts/holes.sh src/Chap41/` and report before/after hole count.

## Rules

- Do NOT add new assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Do NOT delete APAS cost annotations — ever.
