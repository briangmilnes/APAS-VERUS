# Veracity Rule [22] Fix: Stop Flagging Free Functions That Belong Free

## Context

You are working in `~/projects/veracity`. There is another agent on
`~/projects/veracity-agent1` doing tocify work — do not touch that worktree.

## Problem

Rule [22] (split into [22:spec], [22:exec], [22:proof]) fires 1528 times. It flags
every free function as "should be a trait method." But the APAS-VERUS
`helper_function_placement_standard.rs` (Standard 19) explicitly says many free
functions SHOULD be free:

> **Module-level free functions** (internal helpers, not in any trait or impl):
> - Recursive algorithm cores that operate on bare data types: `Link<T>`,
>   `Option<Box<Node<T>>>`, `&[T]`, `ArraySeqStEphS<T>`, `Vec<T>`.
> - Rotation, rebalancing, and structural transformation routines.
> - Pure proof lemmas (section 7) that reason about sequences, maps, or sets.
> - Any helper that takes all its inputs as parameters (no `self`).

The rule needs to distinguish between functions that SHOULD be trait methods
(first param is the module's primary type) and functions that SHOULD be free
(first param is a bare data type, Seq, Set, Map, or no params at all).

## Current counts (filtered, excluding experiments/standards/vstdplus)

```
[22:proof]  558  Free proof fn should be trait method
[22:spec]   532  Free spec fn should be trait signature with impl body
[22:exec]   438  Free exec fn should be trait method
```

## What to change

### Step 1: Identify the module's primary type(s)

Each file has one or more traits. The trait's `Self` type (from the `impl Trait for Type`
block) is the module's primary type. For example:

- `OrderedTableStEph.rs` has `impl OrderedTableStEphTrait for OrderedTableStEphS<K,V>`
  → primary type is `OrderedTableStEphS`
- `BFSMtEph.rs` has `impl BFSMtEphTrait for BFSMtEphS<V>`
  → primary type is `BFSMtEphS`

If a file has multiple traits/impls, collect all primary types.

### Step 2: Classify each free function

Look at the function's first parameter (if any). Classify:

**Should be a trait method (keep as WARNING):**
- First param is `self`, `&self`, `&mut self` (already a method — shouldn't happen)
- First param type matches a primary type: `node: PrimaryType<T>`,
  `table: &PrimaryType<K,V>`, `set: &mut PrimaryType<T>`, etc.
- No params but return type is the primary type (constructor pattern)

**Should be free (demote to INFO):**
- First param is a bare/library type: `Seq<T>`, `Set<T>`, `Map<K,V>`, `Vec<T>`,
  `&[T]`, `Link<T>`, `Option<Box<Node<T>>>`, `Box<Node<T>>`, `Arc<Node<T>>`,
  primitive types (`usize`, `u64`, `bool`, etc.)
- First param is a type from a DIFFERENT module (e.g., `&ArraySeqMtEphS` in BFS)
- No params and return type is not the primary type
- Function name starts with `lemma_` or `proof fn` with no primary-type param
  (pure mathematical lemma)
- Function name starts with `spec_` with no primary-type param
  (module-level spec predicate like `spec_monoid`, `spec_distances_bounded`)

### Step 3: New sub-rules

| Rule | Severity | Meaning |
|------|----------|---------|
| [22:exec] | warning | Free exec fn with primary-type first param → should be trait method |
| [22:spec] | warning | Free spec fn with primary-type first param → should be trait signature |
| [22:proof] | warning | Free proof fn with primary-type first param → should be trait method |
| [22a:exec] | info | Free exec fn on bare data → correctly free per Standard 19 |
| [22a:spec] | info | Free spec fn on bare data → correctly free per Standard 19 |
| [22a:proof] | info | Free proof fn on bare data → correctly free per Standard 19 |

### Step 4: Update summary tables

The summary tables should show the new sub-rules. The [22a] counts should appear
in the info summary (if there is one) or as a note below the warning table:

```
Note: 1120 free functions correctly placed per Standard 19 ([22a] info, not shown).
```

## Type matching heuristics

Exact type matching is hard (generics, aliases). Use these heuristics:

1. Extract the struct name from `impl Trait for StructName<...>` — just the ident
   before the `<`.
2. For each free fn's first param, extract the base type name (strip `&`, `&mut`,
   `Option<...>`, `Box<...>`, `Arc<...>` wrappers, generic args).
3. Compare base ident. If it matches a primary type ident → warning. Otherwise → info.
4. Special case: if the file has NO trait impls, demote ALL [22] to info (no primary
   type to move functions into).

## Expected outcome

Rough estimate based on the sample:
- ~400 of 558 [22:proof] → [22a:proof] info (lemmas on Seq/Set/Map/bare data)
- ~300 of 532 [22:spec] → [22a:spec] info (spec fns with looser bounds on bare data)
- ~200 of 438 [22:exec] → [22a:exec] info (D&C helpers on Link/Vec/slice)
- Remaining ~628 stay as warnings — real traitify targets

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse type parameter
bounds with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
