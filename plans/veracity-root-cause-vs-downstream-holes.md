# Veracity Feature Request: Distinguish Root-Cause vs Downstream Holes

## Problem

When a module has N external_body functions, veracity reports N holes of equal weight.
But often there's a dependency structure: one function is the root cause (e.g., it
touches a RwLock boundary), and the other N-1 are external_body only because they call
the root-cause function and can't get ensures from it.

Example: BSTParaMtEph.rs has 12 external_body functions. But only `expose_internal`
is the real blocker — it does `acquire_read` on an `Arc<RwLock<...>>` and Verus can't
reason through that. The other 11 (`split_inner`, `min_key`, `union_inner`,
`intersect_inner`, `difference_inner`, `join_pair_inner`, `filter_inner`,
`filter_parallel`, `reduce_inner`, `reduce_parallel`, `collect_in_order`) all call
`expose_internal` and are external_body because it lacks strong enough ensures.

If `expose_internal` got ensures (even via assume), the other 11 could potentially
be proved. But right now they all show up as 12 equivalent holes, which:
- Makes it look like 12 independent problems when it's really 1
- Misleads agents into thinking they have 12 things to fix
- Inflates the "actionable" hole count

## What We Need

A way to annotate the dependency between holes, and for veracity to report it.

### Option A: Source annotation

A comment annotation like:
```rust
#[verifier::external_body]
// veracity: blocked_by(expose_internal)
fn split_inner(...) { ... }
```

Veracity would then report:
```
error: external_body [blocked_by: expose_internal] - fn split_inner
```

And in the summary:
```
Holes Found: 12 (actionable)
   1 × root cause external_body
   11 × downstream external_body (blocked by root causes)
```

### Option B: Automatic detection

Veracity could detect this automatically: if function A is external_body AND its body
calls function B which is also external_body, then A is "downstream" of B. The root
causes are external_body functions that don't call other external_body functions (or
whose only external calls are to vstd/stdlib).

This is harder to implement but requires no source annotations.

### Option C: Both

Support annotations for explicit marking, fall back to automatic detection for
unannotated code.

## Scope

- Report root-cause vs downstream in the per-file hole listing
- Include a summary line: "N root cause, M downstream"
- The total hole count should still include both (they're real holes)
- But the "actionable" framing should emphasize root causes

## Examples

**BSTParaMtEph.rs (Chap38):**
- Root cause: `expose_internal` (RwLock acquire_read boundary)
- Downstream: 11 functions that call expose_internal

**BSTParaTreapMtEph.rs (Chap39):**
- Root cause: `expose_internal` (same pattern)
- Downstream: ~14 functions

**NOT an example: ParaPair! functions.**
Functions using `ParaPair!` (e.g., Chap66 Boruvka, Chap62 StarContraction) are NOT
root causes. `ParaPair!` is fully verifiable — Chap06 and Chap36 prove through it
with zero holes. The pattern is named closures with explicit ensures. Functions
wrapped in external_body that use ParaPair are just unproved, not structurally blocked.
Do not classify ParaPair usage as a root cause.

## Impact

With this feature, the 30 holes in Chap38+39 would show as:
- 2 root causes (expose_internal in each file — RwLock boundary)
- 2 algorithmic assumes (find in each file — RwLock reader bridge)
- 26 downstream (blocked by expose_internal's missing ensures)

That tells the user: fix 2 functions and 26 others may fall. Much more actionable
than "30 holes, good luck."
