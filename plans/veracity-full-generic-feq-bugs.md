# veracity-full-generic-feq Bug Report

Date: 2026-03-23
Command: `veracity-full-generic-feq -c ~/projects/APAS-VERUS-agent5 -e experiments -e vstdplus`
Result: Validation fails with 30+ errors across multiple bug categories.

The dry-run (`-n`) analysis is correct and useful. The live transform has six
distinct bug classes that must all be fixed before the tool is production-ready.

## Bug 1: Double Commas in requires Clauses

When the tool removes an `obeys_feq_full(...)` call from a `requires` clause, it
leaves a trailing comma, producing `,,` (double comma) which is a parse error.

**Affected files (7):**

| # | Chap | File | Line | Text |
|---|------|------|------|------|
| 1 | 37 | AVLTreeSeq.rs | 266 | `requires self.spec_avltreeseq_wf(),,` |
| 2 | 37 | AVLTreeSeqStEph.rs | 274 | `requires self.spec_avltreeseqsteph_wf(),, self.spec_seq()...` |
| 3 | 43 | OrderedTableStEph.rs | 227 | `...spec_orderedtablesteph_wf(),, obeys_view_eq::<K>()` |
| 4 | 43 | AugOrderedTableStEph.rs | 271 | `...spec_augorderedtablesteph_wf(),, obeys_view_eq::<K>()` |
| 5 | 43 | OrderedTableStPer.rs | 467 | `...f.requires((v,)),,` |
| 6 | 43 | AugOrderedTableStPer.rs | 146 | `...obeys_view_eq::<K>(),,` |
| 7 | 45 | BalancedTreePQ.rs | 143 | `requires self.spec_balancedtreepq_wf(),,` |

**Root cause:** When `obeys_feq_full<T>()` is removed from a requires list, the
preceding comma is not cleaned up. The tool removes the call but not its separator.

**Fix:** After removing an `obeys_feq_full(...)` term from a requires clause,
also remove the adjacent comma. Handle three cases:
1. Term was the last item: remove the trailing comma before it
2. Term was in the middle: remove one of the two adjacent commas
3. Term was the only item: remove the entire `requires` line if empty

## Bug 2: Missing Space Before Attribute

`pub#[verifier::loop_isolation(false)]` — missing space between `pub` and the
attribute.

**Affected files (1):**

| # | Chap | File | Line |
|---|------|------|------|
| 1 | 43 | OrderedTableStEph.rs | 3203 |

**Root cause:** When inserting `#[verifier::loop_isolation(false)]` before a
`pub fn`, the tool concatenates without a space or newline.

**Fix:** Always emit a newline (or at minimum a space) between `pub` and
`#[verifier::loop_isolation(false)]`. The standard pattern is:

```rust
    #[verifier::loop_isolation(false)]
    pub fn foo(...)
```

The attribute goes on its own line before the `pub fn` line.

## Bug 3: Duplicate Import of obeys_feq_full_trigger

When the tool adds a new `use` line for feq imports, it does not check whether
`obeys_feq_full_trigger` is already imported on an existing `use` line. This
produces `E0252: the name is defined multiple times`.

**Affected files (4):**

| # | Chap | File |
|---|------|------|
| 1 | 37 | AVLTreeSeqStPer.rs |
| 2 | 41 | ArraySetStEph.rs |
| 3 | 41 | AVLTreeSetStEph.rs |
| 4 | 41 | AVLTreeSetStPer.rs |

**Root cause:** The existing files already have
`use crate::vstdplus::feq::feq::{feq, obeys_feq_full_trigger};`
and the tool adds a second use line that also imports `obeys_feq_full_trigger`.

**Fix:** Before adding feq imports, parse all existing `use` lines in the file's
imports section. Merge new imports into existing `use` lines rather than adding
duplicate lines. If `obeys_feq_full_trigger` is already imported, do not import
it again.

## Bug 4: Type Names Not Visible Outside verus! Block

Struct types defined inside `verus! { }` are not visible to `impl` blocks outside
`verus!` (sections 13-14: Debug, Display, PartialEq outside verus). The tool
appears to be moving or renaming struct definitions in a way that breaks these
external references.

**Affected files (8+):**

