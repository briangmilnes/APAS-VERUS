# Veracity Enhancement: Detect cfg-hidden exec functions

## Problem

APAS-VERUS has 55 algorithm implementation functions across 16 files (Chap59-65)
that are gated with `#[cfg(not(verus_keep_ghost))]`. These functions are completely
invisible to Verus — they compile and run under cargo but are never verified.

Previously these functions were inside `verus!{}` with `#[verifier::external_body]`,
so veracity correctly flagged them as proof holes. An agent moved them outside
`verus!{}` and removed the `external_body`, which made them disappear from
veracity's hole count without any actual proof work being done. The hole count
dropped by 38 but zero algorithmic logic was proved.

This is a critical gap. Unverified algorithm implementations hidden behind cfg
gates are just as much proof holes as `external_body` functions — arguably worse,
because they're invisible.

## What to detect

Any `fn` definition (pub or private, free or in an impl block) annotated with
`#[cfg(not(verus_keep_ghost))]` that is an algorithm implementation.

## What to exclude (legitimate uses of the cfg gate)

1. **Derive-style trait impls**: `PartialOrd`, `Ord`, `Hash`, `Debug`, `Display`,
   `Clone`, `PartialEq`, `Eq`, `Default`, `Drop`, `Iterator`, `IntoIterator`,
   `Deref`, `DerefMut`, `From`, `Into`, `AsRef`, `AsMut`, `Borrow`, `fmt::*`.
   These are outside verus!{} per project convention (CLAUDE.md section 14).

2. **Runtime stubs in vstdplus**: `src/vstdplus/accept.rs` has a `cargo_accept`
   module with a no-op `fn accept(_b: bool) {}` stub so the proof function
   compiles under cargo. Similar stubs in vstdplus are legitimate.

3. **`use` statements and `type` aliases** gated with the cfg — not functions.

4. **`macro_rules!`** definitions — legitimately outside verus.

The simplest heuristic: flag cfg-gated `fn` definitions that are NOT inside a
trait impl block for a standard library trait (the derive-style traits listed
above). Algorithm functions are either free functions or in `impl SomeTrait for
SomeType` where `SomeTrait` is a project-defined trait (e.g., `PrimStEphTrait`,
`ConnectivityStEphTrait`).

## Desired output

Same format as other hole detections. Suggested severity and category:

```
file:line: error: cfg_hidden_exec - #[cfg(not(verus_keep_ghost))]
        NN |     pub fn algorithm_name(...)
```

Severity: `error` (same as `external_body`). These are real proof holes.

In the summary section, count them alongside external_body/assume/accept holes.
They should contribute to the file's hole count and the chapter's hole count.

## Where the problem exists today

Grep for the pattern in `~/projects/APAS-VERUS/src/`:

```
#[cfg(not(verus_keep_ghost))]
pub fn some_algorithm(...)
```

Current affected files (16 files, 55 functions):
- `src/Chap59/JohnsonMtEphI64.rs` (5 fns)
- `src/Chap61/EdgeContractionStEph.rs` (2), `EdgeContractionMtEph.rs` (3)
- `src/Chap61/VertexMatchingStEph.rs` (2), `VertexMatchingMtEph.rs` (5)
- `src/Chap62/StarContractionStEph.rs` (3), `StarContractionMtEph.rs` (4)
- `src/Chap62/StarPartitionStEph.rs` (1), `StarPartitionMtEph.rs` (1)
- `src/Chap63/ConnectivityStEph.rs` (5), `ConnectivityMtEph.rs` (7)
- `src/Chap64/TSPApproxStEph.rs` (7), `SpanTreeStEph.rs` (2), `SpanTreeMtEph.rs` (2)
- `src/Chap65/PrimStEph.rs` (3), `KruskalStEph.rs` (3)

Use these as your test corpus. After implementing the detection, run veracity
against these files and confirm all 55 are flagged. Then run against the full
codebase and confirm the legitimate uses (vstdplus, Chap66 derive impls, etc.)
are NOT flagged.

## Context

The project convention is: all algorithm implementations belong inside `verus!{}`
with full verification. `#[cfg(not(verus_keep_ghost))]` on exec functions was an
early scaffolding pattern from before the verification infrastructure was mature.
It was always intended to be temporary. The cfg gate should only be used for
things that genuinely cannot go inside verus (derive impls, macros, Display/Debug).
