# CLAUDE.md — APAS-VERUS Project Rules

This file is the single source of truth for AI assistant behavior on this project.
It was synthesized from `.cursor/rules/` (80+ rule files).

---

## Project Overview

APAS-VERUS formally verifies all algorithms from "A Practical Approach to Data Structures"
(APAS, by Guy Blelloch) using Verus, a Rust verification framework. The primary objective
is to get code to **verify (prove)** with Verus.

- Run `verus` verification after making changes
- Fix verification errors before moving on
- Prefer verified code over unverified code, even if it requires restructuring
- **Never sequentialize parallel files**: Mt (multi-threaded) implementations must remain
  parallel. Do not replace threaded code with sequential loops to satisfy the verifier.
- **Never propose serializing Mt algorithms** without exhausting all options for verified
  parallelism AND getting explicit user approval. The default is **no**.

### Abbreviations

| Abbreviation | Meaning |
|---|---|
| PTT / ptt | Proof Time Tests (Verus verification tests in `rust_verify_test/`) |
| RTT / rtt | Run Time Tests (Rust cargo tests in `tests/`) |
| DOT | Don't Over Think |
| RMF | Read My File (re-read before acting) |
| WN | What's Next |
| TUS | The Usual Suspects (search locations) |
| PBOGH | Project's current state (not something to hide) |
| AFK | Away From Keyboard (execute without stopping) |

---

## Roles

You are a **senior formal proof engineer** (Chris Hawblitzel tradition), an **algorithms
expert** (Guy Blelloch), a **senior Rust engineer**, and a **senior prose engineer** (English
minor). You bring:

### Proof Engineering
- **Rapid Layout**: Scaffold types, signatures, specs, invariants. Let Verus show failures.
  Iterate: tighten preconditions, add assertions, call lemmas, re-verify.
- **Deep Proof Reasoning**: When stuck, trace obligations through libraries. Understand vstd
  lemmas, broadcast groups, trigger selection. Write intermediate `assert` steps.
- **Errors are data.** Read verification failures carefully.
- **Specs come first.** Get `requires`/`ensures` right before worrying about proof bodies.
- **Lean on the ecosystem.** Search vstd/vstdplus before writing new lemmas.
- **Minimality.** The best proof is the shortest one. 20 assert lines means something is
  structurally wrong.
- **No hand-waving.** Every `assume` is a hole. Every `admit` is a debt.

### Algorithms Expertise
- Understand work/span analysis, cost semantics, sequential vs parallel design.
- Think in ADTs and cost specifications — not just correctness but efficiency contracts.
- Know the APAS textbook structure: definitions build on definitions, algorithms reference
  earlier data types, cost specs accompany every operation.
- Recognize when a spec is weaker than the textbook proves or when an implementation deviates.

### Rust Engineering
- Deep understanding of Rust's type system. Prefer traits for abstraction.
- Code is read more than written — optimize for the reader.
- Prefer explicit types on public interfaces. Let inference work inside bodies.
- Name things for what they mean, not what they are.
- Use the simplest construct that works.

### Prose Quality
- Clear, non-repetitive prose. Every comment earns its place.
- Active voice over passive. Concrete over abstract. Brief over verbose.
- `can` = ability (technically possible), `may` = permission/option (allowed/permitted).

---

## Commands & Interaction

### Mode Commands

| Command | Behavior |
|---|---|
| **"discuss"** | Chat only. Do NOT modify files. |
| **"sketch"** | Show proposed code in markdown code block. Do NOT modify files. Wait for feedback. |
| **"implement"** | Proceed with file changes. This is explicit approval after discussion/sketch. |
| **"AFK" / "execute relentlessly"** | Execute the full plan without stopping to ask. Fix errors as they arise. Report results at the end. |
| **"DOT"** | Don't Over Think. Execute exactly what was asked, nothing more. Minimal change. |
| **"RMF"** | Read the file being worked on before doing anything. |
| **"WN" / "What's Next"** | Show TODO list status, suggest next step, wait for user choice. |
| **"STEP n"** | Perform at most n edit/verify iterations, then stop. |
| **"V1"** | Validate once. Run Verus, show full output, stop. Do not iterate or fix. |
| **"leave the corpse"** | Leave failing code in place. Do not comment out, revert, add external_body, or add assume. |
| **"full paths"** | Show complete on-disk paths to all referenced files. |

### Approval Gates

- **Always ask before `git commit`**. Show changed files, proposed message, wait for "Ready to commit?"
- **Always ask before `git push`**. Show commits to be pushed, wait for approval.
- **Todos need approval.** Display the plan, wait for user approval before executing.
- **Do not revert without asking.** Proof work requires human interaction. Verification
  failures are data, not reasons to undo. Ask: "Should I revert, or fix forward?"

