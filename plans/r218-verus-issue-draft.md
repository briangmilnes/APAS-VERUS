# Draft issues for the Verus developers (not filed)

Two separate findings. Evidence lives in the APAS-VERUS repository, branch
`r218/z3-query-diff`, directory `analyses/r218-z3-query-diff/`.

Environment: Verus 0.2026.09.13.671956e (commit
671956ec527d3b7164779f767bdbfe769bedce6c, release build, rust 1.98.1),
Z3 4.16.0, Linux x86_64. Verus flags as in our validate script:
`--crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors --num-threads 8`.

---

## Issue 1: a module's SMT prelude order depends on unrelated modules in the crate, and the order changes verification results

### Summary

Verifying the same module with the same source gives different results
depending on which other modules are compiled into the crate (we select them
with `--cfg` features). With `--log smt --log smt-transcript` we captured the
module's Z3 session in both configurations. After normalizing names that come
from crate-wide counters (`anonymous_closure%N`, `impl_closure&__FnOnceN` in
qids) and from emission order (`%%lambda%%N`, `%%global_location_label%%N`),
the two sessions contain exactly the same SMT commands and the same queries
in the same order. Only the order of the prelude blocks (`Function-Decl`,
`Function-Axioms`, `Broadcast`, `Trait-Impl-Axiom`, ...) differs. Replaying
the logs in standalone Z3 reproduces both outcomes, and reordering the blocks
of one file into the other file's order transfers the outcome.

### Case 1: `prefix_sums_dc_inner` (module `Chap26::ScanDCMtPer::ScanDCMtPer`)

| # | Measure | Small crate | Full crate |
|---|---------|-------------|------------|
| 1 | blocks / asserts / foralls | 215 / 542 / 381 | 215 / 542 / 381 |
| 2 | blocks only in one file | 0 | 0 |
| 3 | queries before the target | 6 | 6 |
| 4 | target result, rlimit used | unsat, 4,888,736 | unknown (canceled), 30,000,000 |

- Renaming the closure ids of one file to the other's does not change either
  result.
- Small-crate content in full-crate block order fails; full-crate content in
  small-crate block order passes.
- Bisecting between the two orders, the result flips at one block: emitting
  `Function-Specs`/`Function-Axioms vstd::seq_lib::impl&%0::drop_last` before
  `Function-Axioms vstd::seq::impl&%2::spec_add` and the broadcasts
  `seq_lib::impl&%0::add_empty_left`, `add_empty_right`,
  `push_distributes_over_add`.

### Case 2: `select_inner` (module `Chap35::OrderStatSelectMtEph::OrderStatSelectMtEph`)

Same pattern: 601 blocks each, none only in one file, Z3 returns unsat
(612,369) with the small crate's order and `unknown` / `incomplete
quantifiers` with the full crate's order; swapping block order swaps the
result. Bisecting between the orders flips the result five times.

### Where we think the order comes from

`verify_crate_inner` builds one `GlobalCtx` for the whole crate
(`rust_verify/src/verifier.rs:2042`). `GlobalCtx::new` computes
`func_call_sccs` with `sort_sccs()` (`vir/src/context.rs:609`,
`vir/src/scc.rs:176`), a DFS postorder over the whole-crate call graph.
Per-module buckets are pruned (`verifier.rs:1930`), but `ast_to_sst_krate`
(`vir/src/ast_to_sst_crate.rs:33`) and `OpGenerator::next`
(`rust_verify/src/commands.rs:104`) iterate `ctx.global.func_call_sccs`, so a
module's prelude is emitted in the restriction of a crate-wide topological
order. Items with no dependency between them (e.g. `drop_last` and
`spec_add`) are ordered by where the DFS over unrelated parts of the crate
reaches them first. We have not instrumented Verus to confirm that this is the
only source of the difference.

### Why it matters

The underlying proofs are fragile (under `smt.random_seed` 0..7 the Chap26
target passes 6/8 with one order and 2/8 with the other; the Chap35 target
6/16 and 5/16), so this is not a claim that the order is wrong. The problem is
reproducibility: a module's verification result changes when unrelated code
is added elsewhere in the crate, which makes bisecting a regression by
subsetting the crate unreliable, and the prelude order cannot be controlled
from the module.

### Request

Would you consider making the per-module prelude order a function of the
pruned module content only (for example, a topological order with a
deterministic tie-break by path, computed on the pruned call graph)? That
would make a module's SMT session independent of unrelated modules. A
per-query `smt.random_seed` or a documented way to shuffle the order would
also help users detect fragile proofs before they break.

### Artifacts

- `chap26/{iso,full}-mod/*.smt2.gz`, `*.smt_transcript.gz`: the two sessions.
- `chap26/variants/`: renamed, reordered and bisected files; `chap26/replay/`
  their Z3 outputs and summary tables.
- `chap35/`: the same for `select_inner`.
- `scripts/`: the shell/awk tools that split, normalize, reorder and replay.

---

## Issue 2: `-V capture-profiles` panics in `profiler.rs:29:45`

### Reproduction

```
verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors \
  --num-threads 1 -V capture-profiles \
  --verify-module Chap05::RelationStEph::RelationStEph
```

Output (from `profile-panic/profile-repro-Chap05-RelationStEph.*.log`):

```
note: verifying module Chap05::RelationStEph::RelationStEph

note: Analyzing prover log for module Chap05::RelationStEph::RelationStEph ...

Z3 4.16.0
note: Log analysis complete for module Chap05::RelationStEph::RelationStEph


thread 'rustc' (1060411) panicked at rust_verify/src/profiler.rs:29:45:
called `Option::unwrap()` on a `None` value
stack backtrace:
   ...
   3: core::option::unwrap_failed
   4: <hashbrown::raw::RawIterRange<...>>::fold_impl::<... rust_verify::profiler::write_instantiation_graph::{closure#0} ...>
   5: rust_verify::profiler::write_instantiation_graph
   6: <rust_verify::verifier::Verifier>::verify_bucket::<rust_verify::verifier::Reporter>
   7: <rust_verify::verifier::Verifier>::verify_crate_inner
```

With `--num-threads 8` on the full crate the worker panic is followed by a
cascade of `could not send the message!: SendError` and `mpsc open:
SendError` panics from `verifier.rs:229`, `verifier.rs:237`,
`verifier.rs:2197`, and `verifier.rs:2198:29` (`worker thread panicked`), and
the log names a different module (`MappingStEph`) near the panic because of
thread interleaving; with one thread the panic follows `RelationStEph`.

Line 29 is `bnd_info.as_ref().unwrap()` in the `QuantifierKind::User` branch:
a qid in the instantiation graph starts with `USER_QUANT_PREFIX` but has no
entry in `qid_map`. We did not identify which qid. The captured profile is
`profile-panic/Chap05__RelationStEph__RelationStEph.profile.gz`. It contains
user qids from items outside the module (for example `Clone`/`PartialEq`
impls of types in other modules, and qids with suffixes such as
`..._clone_0!48`); one of these may be the missing key.

A non-panicking fallback (treating an unknown user qid as internal, or
reporting it) would keep profiling usable for the rest of the crate.
