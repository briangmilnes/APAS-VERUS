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
| 1 | 02 | 631 | 0 | 0 | 41 pass | none | r213 Chap02 |

## Chapter sections

### Chap02

- Start and end: 631 verified, 0 errors, 0 warnings, 0 trigger notes
  (`logs/validate.20260922-050445.log`). No edit.
- RTT: 3 targets, 41 tests pass (`logs/rtt.20260922-050502.log`).
- PTT: none registered.
