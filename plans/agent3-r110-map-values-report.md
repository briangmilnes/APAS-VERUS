# Agent 3 R110 Report: AVLTreeSeqMtPer map_values assume

## Objective

Eliminate the `assume(out@.map_values(|t: T| t@) =~= self.spec_seq())` in
`values_in_order` in `src/Chap37/AVLTreeSeqMtPer.rs`.

## Result

**Already done.** The assume was already removed in a prior commit on this branch.

### Current state of the code

- `inorder_collect` (line 497) has real ensures:
  ```rust
  ensures out@.map_values(|t: T| t@) =~= old(out)@.map_values(|t: T| t@) + spec_inorder(*cur),
  ```
- `values_in_order` (line 695) has no assume — the trait ensures
  `values@.map_values(|t: T| t@) =~= self.spec_seq()` follows directly from
  `inorder_collect`'s postcondition with empty `out`.
- The proof body in `inorder_collect` includes intermediate ghost assertions for
  left traversal, push, and right traversal that guide Z3 through the recursive structure.

### Verification

| # | Chap | Metric | Value |
|---|------|--------|-------|
| 1 | 37 | Verified | 1796 |
| 2 | 37 | Errors | 0 |
| 3 | 37 | Proof holes | 0 |
| 4 | 37 | Clean modules | 19/19 |

### Holes before/after

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|-------------|
| 1 | 37 | AVLTreeSeqMtPer.rs | 0 | 0 |

No changes needed. The file was already clean.
