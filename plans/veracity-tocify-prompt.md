# Veracity Tool: `veracity-tocify`

## Goal

Build `veracity-tocify` — a tool that audits, reorders, and inserts Table of Contents
comment blocks in APAS-VERUS Verus source files, enforcing the TOC standard from
`src/standards/table_of_contents_standard.rs`.

## Motivation

APAS-VERUS has 244 modules. Many were written before the TOC standard was finalized or
modified by agents that didn't maintain section order. The standard defines a strict
bottom-up, per-type ordering that makes files scannable by human readers. An automated
tool can audit compliance, reorder items, and generate TOC comment blocks.

## The Standard

Reference: `src/standards/table_of_contents_standard.rs`

### Section numbering

Sections 1-3 are global (one per file):
```
1. module         — pub mod declarations
2. imports        — use statements
3. broadcast use  — broadcast use statements
```

Sections 4-10 repeat per type, with letter suffixes (a, b, c...) ordered leaf-first
(bottom-up dependency order). Each type gets a complete cycle:
```
4x. type definitions    — struct, enum, type alias
5x. view impls          — impl View for T
6x. spec fns            — open spec fn, closed spec fn
7x. proof fns           — proof fn, broadcast proof fn, broadcast group
8x. traits              — pub trait XTrait
9x. impls               — impl XTrait for X
10x. iterators          — iterator structs, ghost structs, Iterator impl, ForLoop impls, IntoIterator
```

Section 11 appears once (Mt modules only):
```
11. top level coarse locking — Inv struct, RwLockPredicate, Locked struct, LockedTrait
```

Sections 12-14 repeat per type, bottom-up:
```
12x. derive impls in verus!      — Clone, PartialEq, Eq (inside verus!)
13x. macros                      — macro_rules! (outside verus!)
14x. derive impls outside verus! — Debug, Display (outside verus!)
```

### Single-type files

For files with only one type, omit the letter suffix:
```
4. type definitions
5. view impls
...
```

### Multi-type files

Letter suffixes track types. Types are ordered leaf-first (no forward references):
```
4a. type definitions — struct Leaf
5a. view impls — struct Leaf
...
9a. impls — struct Leaf
4b. type definitions — struct Interior
5b. view impls — struct Interior
...
9b. impls — struct Interior
10b. iterators — struct Interior
```

### What goes where

| Item | Section | Inside verus! |
|------|---------|---------------|
| `pub mod` | 1 | yes |
| `use` statements | 2 | yes |
| `broadcast use` | 3 | yes |
| `struct`, `enum`, `type` alias | 4 | yes |
| `impl View for T` | 5 | yes |
| `spec fn`, `open spec fn` | 6 | yes |
| `proof fn`, `broadcast proof fn`, `broadcast group` | 7 | yes |
| `pub trait` | 8 | yes |
| `impl Trait for T` (non-derive, non-iterator) | 9 | yes |
| Iterator structs, ghost structs, all iterator impls | 10 | yes |
| RwLock Inv, Locked, LockedTrait | 11 | yes |
| Clone, PartialEq, Eq impls | 12 | yes |
| `macro_rules!` | 13 | no |
| Debug, Display impls | 14 | no |

### verus! boundary

Sections 1-12 are inside `verus! { ... }`. Sections 13-14 are outside.

## Classification Rules

The tool must classify each top-level item in the file into a (section, type) pair.

### Type detection

- **Primary types**: structs/enums declared in section 4 that have a trait (section 8).
  These are the "real" types of the module.
- **Auxiliary types**: structs used only by one primary type (e.g., iterator structs,
  ghost structs). Grouped with their parent type.
- **Type ordering**: Leaf-first. A type that appears in another type's fields comes first.
  If no dependency exists, alphabetical order.

### Item classification

| Pattern | Section |
|---------|---------|
| `pub mod X {` | 1 |
| `use ...;` | 2 |
| `broadcast use ...;` | 3 |
| `pub struct X`, `pub enum X`, `pub type X =` | 4 |
| `impl View for X` | 5 |
| `open spec fn`, `closed spec fn`, `pub open spec fn` | 6 |
| `proof fn`, `pub proof fn`, `broadcast proof fn` | 7 |
| `broadcast group` | 7 |
| `pub trait XTrait` | 8 |
| `impl XTrait for X` (where XTrait is in section 8) | 9 |
| Iterator-related: `struct XIter`, `struct XGhostIter`, `impl Iterator for X`, `impl ForLoopGhostIteratorNew`, `impl ForLoopGhostIterator`, `impl IntoIterator` | 10 |
| `impl RwLockPredicate`, struct `XInv`, struct `Locked`, `impl LockedTrait` | 11 |
| `impl Clone for X`, `impl PartialEq for X`, `impl Eq for X`, `impl PartialEqSpecImpl for X` | 12 |
| `macro_rules!` | 13 |
| `impl Debug for X`, `impl Display for X` | 14 |

### Free function assignment