| # | Chap | File | Missing type |
|---|------|------|-------------|
| 1 | 37 | AVLTreeSeq.rs | AVLTreeNode, AVLTreeS, AVLTreeSeqIter, AVLTreeSeqGhostIterator |
| 2 | 37 | AVLTreeSeqStEph.rs | AVLTreeNode, AVLTreeSeqStEphS |
| 3 | 43 | AugOrderedTableStEph.rs | AugOrderedTableStEph |
| 4 | 43 | OrderedTableStPer.rs | OrderedTableStPer |
| 5 | 43 | AugOrderedTableStPer.rs | AugOrderedTableStPer |
| 6 | 43 | OrderedSetStEph.rs | OrderedSetStEph |
| 7 | 43 | OrderedSetStPer.rs | OrderedSetStPer |
| 8 | 42 | TableStEph.rs | TableStEph |

**Root cause:** The tool is either (a) moving the struct definition inside an
inner scope, (b) renaming the struct, or (c) wrapping it in a way that changes
its visibility. The `impl Debug for TypeName` blocks in sections 13-14 (outside
`verus!`) can no longer find the type.

**Fix:** The transform must not change struct names, visibility, or scoping.
The `obeys_feq_full` fold only touches `spec_*_wf` predicates, loop invariants,
requires clauses, and trigger asserts. It should never modify `struct` definitions,
`enum` definitions, or the verus!/non-verus! boundary. If the tool is currently
modifying struct definitions as a side effect, that logic must be removed.

## Bug 5: Broken Cross-Module References

Files that import types/traits/functions from transformed files can no longer
find them.

**Affected files:**

| # | Chap | File | Missing import |
|---|------|------|---------------|
| 1 | 45 | HeapsortExample.rs | `BalancedTreePQ::BalancedTreePQ::BalancedTreePQ` |
| 2 | 53 | PQMinStEph.rs | `AVLTreeSeqStEph::AVLTreeSeqStEphTrait` |
| 3 | 53 | PQMinStEph.rs | `AVLTreeSeqStEph::lemma_wf_implies_len_bound_steph` |
| 4 | 53 | GraphSearchStEph.rs | `AVLTreeSeqStEph::AVLTreeSeqStEphTrait` |
| 5 | 43 | OrderedTableMtEph.rs | `OrderedTableStEph` struct, `from_sorted_entries` fn |

**Root cause:** Same as Bug 4 — the transform changes the module's exported
names or structure, breaking downstream imports.

**Fix:** The transform must preserve all public API names. If a struct was named
`Foo` and exported as `mod::Foo`, it must remain `mod::Foo` after transformation.
The tool should verify that the set of public names exported by each module is
identical before and after the transform.

## Bug 6: Trait Renamed to ArraySeqStEphTrait

The Rust compiler suggests `ArraySeqStEphTrait` as a replacement for
`AVLTreeSeqStEphTrait` in PQMinStEph.rs. This means the trait inside
AVLTreeSeqStEph.rs was renamed from `AVLTreeSeqStEphTrait` to
`ArraySeqStEphTrait`, which is wrong.

**Root cause:** The tool appears to be performing text substitutions that
rename traits. This is a catastrophic bug — trait renames break all downstream
consumers.

**Fix:** The transform must NEVER rename traits, structs, enums, or functions.
It only modifies: (1) `spec_*_wf` predicate bodies, (2) `requires` clauses
(removing `obeys_feq_full` calls), (3) loop invariants (removing feq-related
invariants), (4) trigger asserts (removing feq trigger asserts), (5) adding
`#[verifier::loop_isolation(false)]` attributes. Any logic that renames
identifiers must be removed.

## Severity Assessment

| Bug | Severity | Scope | Fix difficulty |
|-----|----------|-------|---------------|
| 1. Double commas | High | 7 files | Easy — comma cleanup after removal |
| 2. Missing space | Medium | 1 file | Easy — newline before attribute |
| 3. Duplicate imports | High | 4 files | Medium — merge with existing imports |
| 4. Types not visible | Critical | 8+ files | Medium — stop modifying struct defs |
| 5. Broken cross-module | Critical | 5+ files | Medium — preserve public API |
| 6. Trait renamed | Critical | 2+ files | Easy — stop renaming identifiers |

Bugs 4, 5, and 6 may share a single root cause: the tool is performing
identifier renames or struct relocations as part of the transform. Fixing the
root cause (don't touch struct/trait/enum definitions or names) would fix all
three.

## Recommendation

The dry-run mode (`-n`) is correct and valuable for analysis. The live mode
needs all six bugs fixed before it can be used. Suggest fixing in this order:

1. Fix Bug 6 (trait renames) — likely fixes 4 and 5 as a side effect
2. Fix Bug 1 (double commas) — most common error
3. Fix Bug 3 (duplicate imports) — import merging
4. Fix Bug 2 (missing space) — trivial
5. Add a post-transform check: run `cargo check` (not full verus verify) on
   each modified file to catch syntax and name resolution errors before
   declaring success
