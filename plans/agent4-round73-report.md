# R73 Agent 4 Report: Uncomment Chap64/65/66 Graph Algorithm Files

## Scope

Fix 8 broken files across 3 graph algorithm chapters (Chap64, Chap65, Chap66) that were
hidden behind `all_chapters` cfg and never cargo-compiled. Uncomment in lib.rs, fix all
compilation and verification errors.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 65 | UnionFindStEph.rs | Added `obeys_key_model` import, removed `feq` + `SetStEph` imports, removed `valid_key_type_Edge`, fixed `Copy` bound on `find` |
| 2 | 65 | KruskalStEph.rs | Added `obeys_key_model`/`iter_invariant`/`obeys_feq_full` imports (cfg-gated), removed broadcast use in external_body |
| 3 | 65 | PrimStEph.rs | Added `obeys_key_model` import, fixed `Copy` bound, added `WrappedF64` `PartialOrd` usage |
| 4 | 64 | SpanTreeStEph.rs | Added `obeys_key_model` import, `iter_invariant` usage, `valid_key_type_Edge` requires |
| 5 | 64 | TSPApproxStEph.rs | Already clean — no changes needed |
| 6 | 66 | BoruvkaStEph.rs | Added `Copy` bounds, `#[verifier::external]` on PartialEq, `#[verifier::external_body]` on 5 impl fns, wf requires/ensures on trait, explicit triggers, cfg-gated imports |
| 7 | 66 | BoruvkaMtEph.rs | Added `Copy` bounds, `#[verifier::external]` on PartialEq, `#[verifier::external_body]` on 11 impl fns, wf requires/ensures, `arc_deref` for Arc indexing, explicit triggers, cfg-gated imports |
| 8 | — | lib.rs | Uncommented Chap64, Chap65, Chap66 module blocks |

## Key Techniques

- **`arc_deref` for Arc indexing**: Verus only supports `[]` on Vec/array/slice. Used `arc_deref(&arc_vec)` to get `&Vec<V>` before indexing.
- **`#[verifier::external]` on PartialEq**: LabeledEdge's PartialEq calls `WrappedF64::eq` which is outside verus! — marking the whole impl `#[verifier::external]` avoids the error.
- **Relaxed spec function bounds**: Spec functions use `V: Copy` instead of `V: View<V=V> + Copy` since `Set<T>` is a spec type with no View requirement.
- **cfg-gated ghost imports**: `iter_invariant`, `obeys_feq_full`, `obeys_key_model` are Verus-only; gated with `#[cfg(verus_keep_ghost)]`.

## Hole Summary

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 64 | SpanTreeStEph.rs | 2 |
| 2 | 64 | TSPApproxStEph.rs | 4 |
| 3 | 65 | UnionFindStEph.rs | 4 (+ 3 accepted) |
| 4 | 65 | KruskalStEph.rs | 3 |
| 5 | 65 | PrimStEph.rs | 4 |
| 6 | 66 | BoruvkaStEph.rs | 6 |
| 7 | 66 | BoruvkaMtEph.rs | 12 |
| — | — | **Total** | **35 holes** |

These holes are all `external_body` annotations on algorithmic functions — the code
compiles and runs correctly but the function bodies are not yet verified by Verus.

## SpanTreeMtEph — Known Blocker

`src/Chap64/SpanTreeMtEph.rs` depends on `StarContractionMtEph` (Chap62) which is
commented out in lib.rs. Not attempted this round.

## Verification Results

- **Validate**: 4462 verified, 0 errors, 1 warning (pre-existing PrimStEph derive Clone)
- **RTT**: 2528 tests passed
- **PTT**: 157 tests passed