Free functions (not in an impl block) are assigned to a type based on:
1. First parameter type (`&self` → that type, `&X` → X, `x: X` → X)
2. Return type if no parameter matches
3. Name prefix (`spec_x_wf` → type X)
4. Proximity (if ambiguous, assign to the type whose section 9 impl is nearest)

## Modes

### `--check` (default): Audit mode

Report items that are out of order. No writes. Output in emacs compile format:
```
file:line: warning: [toc] struct Foo is in section 9 (impls), should be in section 4 (type definitions)
file:line: warning: [toc] spec fn bar should come before trait FooTrait
file:line: warning: [toc] missing TOC comment block
```

### `--fix`: Transform mode

Reorder items to match the standard. Write the file back with correct order.
Insert/update the `// Table of Contents` comment block.

**CRITICAL**: This mode rewrites files. It must be round-trip safe:
- Parse → classify → reorder → emit must produce valid Rust that compiles identically.
- Preserve all comments, attributes, whitespace within items.
- Only reorder top-level items — never modify item contents.
- After rewriting, the file must pass `scripts/validate.sh`.

### `--toc-only`: Just insert/update the TOC comment

Don't reorder items. Just generate the correct `// Table of Contents` block based on
what sections exist in the file, and insert/update it.

## TOC Comment Generation

The generated TOC lists only sections that exist in the file:
```rust
//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4a. type definitions — struct Leaf
//  5a. view impls — struct Leaf
//  6a. spec fns — struct Leaf
//  8a. traits — struct Leaf
//  9a. impls — struct Leaf
//  4b. type definitions — struct Tree
//  5b. view impls — struct Tree
//  8b. traits — struct Tree
//  9b. impls — struct Tree
//  10b. iterators — struct Tree
//  14b. derive impls outside verus! — struct Tree
```

Omit sections with no items (e.g., no section 7 if no proof fns for that type).

## CLI Interface

```bash
veracity-tocify [options] [path]
```

### Options

| Flag | Long | Description |
|------|------|-------------|
| `-c` | `--codebase DIR` | Project root. Path argument is relative to this. Default: cwd. |
| `-d` | `--dry-run` | Show what would change without writing files. Emits emacs compile diagnostics + unified diff of proposed changes. Safe to run on any file at any time. |
| `-e` | `--exclude DIR` | Exclude directory or file (repeatable). Standard excludes: `experiments`, `standards`, `Types.rs`, `Concurrency.rs`, `lib.rs`. |
| `-f` | `--file FILE` | Process a single file. For testing on one file at a time. |
| | `--check` | Audit mode (default). Report items out of order. No writes. |
| | `--fix` | Transform mode. Reorder items and write file. Always dry-run first. |
| | `--toc-only` | Insert/update TOC comment only. Don't reorder. |

### Examples

```bash
# Audit all chapters
veracity-tocify --check -c ~/projects/APAS-VERUS src/

# Dry-run on one file — see proposed reordering
veracity-tocify --fix -d -f src/Chap37/BSTSetAVLMtEph.rs

# Audit one chapter, excluding experiments
veracity-tocify --check -c ~/projects/APAS-VERUS -e experiments src/Chap37/

# Fix one file for real
veracity-tocify --fix -f src/Chap18/ArraySeqStEph.rs

# Insert TOC comments across the codebase
veracity-tocify --toc-only -c ~/projects/APAS-VERUS src/

# Test on the standard itself — output should be identity (no changes)
veracity-tocify --fix -d -f src/standards/table_of_contents_standard.rs
```

### Output

- `--check`: emacs compile format to stdout (`file:line: warning: [toc] ...`)
- `--dry-run`: emacs compile diagnostics + unified diff of proposed changes
- `--fix` / `--toc-only`: writes files, summary of changes to stdout
- Per-chapter logs written to `src/ChapNN/analyses/veracity-tocify.log`
- Exit code: 0 if clean, 1 if warnings/changes found, 2 on tool error

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust
source. All parsing and rewriting must be token-aware or AST-aware. Parse verus! blocks
with brace awareness. A string-hacking detector will flag and kill tools that corrupt
source syntax.

The tool must handle:
- Items spanning multiple lines (multiline struct definitions, trait blocks, impl blocks)
- Nested braces within items
- Attributes on items (`#[verifier::external_body]`, `#[cfg(...)]`)
- Comments between items (attach to the following item)
- The `verus! { ... }` macro boundary
- Items that are inside vs outside verus!

## Validation

After `--fix`, the rewritten file must:
1. Parse as valid Rust
2. Pass `scripts/validate.sh` with the same verified count
3. Have no semantic changes (only item order and TOC comment changed)

## Test strategy

Test on these files (use `-f` flag):
- `src/standards/table_of_contents_standard.rs` — already correct, should be identity
- `src/Chap18/ArraySeqStEph.rs` — reference implementation, should be identity or close
- `src/Chap05/SetStEph.rs` — has all 14 sections
- A deliberately scrambled copy of a clean file
