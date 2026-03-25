# R76b Agent 2 — BSTSetSplayMtEph: fold obeys_feq_clone into wf + BSTSplayMtEph fn_missing_requires

## Objective

Two tasks on Chap37 Splay tree files:

1. Eliminate 8 `assume(obeys_feq_clone::<T>())` holes in `src/Chap37/BSTSetSplayMtEph.rs`
   by folding the predicate into `spec_bstsetsplaymteph_wf()` — same pattern Agent 5 is
   doing for BSTSetAVLMtEph.

2. Fix 2 `fn_missing_requires` warnings in `src/Chap37/BSTSplayMtEph.rs`.

## Baseline

- 4830 verified, 0 errors, 0 warnings
- BSTSetSplayMtEph.rs: 13 holes (8 assume + 5 external_body)
- BSTSplayMtEph.rs: 5 holes + 2 fn_missing_requires warnings

## Task 1: BSTSetSplayMtEph feq_clone into wf

**Read `src/Chap37/BSTSetSplayMtEph.rs` fully first.**

Change the wf spec from:

```rust
open spec fn spec_bstsetsplaymteph_wf(&self) -> bool {
    self.tree.spec_bstsplaymteph_wf()
}
```

to:

```rust
open spec fn spec_bstsetsplaymteph_wf(&self) -> bool {
    self.tree.spec_bstsplaymteph_wf() && obeys_feq_clone::<T>()
}
```

Then remove all 8 `assume(obeys_feq_clone::<T>())` from function bodies.

**Cascading changes:**
- `values_vec` takes `&BSTSplayMtEph<T>`, not `&Self`. Add `obeys_feq_clone::<T>()` to
  its requires. All callers pass `&self.tree` where wf is in scope.
- `empty()` and `singleton()` ensure wf. They can't prove `obeys_feq_clone`. Add one
  `assume(obeys_feq_clone::<T>())` in each. Net: -6 holes (8 removed, 2 added).
- Import `obeys_feq_clone` if not already imported:
  `#[cfg(verus_keep_ghost)] use crate::vstdplus::feq::feq::obeys_feq_clone;`

## Task 2: BSTSplayMtEph fn_missing_requires

Two internal helpers need real requires:

| Line | Function | Likely requires |
|------|----------|----------------|
| 123 | size_link | None needed — `// veracity: no_requires` (takes `&Link<T>`, works on None) |
| 133 | update | `spec_is_bst_link(Some(*node))` or similar wf predicate |

**Read each function body** to determine the real precondition. If the function genuinely
has no precondition, add `// veracity: no_requires`. If it assumes the node is well-formed,
add the wf requires.

**IMPORTANT**: Do NOT add `requires true`. Do NOT add tautological requires.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round76b-report.md` with holes before/after (table with Chap column).
