# R76b Agent 1 — BSTSetRBMtEph: fold obeys_feq_clone into wf + BSTRBMtEph fn_missing_requires

## Objective

Two tasks on Chap37 RB tree files:

1. Eliminate 8 `assume(obeys_feq_clone::<T>())` holes in `src/Chap37/BSTSetRBMtEph.rs` by
   folding the predicate into `spec_bstsetrbmteph_wf()` — same pattern Agent 5 is doing
   for BSTSetAVLMtEph.

2. Fix 3 `fn_missing_requires` warnings in `src/Chap37/BSTRBMtEph.rs`.

## Baseline

- 4830 verified, 0 errors, 0 warnings
- BSTSetRBMtEph.rs: 13 holes (8 assume + 5 external_body)
- BSTRBMtEph.rs: 3 holes + 3 fn_missing_requires warnings

## Task 1: BSTSetRBMtEph feq_clone into wf

**Read `src/Chap37/BSTSetRBMtEph.rs` fully first.**

Change the wf spec from:

```rust
open spec fn spec_bstsetrbmteph_wf(&self) -> bool {
    self.tree.spec_bstrbmteph_wf()
}
```

to:

```rust
open spec fn spec_bstsetrbmteph_wf(&self) -> bool {
    self.tree.spec_bstrbmteph_wf() && obeys_feq_clone::<T>()
}
```

Then remove all 8 `assume(obeys_feq_clone::<T>())` from function bodies. Functions that
already `requires self.spec_bstsetrbmteph_wf()` get it for free.

**Cascading changes:**
- `values_vec` takes `&BSTRBMtEph<T>`, not `&Self`. Add `obeys_feq_clone::<T>()` to its
  requires. All callers pass `&self.tree` where wf is in scope.
- `empty()` and `singleton()` ensure wf. They can't prove `obeys_feq_clone`. Add one
  `assume(obeys_feq_clone::<T>())` in each. Net: -6 holes (8 removed, 2 added).
- Import `obeys_feq_clone` if not already imported:
  `#[cfg(verus_keep_ghost)] use crate::vstdplus::feq::feq::obeys_feq_clone;`

## Task 2: BSTRBMtEph fn_missing_requires

Three internal helpers need real requires:

| Line | Function | Likely requires |
|------|----------|----------------|
| 149 | is_red | None needed — `// veracity: no_requires` (takes `&Link<T>`, works on None) |
| 159 | size_link | None needed — `// veracity: no_requires` (takes `&Link<T>`, works on None) |
| 169 | update | `spec_is_bst_link(Some(*node))` or similar wf predicate |

**Read each function body** to determine the real precondition. If the function genuinely
has no precondition (works on any input including None), add `// veracity: no_requires`.
If it assumes the node is well-formed, add the wf requires.

**IMPORTANT**: Do NOT add `requires true`. Do NOT add tautological requires. Read the
function, understand what it needs, express that as a real contract or annotate no_requires.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round76b-report.md` with holes before/after (table with Chap column).
