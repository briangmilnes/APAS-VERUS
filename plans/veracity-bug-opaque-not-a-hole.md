# Veracity Bug: opaque counted as a hole; no accept mechanism

## Problem

`veracity-review-proof-holes` classifies `#[verifier::opaque]` on spec functions
as `error: opaque`, counting them as proof holes. But opaque is a Z3 performance
tool, not a proof gap. An opaque spec function has a full, verified body — Verus
just hides it from the solver to prevent quantifier blowup. There is nothing to
prove; the obligation is already discharged.

Additionally, there is no `// accept hole` suppression mechanism for the opaque
hole type. The comment is parsed correctly (veracity echoes it in the error line)
but the hole is still counted. Every other hole type (`assume`, `external_body`,
`unsafe impl`, etc.) has an accept path. Opaque does not.

## Evidence

`src/Chap65/UnionFindPCStEph.rs` has three opaque spec functions:

```
:106  #[verifier::opaque]  spec_light_wf       — opaque to prevent 5 foralls from
:122  #[verifier::opaque]  spec_find_preserved —   blowing up Z3 on every call site
:132  #[verifier::opaque]  spec_same_domain    —   (documented in source comments)
```

These are intentional and correct. The functions have full bodies. Verus verifies
them. The opaque attribute is purely a solver hint to fold the definition.

After adding `// accept hole` to all three lines, veracity still reports:

```
error: opaque - #[verifier::opaque] // accept hole
```

The `// accept hole` comment is seen but not honored.

## Impact

- `veracity-review-proof-holes` reports Chap65 as holed (3 holes).
- `chapter-cleanliness-status` correctly reports Chap65 as clean — it must already
  exclude opaque from its count. These two tools are inconsistent.
- The daily proof table and any tool that reads the holes report will overcount
  holes whenever opaque is used for performance reasons.

## Suggested fix

Two changes:

1. **Exclude opaque from the error tier by default.** Opaque on a spec function
   with a body is not a hole. Demote it to `info` (alongside `structural_false_positive`)
   so it does not count toward the hole total.

2. **Honor `// accept hole` for opaque**, consistent with every other hole type,
   as a fallback for projects that want to track opaque usage explicitly but suppress
   it from counts.

The `chapter-cleanliness-status` tool already makes the right call — align
`veracity-review-proof-holes` with it.

## Scope

Any project that uses `#[verifier::opaque]` for solver performance will hit this.
In APAS-VERUS it currently affects only Chap65, but opaque is used in vstdplus and
could appear in future chapters.

## Priority

Medium. The hole count is wrong by 3 until this is fixed. The workaround is to
use `chapter-cleanliness-status` as the authoritative hole count and ignore the
Chap65 entries in `veracity-review-proof-holes`.
