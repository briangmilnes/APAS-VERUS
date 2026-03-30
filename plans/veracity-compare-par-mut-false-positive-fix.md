# Prompt: Fix compare-par-mut False Positives from Ephemeral/Persistent Pattern Mismatch

## Problem

`veracity-compare-par-mut` emits ~150 false-positive warnings in Chap43 alone when
comparing StEph (`&mut self`) against StPer (`&self` + return value). The tool already
detects the Eph→Per return shift (emitted as info), but it does not use that knowledge
when matching ensures clauses. As a result, semantically equivalent ensures are reported
as "with no match."

## The Two Substitution Rules

When the tool detects an Eph→Per return shift for a function, it should apply two
substitution rules before comparing ensures clauses:

### Rule 1: `old(self)` → `self` (pre-state)

In ephemeral APIs, the pre-mutation state is `old(self)`. In persistent APIs, the
pre-state is just `self` (since `self` is immutable). When matching ensures:

| Ephemeral (StEph) | Persistent (StPer) | Status |
|---|---|---|
| `split.0@.subset_of(old(self)@)` | `split.0@.subset_of(self@)` | Should match |
| `split.1 == old(self)@.contains(k@)` | `split.1 == self@.contains(k@)` | Should match |
| `forall\|key\| old(self)@.dom().contains(key) ==> ...` | `forall\|key\| self@.dom().contains(key) ==> ...` | Should match |
| `self@.dom() =~= old(self)@.dom().union(other@.dom())` | `combined@.dom() =~= self@.dom().union(other@.dom())` | See Rule 2 |

### Rule 2: post-`self` → return name (post-state)

In ephemeral APIs, the post-mutation state is `self` (in ensures). In persistent APIs,
the result is a named return value (`filtered`, `combined`, `joined`, `remaining`,
`restricted`, `subtracted`, `mapped`, `updated`). When matching ensures:

| Ephemeral (StEph) | Persistent (StPer) | Status |
|---|---|---|
| `self@.subset_of(old(self)@)` | `filtered@.subset_of(self@)` | Should match (Rule 1 + Rule 2) |
| `self@.dom() =~= old(self)@.dom().union(other@.dom())` | `combined@.dom() =~= self@.dom().union(other@.dom())` | Should match (Rule 1 + Rule 2) |
| `self@.finite()` | `joined@.finite()` | Should match (Rule 2) |
| `self.spec_wf()` | `joined.spec_wf()` | Should match (Rule 2) |

The return name is available from the function signature: `fn join(...) -> (joined: Self)`.
For `&mut self` methods returning `()`, the post-state name is `self`.

## Combined Application

Both rules apply simultaneously. To match an ephemeral ensures clause against a
persistent one:

1. In the **persistent** clause, substitute `self` → `old(self)` for every occurrence
   that refers to the pre-state (i.e., appears after `old(` or refers to the input).
2. In the **persistent** clause, substitute the return-value name → `self` for every
   occurrence that refers to the output.

Or equivalently, normalize both sides to a canonical form:
- Pre-state: always `PRE`
- Post-state: always `POST`

Then:
- Ephemeral: `old(self)` → `PRE`, `self` (in ensures) → `POST`
- Persistent: `self` → `PRE`, return-name → `POST`

After normalization, compare clauses as usual.

## Concrete Examples from Chap43

### Example 1: `filter` (OrderedSetStEph vs OrderedSetStPer)

**StEph** (`&mut self`):
```rust
ensures
    self@.subset_of(old(self)@),           // POST.subset_of(PRE)
    self.spec_wf(),                         // POST.spec_wf()
    forall|v| self@.contains(v) ==> ...,   // POST quantifier
    forall|v| old(self)@.contains(v) && spec_pred(v) ==> self@.contains(v);
```

**StPer** (`&self` → `filtered`):
```rust
ensures
    filtered@.subset_of(self@),            // POST.subset_of(PRE)
    filtered.spec_wf();                     // POST.spec_wf()
```

After normalization, `filtered@.subset_of(self@)` becomes `POST@.subset_of(PRE@)`,
which matches `self@.subset_of(old(self)@)` normalized to `POST@.subset_of(PRE@)`.

Currently reported as: "StPer has ensures clause `filtered@.subset_of(self@)` with no match in StEph."

### Example 2: `split` (OrderedSetStEph vs OrderedSetStPer)

**StEph**: `split.0@.subset_of(old(self)@)` → normalized: `split.0@.subset_of(PRE@)`
**StPer**: `split.0@.subset_of(self@)` → normalized: `split.0@.subset_of(PRE@)`

These are identical after normalization. Currently reported as unmatched.

### Example 3: `join` (OrderedSetStEph vs OrderedSetStPer)

**StEph**: `self@.finite()` → normalized: `POST@.finite()`
**StPer**: `joined@.finite()` → normalized: `POST@.finite()`

Match. Currently reported as: "StPer has ensures clause `joined@.finite()` with no match in StEph."

### Example 4: `union` (OrderedTableStEph vs OrderedTableStPer)

**StEph**: `self@.dom() =~= old(self)@.dom().union(other@.dom())` → `POST@.dom() =~= PRE@.dom().union(other@.dom())`
**StPer**: `combined@.dom() =~= self@.dom().union(other@.dom())` → `POST@.dom() =~= PRE@.dom().union(other@.dom())`

Match. Currently reported as unmatched.

## Implementation Sketch

In the clause-matching phase (where the tool already does fuzzy matching):

1. Detect Eph→Per return shift (already done — emitted as info).
2. When shift is detected, extract the return-value name from the persistent function
   signature.
3. Before comparing ensures clauses, normalize both sides:
   - Ephemeral: `old(self)` → `__PRE__`, `self` → `__POST__`
   - Persistent: `self` → `__PRE__`, `<return_name>` → `__POST__`
4. Compare normalized clauses. If they match, suppress the warning (or downgrade to info).

Edge cases:
- `self` in ephemeral ensures always means post-state (Verus semantics).
- `self` in persistent ensures always means pre-state (immutable `&self`).
- The return name only appears in persistent ensures, never in ephemeral.
- Tuple returns (`(Self, bool, Self)`) keep their field access: `split.0`, `split.2`
  are the same in both variants, not substituted.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source or clause text. All substitutions must be token-aware or AST-aware.
Parse ensures/requires blocks with brace/comma/semicolon awareness. A string-hacking
detector will flag and kill tools that corrupt source syntax.

## Expected Impact

This fix should eliminate ~150 of the 348 remaining Chap43 warnings and likely
hundreds more across the full codebase. The "clause count N vs M" warnings that
accompany these false positives will also be reduced since matched clauses won't
inflate the mismatch count.