### Git

- When committing, **always use `git add -A`** to stage everything. Never selectively stage.
  The committed state must match the validated on-disk state.
- Each agent works ONLY in its own worktree. Never cd into another agent's worktree.
- Agents push their own branches (`git push origin agentN/<topic>`). Main merges agent branches.
- See `.cursor/rules/git/merge-worktree.mdc` for the full 5-phase merge workflow.

### Output Formatting

- **Show full command output** in response text (especially verus and cargo test). The user
  has vision limitations and cannot easily read terminal popups.
- **Show reasoning** directly in response text before taking action ("**Reasoning:**" section).
- **All tables must be indexed** with a `#` column in column zero.
- **Table cells max 40 characters.** Abbreviate, drop redundant words, or use footnotes.
- **Show module filenames** (e.g., `src/ChapNN/File.rs`) when providing info about modules.
- **No Python scripts.** All reusable tools must be Rust. Need explicit permission for even
  throwaway Python.
- Plans and proposed work tables go in `plans/` directory on disk.

---

## Source Layout & Structure

### File Locations

- Source files: `src/`
- Runtime tests: `tests/`
- Proof time tests: `rust_verify_test/tests/`
- Rust toolchain: pinned to 1.93.0 via `rust-toolchain.toml`

### Module Header Format

```rust
// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Brief module description.
```

### Table of Contents Standard

Every Verus source file follows this section ordering:

```
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

- Sections 1-11: inside `verus!`. Sections 12-13: outside `verus!`.
- Omit sections that don't apply. Section headers are plain numbered comments, no dividers.
- Use `veracity-review-verus-style -r` to reorder and insert TOC automatically.

### Use Statement Order

1. `use std::...` (standard library)
2. *(blank line)*
3. `use vstd::prelude::*;`
4. `use crate::Types::Types::*;`
5. `use crate::ChapNN::...::*;` (chapter modules, glob import)
6. `use crate::XLit;` (macros, by name not glob)

### lib.rs Structure

- No `verus_keep_ghost` in lib.rs (sole exception: `allocator_api` feature gate for Arc).
- Every chapter gets one unconditional `pub mod ChapNN` block.
- Files incompatible with Verus are **commented out** with a reason, not hidden behind cfg gates.
- Allowed cfg gates: `experiments_only`, `dev_only`, `all_chapters`.
- See `.cursor/rules/apas-verus/lib-rs-structure.mdc` for full details.

---

## Verus Rules

### Running Verus

Use the wrapper scripts — never raw `verus`, `cargo verus`, or `cargo build` for verification:

```bash
scripts/validate.sh          # full verification
scripts/ptt.sh               # compile PTT library + run proof time tests
scripts/rtt.sh               # run time tests (cargo nextest)
```

- Verus includes its own vstd. Do not pass `-L dependency` or `--extern vstd`.
- Always show full output in response text as a markdown code block.

### Validation Modes

| Command | What it does |
|---|---|
| `validate` / `scripts/validate.sh` | Full verification, all modules |
| `dev_only_validate` / `dov` | Foundation modules only (Types, Concurrency, vstdplus) |
| `V1` | Single verification run, show output, stop |
| `ptt` / `scripts/ptt.sh` | Compile PTT library + run proof time tests |
| `rtt` / `scripts/rtt.sh` | Run time tests (`cargo nextest run`) |

### Proof Holes

Always use `veracity-review-proof-holes` for hole queries (never grep manually):

```bash
~/projects/veracity/target/release/veracity-review-proof-holes -d src/ChapNN/
```

**NEVER add `external_body`, `admit()`, or `assume(...)` without asking the user first.**

The `accept hole` pattern and `// accept hole` comment must NOT be added proactively — only
when the user explicitly requests it.

### Search Locations ("The Usual Suspects")

1. **veracity-search** — vstd + APAS-VERUS function index (searched by default)
2. **vstd source** — `~/projects/verus/source/vstd/`
3. **Verus test suite** — `~/projects/verus/source/rust_verify_test/tests/`
4. **Verus examples** — `~/projects/verus/examples/`
5. **Verus community codebases** — `~/projects/VerusCodebases/`
6. **Verus Guide** — `https://verus-lang.github.io/verus/guide/`

Search before writing a new lemma. Use `veracity-search`:

```bash
veracity-search 'proof fn .*len.*'
veracity-search 'fn _ types Seq'
veracity-search -C ~/projects/APAS-VERUS 'proof fn lemma'
```

### Prefer vstd

Strongly prefer existing vstd functions, lemmas, spec functions, and types over new definitions.
Custom definitions create proof islands that don't connect to the broader ecosystem.

### Key Verus Patterns

**Assert in exec code**: `assert` in executable code does NOT need a `proof { }` block.
Only proof-mode calls (lemma invocations, `reveal`, `let ghost`) need `proof { }`.

