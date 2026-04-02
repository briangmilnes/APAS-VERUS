# R142 Agent 2 — Fix 3 holes in Chap18/ArraySeqMtEphSlice.rs. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap18/ArraySeqMtEphSlice.rs` — the file with all 3 holes.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the Chap19 version where these are
already fixed. Use it as your reference for all 3 fixes.

Report file: `plans/r142-agent2-chap18-slice-fixes-report.md`

## Problem

`src/Chap18/ArraySeqMtEphSlice.rs` was copied from Chap19 before 3 fixes landed.
Port the fixes from Chap19.

## Hole 1: bare_impl (line ~757)

`impl<T: Eq + Clone> ArraySeqMtEphSliceS<T>` bare impl block.

**Fix:** Same as Chap19 — convert all exec functions in the bare impl to
`pub(crate)` free functions. Move type parameters to each function's generic
list. Change all `Self::fn_name(...)` call sites to `fn_name(...)`. Proof
functions can stay in the bare impl or be moved too (agent3 moved them all
in Chap19).

Read `src/Chap19/ArraySeqMtEphSlice.rs` to see exactly how agent3 did it.

## Hole 2: fn_missing_ensures on flatten_dc_vec (line ~1408)

**Fix:** Same as Chap19 — add `spec_sum_inner_lens` spec function and
`lemma_sum_inner_lens_split` proof function. Add ensures to `flatten_dc_vec`
and strengthen `flatten`'s ensures.

Copy the spec, lemma, and ensures from `src/Chap19/ArraySeqMtEphSlice.rs`.
Adjust module paths if needed.

## Hole 3: assume in inject (line ~729)

```rust
assume(injected_seq =~= expected);
```

This assumes the sequential apply loop matches `spec_inject`. The Chap19 slice
version (agent1 R141) proves this. Read how agent1's inject works in Chap19 —
it applies updates end-to-front matching `spec_inject`'s recursion, with
`lemma_spec_inject_len` and `lemma_spec_inject_element` proving the correspondence.

Port the proof from Chap19. The spec functions `spec_inject`, `spec_ninject` and
lemmas `lemma_spec_inject_len`, `lemma_spec_inject_element` should already be in
the Chap18 file (agent2 copied them). If not, add them.

## Validation

Run `scripts/validate.sh isolate Chap18`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- All 3 fixes are mechanical ports from Chap19. Read the Chap19 version first.

## When done

RCP.
