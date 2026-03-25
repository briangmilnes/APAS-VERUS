# R76 Agent 5 — BSTSetAVLMtEph: fold obeys_feq_clone into wf (8 holes)

## Objective

Eliminate 8 `assume(obeys_feq_clone::<T>())` holes in `src/Chap37/BSTSetAVLMtEph.rs` by
folding the predicate into `spec_bstsetavlmteph_wf()`.

## Baseline

- 4794 verified, 0 errors, 0 warnings
- BSTSetAVLMtEph.rs: 13 holes (8 assume + 5 external_body)
- This task targets the 8 assumes only. The 5 external_body are deferred.

## The fix

**One structural change.** In the impl at line 200, change:

```rust
open spec fn spec_bstsetavlmteph_wf(&self) -> bool {
    self.tree.spec_bstavlmteph_wf()
}
```

to:

```rust
open spec fn spec_bstsetavlmteph_wf(&self) -> bool {
    self.tree.spec_bstavlmteph_wf() && obeys_feq_clone::<T>()
}
```

Every function that already `requires self.spec_bstsetavlmteph_wf()` (or
`old(self).spec_bstsetavlmteph_wf()`) now gets `obeys_feq_clone::<T>()` for free.

Then **remove all 8 assumes**:

| # | Line | Function |
|---|------|----------|
| 1 | 148 | values_vec |
| 2 | 234 | delete |
| 3 | 341 | split |
| 4 | 368 | join_pair |
| 5 | 400 | join_m |
| 6 | 434 | filter |
| 7 | 456 | reduce |
| 8 | 474 | iter_in_order |

## Cascading changes

- `values_vec` (line 144) takes `&BSTAVLMtEph<T>` not `&Self`. It currently has
  `requires tree.spec_bstavlmteph_wf()`. It also needs `obeys_feq_clone::<T>()` in its
  requires since it calls `tree.in_order()`. Add it: `requires tree.spec_bstavlmteph_wf(), obeys_feq_clone::<T>()`.
  All callers of `values_vec` pass `&self.tree` where `self.spec_bstsetavlmteph_wf()` is
  already required — so `obeys_feq_clone::<T>()` is available at every call site.

- `empty()` and `singleton()` ensure `spec_bstsetavlmteph_wf()`. With the new definition,
  they must also prove `obeys_feq_clone::<T>()`. This is not provable — it's the same
  structural limitation. Add `assume(obeys_feq_clone::<T>())` in `empty()` and
  `singleton()` bodies. This consolidates 8 assumes into 2 (the construction points).
  Net: -6 holes.

- If `obeys_feq_clone` cannot be proven in `empty`/`singleton`, an alternative: keep wf
  as-is but add `obeys_feq_clone::<T>()` to the trait's requires for every function that
  calls `in_order()`. This is less clean but eliminates assumes entirely — the obligation
  moves to callers. Use your judgment on which is cleaner.

## Important

- Read the file fully before editing.
- Do NOT touch the 5 external_body functions (union, intersection, difference, filter, reduce).
- Do NOT add `accept()` — use `assume()` if needed at construction points.
- Run `scripts/validate.sh` after each change. Fix any cascading errors.
- Run `scripts/rtt.sh` and `scripts/ptt.sh` before committing.

## Validation

Push to `agent5/ready`.

## Report

Write `plans/agent5-round76-report.md` with holes before/after (table with Chap column).
