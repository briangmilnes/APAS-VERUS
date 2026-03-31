# Agent 3 R119 Report: Chap51 DP Spec Strengthening

## Summary

Added `spec_*_wf` predicates and wf requires/ensures propagation to all 8 Chap51
DP modules. Added missing `initialize_base_cases` and `compute_cell_value` functions
to both BottomUpDP Mt variants. Documented intentional API differences for TopDownDP
Mt missing functions and TopDownDPStEph `with_memo_table`.

## Verification

- Isolate Chap51: **1287 verified, 0 errors**
- RTT: **3529 passed, 0 skipped**
- Holes: **0 actionable**

## Changes Per File

| # | Chap | File | Warning Before | Warning After | Changes |
|---|------|------|---------------|---------------|---------|
| 1 | 51 | BottomUpDPStPer.rs | no spec_*_wf | fixed | +wf spec, +wf requires/ensures on 7 fns |
| 2 | 51 | BottomUpDPStEph.rs | no spec_*_wf | fixed | +wf spec, +wf requires/ensures on 9 fns |
| 3 | 51 | BottomUpDPMtPer.rs | no spec_*_wf, missing 3 fns | fixed | +wf spec, +wf req/ens on 7 fns, +2 fns |
| 4 | 51 | BottomUpDPMtEph.rs | no spec_*_wf, missing 3 fns | fixed | +wf spec, +wf req/ens on 9 fns, +2 fns |
| 5 | 51 | TopDownDPStPer.rs | no spec_*_wf | fixed | +wf spec (memo_correct), +wf req/ens on 10 fns |
| 6 | 51 | TopDownDPStEph.rs | no spec_*_wf | fixed | +wf spec (memo_correct), +wf req/ens on 13 fns |
| 7 | 51 | TopDownDPMtPer.rs | no spec_*_wf | fixed | +wf spec, +wf req/ens on 7 fns |
| 8 | 51 | TopDownDPMtEph.rs | no spec_*_wf | fixed | +wf spec, +wf req/ens on 9 fns |

## WF Predicate Bodies

| # | Chap | File | WF Body | Rationale |
|---|------|------|---------|-----------|
| 1 | 51 | BottomUpDPStPer.rs | `true` | Two ArraySeq fields, no structural invariant |
| 2 | 51 | BottomUpDPStEph.rs | `true` | Same |
| 3 | 51 | BottomUpDPMtPer.rs | `true` | Same |
| 4 | 51 | BottomUpDPMtEph.rs | `true` | Same |
| 5 | 51 | TopDownDPStPer.rs | `spec_memo_correct(spec_memo())` | Memo entries match spec_med |
| 6 | 51 | TopDownDPStEph.rs | `spec_memo_correct()` | Same (parameterless variant) |
| 7 | 51 | TopDownDPMtPer.rs | `true` | No memo in struct (created locally) |
| 8 | 51 | TopDownDPMtEph.rs | `true` | Same |

## Missing Function Analysis

### BottomUpDP Mt — Added 2 functions each

`initialize_base_cases` and `compute_cell_value` were inlined in `med_bottom_up_parallel`.
Added as standalone trait methods with identical specs and bodies to the St counterparts,
adapted for Mt ArraySeq types. `med_bottom_up` is intentionally renamed to
`med_bottom_up_parallel` in Mt — not a gap.

### TopDownDP Mt — Intentional architecture difference

The 9 missing memo functions (`spec_memo`, `spec_memo_correct`, `memo_size`,
`is_memoized`, `get_memoized`, `with_memo_table`/`insert_memo`, `clear_memo`,
`med_memoized`, `med_recursive`) are absent because Mt structs don't store memo
state. The memo is created locally in `med_memoized_concurrent`/`med_memoized_parallel`
and accessed through RwLock. Exposing memo accessors on the struct would be
architecturally wrong — the memo is thread-local state, not persistent struct state.

### TopDownDPStEph — `with_memo_table` intentional absence

StPer uses `fn with_memo_table(self, memo) -> Self` (ownership transfer, persistent
pattern). StEph uses `fn insert_memo(&mut self)` and `fn clear_memo(&mut self)`
(in-place mutation, ephemeral pattern). This is the standard St Per/Eph API split.

## Techniques

- Standard wf propagation per `spec_wf_standard.rs`
- Non-trivial wf for TopDownDP St variants (memo correctness as structural invariant)
- Trivially-true wf for BottomUpDP and TopDownDP Mt variants (Vec-wrapper pattern)
- Mt standalone: no imports from St counterparts, duplicated function bodies
