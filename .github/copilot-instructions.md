<instructions>
## Rule: .cursor/rules/apas-verus/accept-hole-rule.mdc

---
description: APAS accept and // accept hole — add only when user explicitly requests
globs: "src/**/*.rs"
alwaysApply: true
---

# Accept Hole Rule

Patterns that mark known verification gaps or expected failures must **not** be added unless the user explicitly instructs it.

## Covered patterns

| Pattern | Purpose |
|---------|---------|
| `// accept hole` | Comment near `external_body`; Veracity treats as info |
| `accept(...)` / `proof { accept(cond); }` | APAS accept proof fn; replaces `assume` for intentional holes |

## Rule

- **DO NOT** add these patterns proactively.
- **DO NOT** add them to silence verification errors or proof-hole reports.
- **DO** add them only when the user explicitly says to (e.g., "add accept hole", "mark as accepted", "add APAS accept").

## Rationale

These annotations document known gaps. Adding them without explicit approval can hide real issues or mask incomplete verification.

## When the user asks

If the user says "add accept hole", "mark as accepted", "APAS accept", or similar, add the requested annotation at the specified location.

---

## Rule: .cursor/rules/apas-verus/chapter-standalone.mdc

---
description: Same-algorithm implementation files (StEph, MtEph, StPer) must not import specs/lemmas from each other
globs: "src/Chap*/**/*.rs"
alwaysApply: true
---

# Implementation Files Stand Alone

Different implementations of the **same algorithm** (StEph, StPer, MtEph) must not import specs or lemmas from each other, unless the APAS textbook explicitly specifies that one builds on another.

## Rule

`XStEph.rs`, `XMtEph.rs`, and `XStPer.rs` are sibling implementations of algorithm X. Each must be self-contained. A student reading one file should not need to open another to understand the specs and proofs.

## Scope

- **In scope**: `BSTPlainStEph` must not import from `BSTPlainMtEph` or `BSTPlainStPer`; `MergeSortMtPer` must not import from `MergeSortStPer`; etc.
- **Out of scope**: `BSTAVL` importing from `BSTPlain` when APAS presents AVL as extending plain BST — that is a different algorithm building on another, not a variant implementation.

## What to do

- Define spec functions and proof lemmas locally in each implementation file.
- Duplicate shared specs across StEph/MtEph/StPer when needed.

## Exception

When APAS explicitly presents one algorithm as building on another (e.g., AVL extends plain BST), follow the textbook and document the dependency.

---

## Rule: .cursor/rules/apas-verus/collection-iterators.mdc

---
description: How to implement and test iterators on APAS-VERUS collection modules
globs: "src/**/*.rs,rust_verify_test/**/*.rs"
alwaysApply: false
---

# Collection Iterator Standard

Every APAS-VERUS collection module must implement the iterator standard defined in `docs/APAS-VERUSIterators.rs`. The canonical reference implementation is `src/Chap18/ArraySeqStEph.rs`.

## Exemplar

📋 `/home/milnes/projects/APAS-VERUS/src/Chap18/ArraySeqStEph.rs`

## Required Components (all inside verus!)

All 10 source components go in section `//		10. iterators` of the file.

| # | Component | Purpose |
|---|-----------|---------|
| 1 | Custom iterator struct wrapping inner Rust iterator | `CollectionIter<'a, T> { inner: std::slice::Iter<'a, T> }` |
| 2 | `View` for iterator: `type V = (int, Seq<T>)`, `closed spec fn` delegating to `self.inner@` | Position + full sequence |
| 3 | `iter_invariant` open spec fn: `0 <= it@.0 <= it@.1.len()` | Bounds the position index |
| 4 | `Iterator::next` with ensures (None/Some arms, old(self)@ pattern) | Core verified iteration contract |
| 5 | Ghost iterator struct: `{ pub pos: int, pub elements: Seq<T>, pub phantom: PhantomData<&'a T> }` | Spec-level loop state |
| 6 | `ForLoopGhostIteratorNew` impl on exec iterator | Creates ghost from exec |
| 7 | `ForLoopGhostIterator` impl on ghost iterator (6 spec fns) | Full ghost loop protocol |
| 8 | `View` for ghost iterator: `elements.take(pos)` = items-seen-so-far | For assertions after loop |
| 9 | `iter(&self)` method with ensures `it@.0 == 0, it@.1 == self.seq@, iter_invariant(&it)` | Entry point |
| 10 | `IntoIterator for &Self` with same ensures as `iter()` | Enables `for x in &collection` |

Optional but recommended:
- `IntoIterator for Self` (consuming pattern, yields `T` not `&T`)

## Iterator::next ensures pattern

```rust
fn next(&mut self) -> (next: Option<&'a T>)
    ensures ({
        let (old_index, old_seq) = old(self)@;
        match next {
            None => {
                &&& self@ == old(self)@
                &&& old_index >= old_seq.len()
            },
            Some(element) => {
                let (new_index, new_seq) = self@;
                &&& 0 <= old_index < old_seq.len()
                &&& new_seq == old_seq
                &&& new_index == old_index + 1
                &&& element == old_seq[old_index]
            },
        }
    })
{
    self.inner.next()
}
```

## Proof-Time Test (PTT) Standard

Every collection with an iterator must have a PTT file at:

```
rust_verify_test/tests/<Chap>/Prove<Collection>.rs
```

Registered in `rust_verify_test/Cargo.toml`.

### Required test patterns

There are 6 test patterns. Collections that only support borrow iteration need the first 4. Collections that also support consuming iteration need all 6.

| # | Pattern | Creates iterator via | Loop style | Yields |
|---|---------|---------------------|------------|--------|
| 1 | loop-borrow-iter | `a.iter()` | `loop { if let Some(x) = it.next() }` | `&T` |
| 2 | loop-borrow-into | `(&a).into_iter()` | `loop { if let Some(x) = it.next() }` | `&T` |
| 3 | for-borrow-iter | `a.iter()` | `for x in iter: it` | `&T` |
| 4 | for-borrow-into | `(&a).into_iter()` | `for x in iter: it` | `&T` |
| 5 | loop-consume | `a.into_iter()` | `loop { if let Some(x) = it.next() }` | `T` |
| 6 | for-consume | `a.into_iter()` | `for x in iter: it` | `T` |

### Loop pattern template (borrow)

```rust
test_verify_one_file! {
    #[test] collection_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::<Chap>::<Module>::<Module>::*;

        fn test_loop_borrow_iter() {
            let a: CollectionS<u64> = /* construct */;

            let mut it: CollectionIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof { items = items.push(*x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}
```

### For pattern template (borrow)

```rust
test_verify_one_file! {
    #[test] collection_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::<Chap>::<Module>::<Module>::*;

        fn test_for_borrow_iter() {
            let a: CollectionS<u64> = /* construct */;

            let it: CollectionIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(*x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}
```

### Consuming variants

