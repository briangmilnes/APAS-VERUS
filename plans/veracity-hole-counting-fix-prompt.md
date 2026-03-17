# Fix: Proof Hole Counting in veracity-review-proof-holes

You are working on `~/projects/veracity`. The binary is
`veracity-review-proof-holes`. Build with `cargo build --release`.

## The Problem

The "Holes Found" count is too high. It includes `accept()` calls and
`external_body_accept_hole` annotations as holes. These are not holes.
They are intentional, human-reviewed acceptances. The proof team uses
"Holes Found" as their work target — every item in that count must
represent real proof work that needs doing. Inflating it with reviewed
acceptances makes the number useless.

## Reference: Correct Output

Run against `~/projects/APAS-VERUS` at commit `9819f89c`, the correct
output is (our working version produces this):

```
Holes Found: 159 total
   20 × assume() (12%)
   1 × assume_specification (0%)
   2 × unsafe impl (1%)
   135 × external_body (84%)
   1 × external (0%)

Warnings: 40 total
   11 × fn_missing_requires
   16 × fn_missing_wf_requires
   11 × fn_missing_wf_ensures
   2 × requires_true

Accepted (reviewed): 327 total
   271 × accept()
   34 × external_body_accept_hole
   10 × trivial_spec_wf
   5 × external_accept_hole
   4 × unsafe_block_accept_hole
   1 × external_type_specification_accept_hole
   1 × struct_outside_verus_accept_hole
   1 × enum_outside_verus_accept_hole

Structural False Positives: 24 detected (language limitations, not missing proof)
   10 × STD_TRAIT_IMPL
   5 × RWLOCK_GHOST
   4 × THREAD_SPAWN
   3 × OPAQUE_EXTERNAL
   2 × UNSAFE_SEND_SYNC

Real Actionable Holes: 135 (159 total - 24 structural FPs in count)
```

## The Three Buckets

Every finding goes into exactly one of three buckets. The buckets are
mutually exclusive. Nothing is counted in more than one bucket.

### Bucket 1: Holes (the work list)

These are UNREVIEWED proof obligations. A proof engineer must write
proof code to close each one. This is the number the team works against.

What goes here:
- `assume(...)` — unverified assertion (NOT inside eq/clone body that
  has been converted to accept)
- `external_body` on a function that does NOT have an `// accept hole`
  or `accept_hole` comment
- `unsafe impl Send/Sync`
- `assume_specification`
- `#[verifier::external]` on a function that does NOT have an accept
  annotation
- `admit()`

What NEVER goes here:
- `accept()` — reviewed, not a hole
- `external_body` with `// accept hole` comment — reviewed, not a hole
- `external_type_specification` with accept — reviewed, not a hole
- `fn_missing_*` — spec warning, not a proof gap
- `requires_true` — spec warning
- `struct_outside_verus` — layout choice, not a proof gap
- `enum_outside_verus` — layout choice, not a proof gap
- `trivial_spec_wf` — spec quality warning

### Bucket 2: Warnings (spec quality issues)

These are not proof gaps. They are places where the spec could be
stronger or more complete. No proof code is needed — just adding or
fixing annotations.

What goes here:
- `fn_missing_requires`
- `fn_missing_wf_requires`
- `fn_missing_wf_ensures`
- `fn_missing_requires_ensures`
- `requires_true`

### Bucket 3: Accepted (human-reviewed, informational)

These WERE holes. A human reviewed them and decided they are either
structurally unprovable or intentionally accepted. They are done —
no more work needed.

What goes here:
- `accept()` calls
- `external_body` with `// accept hole` comment
- `external_type_specification` with accept annotation
- `external` with accept annotation
- `struct_outside_verus_accept_hole`
- `enum_outside_verus_accept_hole`
- `trivial_spec_wf` (these are reviewed)
- `unsafe_block_accept_hole`

## How to identify accept annotations

In the APAS-VERUS codebase, a hole is "accepted" when:

1. The code calls `accept(...)` instead of `assume(...)`. The `accept`
   function is defined in `crate::vstdplus::accept`. It has the same
   effect as assume but signals human review.

2. An `external_body` function has a comment containing `accept hole`
   or `accept_hole` (case insensitive). Example:
   ```rust
   #[verifier::external_body]  // accept hole — thread boundary
   fn spawn_worker(...)
   ```

3. An `external_type_specification` has a similar accept comment.

4. A struct/enum outside verus! has an accept comment.

## Structural False Positives

Structural FPs are a SUBSET of Bucket 1 (Holes). They are real holes
(assume or external_body, NOT accept) that can't be removed due to
language limitations. They stay in "Holes Found" but are separately
flagged and subtracted to get "Real Actionable Holes."

Categories:
- STD_TRAIT_IMPL: external_body on Iterator::next, Ord::cmp, etc.
- THREAD_SPAWN: external_body wrapping thread::spawn / HFScheduler
- RWLOCK_GHOST: assume() bridging ghost state across RwLock (NOT accept)
- OPAQUE_EXTERNAL: external_body calling unspecified std functions
- UNSAFE_SEND_SYNC: unsafe impl Send/Sync on Ghost-containing types

EQ_CLONE_ASSUME: assume() inside eq/clone bodies. In APAS-VERUS, most
of these have been converted to accept() — those go in Bucket 3, NOT
here. Only flag as EQ_CLONE_ASSUME if it's still `assume()` (not
`accept()`). Currently there are very few remaining.

## The Math

```
Real Actionable Holes = Holes Found - Structural FPs in count
```

That's it. Holes Found excludes Accepted. Structural FPs are a subset
of Holes Found. Real Actionable is the difference.

Do NOT compute: `Holes Found - (Structural FPs that happen to be in
the accepted bucket)`. Accepted items are already excluded from Holes
Found. You can't subtract them again.

## Validation

After making changes, run against `~/projects/APAS-VERUS` and verify:
- "Holes Found" is around 159 (not 400+)
- "Accepted" is around 327
- "Warnings" is around 40
- No accept() call appears in "Holes Found"
- No fn_missing_* appears in "Holes Found"
- "Real Actionable" = "Holes Found" minus structural FP count

If "Holes Found" exceeds 200, something is wrong — accept() items are
leaking into the hole count.
