# R150 Agent 4 — Build veracity-fix-import-order Tool. AFK.

## Context

You are working in `~/projects/veracity`. There may be another agent on
`~/projects/veracity-agent1` doing tocify work — do not touch that worktree.

## Problem

Veracity style rule [18] fires 718 times — the #1 warning. It flags files where
`use crate::...` imports appear AFTER `broadcast use` statements. The APAS-VERUS
standard requires this order inside `verus!`:

```rust
verus! {

// Section 2. imports
use crate::Types::Types::*;
use crate::ChapNN::Foo::Foo::*;

// Section 3. broadcast use
broadcast use vstd::seq::group_seq_axioms;
broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
```

The standard says: imports (section 2) come before broadcast use (section 3).
Many files have them interleaved or reversed.

## What to build

A new veracity binary: `veracity-fix-import-order`.

### Usage

```bash
veracity-fix-import-order -c ~/projects/APAS-VERUS
veracity-fix-import-order -d ~/projects/APAS-VERUS/src/Chap37/
veracity-fix-import-order ~/projects/APAS-VERUS/src/Chap37/BSTPlainStEph.rs
```

### Behavior

For each `.rs` file inside a `verus! { ... }` block:

1. Find all `use crate::...` lines and `broadcast use ...` lines.
2. If any `use crate::...` appears AFTER any `broadcast use ...`, reorder:
   - Collect all `use crate::...` lines (preserving their relative order).
   - Collect all `broadcast use ...` lines (preserving their relative order).
   - Place all `use crate::...` lines first, then a blank line, then all
     `broadcast use ...` lines.
3. Preserve `use std::...` and `use vstd::...` lines in their original position
   (they come before `use crate::...` per the standard).
4. Preserve `#[cfg(verus_keep_ghost)]` annotations that precede a `use` line —
   they travel with their `use`.

### Import groups (correct order)

```
1. use std::...           (standard library)
2. (blank line)
3. use vstd::prelude::*;
4. #[cfg(verus_keep_ghost)] use vstd::...;  (verus-only vstd imports)
5. (blank line)
6. use crate::Types::Types::*;
7. use crate::ChapNN::...;
8. use crate::XLit;       (macros, by name)
9. (blank line)
10. broadcast use vstd::...;
11. broadcast use crate::...;
```

Groups 1-8 are section 2 (imports). Groups 10-11 are section 3 (broadcast use).
The fixer only needs to ensure groups 6-8 come before groups 10-11. It does NOT
need to reorder within groups 1-5 (those are usually correct).

### Dry run mode

```bash
veracity-fix-import-order -c ~/projects/APAS-VERUS --dry-run
```

Shows what would change without modifying files. Output format:

```
src/Chap37/BSTPlainStEph.rs: would move 3 imports before 2 broadcast uses
src/Chap39/BSTTreapMtEph.rs: would move 1 import before 4 broadcast uses
```

### What NOT to reorder

- Imports outside `verus!` — leave them alone.
- `use std::fmt::...` at module top (before `verus!`) — leave alone.
- Comments between imports — preserve them with their following `use` line.
- `#[cfg(...)]` attributes — they travel with their `use` line.

## Implementation

Add `src/bin/fix_import_order.rs`. Follow the pattern of existing fixers
(`fix_add_requires.rs`, `fix_add_ensures.rs`). Use `ra_ap_syntax` for parsing
to find the `verus!` macro invocation, then work within its token tree.

The binary name in Cargo.toml should be `veracity-fix-import-order`.

## Testing

Add a test fixture in `tests/fixtures/` with a file that has imports after
broadcast use. Verify the fixer reorders correctly.

Run against APAS-VERUS in dry-run mode to confirm the count matches the
718 [18] warnings from the style checker.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse the verus! macro
body with proper syntax awareness. A string-hacking detector will flag and kill
tools that corrupt source syntax.