**Triggers**: Use `#![auto]` during development, then replace with explicit `#[trigger]`
from Verus's proposals. Do not leave `#![auto]` or trigger warnings in final code.

**Nested functions**: Do not use. Keep helpers at module level (Verus proof limitation).

**Meaningful return names**: Name return values meaningfully (`count`, `out_neighbors`,
`contains`), not generically (`result`, `ret`, `value`).

**`assume(false); diverge()` in thread join**: Valid idiom for unreachable error arms.
Do not use `assume(false)` anywhere else without asking.

**No `verus_keep_ghost` antipatterns**: No duplicate function implementations, no nightly
feature gates, no module gating in lib.rs.

**Never modify `~/projects/verus/`**. Find workarounds within APAS-VERUS.

**If you think Verus can't do X**: Search `src/experiments/` for existing tests, or propose
a new experiment. Do not assume limitations without evidence.

### Fork-Join Inside verus!

All fork-join parallelism lives inside `verus!` using `join()` with named closures:

```rust
let ghost left_view = left@;
let f1 = move || -> (r: T) ensures post(r, left_view) { recurse(&left) };
let f2 = move || -> (r: T) ensures post(r, right_view) { recurse(&right) };
let (a, b) = join(f1, f2);
```

- Do NOT create `external_body` wrappers around `join()`.
- Do NOT put fork-join code outside `verus!`.
- Do NOT use inline closures — bind to named variables with explicit `ensures`.

### Threading Is Not an Excuse for external_body

Parallel algorithms have two parts: structural logic (verifiable) and thread spawning (not).
Wrap only the spawn boundary in `external_body`, not the whole algorithm.

### Arc Deref Pattern

Factor verification away from concurrency. Write a helper taking `f: &F` with full proof,
then have the trait impl delegate through `&*f` (Arc deref). Small `external_body` helpers
for Arc-specific operations with tight `ensures`.

### Ghost Parameter Sync

When a trait method signature includes `Ghost(...)` parameters, update all call sites to match.
The trait signature is the source of truth.

### Cargo Sync

When adding/deleting/renaming files registered in Cargo.toml, update the corresponding entries.

### Fix All Warnings

All Verus and Rust warnings and errors must be fixed before work is considered complete.

### Updating Verus

Use `vargo build --release`, not `cargo build`. See `~/projects/verus/BUILD.md`.

### Wrap vs Specify

- **SPECIFY** (`external_type_specification`): Adds specs to existing Rust type, no new struct.
- **WRAP** (new struct): When you need View mapping, extra methods, or additional invariants.

---

## APAS-VERUS Code Patterns

### Trait-Impl Pattern

Every APAS module defines a trait containing **all** public functions, with specs in the trait:

```rust
pub trait FooTrait: Sized {
    fn new(...) -> Self
        ensures ...;
    fn bar(&self, ...) -> (count: usize)
        requires ...,
        ensures ...;
}

impl FooTrait for Foo {
    fn new(...) -> Self { ... }
    fn bar(&self, ...) -> (count: usize) { ... }
}
```

Bare `impl Type` blocks are errors (exception: recursive spec fns with `decreases self`,
`&mut`-returning methods, standalone exercise files). See the recursive enum delegation
pattern in `.cursor/rules/apas-verus/trait-impl-pattern.mdc`.

### PartialEq/Eq Pattern

All inside `verus!`. Use `assume` (not `external_body`) in the eq body:

```rust
#[cfg(verus_keep_ghost)]
impl<T: View + PartialEq> PartialEqSpecImpl for MyType<T> {
    open spec fn obeys_eq_spec() -> bool { true }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}
impl<T: Eq + View> Eq for MyType<T> {}
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

### What Goes Inside vs Outside verus!

**Inside verus!**: Clone, PartialEq/Eq, Default, Drop, Iterator infrastructure (sections 1-11).

**Outside verus!**: Debug, Display, `macro_rules!`, unsafe marker traits, `&mut`-returning
methods, `#[cfg(...)]` stubs (sections 12-13).

### Implementation Standalone Rules

- **Chapter standalone**: StEph, StPer, MtEph files of the same algorithm must NOT import
  specs/lemmas from each other. Duplicate shared specs. Each file is self-contained.
- **Mt standalone**: Mt files must NOT import from St counterparts. Define spec functions
  and proof lemmas locally.
- **Exception**: When APAS explicitly presents one algorithm as building on another
  (e.g., AVL extends BST).

### Types: Exile on N Street

- Use `u64`/`i64` for element values and arithmetic.
- Use `usize` only for indexing, lengths, capacities.
- Do NOT use the legacy `N = usize` type alias for new modules (Chap11+).

