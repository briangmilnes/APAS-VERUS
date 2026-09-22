# Chapters in order (r213)

Plan: `plans/r213-chapters-in-order.md`. Start: `main` at `9f9485a2f`.
Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0, Rust 1.98.1.

Each chapter is run with `scripts/validate.sh isolate ChapNN` (log under
`logs/validate.*.log`), its run-time tests with `scratch/r212-rtt.sh ChapNN`
(`cargo test --release --no-fail-fast` on the chapter's `[[test]]` targets,
because `cargo-nextest` is not installed; log `logs/rtt.*.log`), and its
registered proof-time tests with `scratch/r212-ptt.sh ChapNN` (log
`logs/ptt-ChapNN.*.log`). Both scripts are copied unchanged from the r212
fixture. "Warnings" counts Verus and rustc warnings in the validate log;
"trigger notes" counts `automatically chose triggers` notes.

The cargo build of the library reports four rustc warnings outside the
chapter under test (`Chap43/OrderedTableStEph.rs` `cfg(never)`,
`Chap18/ArraySeqMtEph.rs` unneeded `mut`, two negative impls in
`vstdplus/threads_plus.rs`); they are handled with the chapter that owns the
file, or listed at the end.

## Summary

| # | Chap | Verified | Err | Warn | RTT | PTT | Commit |
|---|------|---------:|----:|-----:|-----|-----|--------|
| 1 | 02 | 631 | 0 | 0 | 41 pass | none | 1afd25de1 |
| 2 | 03 | 622 | 0 | 0 | 40 pass | none | 9da5c4aba |
| 3 | 05 | 760 | 0 | 0 | 89 pass | 5 of 21 (1) | 7680a0ea8 |
| 4 | 06 | 1037 | 0 | 0 | 275 pass | not registered | 8c5300b01 |
| 5 | 11 | 651 | 0 | 0 | 40 pass | none | 58e3ecf69 |
| 6 | 12 | 635 | 0 | 0 | 40 pass | none | r213 Chap12 |

Notes: (1) the failing proof-time tests are on the pre-09.13 iterator model
and do not compile; see the chapter section.

## Chapter sections

### Chap02

- Start and end: 631 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050445.log`). No edit.
- RTT: 3 targets, 41 tests pass (`logs/rtt.20260922-050502.log`).
- PTT: none registered.

### Chap03

- Start and end: 622 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050550.log`). No edit.
- RTT: 1 target, 40 tests pass (`logs/rtt.20260922-050553.log`).
- PTT: none registered.

### Chap05

- Start and end: 760 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050624.log`). No edit; no `finite()` left in
  source.
- RTT: 5 targets, 89 tests pass (`logs/rtt.20260922-050632.log`).
- PTT (`logs/ptt-Chap05.20260922-050636.log`): 7 registered files, 5 of 21
  tests pass (`ProveKleeneStPer` 3, one each in `SetStEph.rs`,
  `SetMtEph.rs`); 16 fail to compile because they are still written on the
  pre-09.13 iterator model (`SetStEphIter`, `SetMtEphIter`,
  `RelationStEphIter`, `MappingStEphIter`, `iter_invariant`, `it@.0`). The
  iterator tool reports them as `ptt-no-definer` (the definers were migrated
  by hand in r208), so they need a hand rewrite to the templates of
  `src/standards/iterator_ptt_standard.rs`. Not done in this pass.

### Chap06

- Start and end: 1037 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050731.log`). No edit; no `finite()` left in
  source.
- RTT: 20 targets, 275 tests pass (`logs/rtt.20260922-050744.log`).
- PTT: the 20 files in `rust_verify_test/tests/Chap06/` are not registered
  in `rust_verify_test/Cargo.toml`, so the harness cannot run them; they are
  on the pre-09.13 iterator model. Not run.

### Chap11

- First run on 09.13: 651 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050801.log`). No edit.
- RTT: 2 targets, 40 tests pass (`logs/rtt.20260922-050809.log`).
- PTT: none registered.

### Chap12

- First run on 09.13: 635 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050809.log`). No edit.
- RTT: 3 targets, 40 tests pass (`logs/rtt.20260922-050812.log`).
- PTT: none registered.

### Chap17

- Start and end: 645 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050830.log`). No edit.
- RTT: 1 target, 40 tests pass (`logs/rtt.20260922-050837.log`).
- PTT (`logs/ptt-Chap17.20260922-050838.log`): 2 of 9 tests pass; the 7
  failures are the old iterator model (`MathSeqIter`, `iter_invariant`,
  `it@` on `vec::IntoIter`, `.pos`/`.elements`/`.cur` on the for-loop
  wrapper). Hand rewrite needed; not done in this pass.
