# Veracity Rule [14] Fix: Stop Flagging Structs That Already Have Debug or Are Ghost-Only

## Context

You are working in `~/projects/veracity`. There may be another agent on
`~/projects/veracity-agent1` — do not touch that worktree.

## Problem

Rule [14] fires on structs that already have `Debug` via `#[derive(Debug)]` inside
`verus!`, and on ghost-only types that have no runtime representation. Both are
false positives.

### False positive 1: `#[derive(Debug)]` inside verus!

Some structs use `#[derive(Clone, Copy, PartialEq, Eq, Debug)]` inside `verus!`.
Rule [14] flags them for missing `impl Debug outside verus!`. Adding a manual
`impl Debug` outside `verus!` would cause a duplicate impl conflict.

Examples:
- `MatrixDim` in Chap50 MatrixChain{StEph,StPer,MtEph,MtPer}.rs — 4 false positives
- `Exposed`, `NodeInner` in Chap38 BSTParaStEph.rs — 2 false positives

### False positive 2: Ghost-only types

Structs that exist only at spec/proof level (ghost structs, view enums) have no
runtime representation and cannot implement `Debug`/`Display` outside `verus!`.

Examples:
- `PrimTreeSeqStTreeView` in Chap23 PrimTreeSeqStPer.rs — ghost enum

## Fix

### For derive(Debug)

Before flagging a struct for missing `impl Debug`, check whether the struct has
`#[derive(...Debug...)]` anywhere in the file. If it does, suppress the [14]
warning for Debug on that struct. Same logic for Display if `#[derive(...Display...)]`
exists (rare but possible).

Implementation: when collecting struct definitions, also scan for `#[derive(...)]`
attributes on each struct. Store which traits are derived. In the [14] check,
skip the warning if the trait is already derived.

### For ghost-only types

Structs declared with `pub ghost struct` or annotated with ghost markers should
not get [14] warnings. Also, structs whose fields are ALL `Ghost<T>` or
`Tracked<T>` types (no exec-visible fields) are effectively ghost-only.

Implementation: when collecting struct definitions, check:
1. Is the struct keyword preceded by `ghost`? If so, skip [14].
2. Are ALL fields `Ghost<T>` or `Tracked<T>`? If so, skip [14].

### Expected impact

~12 false positives eliminated. Small count but agents waste time investigating
each one and documenting why they skipped it.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse derive attributes
with proper token awareness. A string-hacking detector will flag and kill tools
that corrupt source syntax.