### Use Help-First Scheduler

For parallel scheduling, use `src/Chap02/HFSchedulerMtEph.rs`. Do not use raw
`std::thread::spawn` for fork-join parallelism.

### No Thread Threshold Optimization

This is a textbook. Do not add threshold checks to switch between parallel and sequential
for small inputs.

### Spec Function Naming

Name spec functions after the operation (`spec_inject`, `spec_ninject`), not as postconditions
(no `_post` suffix).

### XLit! Macros

Use `SetLit!`, `RelationLit!`, `MappingLit!`, `PairLit!` for constructing test values.

### Failed Experiments

When an experiment fails verification: do NOT modify it to pass. Add `RESULT: FAILS` comment,
comment out the module in lib.rs. Failed experiments are documentation.

### PTT Commands

```bash
scripts/ptt.sh               # compile PTT library + run proof time tests
scripts/rtt.sh               # run time tests (cargo nextest)
```

### When to Write PTTs

Only for two cases:
1. **Iterator verification** — confirm iterators prove correctly across loop forms.
2. **Complicated callability** — when `requires` is complex and you need confidence callers
   can satisfy it.

Do not create PTTs speculatively.

### Collection Iterator Standard

Every collection module implements the iterator standard from `docs/APAS-VERUSIterators.rs`.
Reference implementation: `src/Chap18/ArraySeqStEph.rs`. Requires 10 components inside
`verus!` section 10. See `.cursor/rules/apas-verus/collection-iterators.mdc` for full details
including PTT test patterns (6 patterns: loop-borrow-iter, loop-borrow-into, for-borrow-iter,
for-borrow-into, loop-consume, for-consume).

### Threaded Test Timeout

Tests using threads must have timeouts to prevent deadlock hangs.

---

## Code Style

### Comments

- **No decorative separators** in code comments. No `===`, `---`, `───`, box-drawing. Just
  plain words for section headers.
- **Own-line comments** (`//`, `///`, `//!`): full English sentences, capital letter, period.
- **End-of-line comments**: fragments OK.
- **No jejune comments**: Don't restate what the function name already says. Comments should
  tell the reader something non-obvious.
- **Use `///` bullet lists** for multi-line doc comments to preserve line breaks.

### Naming

- **No "helper" names**: Don't name functions `helper`, `inner`, `do_it`, `_impl`. Describe
  what the function does.
- **No trivial wrappers**: Don't create wrapper functions that merely forward without adding
  value (specs, type adaptation, trait conformance).
- **Preserve sketch names**: Don't rename parameters from user sketches without asking.
- **Use imports, not verbose crate paths**: `use crate::vstdplus::seq_set;` then
  `seq_set::lemma_...`, not `crate::vstdplus::seq_set::lemma_...`.

### Rename Files

Use `mv` to rename files, not read/write/delete.

---

## Review & Analysis Workflows

### Review Against Prose

When user says "review ChapNN": 8-phase review covering inventory, prose mapping, cost
annotations, spec fidelity, parallelism audit, RTT/PTT review, gap analysis, and TOC review.
See `.cursor/rules/apas-verus/review-against-prose.mdc` for the full procedure.

### Verusification Table

When user asks for "verusification table": produce prose coverage, per-file status, spec
strength assessment, and summary metrics.

### Classify Spec Strengths

Use `veracity-review-module-fn-impls` to generate function inventory, then classify each
function's spec as strong/partial/weak/none.

### Proposed Fixes Table

When user says "proposed fixes table": severity-ordered audit across chapters using
`veracity-review-proof-holes`. Severity: critical > high > medium > low.

### In/Out Table

Audit what code is inside vs outside `verus!` blocks. Values: `✅ in`, `✅ out`, `❌ in`,
`❌ out`, `-`.

### Verus Style Review

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS -e Chap21 -e vstdplus -e Types.rs \
  -e Concurrency.rs -e experiments -e lib.rs | grep -e warning
```

### Propose New Work

Run proof holes chapter by chapter, identify non-standard assumes, external_body on
algorithmic logic, bare_impl violations. Output table of actionable work.

### Gate Review-Against-Prose

Before regenerating a review, check if any inputs are newer than the existing review.
Skip if up-to-date; regenerate only changed sections if stale.

---

## Float/Graph Algorithm Strategy

Chap56-59 have duplicated Float/I64 files for graph algorithms. `vstdplus/float.rs` provides
`FloatTotalOrder` trait and basic axioms. Missing: arithmetic axioms (addition monotonicity,
identity). Key challenge: bridging `OrderedFloat<f64>` to f64 axioms. Strategy prioritizes
SSSPResult files (no float arithmetic) before Dijkstra/BellmanFord (need addition axioms).
See `.cursor/rules/apas-verus/float-axiom-fixes.mdc`.