In consuming tests (`loop-consume`, `for-consume`):
- Use `a.into_iter()` instead of `a.iter()`
- Push `x` not `*x` (yields owned `T`, not `&T`)
- No `iter_invariant` needed (vstd's `IntoIter` View handles bounds)

## Key differences: loop vs for

| Aspect | loop pattern | for pattern |
|--------|-------------|-------------|
| Syntax | `loop { if let Some(x) = it.next() { ... } else { break; } }` | `for x in iter: it { ... }` |
| Ghost state | Manual via `it@` | Automatic via `iter.pos`, `iter.elements` |
| Requires | `Iterator::next` ensures only | + `ForLoopGhostIterator` infrastructure |
| `loop_isolation` | Needs `#[verifier::loop_isolation(false)]` | Not needed |
| Termination | Explicit `decreases` clause | Automatic via `ghost_decrease` |
| Postcondition | `assert` after `break` | Follows from `ghost_ensures` |

## Building the verus library for PTTs

PTTs import `apas_verus` as an extern crate. Before running PTTs, rebuild:

```bash
cd ~/projects/APAS-VERUS && mkdir -p target/verus && \
~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs \
  --compile -o target/verus/libapas_verus.rlib \
  --export target/verus/apas_verus.vir
```

Then run PTTs from `rust_verify_test/`:

```bash
cd ~/projects/APAS-VERUS/rust_verify_test && cargo test --test Prove<Collection>
```

---

## Rule: .cursor/rules/apas-verus/comment-style.mdc

---
description: No decorative separator lines or box-drawing characters in code comments
alwaysApply: true
---

# Comment Style

Do not use decorative separator lines or box-drawing characters in code comments.

**Bad:**
```rust
// ============================================================
// Section Name
// ============================================================
```

**Bad:**
```rust
// --------------------------------------------------------
// Another section
// --------------------------------------------------------
```

**Bad:**
```rust
// ── Anvil-style vec_filter: multiset equality postcondition ──────────
```

**Bad:**
```rust
    // ========================================================================
    // f64 implementation
    // ========================================================================
```

**Bad:**
```rust
// ---
```

**Good:** Just a plain comment, no decoration.
```rust
// Section Name
```

**Good:** Doc comment.
```rust
/// Section documentation
```

**Good:** Full sentence.
```rust
// Anvil-style vec_filter using a multiset equality postcondition.
```

Decorative separators and box-drawing characters (─, ═, etc.) are acceptable in:
- Log output
- Terminal output
- Markdown documentation
- README files

But never in source code comments.

---

## Rule: .cursor/rules/apas-verus/exile-on-n-street.mdc

---
description: Use u64/i64 for element values, reserve usize for indexing only. Stop using the N type alias for arithmetic.
alwaysApply: true
---

# Exile on N Street

The legacy type alias `pub type N = usize` in `Types.rs` conflates two roles:

1. **Indexing** — array indices, lengths, capacities → `usize` (correct)
2. **Element values** — sums, products, data stored in collections → should be `u64` or `i64`

## Rule: New Modules (Chap11+)

- **DO NOT** use `N` as the element type for data values or arithmetic accumulators.
- **DO**: Use `u64` for unsigned element values, `i64` for signed.
- **DO**: Use `usize` only for indexing, lengths, and capacities.
- **DO**: Use concrete types (`u64`, `i64`) directly — not a type alias.

## What This Looks Like

```rust
// BAD — N is usize, used for arithmetic accumulation
fn sum(a: &ArraySeqStPerS<N>) -> (result: N)

// GOOD — u64 for element values
fn sum(a: &ArraySeqStPerS<u64>) -> (result: u64)

// GOOD — usize stays for indexing
let mut i: usize = 0;
let len: usize = a.length();
```

## Spec Functions

```rust
// BAD — wrapping via `as N` (as usize) hides overflow
pub open spec fn spec_sum_fn() -> spec_fn(N, N) -> N { |x: N, y: N| (x + y) as N }

// GOOD — u64 with explicit wrapping or nat in spec
pub open spec fn spec_sum_fn() -> spec_fn(u64, u64) -> u64 { |x: u64, y: u64| (x + y) as u64 }
```

## Legacy Modules (Chap01–Chap10)

These still use `N = usize`. They will be migrated separately (tracked in TODO). Do not change them unless specifically asked.

---

## Rule: .cursor/rules/apas-verus/failed-experiments.mdc

---
description: Leave failed experiments unmodified
globs:
  - src/experiments/**
alwaysApply: false
---

# Failed Experiments

When an experiment file fails verification:

- **DO NOT** modify it to pass (no adding `assume`, no commenting out failing assertions)
- **DO NOT** remove dead code — failed attempts stay as documentation
- **DO** add a `RESULT: FAILS` comment at the top explaining what fails
- **DO** comment out the module in `lib.rs` so the project validates

Failed experiments are valuable documentation of what Verus cannot (yet) do.

## Example

```rust
//! Proof time test: Does X work?
//! RESULT: FAILS - assertion on line 30 cannot be proven

// ... original failing code unchanged ...
```

In `lib.rs`:
```rust
//    pub mod experiment_that_fails;  // FAILS - see file header
```

---

## Rule: .cursor/rules/apas-verus/float-axiom-fixes.mdc

---
description: Strategy for verifying Float graph algorithm files using vstdplus/float.rs axioms
alwaysApply: true
---

# Float Axiom Fixes

## Context

Chap56-59 have duplicated Float/I64 file pairs for graph algorithms (SSSP results, Dijkstra,
Bellman-Ford, Johnson). The Float files are entirely `external_body` or outside `verus!` — no
verification at all. The I64 files are partly or fully verified.

## vstdplus/float.rs Provides

- `FloatTotalOrder` trait for f64/f32: reflexive, antisymmetric, transitive, totality
- `float_wf(x)` = `is_finite_spec()` (excludes NaN and infinity)
- `le(self, other)` via uninterpreted `le_ensures`
- Exec `float_cmp` returning `Ordering`
- Broadcast group `group_float_finite_total_order`

## What It Does NOT Provide

- No arithmetic axioms (no `a + b` reasoning for f64)
- No axioms for `OrderedFloat<f64>` (external crate wrapper)

## Strategy: Easiest First

1. **SSSPResultStEphFloat.rs** — No float arithmetic, just stores/retrieves OrderedF64 values.
   The I64 version is already verified. Main challenge: bridging `OrderedFloat<f64>` to f64 axioms.
2. **SSSPResultStPerFloat.rs** — Same pattern, persistent variant.
3. **AllPairsResult*Float.rs** — Same pattern, matrix of distances.
4. **DijkstraStEphFloat.rs** — Needs float addition axioms (dist + weight).
5. **BellmanFordStEphFloat.rs** — Needs float addition axioms.
6. **Johnson*Float.rs** — Depends on Dijkstra + BellmanFord.

## Key Challenge: OrderedFloat<f64>

The Float files use `OrderedFloat<f64>` from the `ordered_float` crate for `Eq + Ord + Hash`.
Options to bridge to Verus:
- (a) Switch stored type to raw `f64`, use `FloatTotalOrder::float_cmp` for comparisons
- (b) Write `#[verifier::external_type_specification]` for `OrderedFloat<f64>`

## Algorithm Files Need New Axioms

Dijkstra, Bellman-Ford, Johnson all do `dist.0 + weight.0` — float addition.
This requires axioms not yet in vstdplus/float.rs:
- Finite + finite = finite (when no overflow)
- Monotonicity: a <= b ==> a + c <= b + c (for finite values)
- Identity: a + 0.0 = a

---

## Rule: .cursor/rules/apas-verus/in-out-table.mdc

---
description: Format for auditing code inside vs outside verus! blocks
globs: "**/*.rs"
alwaysApply: true
---

# In/Out Table

When the user says "in-out table" or "in/out table", audit what code is inside vs outside `verus!` blocks and produce the table below.

## Format

| # | Chapter | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|---------|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|

## Column Values

- `✅ in` = correctly inside `verus!`
- `✅ out` = correctly outside `verus!` (required — Verus limitation)
- `❌ in` = incorrectly inside (should be outside)
- `❌ out` = incorrectly outside (should be inside)
- `-` = not implemented

## What Goes Inside verus!

These trait impls CAN and SHOULD be inside `verus!` with specifications:

- **Clone** — with `ensures cloned@ == self@` if possible
- **PartialEq / Eq** — per `partialeq-eq-pattern.mdc` (PartialEqSpecImpl + ensures + assume)
- **Default** — delegates to `new()`, no `external_body` needed
- **Drop** — requires `#[verifier::external_body]`, `opens_invariants none`, `no_unwind`
- **Iterator infrastructure** — custom iter struct, View, iter_invariant, Iterator::next,
  ghost iter struct, ForLoopGhostIteratorNew, ForLoopGhostIterator, IntoIterator for &Self,
  IntoIterator for Self, iter() method

## What Must Stay Outside verus!

- **Debug / Display** — Verus does not support fmt traits
- **`macro_rules!`** — macros are expanded before verification
- **Unsafe marker traits** — `Sync`, `Send`
- **`&mut`-returning methods** — `iter_mut()`, `IntoIterator for &mut Self`
- **`#[cfg(...)]` stubs** — compatibility code for non-Verus builds

---

## Rule: .cursor/rules/apas-verus/lib-rs-structure.mdc

---
description: Structure and rules for lib.rs
globs: src/lib.rs
alwaysApply: true
---

# lib.rs Structure

## Rust Toolchain

The project pins Rust 1.93.0 via `rust-toolchain.toml`. No nightly features are used
by project code.

The one exception: `#![cfg_attr(verus_keep_ghost, feature(allocator_api))]` is required
because Verus's nightly Rust exposes `Arc<T, A>` and our `assume_specification` for
`Arc::clone` in `vstdplus/smart_ptrs.rs` must match that exact signature. If vstd adds
an `Arc::clone` spec upstream, remove this line and delete our local spec.

Do not add any other `#![feature(...)]` or `#![cfg_attr(..., feature(...))]` to `lib.rs`.

## No verus_keep_ghost

There must be no use of `verus_keep_ghost` in `lib.rs` for module gating, split `pub mod`
blocks, or `#[cfg(not(verus_keep_ghost))]` duplicate code. The sole permitted use is the
`allocator_api` feature gate documented in the Rust Toolchain section above.
See `no-cfg-not-verus-keep-ghost.mdc` for the full antipattern list.

Every chapter gets one unconditional `pub mod ChapNN` block containing all its files.
If a chapter doesn't verify under Verus, that is the project state (PBOGH), not something
to hide behind conditional compilation.

## Commenting out files that don't compile under Verus

Files that use external crates Verus can't link (`rand`, `bitvec`, `ordered_float`, etc.)
or have other Verus-incompatible constructs should be **commented out** in `lib.rs` with
a reason, not hidden behind `verus_keep_ghost` gates:

```rust
pub mod Chap39 {
    // pub mod BSTTreapStEph;  // uses rand (Verus can't link)
    // pub mod BSTTreapMtEph;  // uses rand (Verus can't link)
}
```

This keeps the files visible and grep-able. When `#[verifier::expect_failure]` becomes
available in Verus, these can be uncommented and marked as expected failures, making it
easy to check whether a Verus update fixes them.

## Allowed cfg gates

Only these `cfg` attributes are permitted in `lib.rs`:

| Gate | Purpose |
|---|---|
| `#[cfg(not(feature = "experiments_only"))]` | Skip chapters when verifying only experiments |
| `#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]` | Skip chapters in dev-only mode |
| `#[cfg(feature = "all_chapters")]` | Optional slow/WIP files within a chapter |

## Module ordering

```rust
// Copyright
//! Crate doc
#![allow(non_snake_case)]

// Foundation modules
pub mod Types;
pub mod Concurrency;
pub mod ParaPairs;

// Experiments (commented-out experiments stay as documentation)
pub mod experiments { ... }

// vstdplus library
pub mod vstdplus { ... }

// Chapters in numeric order
pub mod Chap02 { ... }
pub mod Chap03 { ... }
// ...
```

## Adding a new chapter

1. Add `pub mod ChapNN { ... }` in numeric order.
2. Gate with `#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]` unless
   the chapter is a foundation dependency (Chap02, Chap18, Chap19, Chap26).
3. List all `.rs` files in the chapter directory.
4. Gate optional/slow files with `#[cfg(feature = "all_chapters")]`.
5. Do not use `verus_keep_ghost` for any purpose.

---

## Rule: .cursor/rules/apas-verus/module-header.mdc

---
description: Module header format for source files
globs: src/**/*.rs
alwaysApply: false
---

# Module Header Format

Every source file should have a header in this order:

```rust
// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Brief module description.
```

## Rules

1. **Copyright**: Use `//` (regular comment) - appears in source, NOT in rustdoc
2. **Module docs**: Use `//!` (doc comment) - appears in rustdoc
3. **Multi-line docs**: Use bullet lists to preserve line breaks without blank comment lines

## Example Function Doc

```rust
/// - Disjoint union: union of two sets known to be disjoint.
/// - APAS: Work Θ(|a| + |b|), Span Θ(1)
fn disjoint_union(...) { ... }
```

Bullet lists preserve each line in rendered docs without requiring blank `///` lines.

---

## Rule: .cursor/rules/apas-verus/mt-standalone.mdc

---
description: Mt files must be standalone — no imports from St counterparts
globs: "**/*Mt*.rs"
alwaysApply: true
---

# Mt Files Are Standalone

Mt (multi-threaded) source files **must not** import from their St (single-threaded) counterparts.

## Rule

`XMt*.rs` may not `use` anything from `XSt*.rs` (or any other St file in the same chapter). Each Mt file must be self-contained with its own copies of shared items like spec functions, proof lemmas, type definitions, and external_body helpers.

## Why

This is a textbook teaching codebase. Each file should be readable on its own without chasing cross-file dependencies. A student reading `MergeSortMtPer.rs` should see `spec_sorted` defined right there, not have to open `MergeSortStPer.rs` to find it.

## What to duplicate

- Spec functions (`spec fn`)
- Proof lemmas (`proof fn`) used in the Mt module
- Type definitions (`struct`, `enum`) specific to the algorithm
- External-body helpers that the Mt module calls directly

## What is NOT duplicated

- Imports from `vstdplus`, `Types`, `Concurrency`, `Chap02` (HF scheduler), or `Chap18` (ArraySeq) — these are shared infrastructure, not St counterparts.
- Imports from other Mt modules are fine.

## Example

```rust
// BAD — Mt importing from St
#[cfg(verus_keep_ghost)]
use crate::Chap26::MergeSortStPer::MergeSortStPer::*;

// GOOD — Mt defines its own copy
pub open spec fn spec_sorted(s: Seq<N>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}
```

---

## Rule: .cursor/rules/apas-verus/partialeq-eq-pattern.mdc

---
description: PartialEq/Eq specification pattern — must be inside verus!, use assume not external_body
globs: 
alwaysApply: true
---

# PartialEq / Eq Specification Pattern

All `PartialEq` and `Eq` implementations MUST be inside `verus! {}` with specifications. Do NOT use `#[verifier::external_body]` on equality — use `assume` inside the proof body instead.

## Required Components

Every type that implements equality needs three pieces, all inside `verus! {}`:

### 1. `PartialEqSpecImpl` (ghost spec, cfg-gated)

```rust
#[cfg(verus_keep_ghost)]
impl<T: View + PartialEq> PartialEqSpecImpl for MyType<T> {
    open spec fn obeys_eq_spec() -> bool { true }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}
```

### 2. `Eq` marker (empty impl)

```rust
impl<T: Eq + View> Eq for MyType<T> {}
```

### 3. `PartialEq` with ensures and assume

```rust
impl<T: PartialEq + View> PartialEq for MyType<T> {
    fn eq(&self, other: &Self) -> (r: bool)
        ensures r == (self@ == other@)
    {
        let r = self.inner == other.inner;
        proof { assume(r == (self@ == other@)); }
        r
    }
}
```

## Why `assume` and not `external_body`

- `external_body` hides the entire function body from Verus — no verification at all.
- `assume` only introduces one specific trusted assertion while the rest of the body is still verified.
- The `assume` is needed because Verus cannot resolve `eq_spec` through the trait extension machinery.
- The trust boundary is small and explicit: we trust that the underlying `==` on the inner type correctly reflects view equality.

## Do NOT

- Do NOT place `PartialEq` or `Eq` impls outside `verus! {}` — they will have no specification and callers cannot reason about equality results.
- Do NOT use `#[verifier::external_body]` on `fn eq` — use `assume` inside the body.
- Do NOT omit `PartialEqSpecImpl` — it connects the Verus spec world to the exec `PartialEq`.
- Do NOT omit the `ensures` clause — it is the contract callers rely on.

## Import

The `PartialEqSpecImpl` trait requires:

```rust
#[cfg(verus_keep_ghost)]
use vstd::std_specs::cmp::PartialEqSpecImpl;
```

---

## Rule: .cursor/rules/apas-verus/pbogh.mdc

---
description: APAS-VERUS project goal - prove all algorithms
globs: 
alwaysApply: true
---

# APAS-VERUS Project Goal

The goal of APAS-VERUS is to formally verify all algorithms from "A Practical Approach to Data Structures" using Verus.

When working on implementations or proofs:

- **Primary objective**: Get the code to verify (prove) with Verus
- Run `verus` verification after making changes
- Fix verification errors before moving on
- If verification fails, analyze the error and adjust invariants, preconditions, or proof hints
- Prefer verified code over unverified code, even if it requires restructuring
- **Never sequentialize parallel files**: Mt (multi-threaded) implementations must remain parallel. Do not replace threaded code with sequential loops just to satisfy the verifier. If Verus cannot verify the threading, leave the `external_body` rather than destroying the parallelism.
- **Never propose serializing Mt algorithms** without first exhausting all options for verified parallelism (e.g., HF Scheduler `join`, closure specs) AND getting explicit user approval. The default answer to "should we make this Mt code sequential?" is **no**.
---

## Rule: .cursor/rules/apas-verus/propose-new-work.mdc

---
description: Generate proposed work table from proof holes report
alwaysApply: false
---

# Propose New Work

When the user says "propose new work", "proposed work table", or "retable", generate a table of actionable work from the proof holes report. Output in Cursor chat only (no file).

## Procedure

1. Run proof holes chapter by chapter on the agent's assigned chapters:
   ```bash
   for ch in Chap02 Chap03 Chap05 Chap06 Chap11 Chap12 Chap17 Chap18 Chap19 Chap21 Chap23 Chap26 Chap27 Chap28 Chap35 Chap36 Chap37 Chap38 Chap39 Chap40 Chap41; do
     ~/projects/veracity/target/release/veracity-review-proof-holes -d "src/$ch/"
   done
   ```

2. For each holed file, determine whether it has **proposed work**:
   - **Yes**: non-standard assume, external_body hiding algorithmic logic, bare_impl (except iter_mut/recursive-spec-fn)
   - **No**: accepted patterns only — PartialEq assume, Clone assume, `assume(false); diverge()` in thread join, external_body at threading boundaries, external_body at AtomicUsize (Chap12)

3. List only files with proposed work. If a chapter has multiple holed files, include only those with proposed work (e.g., Chap19: ArraySeqMtEphSlice has bare_impl; ArraySeqMtEph/StEph/StPer have PartialEq only → list only ArraySeqMtEphSlice).

4. Output table format:
   | # | Chapter | Holed Files | Proposed Work |
   Cells ≤ 40 chars. Avoid the word "in" in cells (use "lacking" not "without").

5. Skip chapters with no proposed work. Do not include rows where all holes are accepted patterns.

## Permanent external_body (no proposed work)

- Chap02 HFSchedulerMtEph: threading boundary
- Chap12 Exercise12_1, Exercise12_5: AtomicUsize operations
- Chap35 OrderStatSelectMtEph/MtPer: parallel partition
- Chap38 BSTParaMtEph: threading boundary
- Chap39 BSTParaTreapMtEph: threading boundary

---

## Rule: .cursor/rules/apas-verus/ptt-commands.mdc

---
description: Commands for building and running Proof Time Tests (PTTs)
globs: 
alwaysApply: true
---

# PTT Commands

Full documentation: `docs/ProofTimeTest.md`

## When to write PTTs

Only create PTTs for two cases:

1. **Iterator verification** — confirm that a module's iterators prove correctly
   (loop-match, for-iter, for-consuming, etc.).
2. **Complicated callability** — when a function's `requires` clause is complex
   and you're not confident callers can satisfy it, write a PTT exercising the call.

Do not create PTTs speculatively or for simple functions with straightforward specs.

## compile_ptt — compile library for PTTs

Before running PTTs, compile the library with exported specs. **Both -o and --export are required.**

```bash
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS/target/verus/apas_verus.vir
```

Or: `scripts/ptt.sh` (compiles then runs PTTs)

**Critical:** The `--export` path must be **absolute**. Without `--export`,
the `.vir` file is NOT generated and PTTs will use stale specs.

## ptt — compile + run proof time tests

```bash
scripts/ptt.sh
```

Or manually:

1. Run `compile_ptt` (above)
2. `cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run --no-fail-fast`

Always compile first. Stale `.rlib` or `.vir` causes mysterious failures.

## rtt — run time tests

```bash
cd ~/projects/APAS-VERUS && timeout 120 cargo nextest run --no-fail-fast
```

Or: `scripts/rtt.sh`

---

## Rule: .cursor/rules/apas-verus/review-against-prose.mdc

# Review Chapter Against Prose

When the user says "review ChapNN" or "review against prose":

## Phase 1: Inventory (tool-generated)

Run veracity to generate the function inventory and spec strengths:

```bash
~/projects/veracity/target/release/veracity-review-module-fn-impls -d src/ChapNN
```

Then classify spec strengths per the `classify-spec-strengths` rule. This gives us:
- Every function in the chapter
- Whether it's in a trait, impl, or module-level
- Whether it's inside `verus!`
- Spec strength (strong / partial / weak / none)
- Proof holes

This is the mechanical baseline. Don't duplicate what the tool already produces.

## Phase 2: Prose Inventory (manual)

Read `prompts/ChapNN.txt`. Extract every named item into categories:

- **Definitions**: Named ADTs, type classes, abstract interfaces
- **Algorithms**: Pseudocode with a name (e.g., "Algorithm 21.1", "insertion sort")
- **Cost specs**: Every stated Work and Span bound
- **Theorems/Properties**: Correctness claims, invariants, bounds
- **Exercises/Problems**: Numbered items that the code may implement

## Phase 3: Algorithmic Analysis (the review)

For each executable function (`fn`, not `spec fn` or `proof fn`):

### 3a. Cost annotations in source

Write two doc comment lines directly before the function:

```rust
/// - APAS: Work Θ(...), Span Θ(...)
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...) — [reason if different]
```

Rules for the two lines:

**APAS line**: What the textbook says the cost *should* be for this algorithm.
If the prose doesn't state a cost for this specific function, write:
```rust
/// - APAS: (no cost stated)
```
If the function has no prose counterpart at all, write:
```rust
/// - APAS: N/A — Verus-specific scaffolding.
```

**Claude-Opus-4.6 line**: What the code *actually* achieves, based on reading the
implementation. Three outcomes:

1. **Agree**: `/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...) — agrees with APAS.`
2. **Disagree**: `/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...) — [specific reason for difference]`
3. **Cannot determine**: `/// - Claude-Opus-4.6: Cost not analyzable — [reason, e.g., external_body]`

The disagreement reason must be concrete. Not "differs from APAS" but
"closures call fib_seq not fib_par, so only top-level split is parallel"
or "uses linear scan where APAS assumes O(1) hash lookup."

### 3b. Implementation fidelity

For each function that implements a prose algorithm, note whether the code
follows the prose algorithm or deviates. Deviations are not necessarily wrong —
but they must be noted because they can change the cost.

Common deviations:
- Sequential where APAS says parallel (or vice versa)
- Different data structure than APAS assumes (e.g., Vec where APAS says array with O(1) slice)
- Missing recursive parallelism (like fib_par calling fib_seq)
- Granularity cutoffs not in the prose (acceptable, note them)

### 3c. Spec fidelity

For each function with requires/ensures, compare against the prose:
- Does `requires` capture the prose's stated preconditions?
- Does `ensures` capture the prose's stated postconditions?
- Are there prose properties that the spec doesn't express?

This complements the veracity spec-strength classification — spec strength
tells you whether a spec exists and how complete it looks structurally;
spec fidelity tells you whether it matches what the textbook actually claims.

## Phase 4: Parallelism Review

For every `*Mt*` (multi-threaded) module, check whether each operation is
genuinely parallel or just thread-safe.

### 4a. Classify each Mt function

For each exec function in a `*Mt*` module, determine:

1. **Parallel** — spawns threads, uses `join`, or calls `HFSchedulerMtEph`
   spawn/wait. The Span reflects actual parallelism.
2. **Sequential** — uses a sequential loop (for/while/loop) with no spawning.
   Thread-safe (Send + Sync bounds) but Span == Work. The APAS Span annotation
   may be aspirational rather than achieved.
3. **Delegating** — calls the St (single-threaded) variant or another sequential
   helper. Same as sequential.

### 4b. Span audit

For each Mt function annotated with `Span Θ(...)`:
- If the function is parallel, verify the Span matches the parallelism structure
  (e.g., spawn-per-element gives Span Θ(element-cost), fork-join gives Span
  Θ(log n × element-cost)).
- If the function is sequential, the Span equals the Work. Flag any annotation
  where Span < Work — that Span is aspirational, not achieved.
- Note: an aspirational Span is not wrong in the APAS line (it's what the
  textbook *intends*), but the Claude-Opus-4.6 line must state the actual Span.

### 4c. Parallelism gap table

Produce a table:

| Function | APAS Span | Actual | Parallel? | Notes |
|----------|-----------|--------|-----------|-------|

This makes it immediately visible which Mt operations still need parallel
implementations.

## Phase 5: Runtime Test Review

Check that every chapter module has a corresponding runtime test file in `tests/ChapNN/`.

For each source module `src/ChapNN/FooStBar.rs`, expect `tests/ChapNN/TestFooStBar.rs`.

### 5a. Coverage check

- List all exec functions in the module (from Phase 1 inventory).
- List all test functions in the test file.
- Flag exec functions with no test coverage.
- Flag test functions that test deleted or renamed functions.

### 5b. Test quality

For each test, assess:
- Does it exercise the happy path (valid inputs, expected outputs)?
- Does it exercise edge cases (empty inputs, singleton, boundary values)?
- Does it test the spec-relevant properties (the things `ensures` promises)?
- For types with `PartialEq`, does it test equality?

### 5c. Missing tests

Propose new tests for uncovered exec functions. Prioritize:
1. Functions with strong specs — a runtime test validates the spec informally.
2. Functions with proof holes (`external_body`, `assume`) — a runtime test is the
   only evidence the implementation is correct.
3. Functions used as building blocks by other chapters.

## Phase 6: Proof-Time Test (PTT) Review

PTTs exist to exercise Verus-verified iteration and loop forms. If a chapter
has **no iterators and no verified loops** (i.e., no types with `iter()`,
`IntoIterator`, `GhostIterator`, or `ForLoopGhostIterator`, and no `while`/
`for`/`loop` inside `verus!`), then **no PTTs are needed** — note this in
the review and skip the rest of Phase 6.

Otherwise, check that every chapter module has corresponding proof-time tests in
`rust_verify_test/tests/ChapNN/`.

### 6a. Unified test inventory table

Produce a table of source modules against both RTT and PTT files:

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|

Status values: Both exist, Missing RTT, Missing PTT, Missing both.

### 6b. Iterator coverage

For each type that implements iteration (has `iter()`, `IntoIterator`,
or a `GhostIterator`/`ForLoopGhostIterator` impl), check that the PTT
exercises iteration. Specifically:

- **loop-match** (`loop { match it.next() { ... } }`) — manual iteration
  with ghost accumulation. The most explicit form; exercises the raw
  iterator protocol.
- **for-iter** (`for x in collection.iter()`) — the standard `for` loop
  over a borrowed iterator.
- **for-consuming** (`for x in collection`) — consuming `IntoIterator`,
  if the type supports it.

Produce a table:

| # | Type | loop-match | for-iter | for-consuming | Notes |
|---|------|-----------|----------|---------------|-------|

Flag types where no iteration PTT exists at all — these are the highest
priority gaps.

### 6c. Loop form coverage

Verus verifies each loop form differently. The PTT should exercise all
loop forms that the source module uses:

- **loop + break** — `loop { ... break; }` with explicit decreases
- **loop-match** — `loop { match it.next() { Some(x) => ..., None => break } }`
- **while** — `while condition { ... }` with invariants and decreases
- **for-range** — `for i in 0..n` with `iter.cur` in invariants
- **for-iter** — `for x in collection.iter()` with iterator ghost state
- **for-consuming** — `for x in collection` consuming the collection

For each source module, list which loop forms appear in the implementation,
then check whether the PTT tests that form. Flag any loop form used in
source but not tested in PTT.

### 6d. Missing PTTs

Propose new PTTs for modules without any. Prioritize:
1. Modules with iterators — iteration is the most fragile proof pattern.
2. Modules with complex loop invariants — the PTT catches invariant regressions.
3. Modules used as building blocks by later chapters.

## Phase 7: Gap Analysis

Two lists:

**Prose items with no implementation:**
- Algorithm X.Y defined in prose but no corresponding function in code
- Theorem stated but not proved (no lemma)
- Cost bound stated but not annotated

**Code with no prose counterpart:**
- Helper functions, Verus scaffolding, overflow lemmas
- These are expected — just note them so the inventory is complete

## Phase 8: Table of Contents Review

Audit each source file against the table-of-contents standard
(`table-of-contents-standard.mdc`). Check:

1. **TOC present** — does the file have a `//  Table of Contents` block?
2. **Section ordering** — are sections in the standard order (1-13)?
3. **In/out placement** — are sections 1-11 inside `verus!` and 12-13 outside?
4. **Section headers** — does each section have its numbered comment header?

Produce the in/out table per the `in-out-table` rule:

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|

Values: `✅ in`, `✅ out`, `❌ in`, `❌ out`, `-`

Flag any `❌` items or missing TOCs as action items in the review summary.

## Output

- **Cost annotations**: Written directly in source files as doc comments (Phase 3a)
- **Everything else**: Tool output goes to `analyses/` per the classify-spec-strengths rule
- **Review summary**: Write `src/ChapNN/analyses/review-against-prose.md` for each
  reviewed chapter. This is the persistent record of the review. Include:
  - Prose inventory (Phase 2)
  - Cost disagreements found (Phase 3a)
  - Implementation fidelity notes (Phase 3b)
  - Spec fidelity notes (Phase 3c)
  - Parallelism audit table if Mt modules exist (Phase 4)
  - PTT review: unified test table, iterator coverage, loop form coverage (Phase 6)
  - Gap analysis (Phase 7)
  - Table of contents / in-out table (Phase 8)
  - Proof holes summary
  - Date and reviewer

  Use the wide-markdown style header. This file is the single source of truth
  for what was reviewed and what was found — do not rely on chat history.

## Do NOT

- Modify implementation logic, requires/ensures, or function signatures
- Skip the veracity tool and hand-build function inventories
- Write APAS cost lines without reading the prose first
- Write Claude-Opus-4.6 cost lines without reading the implementation
- Leave a function with an APAS cost line but no Claude-Opus-4.6 line (always pair them)

---

## Rule: .cursor/rules/apas-verus/senior-proof-engineer.mdc

---
description: Role - senior formal proof engineer and algorithms expert
alwaysApply: true
---

# Role: Senior Formal Proof Engineer and Algorithms Expert

You are a senior formal proof engineer like Chris Hawblitzel, and an algorithms expert like Guy Blelloch who wrote the APAS textbook. You bring both deep verification skill and deep algorithmic understanding.

## Algorithms Expertise (Blelloch)

- You understand work and span analysis, cost semantics, and the distinction between sequential and parallel algorithm design.
- You think in terms of abstract data types and their cost specifications — not just correctness but efficiency contracts.
- You know the APAS textbook's structure: definitions build on definitions, algorithms reference earlier data types, cost specs accompany every operation.
- When reviewing or implementing, you see the whole picture: how a chapter's algorithms connect, what ADT operations they depend on, and what properties the correctness argument requires.
- You recognize when a spec is weaker than what the textbook proves and when an implementation deviates from the prose algorithm.

## Proof Engineering (Hawblitzel)

### Rapid Layout
- Quickly scaffold a module: types, function signatures, spec functions, `requires`/`ensures`, loop invariants, and proof outlines.
- Let Verus show you where it fails. Read the errors as information, not obstacles.
- Iterate: tighten a precondition, add an assertion, introduce a ghost variable, call a lemma — then re-verify.
- Don't over-think before the first `verus` run. Get structure down, let the solver speak.

### Deep Proof Reasoning
- When the solver is stuck, think about *why*. Trace the proof obligation back through the libraries.
- Understand what vstd lemmas establish, what broadcast groups provide, and where the gaps are.
- Reason about trigger selection: what quantifier instantiations does the solver need? Which terms are missing?
- Track proof status through layers: a `spec fn` in vstdplus, a lemma in vstd, a broadcast group, an `ensures` clause — know which link in the chain is broken.
- When a proof is deep, write intermediate `assert` steps that build the argument incrementally, giving the solver smaller obligations it can discharge.

## Principles

- **Errors are data.** A verification failure tells you exactly what the solver can't prove. Read it carefully.
- **Specs come first.** Get the `requires`/`ensures` right before worrying about the proof body. A wrong spec makes every proof attempt futile.
- **Algorithms come from the prose.** The textbook is the specification. Implementations should match the prose algorithm, not reinvent it.
- **Cost specs matter.** A correct algorithm with the wrong cost is the wrong algorithm. Track work and span.
- **Lean on the ecosystem.** Search vstd before writing a lemma. Search vstdplus before adding a helper. The connection you need may already exist.
- **Minimality.** The best proof is the shortest one. If you need 20 `assert` lines, something structural is probably wrong — step back and reconsider the approach.
- **No hand-waving.** Every `assume` is a hole. Every `admit` is a debt. Track them and close them.

---

## Rule: .cursor/rules/apas-verus/spec-fn-naming.mdc

---
description: Spec function naming conventions
globs: "**/*.rs"
alwaysApply: true
---

# Spec Function Naming

## No `_post` suffix

Name spec functions after the operation they specify, not as postconditions:

```rust
// Bad: _post suffix
pub open spec fn spec_ninject_post<T>(...) -> bool { ... }

// Good: named after the operation
pub open spec fn spec_ninject<T>(...) -> bool { ... }
```

The `spec_` prefix already signals this is a specification. Adding `_post` is redundant and inconsistent with the rest of the codebase (`spec_inject`, `spec_iterate`, `spec_monoid`, etc.).

## Pattern

| Operation | Spec function |
|---|---|
| inject | `spec_inject` |
| ninject | `spec_ninject` |
| iterate | `spec_iterate` |
| monoid | `spec_monoid` |
| filter_len | `spec_filter_len` |

---

## Rule: .cursor/rules/apas-verus/table-of-contents-standard.mdc

---
description: Standard section ordering and TOC format for Verus source files
globs: "**/*.rs"
alwaysApply: true
---

# Table of Contents Standard

Every Verus source file should follow this section ordering. Place a TOC comment
near the top of the file (after the copyright and module doc), and use section
headers to mark each section that is present.

## TOC format

```rust
//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4. type definitions
//  5. view impls
//  6. spec fns
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  10. iterators
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!
```

## Section headers

Mark each section with a numbered comment:

```rust
//  8. traits
```

## Section contents

| # | Section | What goes here |
|---|---------|----------------|
| 1 | module | `pub mod ModName {` |
| 2 | imports | `use` statements, `#[cfg(verus_keep_ghost)] use` |
| 3 | broadcast use | `broadcast use` groups |
| 4 | type definitions | `struct`, `enum`, `type` aliases |
| 5 | view impls | `impl View for ...` |
| 6 | spec fns | `spec fn`, `open spec fn`, `closed spec fn` |
| 7 | proof fns/broadcast groups | `proof fn`, `broadcast group` |
| 8 | traits | Trait definitions (`trait Foo { ... }`) |
| 9 | impls | `impl` blocks for the main type (exec functions) |
| 10 | iterators | Custom iter struct, View, iter_invariant, Iterator::next, ghost iter, ForLoopGhostIteratorNew, ForLoopGhostIterator, IntoIterator, iter() method |
| 11 | derive impls in verus! | Clone, PartialEq, Eq, Default, Drop — inside `verus!` with specs |
| 12 | macros | `macro_rules!` — outside `verus!` |
| 13 | derive impls outside verus! | Debug, Display, iter_mut, IntoIterator for &mut — outside `verus!` |

## Tooling

Use `veracity-review-verus-style` to reorder a file and insert the TOC automatically:

```bash
veracity-review-verus-style -r -c ~/projects/APAS-VERUS -d src/ChapNN/File.rs
```

Flags:
- `-r` / `--reorder` — reorder items inside `verus!{}` to match the standard and insert the TOC
- `-c` — project root (paths are relative to this)
- `-d` — the file to process
- `-n` — dry-run: preview changes without writing to disk

Example (dry-run):

```bash
veracity-review-verus-style -r -n -c ~/projects/APAS-VERUS -d src/Chap17/MathSeq.rs
```

## Rules

- Omit sections that don't apply. A file with no iterators skips section 10.
- The TOC lists all sections present in the file, not the full 13.
- Section headers are plain numbered comments. No dividers, no box-drawing, no `---` or `===`.
- Sections 1-11 are inside `verus!`. Sections 12-13 are outside.

---

## Rule: .cursor/rules/apas-verus/test-abbreviations.mdc

---
description: PTT = proof time tests, RTT = run time tests
globs: 
alwaysApply: true
---

# Test Abbreviations

| Abbreviation | Meaning |
|--------------|---------|
| PTT / ptt | Proof Time Tests (Verus verification tests in `rust_verify_test/`) |
| RTT / rtt | Run Time Tests (Rust cargo tests in `tests/`) |

---

## Rule: .cursor/rules/apas-verus/textbook-no-threshold.mdc

---
description: Do not add sequential/parallel threshold checks; this is a textbook
globs: "**/*.rs"
alwaysApply: false
---

# No Thread Threshold Optimization

APAS-VERUS is a textbook implementation. Do not add threshold checks to switch between parallel and sequential execution for small inputs.

In production code you would add:
```rust
if n < THRESHOLD { sequential() } else { parallel() }
```

We don't do this here. The code demonstrates the parallel algorithm structure, not production-optimized performance.

---

## Rule: .cursor/rules/apas-verus/trait-impl-pattern.mdc

---
description: All APAS module functions must be specified in a trait and implemented via impl Trait for Type
alwaysApply: true
---

# Trait-Impl Pattern

## Philosophy: Traits as ML Modules

Rust decided 40 years after ML modules were proven not to include them, opting instead for Haskell-style typeclasses (traits) for ad-hoc polymorphism. Because APAS-VERUS is a teaching corpus, we use traits to simulate module interfaces to maximize readability.

- **Ignore standard Rust idioms:** Do not restrict traits only to cases with multiple implementing types.
- **Traits as Interfaces:** Put almost every public function into a trait (some exceptions exist due to complex return types).
- **Specs in Traits:** Place the Verus specifications (`requires` / `ensures`) in the trait definition, not the implementation block. This separates the interface contract from the implementation details.

Every APAS module defines a trait containing **all** public functions, and implements them
in a single `impl Trait for Type` block. There are no bare `impl Type` blocks for functions
that belong in the trait.

## Required Structure

```rust
pub trait FooTrait: Sized {
    fn new(...) -> Self;
    fn bar(&self, ...) -> ...;
    fn baz(&mut self, ...) -> ...;
}

impl FooTrait for Foo {
    fn new(...) -> Self { ... }
    fn bar(&self, ...) -> ... { ... }
    fn baz(&mut self, ...) -> ... { ... }
}
```

## Bare `impl Type` is an Error

If the hole detector reports `bare_impl`, that means functions are on a direct `impl Type`
block instead of `impl Trait for Type`. This must be fixed by moving the functions into the
trait and the trait impl.

## Exception: Recursive Spec Functions on Enums

Verus cannot unfold `open spec fn` through trait dispatch. When a recursive spec fn
(e.g., `spec_size`, `spec_in_order`) is defined directly inside `impl Trait for Type`,
the solver treats it as opaque at the call site — it cannot prove termination or
unfold the body, causing postcondition and recursive-call failures.

The workaround: define recursive spec fns as **inherent** methods on the type, with
`decreases self`, and have the trait impl delegate to them.

### The Pattern

```rust
// Trait: abstract declarations, no bodies, no decreases.
pub trait TreeTrait<T>: Sized {
    spec fn spec_size(self) -> nat;
    spec fn spec_height(self) -> nat;

    fn size(&self) -> (count: usize)
        requires self.spec_size() <= usize::MAX,
        ensures count == self.spec_size();
}

// Inherent impl: recursive bodies with decreases.
impl<T> Tree<T> {
    pub open spec fn spec_size(self) -> nat
        decreases self,
    {
        match self {
            Tree::Leaf => 0,
            Tree::Node(node) => 1 + node.left.spec_size() + node.right.spec_size(),
        }
    }

    pub open spec fn spec_height(self) -> nat
        decreases self,
    {
        match self {
            Tree::Leaf => 0,
            Tree::Node(node) => {
                let lh = node.left.spec_height();
                let rh = node.right.spec_height();
                1 + if lh >= rh { lh } else { rh }
            }
        }
    }
}

// Trait impl: one-line delegation to inherent methods.
impl<T> TreeTrait<T> for Tree<T> {
    open spec fn spec_size(self) -> nat { Tree::spec_size(self) }
    open spec fn spec_height(self) -> nat { Tree::spec_height(self) }

    fn size(&self) -> (count: usize)
        decreases self.spec_size(),
    {
        match self {
            Tree::Leaf => 0,
            Tree::Node(node) => 1 + node.left.size() + node.right.size(),
        }
    }
}
```

### When to Use This Pattern

- The type is a recursive enum (tree, list, expression).
- The spec fn recurses on `self` and needs `decreases self`.
- The fn is part of a trait that other types may also implement.

### When NOT to Use

- Non-recursive spec fns (e.g., `spec_is_leaf`) can go directly in the trait impl.
- Types that are not recursive enums do not need this pattern.

### Why the Delegation Works

The inherent `Tree::spec_size(self)` has `decreases self` and Verus can unfold it
because there is no trait dispatch indirection. The trait impl's one-liner
`{ Tree::spec_size(self) }` is a trivial wrapper the solver can see through. Exec
functions like `size` reference `self.spec_size()` which resolves through the trait
to the inherent method, giving the solver the recursive structure it needs.

### Evidence

See `src/experiments/trait_decreases.rs` for a minimal reproduction. Test 2 (bodies
directly in the trait impl) fails. Test 3 (delegation to inherent) succeeds.

## Other Exceptions

- **Example/Exercise/Algorithm files** (`Ex*`, `Pr*`, `Alg*`) that have not been given a
  trait are exempt. These are standalone demonstrations, not reusable ADT modules.
- **Module-level functions** whose return type cannot be named in the trait signature
  (e.g., closures, complex generic returns) may remain as free functions outside the trait.
  These should be rare.
- **`&mut`-returning methods** like `iter_mut()` cannot be verified by Verus and must
  remain outside `verus!`, so they belong in a bare `impl Type` block.

## What This Means for the Bare Impl Detector

A `bare_impl` report from `veracity-review-proof-holes` means functions are on a direct
`impl Type` block instead of `impl Trait for Type`. This is usually a violation, but check
whether the bare impl contains recursive spec fns or `&mut` methods before moving them.

To fix a genuine violation:
1. Adding the missing functions to the trait definition.
2. Moving the function bodies from `impl Type` into `impl Trait for Type`.
3. Removing the bare `impl Type` block.

---

## Rule: .cursor/rules/apas-verus/use-hf-scheduler.mdc

---
description: Use the project help-first scheduler, not raw thread::spawn
globs: "**/*.rs"
alwaysApply: false
---

# Use Help-First Scheduler

For parallel scheduling, use the project's bounded help-first scheduler:

```
src/Chap02/HFSchedulerMtEph.rs
```

Do not use raw `std::thread::spawn` or `vstd::thread::spawn` for fork-join parallelism. Use the help-first scheduler which provides proper task management with bounded parallelism and deadlock prevention.

---

## Rule: .cursor/rules/apas-verus/validate.mdc

---
description: Validation commands for Verus verification
globs: 
alwaysApply: true
---

# Validation Modes

## validate (default) — full verification

When the user says "validate", run the full verification with all modules:

```bash
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors
```

Or: `scripts/validate.sh`

## dev_only_validate — foundation modules only

When the user says "dev_only_validate" or "dov", run with the dev_only feature.
This skips all chapters and verifies only foundation (Types, Concurrency, vstdplus):

```bash
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors --cfg 'feature="dev_only"'
```

Or: `python3 scripts/dev_only_validate.py`

To include specific chapters in dev_only mode, edit `src/lib.rs` and change their cfg from:
```rust
#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
```
to:
```rust
#[cfg(not(feature = "experiments_only"))]
```

Remember to include transitive dependencies (see dependency table in lib.rs comments).

## V1 — validate once

When the user says "V1", run a single Verus verification. If working with `experiments_only`:

```bash
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors --cfg 'feature="experiments_only"'
```

Or: `scripts/validate.sh exp`

Otherwise, same as validate. Do not iterate or fix errors. Just run once, show the full output, and stop.

---

## Rule: .cursor/rules/apas-verus/verusification-table.mdc

# Verusification Status Table

When the user asks for a "verusification table" or "verusification status" for a chapter, produce a structured review with the following sections.

## 1. Prose Coverage

Map each prose section/algorithm to implementation files. Flag missing algorithms.

| # | Prose Section | Description | File(s) | Status |
|---|---|---|---|---|
| 1 | §N Algorithm Name | Brief description | `File.rs` | Implemented / Missing |

## 2. Per-File Verusification Status

For each source file, report structural verusification:

| # | File | verus! block | Spec fns | Trait specs | Impl bodies | Proof holes |
|---|---|---|---|---|---|---|
| 1 | `File.rs` | Yes/No | count | count with ensures | verified / external_body | count × type |

## 3. Spec Strength Assessment

Classify every public function's spec strength and proof hole status:

| # | Function | File | Spec | Strength | Holes | Notes |
|---|---|---|---|---|---|---|
| 1 | `fn_name` | File.rs | Brief spec summary | strong/partial/weak/none | verified / external_body / assume(N) / admit | Gap description |

The **Holes** column reports the verification status of the function body:
- **verified** — body fully verified, no holes
- **external_body** — body not checked by Verus
- **assume(N)** — N `assume(...)` statements in body
- **admit** — contains `admit()`
- A function may have multiple hole types (e.g., `external_body` on one arm, `assume` on another)

Use the standard classification from `classify-spec-strengths.mdc`.

## 4. Summary

| Metric | Value |
|---|---|
| Strong specs | N |
| Partial specs | N |
| Weak / None specs | N |
| Fully verified bodies | N |
| external_body holes | N |
| assume holes | N |
| Prose algorithms missing | N |

## When to Produce

- When the user says "verusification table", "verusification status", or "verus status"
- At the start of work on a new chapter (combined with review-against-prose)
- Before claiming a chapter is complete

---

## Rule: .cursor/rules/apas-verus/when-to-write-ptts.mdc

---
description: When proof-time tests are needed vs not
globs: "**/*.rs"
alwaysApply: false
---

# When to Write Proof-Time Tests

PTTs (`rust_verify_test/tests/`) are expensive to write and maintain. Only
create them when there is a real risk that specs are wrong and a downstream
module will suffer.

## Write a PTT when

1. **The module exposes iterators.** Iteration is the most fragile Verus proof
   pattern. A PTT should exercise every supported loop form (loop-match,
   for-iter, for-consuming) to catch invariant regressions.
2. **The module is a building block for later chapters.** If Chap18 depends on
   Chap05's SetStEph, a PTT on SetStEph catches spec errors before they
   propagate. Leaf modules used by no one else are lower priority.
3. **The spec is non-obvious or has been wrong before.** If a function's
   `ensures` was corrected during review, a PTT locks in the fix.

## Do NOT write a PTT when

- The module is self-contained and not imported by other modules.
- The module is purely algorithmic with no iteration (e.g., recursive Fibonacci).
- The module's specs are trivial (accessors, constructors, delegating calls).
- A runtime test already exercises the same code path — RTTs catch exec bugs,
  PTTs catch spec bugs. If the spec is simple and correct, an RTT suffices.

## Priority order

1. Iterable types used by other chapters (highest risk).
2. ADT modules with complex specs imported by algorithm chapters.
3. Everything else (skip unless a specific concern arises).

---

## Rule: .cursor/rules/apas-verus/xlit-macros.mdc

---
description: Use XLit! macros (SetLit!, RelationLit!, etc.) to construct test values
globs: "**/*.rs"
alwaysApply: false
---

# Use XLit! Macros for Constructing Values

In APAS-VERUS, all data types have a literal macro `XLit!` which should be used to construct values of that type in runtime and proof time testing.

## Available Macros

| # | Data Type | Macro | Example |
|---|-----------|-------|---------|
| 1 | SetStEph | `SetLit!` | `SetLit![1, 2, 3]` |
| 2 | RelationStEph | `RelationLit!` | `RelationLit![(1, 'a'), (2, 'b')]` |
| 3 | MappingStEph | `MappingLit!` | `MappingLit![(1, 'a'), (2, 'b')]` |
| 4 | Pair | `PairLit!` | `PairLit!(1, 'a')` |

## Imports

When using macros, import them from `apas_verus`:

```rust
use apas_verus::{SetLit, RelationLit, MappingLit, PairLit};
```

For `RelationLit!` and `MappingLit!`, also import `SetStEphTrait`:

```rust
use apas_verus::Chap05::SetStEph::SetStEph::*;
```

## Runtime Safety

`MappingLit!` includes a duplicate-key check that panics at runtime if the same key appears twice. This check is skipped in Verus proof mode (the `from_vec` precondition handles verification).

---

## Rule: .cursor/rules/git/create-worktree.mdc

# Create Agent Worktree

## Naming convention

| Worktree | Directory | Branch prefix |
|---|---|---|
| main | `~/projects/APAS-VERUS` | `main` |
| agent1 | `~/projects/APAS-VERUS-agent1` | `agent1/` |
| agent2 | `~/projects/APAS-VERUS-agent2` | `agent2/` |
| agent3 | `~/projects/APAS-VERUS-agent3` | `agent3/` |

Branch names follow the pattern `agentN/<topic>` (e.g., `agent1/verusify-chap37`).

## Creating a worktree

```bash
# From the main worktree
cd ~/projects/APAS-VERUS

# Create agent1 worktree with a topic branch
git worktree add -b agent1/<topic> ~/projects/APAS-VERUS-agent1 main

# Create agent2 worktree with a topic branch
git worktree add -b agent2/<topic> ~/projects/APAS-VERUS-agent2 main

# Create agent3 worktree with a topic branch
git worktree add -b agent3/<topic> ~/projects/APAS-VERUS-agent3 main
```

## Switching an agent's topic

When an agent finishes a topic and starts a new one, rename the branch — don't recreate the worktree:

```bash
# In the agent worktree (e.g., ~/projects/APAS-VERUS-agent1)
git checkout main
git pull origin main
git checkout -b agent1/<new-topic>
```

## Removing a worktree

```bash
cd ~/projects/APAS-VERUS
git worktree remove ~/projects/APAS-VERUS-agent1
git branch -d agent1/<topic>

# Similarly for agent2, agent3:
# git worktree remove ~/projects/APAS-VERUS-agent2
# git worktree remove ~/projects/APAS-VERUS-agent3
```

## Rules

- Each agent works ONLY in its own worktree directory. Never `cd` into another agent's worktree.
- Main is the integrator. Agents do not push to main directly.
- Agents push their own branches: `git push origin agent1/<topic>`.
- Main merges agent branches one at a time (see merge-worktree rule).

---

## Rule: .cursor/rules/git/merge-worktree.mdc

# Merge Worktree

## When to use

When merging agent worktree branches into main. Execute phases 0–5 in strict order.
Do NOT skip phases. Do NOT reorder phases.

## Agent worktree layout

| Worktree | Directory | Branch |
|---|---|---|
| main | `~/projects/APAS-VERUS` | `main` |
| agent1 | `~/projects/APAS-VERUS-agent1` | `agent1/ready` |
| agent2 | `~/projects/APAS-VERUS-agent2` | `agent2/ready` |
| agent3 | `~/projects/APAS-VERUS-agent3` | `agent3/ready` |

## Phase 0: Ensure all worktrees are committed and pushed

All branches must be clean and pushed before merging. Do not proceed if any
worktree has uncommitted changes or unpushed commits.

```bash
set -e
# 1. Check main
cd ~/projects/APAS-VERUS && git status
cd ~/projects/APAS-VERUS && git log origin/main..HEAD --oneline

# 2. Check agent1
cd ~/projects/APAS-VERUS-agent1 && git status
cd ~/projects/APAS-VERUS-agent1 && git log origin/agent1/ready..HEAD --oneline

# 3. Check agent2
cd ~/projects/APAS-VERUS-agent2 && git status
cd ~/projects/APAS-VERUS-agent2 && git log origin/agent2/ready..HEAD --oneline

# 4. Check agent3
cd ~/projects/APAS-VERUS-agent3 && git status
cd ~/projects/APAS-VERUS-agent3 && git log origin/agent3/ready..HEAD --oneline
```

- `git status` must show "nothing to commit, working tree clean" in each worktree.
- `git log origin/<branch>..HEAD --oneline` must show no output (no unpushed commits).

STOP. Do not proceed to Phase 1 until all four worktrees are committed and pushed.

## Phase 1: Merge agent1/ready into main

All commands run in `~/projects/APAS-VERUS`.

```bash
set -e
# 1. Merge
cd ~/projects/APAS-VERUS && git merge agent1/ready

# 2. If conflicts: fix them, then
cd ~/projects/APAS-VERUS && git add -A && GIT_EDITOR=true git merge --continue

# 3. Verify: validate (full)
cd ~/projects/APAS-VERUS && ./scripts/validate.sh

# 4. Verify: validate (dev_only)
cd ~/projects/APAS-VERUS && ./scripts/validate.sh dev_only

# 5. Verify: RTTs
cd ~/projects/APAS-VERUS && ./scripts/rtt.sh

# 6. Verify: PTTs (compile lib + run proof time tests)
cd ~/projects/APAS-VERUS && ./scripts/ptt.sh

# 7. Push main
cd ~/projects/APAS-VERUS && git push
```

STOP. Do not proceed to Phase 2 until Phase 1 step 7 succeeds.

## Phase 2: Rebase agent2 onto post-agent1 main

Main now contains agent1's changes. Agent2 must rebase onto this main
BEFORE being merged. This ensures agent2's branch includes agent1's changes
so the Phase 3 merge is clean.

No verification needed — main was just verified in Phase 1.

```bash
set -e
# 9. Rebase agent2 (picks up agent1's changes)
cd ~/projects/APAS-VERUS-agent2 && git fetch origin && git rebase origin/main

# 10. Force-push agent2
cd ~/projects/APAS-VERUS-agent2 && git push origin agent2/ready --force
```

STOP. Do not proceed to Phase 3 until agent2 is rebased and force-pushed.

## Phase 3: Merge agent2/ready into main

Agent2 now sits on top of the post-agent1 main, so this merge only brings in
agent2's own changes. All commands run in `~/projects/APAS-VERUS`.

```bash
set -e
# 11. Merge
cd ~/projects/APAS-VERUS && git merge agent2/ready

# 12. If conflicts: fix them, then
cd ~/projects/APAS-VERUS && git add -A && GIT_EDITOR=true git merge --continue

# 13. Verify: validate (full)
cd ~/projects/APAS-VERUS && ./scripts/validate.sh

# 14. Verify: validate (dev_only)
cd ~/projects/APAS-VERUS && ./scripts/validate.sh dev_only

# 15. Verify: RTTs
cd ~/projects/APAS-VERUS && ./scripts/rtt.sh

# 16. Verify: PTTs (compile lib + run proof time tests)
cd ~/projects/APAS-VERUS && ./scripts/ptt.sh

# 17. Push main
cd ~/projects/APAS-VERUS && git push
```

STOP. Do not proceed to Phase 2.5 until Phase 3 step 17 succeeds.

## Phase 2.5: Rebase agent3 onto post-agent2 main

Agent3 must rebase onto main (which now contains agent1 and agent2) before merge.

```bash
set -e
# 18. Rebase agent3
cd ~/projects/APAS-VERUS-agent3 && git fetch origin && git rebase origin/main

# 19. Force-push agent3
cd ~/projects/APAS-VERUS-agent3 && git push origin agent3/ready --force
```

STOP. Do not proceed to Phase 3.5 until Phase 2.5 succeeds.

## Phase 3.5: Merge agent3/ready into main

```bash
set -e
# 20. Merge
cd ~/projects/APAS-VERUS && git merge agent3/ready

# 21. If conflicts: fix them, then
cd ~/projects/APAS-VERUS && git add -A && GIT_EDITOR=true git merge --continue

# 22. Verify: validate (full)
cd ~/projects/APAS-VERUS && ./scripts/validate.sh

# 23. Verify: validate (dev_only)
cd ~/projects/APAS-VERUS && ./scripts/validate.sh dev_only

# 24. Verify: RTTs
cd ~/projects/APAS-VERUS && ./scripts/rtt.sh

# 25. Verify: PTTs
cd ~/projects/APAS-VERUS && ./scripts/ptt.sh

# 26. Push main
cd ~/projects/APAS-VERUS && git push
```

STOP. Do not proceed to Phase 4 until Phase 3.5 step 26 succeeds.

## Phase 4: Rebase all agents onto final main

All agents must end on the same commit as main.

```bash
set -e
# 27. Rebase agent1
cd ~/projects/APAS-VERUS-agent1 && git fetch origin && git rebase origin/main

# 28. Force-push agent1
cd ~/projects/APAS-VERUS-agent1 && git push origin agent1/ready --force

# 29. Rebase agent2
cd ~/projects/APAS-VERUS-agent2 && git fetch origin && git rebase origin/main

# 30. Force-push agent2
cd ~/projects/APAS-VERUS-agent2 && git push origin agent2/ready --force

# 31. Rebase agent3
cd ~/projects/APAS-VERUS-agent3 && git fetch origin && git rebase origin/main

# 32. Force-push agent3
cd ~/projects/APAS-VERUS-agent3 && git push origin agent3/ready --force
```

STOP. Do not proceed to Phase 5 until Phase 4 step 32 succeeds.

## Phase 5: Verify completion

All four must show the same commit hash:

```bash
set -e
# 33. main
cd ~/projects/APAS-VERUS && echo "main: $(git log --oneline -1)"

# 34. agent1
cd ~/projects/APAS-VERUS-agent1 && echo "agent1/ready: $(git log --oneline -1)"

# 35. agent2
cd ~/projects/APAS-VERUS-agent2 && echo "agent2/ready: $(git log --oneline -1)"

# 36. agent3
cd ~/projects/APAS-VERUS-agent3 && echo "agent3/ready: $(git log --oneline -1)"
```

## Notes

- Execute phases 0, 1, 2, 2.5, 3, 3.5, 4, 5 in strict order. Never skip a phase.
- Each phase uses `set -e`: stop immediately if any command fails. Fix the failure before proceeding.
- Phase 2 is critical: agent2 must rebase onto post-agent1 main BEFORE Phase 3.
- Phase 2.5 is critical: agent3 must rebase onto post-agent2 main BEFORE Phase 3.5.
- All verification uses project scripts: `validate.sh`, `rtt.sh`, `ptt.sh`. These get paths and flags right.
- Order: validate (full), validate (dev_only), RTTs, PTTs. The `ptt.sh` script compiles the PTT lib then runs proof time tests.
- Agent worktrees are only used for rebase + force-push. No verification there.
- Use `GIT_EDITOR=true` for merge/rebase --continue to avoid the "dumb terminal" error.
- Fix conflicts before verifying.
- If a rebase has conflicts: fix, then `git add -A && GIT_EDITOR=true git rebase --continue`.

---

## Rule: .cursor/rules/interaction/ask-before-commit.mdc

---
description: Always ask for confirmation before git commit; use git add -A
globs: 
alwaysApply: true
---

# Ask Before Commit

**Always** ask the user for confirmation before running `git commit`.

Before committing:
1. Show what files changed (`git diff --stat` or similar)
2. Show the proposed commit message
3. Ask: "Ready to commit?"

Only proceed with the commit after explicit user approval.

## A commit is `git add -A`

When committing, **always** use `git add -A` to stage everything on disk.
Never selectively stage individual files with `git add <file>`.
The committed state must exactly match the on-disk state that was validated.
If there are files on disk you don't want to commit, discuss with the user first.

---

## Rule: .cursor/rules/interaction/ask-before-push.mdc

---
description: Always ask for confirmation before git push
globs: 
alwaysApply: true
---

# Ask Before Push

**Always** ask the user for confirmation before running `git push`.

Before pushing:
1. Show what commits will be pushed (`git log origin/main..HEAD --oneline` or similar)
2. Ask: "Ready to push?"

Only proceed with the push after explicit user approval.

---

## Rule: .cursor/rules/interaction/copy-icons.mdc

---
description: Make paths and URLs easy to copy
globs: 
alwaysApply: true
---

# Copyable Paths and URLs

## In tables and inline text

Wrap file paths and URLs in backticks for easy selection:

| # | Type | Link |
|---|------|------|
| 1 | URL | `https://example.com/path` |
| 2 | File | `/home/user/project/file.rs` |

## When a single path or URL needs to be easily copied

Use a fenced code block — this is the **only** element in Cursor chat that renders with a real copy button:

```
/home/user/project/file.rs
```

Use this when the user is likely to need to copy the value (e.g., a command to run, a single important path).

## Do NOT use emoji copy icons

The 📋 emoji has no copy functionality — it is purely decorative and adds clutter. Do not use it.

---

## Rule: .cursor/rules/interaction/discuss.mdc

---
description: When user says "discuss", only chat - do not modify files
globs: 
alwaysApply: true
---

# Discuss Means Chat Only

When the user says "discuss" (or any variation like "let's discuss", "discuss this", etc.), this means:

- **DO**: Chat about the issue, explain options, analyze tradeoffs, ask clarifying questions
- **DO NOT**: Make any changes to files on disk
- **DO NOT**: Use write, search_replace, edit_notebook, or delete_file tools

Only proceed with code changes after the user explicitly approves or requests implementation.

---

## Rule: .cursor/rules/interaction/dot.mdc

---
description: DOT = Don't Over Think. Focus on the simple task the user asked for.
globs: 
alwaysApply: true
---

# DOT — Don't Over Think

When the user says "DOT" or the task is straightforward:

- **DO**: Execute exactly what was asked, nothing more.
- **DO**: Take the simplest path to the goal.
- **DO**: Make the minimal change that satisfies the request.

- **DO NOT**: Anticipate future problems that weren't asked about.
- **DO NOT**: Refactor, restructure, or "improve" beyond the request.
- **DO NOT**: Explore alternatives when the user gave a clear direction.
- **DO NOT**: Add defensive code, extra assertions, or commentary beyond what's needed.

If the user said to do X, do X. Not X plus Y plus Z.

---

## Rule: .cursor/rules/interaction/execute-relentlessly-afk.mdc

---
description: When user says AFK, execute the full plan without stopping to ask
globs: 
alwaysApply: true
---

# Execute Relentlessly AFK

When the user says "execute relentlessly" or "AFK" (away from keyboard), this means:

- **DO**: Execute the current plan/todos without stopping
- **DO**: Make all necessary file changes
- **DO**: Run verification and tests
- **DO**: Fix errors as they arise
- **DO**: Continue through the entire todo list
- **DO**: Commit and push when complete (if requested)

- **DO NOT**: Stop to ask for confirmation
- **DO NOT**: Wait for user approval between steps
- **DO NOT**: Ask "should I continue?"
- **DO NOT**: Pause to discuss alternatives

The user trusts the AI to execute the plan. If something truly breaks and cannot be fixed after reasonable attempts, leave the code in a working state and document what failed.

Report results at the end, not during execution.

---

## Rule: .cursor/rules/interaction/full-paths.mdc

---
description: Show full on-disk paths when user asks
globs: 
alwaysApply: true
---

# Full Paths

When the user asks for "full paths" or "full path", show the complete on-disk path to all referenced files.

For example:
- Instead of `src/Chap05/SetStEph.rs`
- Show `/home/milnes/projects/APAS-VERUS/src/Chap05/SetStEph.rs`

---

## Rule: .cursor/rules/interaction/gate-review-against-prose.mdc

---
description: Gate review-against-prose on changed inputs
alwaysApply: true
---

# Gate Review-Against-Prose on Changed Inputs

Before regenerating `review-against-prose.md` for any chapter, check whether any inputs are newer than the existing review:

```bash
find src/ChapNN/*.rs prompts/ChapNN.txt \
     src/ChapNN/analyses/veracity-review-verus-proof-holes.log \
     tests/test_ChapNN*.rs \
     -newer src/ChapNN/analyses/review-against-prose.md 2>/dev/null | head -1
```

Replace `ChapNN` with the actual chapter (e.g., `Chap56`).

## Decision

- **Empty output** → review is up to date. Skip it.
- **Any output** → at least one input changed. Regenerate.

## Inputs that matter

| Input | Why |
|-------|-----|
| `src/ChapNN/*.rs` | Source code changed |
| `prompts/ChapNN.txt` | Prose text changed |
| `src/ChapNN/analyses/veracity-review-verus-proof-holes.log` | Proof holes changed |
| `tests/test_ChapNN*.rs` | Runtime tests changed |

## When regenerating

Only update sections affected by the changed files. For example, if only the proof holes log changed, update the Proof Holes Summary and Action Items — don't rewrite Phase 2 (Prose Inventory).

---

## Rule: .cursor/rules/interaction/implement.mdc

---
description: When user says "implement", proceed with code changes
globs: 
alwaysApply: true
---

# Implement Means Make Changes

When the user says "implement" (or any variation like "implement it", "implement this", "go ahead and implement", etc.), this means:

- **DO**: Make the discussed/sketched changes to files on disk
- **DO**: Use write, search_replace, or other file modification tools
- **DO**: Verify the changes compile/verify after implementation
- **DO**: Keep changes focused on the discussed solution

This is the explicit approval to proceed with file modifications after a discussion or sketch.

---

## Rule: .cursor/rules/interaction/indexed-tables.mdc

---
description: Index all tables and suggestions for easy reference
globs: 
alwaysApply: true
---

# Indexed Tables and Suggestions

All tables and suggestion lists must include a numeric index in column zero (or as a numbered list) so the user can easily refer to specific items.

## Tables

Always include an `#` or `Index` column as the first column:

| # | Loop Type | Provable? |
|---|-----------|-----------|
| 1 | loop-match | Yes |
| 2 | while | No |
| 3 | for-in | No |

## Suggestion Lists

Use numbered lists instead of bullets:

1. First suggestion
2. Second suggestion
3. Third suggestion

This allows the user to say "do #2" or "explain row 3" without ambiguity.

---

## Rule: .cursor/rules/interaction/no-python.mdc

---
description: No Python scripts for anything reusable
globs:
alwaysApply: true
---

# No Python

**NEVER write Python scripts** for anything that will be used more than once.

- All reusable tools, scripts, and utilities must be written in **Rust**.
- If a tool is needed, the USER will take the output and specification into another agent (on `~/projects/veracity`) and make that tool.
- The user will then come back and show you how to use that tool.
- You must have the user's **explicit permission** to write even a throwaway Python script.
- When in doubt, write Rust.

---

## Rule: .cursor/rules/interaction/plans-directory.mdc

---
description: Put plans, proposed work tables, and similar outputs in plans/
alwaysApply: true
---

# Plans Directory

When generating plans, proposed work tables, retables, or similar markdown outputs that the user may need to read or reference:

- **Always** write them to `plans/` on disk
- Use descriptive filenames: `proposed-work.md`, `fixes-table.md`, etc.
- Do not rely on chat-only output for tables; the user may not be able to read them

## Examples

- Proposed work table → `plans/proposed-work.md`
- Proposed fixes table → `plans/proposed-fixes.md`
- Other plan documents → `plans/<descriptive-name>.md`

---

## Rule: .cursor/rules/interaction/proposed-fixes-table.mdc

---
description: Generate a prioritized table of proposed fixes across chapters
alwaysApply: true
---

# Proposed Fixes Table

When the user says "proposed fixes table", "fixes table", or "audit my chapters", generate a
severity-ordered table of issues across the agent's assigned chapters.

## Procedure

1. Run proof holes on each chapter:
   ```bash
   ~/projects/veracity/target/release/veracity-review-proof-holes -d src/ChapNN/
   ```

2. Check for stale reviews (per `gate-review-against-prose` rule).

3. Scan for code issues:
   - `assume()` that isn't the standard PartialEq or thread-join pattern
   - `external_body` on functions with algorithmic logic (not threading/FFI wrappers)
   - `ensures true` or missing ensures on non-trivial functions
   - `#![auto]` triggers that should be explicit
   - `#[cfg(not(verus_keep_ghost))]` duplicate blocks (the old antipattern)
   - `bare_impl` not covered by the recursive-spec-fn exception

4. Classify severity:
   - **critical**: non-standard `assume`/`admit`, `external_body` hiding algorithmic logic
   - **high**: `ensures true`, stale reviews, cfg(not) antipattern blocks
   - **medium**: `#![auto]` triggers, bare_impl violations, missing cfg guards
   - **low**: informational (accepted patterns like PartialEq assume, thread-join assume(false))

5. Output a single table sorted by severity, then by chapter number:

   ```
   | # | Sev | Chapter | File | Issue | Description |
   |---|-----|---------|------|-------|-------------|
   ```

   Include a summary count at the bottom:

   ```
   | Severity | Count |
   |----------|-------|
   | critical | N |
   | high | N |
   | medium | N |
   ```

## Do NOT include

- Accepted patterns in the main table (list them separately as "informational" if at all)
- Issues outside the agent's assigned chapter range
- Bare impls that are valid exceptions (recursive spec fns with `decreases self`)

---

## Rule: .cursor/rules/interaction/rmf.mdc

---
description: RMF means read the file being worked on before doing anything
globs: 
alwaysApply: true
---

# RMF — Read My File

When the user says "RMF", read the file currently being worked on before taking any other action.

This ensures you have the latest on-disk state and don't operate on stale context or assumptions from earlier in the conversation.

---

## Rule: .cursor/rules/interaction/show-full-output.mdc

---
description: Always show complete terminal output in responses, never just summarize
globs: 
alwaysApply: true
---

# Show Full Command Output

## Rule

When running terminal commands (especially verus verification and cargo test):

- Always show the complete output in the response text
- Do not just summarize "it passed" or "verification succeeded"
- Paste the full verbatim output so the user can read it in the chat

## Rationale

The user cannot easily read terminal popups due to vision limitations. All command output must be visible in the response text.

---

## Rule: .cursor/rules/interaction/show-module-filenames.mdc

---
description: Show file names (ChapNN, vstdplus/, experiments) when providing module info
alwaysApply: true
---

# Show Module Filenames

When providing information about modules — functions, summaries, tables, analyses — **always include the file path** so the user can locate it.

## Format

Use paths like:

- `src/ChapNN/ModuleName.rs` — chapter modules
- `src/vstdplus/module.rs` — vstdplus
- `src/experiments/module.rs` — experiments

## When to Apply

- Summarizing what a module does
- Listing functions, types, or proof holes
- Describing changes or fixes
- Tables of modules, errors, or status

## Example

**Bad:** "The predicate is in BstParaWf."

**Good:** "The predicate is in `src/Chap38/BSTParaStEph.rs` (BstParaWf)."

---

## Rule: .cursor/rules/interaction/show-thinking.mdc

---
description: Show AI reasoning in response text
globs: 
alwaysApply: true
---

# Show Thinking

Always include your reasoning and analysis directly in your response text, not just in the collapsed thinking bubble.

Before taking action, explain:
- What you understand the task to be
- Your approach/strategy
- Key decisions and why

Format as a brief "**Reasoning:**" section at the start of responses when making non-trivial decisions.

---

## Rule: .cursor/rules/interaction/sketch.mdc

---
description: When user says "sketch", show proposed code in a code block - do not modify files
globs: 
alwaysApply: true
---

# Sketch Means Propose in Code Block

When the user says "sketch" (or any variation like "sketch out", "sketch this", etc.), this means:

- **DO**: Show proposed code/proof in a markdown code block
- **DO**: Explain the approach and key insights
- **DO**: Wait for feedback before implementing
- **DO NOT**: Make any changes to files on disk
- **DO NOT**: Use write, search_replace, edit_notebook, or delete_file tools

A sketch is a proposal for discussion. Only proceed with file modifications after the user explicitly approves or requests implementation.

---

## Rule: .cursor/rules/interaction/step-n.mdc

---
description: STEP n limits iterations before stopping
globs: 
alwaysApply: true
---

# STEP n

When the user says "STEP n" (e.g., "STEP 3"), this means:

- **DO**: Perform at most n iterations of file writing / verus verification
- **DO**: Stop after n iterations, even if problems remain
- **DO**: Leave the code in its current state with any errors/issues visible
- **DO NOT**: Continue fixing beyond n iterations

This allows the user to review intermediate states and understand what's happening.

Example: "STEP 2" means make at most 2 edit/verify cycles, then stop and show where things stand.

---

## Rule: .cursor/rules/interaction/table-cell-length.mdc

---
description: Limit markdown table cell length to stay readable in Cursor chat
alwaysApply: true
---

# Table Cell Length Limit

Every cell in a markdown table must be **at most 40 characters**. If a cell would exceed 40 characters, shorten it:

1. Abbreviate: "Remove external\_body from OrderedSetStEph delegation methods" → "Remove ext\_body OrderedSetStEph"
2. Drop redundant words: "New verification coverage" → "New coverage"
3. Move detail to a footnote or a row below the table.

## Self-check

Before emitting any markdown table, mentally scan every cell. If any cell exceeds 40 characters, rewrite it shorter. This applies to all columns including the last one.

## Bad

```
| Fix external_body from OrderedSetStEph delegation methods | 1 file | Enables future OrderedTable verification |
```

## Good

```
| Remove ext_body OrderedSetStEph | 1 | Enables OrderedTable fixes |
```

---

## Rule: .cursor/rules/interaction/tables-in-cursor.mdc

---
description: Present tables in Cursor chat by default; write md file only when asked
alwaysApply: true
---

# Tables in Cursor

By default, **present tables in Cursor chat** (in the response text).

- **DO**: Render tables as markdown in the response so the user sees them directly.
- **DO NOT**: Write tables to `.md` files unless the user explicitly asks for an md file.

## When the user wants an md file

The user will say so, e.g.:
- "write that to a file"
- "save as md"
- "put it in analyses/"
- "create an md file"

Only then write the table (or full report) to disk.

---

## Rule: .cursor/rules/interaction/todos-need-approval.mdc

---
description: Display todos and wait for user approval before executing them
globs: 
alwaysApply: true
---

# Todos Need Approval

When creating or showing a todo list:

1. Display the todos to the user
2. **Wait** for user approval before executing any of them
3. Do not start working on todos until the user says "OK", "go", "execute", or similar

This gives the user a chance to review, modify, or reprioritize the plan.

---

## Rule: .cursor/rules/interaction/whats-next.mdc

# WN — What's Next

When the user says "WN" or "What's Next":

1. Show the current TODO list status (if any todos exist)
2. List open work items from the project TODO file
3. Suggest the most productive next step based on recent work
4. Wait for the user to choose before starting

---

## Rule: .cursor/rules/interaction/wide-markdown.mdc

---
description: 
globs: 
alwaysApply: true
---
# Wide Markdown

When creating markdown files (`.md`), include this `<style>` block at the top so tables and content fill the browser window:

```html
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>
```

This overrides GitHub and local markdown viewer width constraints.

---

## Rule: .cursor/rules/rust/senior-rust-engineer.mdc

---
description: Role - senior Rust engineer focused on clarity and pragmatic use of the type system
alwaysApply: true
---

# Role: Senior Rust Engineer

You are a senior software engineer who writes clean, clear Rust code optimized for readability.

## Type System Philosophy

- You have a deep understanding of Rust's type system — generics, associated types, trait bounds, lifetime elision, where clauses, GATs.
- You prefer **traits (weak type classes)** for abstraction and clarity, up to their limitations. Traits make interfaces explicit, discoverable, and composable.
- You recognize where traits fall short compared to a full module system — orphan rules, lack of multiple implementations for the same type, no higher-kinded types without workarounds — and you work within those boundaries rather than fighting them.
- When traits aren't enough, you reach for newtype wrappers, marker types, or module-level organization — not macro towers or unsafe backdoors.

## Code Clarity

- Code is read far more than it is written. Optimize for the reader.
- Prefer explicit types on public interfaces. Let inference work inside function bodies.
- Name things for what they mean, not what they are. `fn merge(left: Seq, right: Seq)` over `fn f(a: Seq, b: Seq)`.
- Short functions with clear contracts beat long functions with inline comments explaining what's happening.
- If a function needs a paragraph of comments, it needs to be broken up.

## Pragmatism

- Use the simplest construct that works. `Vec` before `SmallVec`. `String` before `Cow<str>`. Generics before trait objects, unless dynamic dispatch is actually needed.
- Don't abstract prematurely. Write the concrete version first; extract a trait when a second use case appears.
- Clippy is a colleague, not a boss. Follow its advice by default, but override with `#[allow(...)]` and a comment when clarity demands it.

---

## Rule: .cursor/rules/rust/source-layout.mdc

---
description: Rust project source and test layout
globs: ["**/*.rs"]
alwaysApply: true
---

# Rust Source Layout

- Source files go in `src/`
- Test files go in `tests/`
- Proof time tests go in `rust_verify_test/tests/`

---

## Rule: .cursor/rules/rust/threaded-test-timeout.mdc

---
description: Threaded tests need timeouts
globs: ["tests/**/*.rs", "rust_verify_test/tests/**/*.rs"]
alwaysApply: true
---

# Threaded Test Timeout

When creating or modifying a test that uses threads (`std::thread`, `spawn`, `Pool`, etc.):

- Wrap the test with a timeout to prevent hangs from deadlocks
- Use `timeout` command when running: `timeout 60 cargo nextest run --test test_name`
- Or use a runtime timeout inside the test itself

Threaded tests can deadlock. A timeout ensures CI doesn't hang forever.

---

## Rule: .cursor/rules/style/comment-style.mdc

---
description: Own-line comments must be full sentences; end-of-line fragments OK
globs: "**/*.rs"
alwaysApply: false
---

# Comment Style

Comments should follow these formatting rules:

## Full Sentences Required

These comment types must be full English sentences with correct punctuation (capital letter, ending period):

| Type | Example |
|------|---------|
| `///` doc comments | `/// Returns the size of the set.` |
| `//!` module docs | `//! This module implements ephemeral sets.` |
| `//` on own line | `// The domain must be finite for this to work.` |

## Fragments Allowed

Inline comments after code may be sentence fragments:

```rust
let x = foo();  // temporary for the swap
result.insert(a);  // already checked membership
```

## Exceptions

Structured annotations don't need sentence form:

```rust
/// APAS: Work Θ(|v|), Span Θ(1)
/// - requires: valid_key_type::<T>()
/// - ensures: result@.finite()
```

## Summary

- Own-line comments → full sentences
- End-of-line comments → fragments OK
- Structured metadata → special format OK

---

## Rule: .cursor/rules/style/may-vs-can.mdc

---
description: Use 'may' vs 'can' correctly in comments and documentation
globs: ["**/*.rs", "**/*.md"]
alwaysApply: false
---

# May vs Can

- **can** = ability (is it physically/technically possible?)
- **may** = permission or option (is it allowed/permitted?)

## Examples

| Wrong | Right | Why |
|-------|-------|-----|
| User can set this before use | User may set this before use | It's an option, not an ability question |
| You may call `len()` on a `Vec` | You can call `len()` on a `Vec` | It's about capability |
| This can be `None` | This may be `None` | It's a permitted state |
| Threads can be reused | Threads may be reused | It's about whether reuse is allowed |

## Quick Test

Ask: "Is this about whether something is *possible* or whether something is *permitted/optional*?"

- Possible → can
- Permitted/optional → may

---

## Rule: .cursor/rules/style/no-dividers-in-comments.mdc

---
description: No horizontal dividers in code comments
globs: "**/*.rs"
alwaysApply: true
---

# No Dividers in Comments

Do not use `---`, `===`, `───`, or any other horizontal rule/divider syntax in code comments. No ASCII dividers, no Unicode box-drawing dividers.

Section headers in comments are just words:

```rust
// BAD:
// ---
// === Section ===
// ── Section ──────────────────────────────────────────────

// GOOD:
// Section
```

---

## Rule: .cursor/rules/style/no-helper-names.mdc

---
description: Do not name functions helper, inner, do_it, or similar meaningless names
globs: "**/*.rs"
alwaysApply: false
---

# No "helper" Function Names

Do not name functions `helper`, `helper1`, `helper2`, `do_it`, `inner`, or use suffixes like `_impl`, or similar meaningless names.

## Bad Examples

```rust
fn helper<V>(arcs: Vec<Edge<V>>, v: V) -> SetStEph<V> { ... }
fn do_work(x: i32) -> i32 { ... }
fn inner(s: &str) -> bool { ... }
fn flatten_impl(ss: &Seq<Seq<T>>) -> Seq<T> { ... }  // all functions "implement" something
```

## Good Examples

```rust
fn filter_arcs_by_source<V>(arcs: Vec<Edge<V>>, v: V) -> SetStEph<V> { ... }
fn compute_checksum(x: i32) -> i32 { ... }
fn is_valid_identifier(s: &str) -> bool { ... }
```

## Rule

A function's name should describe what it does. Generic names like "helper" provide no information about purpose or behavior.

This applies to documentation too — do not describe a function as a "helper" in comments or doc strings. Say what it does, not that it "helps".

---

## Rule: .cursor/rules/style/no-jejune-comments.mdc

---
description: Do not add comments that merely restate what the function name already says
globs: "**/*.rs"
alwaysApply: false
---

# No Jejune Comments

Do not add comments that merely restate what is already clear from the function's name, parameters, and types.

## Bad Examples

```rust
/// Returns the size of the set.
fn size(&self) -> N { ... }

/// Checks if the element is in the set.
fn contains(&self, elem: &T) -> bool { ... }

/// Creates a new empty set.
fn empty() -> Self { ... }
```

These comments add no value—the function names already convey the meaning.

## Good Examples

```rust
/// APAS: Work Θ(|v|), Span Θ(1)
fn from_vec(v: Vec<T>) -> Self { ... }

/// Uses parallel divide-and-conquer; falls back to sequential for n < 1000.
fn parallel_filter(items: Vec<T>, pred: F) -> Vec<T> { ... }

/// The result excludes self-loops even if present in the input.
fn neighbors(&self, v: &V) -> Set<V> { ... }
```

These comments tell you something non-obvious: complexity, implementation strategy, or edge case behavior.

## Rule

A comment should pass this test: "Does this tell me something I couldn't infer from the signature alone?"

If not, omit it.

---

## Rule: .cursor/rules/style/no-trivial-wrappers.mdc

---
description: Do not create wrapper functions that merely forward to another function
globs: "**/*.rs"
alwaysApply: false
---

# No Trivial Wrappers

Do not create wrapper functions that merely forward to another function without adding value.

## Bad Example

```rust
fn n_plus(&self, v: &V) -> SetStEph<V> {
    parallel_n_plus(self.A.to_seq(), v.clone_plus())
}

fn n_minus(&self, v: &V) -> SetStEph<V> {
    parallel_n_minus(self.A.to_seq(), v.clone_plus())
}
```

If `parallel_n_plus` can be called directly, the wrapper adds nothing.

## When Wrappers Are Justified

Wrappers are acceptable when they:

1. **Add a spec/contract** that the underlying function lacks
2. **Adapt the interface** (different parameter order, types, or defaults)
3. **Hide implementation details** (the inner function is private/internal)
4. **Provide trait conformance** (implementing a trait method)

## Rule

Before creating `fn2` that calls `fn1`:
- Can callers just call `fn1` directly?
- Does `fn2` add specs, adapt types, or satisfy a trait?

If `fn2` adds nothing, eliminate it.

---

## Rule: .cursor/rules/style/senior-prose-engineer.mdc

---
description: Role - clear, grammatical, non-redundant prose in comments and docs
alwaysApply: true
---

# Role: Senior Software Engineer with an English Minor

You are a senior software engineer who also has a minor in English grammar. This shapes how you write comments and documentation.

## Prose Quality

- Write clear, non-repetitive prose. Every comment should earn its place.
- Do not restate what the types, function names, and signatures already say. The reader can see `fn size(&self) -> usize` — don't write `/// Returns the size.`
- Instead, comment on what is *not* obvious: algorithmic strategy, edge cases, why a particular approach was chosen, cost bounds, or invariants the reader needs to hold in mind.

## Grammar

- Own-line comments (`//`, `///`, `//!`) are full sentences: capital letter, proper punctuation, period at the end.
- End-of-line comments are sentence fragments — that's fine. They annotate, they don't narrate.

```rust
// The union requires both sets to be finite, since we iterate over one
// and insert into the other.
fn union(&self, other: &Self) -> Self {
    let mut result = self.clone();  // donor for the accumulation
    for elem in other.iter() {
        result.insert(elem);  // idempotent if already present
    }
    result
}
```

## Voice

- Active voice over passive. "The loop maintains the invariant" not "The invariant is maintained by the loop."
- Concrete over abstract. "Splits at the midpoint" not "Performs a division operation."
- Brief over verbose. If one sentence suffices, don't write two.

---

## Rule: .cursor/rules/style/use-imports-not-crate-paths.mdc

---
description: Prefer use imports over verbose crate:: paths
globs: "**/*.rs"
alwaysApply: true
---

# Use Imports, Not Verbose Crate Paths

When calling functions from other modules, prefer `use` imports over verbose `crate::...` paths.

## Anti-pattern

```rust
// Bad: verbose crate path repeated multiple times
crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(seq, i);
crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(seq, j);
```

## Correct Pattern

```rust
// Good: import the module, use short path
use crate::vstdplus::seq_set;

seq_set::lemma_seq_index_in_map_to_set(seq, i);
seq_set::lemma_seq_index_in_map_to_set(seq, j);
```

## Why

- Reduces visual noise
- Makes code more readable
- Easier to refactor if module paths change
- Follows Rust idioms

---

## Rule: .cursor/rules/style/use-statement-order.mdc

---
description: Order use statements as std, vstd, crate types, chapter modules, macros
globs: "**/*.rs"
alwaysApply: true
---

# Use Statement Order

All `use` statements should appear immediately after the module declaration, in this order:

## Order

1. `use std::...` - standard library imports
2. *(blank line)*
3. `use vstd::prelude::*;` - Verus prelude
4. `use crate::Types::Types::*;` - project types
5. `use crate::Chap05::...::*;` - chapter modules (glob import)
6. `use crate::XLit;` - macro imports (not glob)

## Example

```rust
pub mod MyModule {

    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;

    // ... rest of module
}
```

## Rules

- APAS modules always use a single glob import (`*`) - never `{*, SomeType}`
- Macros (like `SetLit`, `RelationLit`) are imported by name, not glob
- Keep a blank line between std imports and the rest

---

## Rule: .cursor/rules/verus/arc-deref-pattern.mdc

# Anti-Pattern: "Closures Can't Capture Data with Specs"

## The Wrong Assumption

When implementing parallel (Mt) algorithms in Verus, do NOT assume:

- "Closures can't capture `ArraySeqMtEphS<T>` with specs"
- "Arc<F> prevents verification because Verus can't deref it"
- "`Send + 'static` closures can't reference ghost variables like `spec_f`"
- "The only way to verify this is to make it sequential"

These are FALSE. The pattern below solves all of them.

## The Pattern: Factor Verification Away from Concurrency

### Problem

`join` closures need `Send + 'static`. The verified algorithm needs `f: &F` (for direct Fn trait spec access). `Arc<F>` is opaque to Verus — you can't deref it with spec guarantees.

### Solution

1. **Factor the verified algorithm** into a helper that takes `f: &F`:

   ```rust
   fn reduce_contract_verified<T, F>(a: &ArraySeqMtEphS<T>, f: &F, ...) -> (result: T)
       requires f.requires(...), ...
       ensures result == ...
       decreases a.spec_len(),
   { /* Full proof here — identical to StEph version */ }
   ```

2. **The trait impl delegates** through Arc deref:

   ```rust
   fn reduce_contract_parallel(a, f: Arc<F>, ...) -> (result: T) {
       reduce_contract_verified(a, &*f, Ghost(spec_f), id)
   }
   ```

3. **Small external_body helpers** for Arc-specific operations:

   ```rust
   #[verifier::external_body]
   pub fn call_f<T, F>(f: &Arc<F>, a: &T, b: &T) -> (result: T)
       requires f.requires((a, b)),
       ensures f.ensures((a, b), result),
   { (**f)(a, b) }
   ```

4. **Parallel primitives** as external_body with strong specs:

   ```rust
   #[verifier::external_body]
   pub fn contract_parallel<T, F>(a, f: &Arc<F>, Ghost(spec_f), half) -> (b: ArraySeqMtEphS<T>)
       ensures b.spec_index(j) == spec_f(a.spec_index(2*j), a.spec_index(2*j+1)),
   { /* Uses join internally */ }
   ```

### Trust Boundaries

| Helper | Trust | Obviousness |
|---|---|---|
| `call_f` | Arc deref preserves function | Trivially correct |
| `contract_parallel` | Parallel loop computes pairs | Auditable in 10 lines |

The algorithm proof (base cases, recursion, contraction lemma, expansion) is **fully verified**.

## Key Insights

- `&*f` converts `Arc<F>` → `&F` at the call site — Verus handles this
- `f.requires(...)` auto-derefs through `&Arc<F>` in Verus specs
- Ghost variables like `spec_f` are accessible in external_body function specs via `Ghost(spec_f)` parameters
- The recursive helper takes `f: &Arc<F>` (reference, not owned), so no Arc cloning needed for recursion
- Parallel helpers clone the Arc internally (outside verification) for fork-join closures

## When You Think Mt Is Impossible

Before concluding that an Mt algorithm can't be verified:

1. Write the StEph (sequential) verified version first
2. Factor the proof into a helper taking `&F` or `&Arc<F>`
3. Add small external_body helpers for Arc operations
4. Replace the sequential contraction/expansion with parallel helpers
5. The proof transfers with minimal changes

---

## Rule: .cursor/rules/verus/assert-no-proof-block.mdc

---
description: Assert statements in Verus exec code do not need proof blocks
globs: ["**/*.rs"]
alwaysApply: true
---

In Verus, `assert` statements inside executable (`fn`) code do **not** need to be wrapped in a `proof { }` block. Only proof-mode calls (like lemma invocations, `reveal`, ghost variable bindings with `let ghost`, etc.) require a `proof { }` block. Plain `assert(...)` can appear directly in exec code.

---

## Rule: .cursor/rules/verus/assume-false-diverge.mdc

---
description: assume(false); diverge() is valid idiom for unreachable thread join errors
globs: "**/*.rs"
alwaysApply: true
---

# assume(false); diverge() in Thread Join

The pattern `assume(false); diverge()` in a `JoinHandle::join()` error arm
is a **valid and unavoidable Verus idiom**, not a proof hole to fix.

## Why it exists

`thread::join()` returns `Result<T, Box<dyn Any>>` where `Err` means the
spawned thread panicked. Verus cannot prove that a thread will not panic —
that would require verifying the absence of all panics in the spawned
closure's entire transitive call graph, which is beyond Verus's current
scope.

## The pattern

```rust
let result = match join_handle.join() {
    Result::Ok(out) => out,
    Result::Err(_) => { assume(false); diverge() }
};
```

`assume(false)` tells Verus "this branch is unreachable." `diverge()` is
a Verus built-in that satisfies any return type (it has postcondition
`ensures false`). Together they say: "if the thread panicked, we assume
that cannot happen."

## When to use

- In any `JoinHandle::join()` error arm.
- In any `Result::Err` arm from a thread operation that cannot fail if
  the spawned code is verified.

## When NOT to use

- Do not use `assume(false)` anywhere else without asking the user.
- Do not use it to suppress verification failures in non-threading code.

## Avoiding it

The `ParaPair!` macro encapsulates spawn/join and handles the error arms
internally, so callers never see the `assume(false)`. Prefer `ParaPair!`
over raw `spawn`/`join` when the fork-join structure is symmetric.

---

## Rule: .cursor/rules/verus/cargo-sync.mdc

---
description: Keep Cargo.toml in sync with file changes
globs: 
alwaysApply: true
---

# Cargo Sync

When adding, deleting, or renaming a file that is registered in Cargo.toml (e.g., test files, binaries, examples):

- **Add**: Add the corresponding `[[test]]`, `[[bin]]`, or `[[example]]` entry
- **Delete**: Remove the corresponding entry
- **Rename**: Update the `name` and `path` in the corresponding entry

Do not leave Cargo.toml out of sync with the file system.

---

## Rule: .cursor/rules/verus/classify-spec-strengths.mdc

---
description: Classify Verus spec strengths from review-module-fn-impls JSON
globs:
alwaysApply: true
---

# Classify Spec Strengths

## Generate

When the user says "review specs for", "generate spec review for", or provides directories/files
to analyze, first generate the markdown and JSON:

```bash
~/projects/veracity/target/release/veracity-review-module-fn-impls -d src/ChapNN
```

Or for specific files:

```bash
~/projects/veracity/target/release/veracity-review-module-fn-impls -f src/ChapNN/File.rs
```

Multiple directories can be combined in one run (`-d src/Chap05 -d src/Chap18`).

Output goes to `analyses/veracity-review-module-fn-impls.md` and `.json`.

## Classify

When the user says "classify spec strengths", "review spec strengths", "classify specs",
or similar:

### Inputs

- **JSON file**: `analyses/veracity-review-module-fn-impls.json` (or a path the user provides)
- **Prompt**: `~/projects/veracity/docs/veracity-classify-spec-strengths-prompt.md`

### Procedure

1. Read `~/projects/veracity/docs/veracity-classify-spec-strengths-prompt.md` to understand the classification criteria.
2. Read the JSON file the user specifies (default: `analyses/veracity-review-module-fn-impls.json` in the current project).
3. For each entry, examine the `snippet` field and classify `spec_strength` as one of:
   - **strong** — `requires` and `ensures` fully capture the function's contract.
   - **partial** — some spec exists but is incomplete or underspecified.
   - **weak** — spec exists but is trivially true or nearly useless.
   - **none** — no `requires`/`ensures` at all (empty snippet or signature only).
4. Write the classifications to `analyses/review-module-fn-impl-spec-strengths.json` in the same directory as the input JSON. Format:
   ```json
   [
     { "id": 1, "spec_strength": "strong" },
     { "id": 2, "spec_strength": "none" }
   ]
   ```
5. Run the patch command:
   ```bash
   veracity-review-module-fn-impls --patch \
     analyses/veracity-review-module-fn-impls.md \
     analyses/review-module-fn-impl-spec-strengths.json
   ```
6. Print a summary table:
   | Classification | Count |
   |---------------|-------|
   | strong | N |
   | partial | N |
   | weak | N |
   | none | N |

## Classification Guidelines

- A function with **no snippet** or a snippet that is just the signature with no `requires`/`ensures` → **none**.
- A function with `ensures true` or `ensures self == self` → **weak**.
- A function whose `ensures` mentions return value and key state but misses edge cases or doesn't constrain all parameters → **partial**.
- A function whose `requires` + `ensures` together fully describe the input/output contract (preconditions, postconditions, return value, relevant state changes) → **strong**.
- When in doubt between partial and strong, prefer **partial** — it's better to flag for human review than to miss a gap.

## Do NOT

- Do not modify the source `.rs` files.
- Do not modify the `.md` file by hand — always use `--patch`.
- Do not skip entries — every `id` in the input must appear in the output.

---

## Rule: .cursor/rules/verus/fix-warnings.mdc

---
description: All Verus and Rust warnings and errors must be fixed
globs: 
alwaysApply: true
---

# Fix All Warnings and Errors

All Verus and Rust warnings and errors must be fixed before considering work complete.

- Do not leave warnings in the codebase
- Do not leave errors unresolved
- Fix issues as they arise, not later

---

## Rule: .cursor/rules/verus/fork-join-inside-verus.mdc

---
description: Fork-join parallelism uses join() directly inside verus! with named closures. No external_body wrappers.
alwaysApply: true
---

# Fork-Join Inside verus!

In Mt files, **all fork-join parallelism lives inside `verus!`** using `join()` directly with named closures. Never create `external_body` wrappers around `join()` or `ParaPair!`.

## The Pattern

1. Build owned data for each branch (clone, split, subseq_copy).
2. Capture ghost views **before** the `move`.
3. Define named closures with explicit `ensures` referencing the ghost views.
4. Pass them to `join()`.

```rust
let (left, right) = split(input);

let ghost left_view = left@;
let ghost right_view = right@;

let f1 = move || -> (r: ResultType)
    ensures post(r, left_view)
{ recurse(&left) };

let f2 = move || -> (r: ResultType)
    ensures post(r, right_view)
{ recurse(&right) };

let (a, b) = join(f1, f2);
```

## Why This Works

- `join()` is already `external_body` with verified spec propagation — it requires `fa.requires(())` and ensures `fa.ensures((), result.0)`.
- Named closures with explicit `ensures` propagate specs through `join()`.
- Ghost variables are spec-level (always Copy) — they survive `move` captures.
- The closure body is verified at definition time with the full ambient context.
- Add `decreases` on the recursive function for termination through closures.

## Do NOT

- **Do NOT create `external_body` wrappers** around `join()` — it IS the verified fork-join primitive. Wrapping it adds a trust boundary for no reason.
- **Do NOT put fork-join code outside `verus!`** — code outside has no specs.
- **Do NOT use `Arc`** to share data between closures. Move owned copies directly.
- **Do NOT use inline closures** (`move || foo()`) — their ensures don't propagate. Always bind to a named variable with explicit `ensures`.

## Closure Patterns

| # | Pattern | Propagates? |
|---|---------|-------------|
| 1 | `let f = move \|\| -> (r: T) ensures P { body };` | Yes |
| 2 | `ParaPair!(f1, f2)` with named closures | Yes |
| 3 | `ParaPair!(foo, bar)` with direct fn refs | Yes |
| 4 | `ParaPair!(move \|\| foo(), move \|\| bar())` | **No** |
| 5 | Inline closure with ensures inside macro | **No** (parse error) |

---

## Rule: .cursor/rules/verus/ghost-param-sync.mdc

---
description: Keep Ghost parameter calls in sync with trait signatures
alwaysApply: true
---

# Ghost Parameter Sync

When a trait method signature includes a `Ghost(...)` parameter (e.g., `Ghost(spec_f): Ghost<spec_fn(T, T) -> T>`) and a call site in a later chapter omits it, **update the call site** to pass the Ghost argument.

## Detection

If compilation fails with "cannot find method" or argument-count mismatch on a trait method that exists, check whether the trait added a `Ghost(...)` parameter that the caller doesn't supply.

## Fix

Add the missing `Ghost(...)` argument at the call site. Example:

```rust
// ❌ Old call (before Ghost was added to trait)
<ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(a, &|x, y| x + y, 0)

// ✅ Updated call (matches current trait signature)
<ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(
    a,
    &|x, y| x + y,
    Ghost(|x: i32, y: i32| -> i32 { (x + y) as i32 }),
    0,
)
```

## Rule

- The trait signature is the source of truth.
- Never remove or skip the Ghost parameter in the trait to accommodate old callers.
- Always update the caller to match the trait.

---

## Rule: .cursor/rules/verus/if-you-think-verus-cant.mdc

---
description: When AI thinks Verus can't do something, verify with experiments
globs: 
alwaysApply: true
---

# If You Think Verus Can't Do X

When you believe Verus cannot do something (e.g., "Verus doesn't support X", "This pattern won't verify"):

1. **Ask the user** if they want you to search `src/experiments/` for an existing test of this capability

2. **If searching**: Look for existing experiment files that test this pattern
   ```bash
   # First, use veracity-search on the codebase
   veracity-search -C ~/projects/APAS-VERUS --no-vstd 'fn _ types X'
   
   # Then grep for specific patterns
   ls src/experiments/
   grep -r "pattern" src/experiments/
   ```

3. **If not found**: Propose a new experiment file at `src/experiments/X.rs` to prove or disprove the case

Do not assume Verus can't do something without evidence. Many "impossible" things turn out to be possible with the right approach.

---

## Rule: .cursor/rules/verus/leave-the-corpse.mdc

---
description: When user says "show me the corpse" or "leave the corpse", keep failing code visible
globs: 
alwaysApply: true
---

# Leave the Corpse

When the user says "show me the corpse" or "leave the corpse", this means:

- **DO**: Leave the failing code in place exactly as written
- **DO**: Let verification errors remain visible in the output
- **DO NOT**: Comment out the module in `lib.rs`
- **DO NOT**: Comment out the failing assertions or functions
- **DO NOT**: Add `#[verifier::external_body]` to skip verification
- **DO NOT**: Add `assume(...)` or `admit()` to force it through
- **DO NOT**: Revert or delete the failing code

The point is to preserve the failure as evidence. The user wants to see exactly what fails and why.

This is related to the `failed-experiments` rule — failed code is valuable documentation.

---

## Rule: .cursor/rules/verus/meaningful-return-names.mdc

---
description: Name return values and variables meaningfully, not generically
globs: "**/*.rs"
alwaysApply: false
---

# Meaningful Names

Name return values and variables meaningfully. Avoid generic names that convey no information about purpose.

## Bad Names

| # | Name | Why it's bad |
|---|------|-------------|
| 1 | `result` | Every function returns a result |
| 2 | `target` | Almost everything is a target of something |
| 3 | `value` / `val` | Almost everything is a value |
| 4 | `data` | Almost everything is data |
| 5 | `temp` / `tmp` | Says nothing about what it holds |
| 6 | `ret` | Just means "return" — as generic as `result` |

## Good Return Names

```rust
fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
fn size(&self) -> (count: N)
fn empty() -> (graph: Self)
fn mem(&self, x: &T) -> (contains: B)
```

## Bad Return Names

```rust
fn n_plus(&self, v: &V) -> (result: SetStEph<V>)
fn size(&self) -> (result: N)
fn empty() -> (result: Self)
fn mem(&self, x: &T) -> (result: B)
```

## Exception

Files in `src/experiments/` may use generic names - these are throwaway explorations.

## Why

1. Named returns appear in ensures clauses - meaningful names make specs readable
2. `ensures out_neighbors@ == self.spec_n_plus(v@)` is clearer than `ensures result@ == ...`
3. The name documents what the function returns without reading the body
4. Generic names in ghost variables make proof blocks harder to follow

---

## Rule: .cursor/rules/verus/no-cfg-not-verus-keep-ghost.mdc

---
description: No verus_keep_ghost in lib.rs or as duplicate function gates
alwaysApply: true
---

# No verus_keep_ghost Antipatterns

There must be **no** use of `verus_keep_ghost` in `lib.rs`. Period.

## Antipattern 1: Duplicate function implementations

**NEVER** create `#[cfg(not(verus_keep_ghost))]` duplicate implementations of functions that already exist inside `verus! {}` blocks.

This is an old antipattern from before the codebase was fully verusified. It duplicated every function — once inside `verus! {}` with specs, and once outside with `#[cfg(not(verus_keep_ghost))]` for cargo test compatibility.

### Why it's wrong

- The duplicate drifts out of sync with the verified version
- It doubles maintenance burden
- It's no longer needed — the verusified code compiles and runs under both Verus and cargo

### What to do instead

- Put the implementation inside `verus! {}` with its specs
- That's it. No duplicate needed.

## Antipattern 2: Nightly feature gates

**NEVER** add `#![cfg_attr(verus_keep_ghost, feature(...))]` to `lib.rs`.

Lines like `#![cfg_attr(verus_keep_ghost, feature(sized_hierarchy))]` are cargo-cult copies from vstd's own `vstd.rs`. vstd needs nightly features for deep Rust intrinsics work. User crates do not — Verus links vstd as a precompiled dependency.

## Antipattern 3: Module gating in lib.rs

**NEVER** use `verus_keep_ghost` or `not(verus_keep_ghost)` to conditionally compile module blocks in `lib.rs`. This includes:

- `#[cfg(all(..., not(verus_keep_ghost)))] pub mod ChapNN { ... }` — hiding a chapter from Verus
- Split declarations — two `pub mod ChapNN` blocks, one gated `verus_keep_ghost`, one `not(verus_keep_ghost)`, with different file lists

If a chapter doesn't verify, that's the project's current state (PBOGH). Every chapter gets one unconditional `pub mod ChapNN` block containing all its files. Verus reports errors on unverified code until it's fixed.

## If you see it

If you encounter any of these antipatterns, remove them:
- Duplicate function bodies: delete the `#[cfg(not(verus_keep_ghost))]` block.
- Nightly feature gates: delete the `#![cfg_attr(verus_keep_ghost, ...)]` line.
- Module gating: merge split `pub mod` blocks into one unconditional block with all files.

---

## Rule: .cursor/rules/verus/no-modify-verus.mdc

---
description: Never modify files in ~/projects/verus/
globs: 
alwaysApply: true
---

# Do Not Modify Verus Source

**Never** modify files in `~/projects/verus/`.

This includes:
- `~/projects/verus/source/vstd/` (the verified standard library)
- `~/projects/verus/source/rust_verify/`
- Any other files in the Verus repository

If a change to Verus/vstd seems necessary:
1. Document what change would be needed
2. Suggest filing an issue or PR upstream
3. Find a workaround within APAS-VERUS

The user maintains Verus separately and any modifications must go through their own process.

---

## Rule: .cursor/rules/verus/no-nested-functions.mdc

---
description: Do not define functions inside other functions; keep helpers at module level
globs: "**/*.rs"
alwaysApply: false
---

# No Nested Functions

Due to Verus proof limitations, do not use nested functions (functions defined inside other functions).

## Bad Example

```rust
fn n_plus(&self, v: &V) -> SetStEph<V> {
    fn recursive_filter(arcs: Vec<Edge<V>>, v: V) -> SetStEph<V> {
        // ...
    }
    recursive_filter(self.A.to_seq(), v.clone_plus())
}
```

## Good Example

```rust
fn parallel_n_plus<V>(arcs: Vec<Edge<V>>, v: V) -> SetStEph<V> {
    // recursive implementation
}

fn n_plus(&self, v: &V) -> SetStEph<V> {
    parallel_n_plus(self.A.to_seq(), v.clone_plus())
}
```

## Rule

Keep helper functions at module level, placed immediately before the function that calls them.

---

## Rule: .cursor/rules/verus/no-random-revert.mdc

---
description: Do not revert without asking; proof work requires human interaction
alwaysApply: true
---

# No Random Revert

This is coding **plus proving**. It requires more effort and human interaction than ordinary programming.

## Do not

- Revert to previous states without asking the user first
- Suggest reverting as a fix for small or fixable problems
- Treat verification failures as a signal to undo work

## Do

- Ask before reverting: "Should I revert these changes, or would you prefer to fix forward?"
- Treat reverts as a last resort, not a default response to failure
- Preserve the current state when stuck; report the failure and ask how to proceed

## Rationale

Proof engineering is iterative. A verification failure is data, not a reason to undo. Reverting discards evidence and may hide the root cause. The user may want to fix forward, debug the failure, or discuss the approach before any revert.

---

## Rule: .cursor/rules/verus/prefer-vstd.mdc

---
description: Strongly prefer using vstd functions, lemmas, and types over defining new ones
alwaysApply: true
---

# Prefer vstd Over New Definitions

**Strongly prefer** using existing functions, lemmas, spec functions, and types from `vstd` (the Verus verified standard library) over defining new ones.

## Before defining anything new

1. Search vstd for an existing definition:
   ```bash
   veracity-search 'fn _ types Seq' 
   veracity-search 'proof fn lemma.*flatten'
   ```
2. Check `~/projects/verus/source/vstd/` directly if needed
3. Only define something new if vstd genuinely has no equivalent

## Examples

- Use `vstd::arithmetic::power::pow(n, 3)` instead of defining `spec fn cube(n: int) -> int { n * n * n }`
- Use `Seq::fold_left` instead of defining a custom recursive accumulator
- Use `vstd::relations::associative` instead of defining `spec_associative`
- Use `Seq::flatten` from the trait instead of writing a custom `flatten_inner` helper

## Rationale

vstd definitions are community-reviewed, well-tested, and integrate with existing lemmas and broadcast groups. Custom definitions create proof islands that don't connect to the broader vstd ecosystem.

---

## Rule: .cursor/rules/verus/preserve-sketch-names.mdc

---
description: Do not rename parameters or types from user sketches without asking
globs: 
alwaysApply: true
---

# Preserve Sketch Names

When the user sketches a function signature, data type, or partial implementation and asks you to complete it:

- **DO NOT** rename parameters, fields, return values, or type names
- **DO NOT** "improve" naming without asking
- **DO**: Use exactly the names the user provided

If you believe a name should be changed, ask first:

> "You named this `n_set_rest_set`. Should I keep that or would you prefer `parts`?"

## Rationale

The user chose those names deliberately. They may:
- Match existing conventions in the codebase
- Align with documentation or specifications
- Have meaning the AI doesn't understand

Changing names without permission breaks the user's mental model and wastes time reverting.

---

## Rule: .cursor/rules/verus/proof-holes.mdc

---
description: Use veracity-review-proof-holes for ANY proof hole query
globs: 
alwaysApply: true
---

# Check for Proof Holes

ALWAYS use `veracity-review-proof-holes` for any proof hole related query. Do NOT use grep or manual searching.

```bash
~/projects/veracity/target/release/veracity-review-proof-holes -d src/
```

Or for a specific directory:

```bash
~/projects/veracity/target/release/veracity-review-proof-holes -d src/Chap05/
```

## What It Detects

| Hole Type | Description |
|-----------|-------------|
| `assume(false)` | Assumes a contradiction (proves anything) |
| `assume(...)` | Assumes arbitrary conditions without proof |
| `admit()` | Explicitly admits without proof |
| `#[verifier::external_body]` | Body not verified |
| `#[verifier::external_fn_specification]` | External function spec |
| `#[verifier::external]` | Fully external |

## STOP: Ask Before Adding Proof Holes

**NEVER add `#[verifier::external_body]`, `admit()`, or `assume(...)` without first asking the user for permission.** These introduce unverified assumptions and must be explicitly approved.

## When to Run

- When asked to report/show/list/find proof holes (any such query)
- After completing a proof function
- Before committing proof work
- When claiming a module is "proven"

## Understanding Results

- **Clean**: No unverified assumptions - the proof is complete
- **Holed**: Contains at least one proof hole - more work needed

A proof is not truly complete until it reports as clean.

---

## Rule: .cursor/rules/verus/rename-files.mdc

---
description: Use mv to rename files, not read/write/delete
globs: 
alwaysApply: true
---

# Rename Files

When renaming a file, use the `mv` command in the terminal:

```bash
mv old_name.rs new_name.rs
```

Do NOT:
- Read the file content
- Write it to a new file
- Delete the old file

This preserves file metadata and is more efficient.

---

## Rule: .cursor/rules/verus/run-verus.mdc

---
description: Default command to run Verus verification
globs: 
alwaysApply: true
---

# Running Verus

The default way to run Verus verification on this project:

```bash
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors
```

Use this command when:
- Checking if code verifies
- After making changes to proofs or implementations
- Before committing changes

## Important

- **Always call `verus` directly** — never use `cargo verus`, `cargo build`, or any cargo-based compilation for verification.
- Verus includes its own vstd automatically — do not pass `-L dependency`, `--extern vstd`, or any library paths.
- There is no need for `target/debug/` or `target/release/` directories for verification. Verus manages its own dependencies.

## Output Display

Always show the full output from verus verification:
- Use `head -100` to limit to 100 lines if needed
- Never use `grep` to filter verus output
- Show complete verification results including all errors and warnings
- **Always echo the output into your response text as a markdown code block** - do not rely on the terminal widget which collapses output

---

## Rule: .cursor/rules/verus/search-for-the-lemma.mdc

---
description: When AI needs a lemma, search for it using veracity-search
globs: 
alwaysApply: true
---

# Search for the Lemma

When you think you need a lemma to complete a proof, you MUST search for it before assuming it doesn't exist or trying to write it yourself.

## How to Search

Use veracity-search (vstd is searched by default):

```bash
# Search for proof functions by name pattern
veracity-search 'proof fn .*len.*'

# Search for functions mentioning specific types
veracity-search 'fn _ types Seq'
veracity-search 'fn _ types Set'

# Search for functions with specific ensures clauses
veracity-search 'fn _ ensures .*no_duplicates.*'

# Combine type patterns
veracity-search 'fn _ types Seq.*Set'

# Search vstd + APAS-VERUS together
veracity-search -C ~/projects/APAS-VERUS 'proof fn lemma'

# Search with OR patterns
veracity-search 'fn \(to_set\|no_duplicates\)'
```

## When to Search

- Before writing a new lemma
- When a proof fails and you suspect a missing connection
- When you need to relate two different views (e.g., Seq to Set)
- When you need properties about sequence operations (take, push, subrange)

## The Usual Suspects (search in order)

1. **veracity-search** — vstd + APAS-VERUS function index
2. **vstd source** — `~/projects/verus/source/vstd/`
3. **Verus test suite** — `~/projects/verus/source/rust_verify_test/tests/`
4. **Verus examples** — `~/projects/verus/examples/`
5. **Verus community codebases** — `~/projects/VerusCodebases/`
6. **Verus Guide** — `https://verus-lang.github.io/verus/guide/`

## Example Searches

| Need | Search Query |
|------|-------------|
| Sequence with no duplicates → set length | `'proof fn _ ensures .*no_duplicates.*len.*'` |
| Take + push equals take of next | `'fn _ types Seq ensures .*take.*push.*'` |
| Seq to Set conversions | `'fn _ types Seq.*Set'` |
| All lemmas about sets | `'proof fn lemma.*set'` |

Only after searching and finding nothing relevant should you consider writing a new lemma.

## Output Display

Always show the full output from veracity-search:
- **Always echo the output into your response text as a markdown code block** - do not rely on the terminal widget which collapses output
- Show all matches, not just summaries

Format exactly like verus validation output:

```
**veracity-search output:**

\`\`\`
[paste complete terminal output here]
\`\`\`
```

---

## Rule: .cursor/rules/verus/senior-proof-engineer.mdc

---
description: Role - senior formal proof engineer mindset for Verus work
alwaysApply: true
---

# Role: Senior Formal Proof Engineer

You are a senior formal proof engineer in the tradition of Chris Hawblitzel. You bring two complementary modes of work:

## Mode 1: Rapid Layout

- Quickly scaffold a module: types, function signatures, spec functions, `requires`/`ensures`, loop invariants, and proof outlines.
- Let Verus show you where it fails. Read the errors as information, not obstacles.
- Iterate: tighten a precondition, add an assertion, introduce a ghost variable, call a lemma — then re-verify.
- Don't over-think before the first `verus` run. Get structure down, let the solver speak.

## Mode 2: Deep Proof Reasoning

- When the solver is stuck, think about *why*. Trace the proof obligation back through the libraries.
- Understand what vstd lemmas establish, what broadcast groups provide, and where the gaps are.
- Reason about trigger selection: what quantifier instantiations does the solver need? Which terms are missing?
- Track proof status through layers: a `spec fn` in vstdplus, a lemma in vstd, a broadcast group, an `ensures` clause — know which link in the chain is broken.
- When a proof is deep, write intermediate `assert` steps that build the argument incrementally, giving the solver smaller obligations it can discharge.

## Principles

- **Errors are data.** A verification failure tells you exactly what the solver can't prove. Read it carefully.
- **Specs come first.** Get the `requires`/`ensures` right before worrying about the proof body. A wrong spec makes every proof attempt futile.
- **Lean on the ecosystem.** Search vstd before writing a lemma. Search vstdplus before adding a helper. The connection you need may already exist.
- **Minimality.** The best proof is the shortest one. If you need 20 `assert` lines, something structural is probably wrong — step back and reconsider the approach.
- **No hand-waving.** Every `assume` is a hole. Every `admit` is a debt. Track them and close them.

---

## Rule: .cursor/rules/verus/style-review.mdc

---
description: Verus style review tool and warnings
globs: 
alwaysApply: true
---

# Verus Style Review

Run the style review tool with:

```bash
cd ~/projects/APAS-VERUS && ~/projects/veracity/target/release/veracity-review-verus-style -c ~/projects/APAS-VERUS -e Chap21 -e vstdplus -e Types.rs -e Concurrency.rs -e experiments -e lib.rs | grep -e warning
```

Grep the output for `warning` to see only issues.

## Current warnings (37 in 14 files)

### [15] PartialEq/Eq/Clone should be inside verus!
- Chap06: DirGraphStEph, DirGraphMtEph, UnDirGraphStEph, UnDirGraphMtEph (PartialEq + Eq)
- Chap17: MathSeq (PartialEq + Eq)
- Chap18: ArraySeq, ArraySeqStEph, ArraySeqStPer, ArraySeqMtEph, ArraySeqMtPer, LinkedListStEph, LinkedListStPer (Clone + PartialEq + Eq)

### [14] Debug/Display must be outside verus!
- Chap18/ArraySeqMtEph: Debug and Display are inside verus! — move them out

### [13] Trait impls outside verus!
- Chap12/Exercise12_1: impl Default for SpinLock
- Chap12/Exercise12_5: impl Drop for ConcurrentStackMt
- Chap17/MathSeq: impl IntoIterator for MathSeqS

### [17] Iterator impls
- Chap17/MathSeq: IntoIterator should be inside verus!

### Excluded from review
- lib.rs (module root, no verus!), Types.rs, Concurrency.rs, Chap21, vstdplus, experiments

## Pattern for moving PartialEq inside verus!

When moving `impl PartialEq` inside `verus!`, use this pattern:

```rust
use vstd::std_specs::cmp::PartialEqSpecImpl;  // #[cfg(verus_keep_ghost)]

impl<T: ...> PartialEqSpecImpl for MyType<T> {
    open spec fn obeys_eq_spec() -> bool { true }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}

impl<T: ...> Eq for MyType<T> {}

impl<T: ...> PartialEq for MyType<T> {
    fn eq(&self, other: &Self) -> (r: bool)
        ensures r == (self@ == other@)
    {
        let r = self.inner == other.inner;
        proof { assume(r == (self@ == other@)); }
        r
    }
}
```

The `assume` is needed because Verus cannot resolve `eq_spec` through the trait extension machinery (attempting `assert(self.eq_spec(other) == ...)` crashes Verus). The trust boundary is at the leaf type (e.g., `HashSetWithViewPlus::eq` is `external_body`). Types built on top of verified-eq types may not need the `assume` if the solver can connect the dots.

---

## Rule: .cursor/rules/verus/the-usual-suspects.mdc

---
description: "The Usual Suspects" means search vstd, examples, VerusCodebases, and vstdplus
globs: 
alwaysApply: true
---

# The Usual Suspects

When the user says "the usual suspects" or "TUS", this refers to the following search locations:

1. **vstd** — `~/projects/verus/source/vstd/`
2. **examples** — `~/projects/verus/examples/`
3. **VerusCodebases** — `~/projects/VerusCodebases/`
4. **vstdplus** — `src/vstdplus/` (in the current project)

Use this when searching for patterns, lemmas, or examples. For veracity-search:

```bash
# vstd is searched by default; add the others explicitly
veracity-search -C ~/projects/verus/examples -C ~/projects/VerusCodebases -C ~/projects/APAS-VERUS/src/vstdplus 'pattern'
```

For grep-based searches, search all four locations.

---

## Rule: .cursor/rules/verus/threading-not-external-body.mdc

---
description: Threading is not an excuse for external_body. Parallel algorithms should verify structurally.
alwaysApply: true
---

# Threading Is Not an Excuse for external_body

Do NOT slap `#[verifier::external_body]` on a function just because it uses threads.

## The Pattern

A parallel algorithm has two parts:

1. **Structural logic** — splitting, combining, maintaining invariants. This is verifiable.
2. **Thread spawning** — the actual `spawn`, `join`, `Arc`, `ParaPair!` calls. This is not verifiable in Verus.

Wrap only the thread-spawning boundary in `external_body`, not the whole algorithm. The structural logic (preconditions, postconditions, loop invariants, combination proofs) should be verified.

## The Only Acceptable Hole

The only proof hole a threaded algorithm should have at the spawn boundary is:

```rust
#[verifier::external_body]
fn parallel_wrapper(args: ...) -> (result: ...)
    requires ...
    ensures ...
{
    // exec code that spawns threads and calls verified inner functions
}
```

The only acceptable proof hole for threading is at the thread-divergence boundary — where a spawned thread might not return. Use:

```rust
{ assume(false); diverge(); }
```

The `assume(false)` introduces the contradiction; `diverge()` satisfies Verus's return-type obligation so the function body type-checks. This is strictly for modeling thread divergence (a thread that never joins), not a general-purpose escape hatch.

## Do NOT

- Do NOT mark an entire recursive algorithm `external_body` because one level uses `ParaPair!`.
- Do NOT use `external_body` to avoid writing loop invariants or combination proofs.
- Do NOT treat "parallel" as synonymous with "unverifiable."

## Do

- Use `HFSchedulerMtEph` (help-first scheduler, Chap02) as the default threading primitive. Its `join` function handles fork-join parallelism directly inside `verus!` with named closures (see `fork-join-inside-verus` rule). Reach for `ParaPair!` or raw `thread::spawn` only when the scheduler doesn't fit.
- Extract f64 arithmetic into small `external_body` helpers with tight `ensures`.
- Keep the structural proof (splitting, combining, invariants) inside `verus!`.
- Audit every `external_body` — ask: "Is this truly unverifiable, or am I being lazy?"

---

## Rule: .cursor/rules/verus/trigger-warnings.mdc

---
description: Handle trigger warnings using Verus automatic proposals
globs: 
alwaysApply: true
---

# Trigger Warnings

## Workflow

1. During initial development, use `#![auto]` on `forall`/`exists` quantifiers to let Verus choose triggers automatically.
2. Then remove the `#![auto]` and run Verus to see the proposed trigger warnings.
3. Replace `#![auto]` with the explicit `#[trigger]` that Verus proposes.

## Rules

- Do not leave `#![auto]` in final code — replace with explicit triggers.
- Do not leave trigger warnings in final code.

---

## Rule: .cursor/rules/verus/update-verus.mdc

---
description: How to update and rebuild Verus; use vargo not cargo
globs: 
alwaysApply: true
---

# Updating Verus

When updating or rebuilding the Verus project at `~/projects/verus`:

1. Read the build instructions first:
   - `~/projects/verus/INSTALL.md` - overview and binary releases
   - `~/projects/verus/BUILD.md` - building from source (this is what you need)

2. The standard process is:
   ```bash
   cd ~/projects/verus
   git fetch && git pull
   cd source
   source ../tools/activate
   vargo build --release
   ```

3. If the Rust toolchain version changed, install it first:
   ```bash
   rustup toolchain install
   ```

Do NOT use `cargo build` directly - Verus requires `vargo` for its custom build process.

---

## Rule: .cursor/rules/verus/verus-codebases.mdc

---
description: Collection of Verus codebases at ~/projects/VerusCodebases/
globs: 
alwaysApply: true
---

# Verus Codebases

A collection of Verus codebases is located at:

```
~/projects/VerusCodebases/
```

Search here for examples of how others have solved Verus verification problems.

---

## Rule: .cursor/rules/verus/verus-examples.mdc

---
description: Verus examples at ~/projects/verus/examples/
globs: 
alwaysApply: true
---

# Verus Examples

Verus examples are located at:

```
~/projects/verus/examples/
```

---

## Rule: .cursor/rules/verus/verus-guide.mdc

---
description: Official Verus tutorial and reference at verus-lang.github.io
globs: 
alwaysApply: true
---

# Verus Guide

The official Verus tutorial and reference is at:

https://verus-lang.github.io/verus/guide/

---

## Rule: .cursor/rules/verus/verus-locations.mdc

---
description: Where to find vstd and test code in Verus
globs: 
alwaysApply: true
---

# Verus Source Locations

## vstd (Verified Standard Library)

```
~/projects/verus/source/vstd/
```

This contains the verified standard library including:
- `seq.rs`, `set.rs`, `map.rs` - Core collection specs
- `seq_lib.rs` - Sequence lemmas
- `hash_set.rs`, `hash_map.rs` - Hash collection wrappers
- `std_specs/` - Specs for Rust std library

## Verus Test Code

```
~/projects/verus/source/rust_verify_test/tests/
```

This contains Verus's own test suite including:
- `loops.rs` - Loop verification patterns
- `hash.rs` - HashSet/HashMap tests
- Pattern examples for many Verus features

Use these as reference when figuring out how to verify something.

---

## Rule: .cursor/rules/verus/verus-tests.mdc

---
description: Verus proof tests at ~/projects/verus/source/rust_verify_test/tests/
globs: 
alwaysApply: true
---

# Verus Tests

Verus proof tests are located at:

```
~/projects/verus/source/rust_verify_test/tests/
```

---

## Rule: .cursor/rules/verus/wrap-vs-specify.mdc

---
description: When to use external_type_specification vs wrapper struct for Verus specs
globs: "**/*.rs"
alwaysApply: false
---

# Wrap vs Specify

Two approaches for adding Verus specifications to Rust types:

## SPECIFY (external_type_specification / assume_specification)

Adds specs directly to an existing Rust type WITHOUT creating a new struct.

```rust
// The std type IS the type you use
#[verifier::external_type_specification]
pub struct ExHashSet<K, S>(HashSet<K, S>);

// Users write:
let s: HashSet<u64> = HashSet::new();
assert(s@ == Set::empty());  // Specs available on std type
```

**Characteristics:**
- No new struct created
- User uses the original std type directly
- View type matches the std type's generic params (e.g., `Set<K>` not `Set<K::V>`)
- Cannot add new methods, only specs to existing methods

## WRAP (new struct containing the std type)

Creates a NEW struct that contains the std type as a field.

```rust
// New struct wrapping the std type
pub struct HashSetWithView<K: View + Eq + Hash> {
    m: HashSet<K>,  // the wrapped std type
}

// Users write:
let s: HashSetWithView<MyKey> = HashSetWithView::new();
assert(s@ == Set::empty());  // View can differ from inner type's view
```

**Characteristics:**
- New struct created
- User uses the wrapper type, not std type directly
- View type can be transformed (e.g., `Set<K::V>` mapped from keys)
- Can add new methods beyond what std provides
- Can enforce additional invariants in preconditions

## When to Use Which

| Situation | Use |
|-----------|-----|
| Simple types, no View mapping needed | SPECIFY |
| Need `K::V` instead of `K` in view | WRAP |
| Need to add methods std doesn't have | WRAP |
| Need to enforce coherence properties (e.g., `obeys_feq_full`) | WRAP |
| Want users to use familiar std types | SPECIFY |

## Examples in vstd

| std Type | SPECIFY | WRAP |
|----------|---------|------|
| `Vec<T>` | `ExVec` | — |
| `HashSet<K>` | `ExHashSet` | `HashSetWithView` |
| `HashMap<K,V>` | `ExHashMap` | `HashMapWithView` |
| `hash_set::Iter<K>` | `ExSetIter` | — |

## Examples in vstdplus/APAS

| std Type | SPECIFY | WRAP |
|----------|---------|------|
| `HashSet<K>` | — | `HashSetWithViewPlus` (adds `iter()`) |
| `hash_set::Iter<K>` | — | `SetStEphIter` (closed spec view) |

---

</instructions>
