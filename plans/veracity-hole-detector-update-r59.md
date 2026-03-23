# Veracity Proof Hole Detector: Remaining Improvements (R59)

You are working on `~/projects/veracity`. The binary is
`veracity-review-proof-holes`. Build with `cargo build --release`.
Test against `~/projects/APAS-VERUS/src/`.

## Context

Most bugs from the original PIP (`plans/veracity-hole-detector-pip.md`) are fixed.
Current state at APAS-VERUS commit `6a28f671`:

```
Holes Found: 24 (actionable)
   2 × assume(false) (8%)
   17 × assume() (70%)
   4 × external_body (16%)
   1 × external (4%)

Warnings: 178 total
   5 × fn_missing_requires
   16 × fn_missing_wf_requires
   6 × fn_missing_wf_ensures

Accepted (reviewed): 317 total
   271 × accept()
   14 × accept()
   10 × trivial_spec_wf
   5 × external_accept_hole
   4 × unsafe_block_accept_hole
   1 × struct_outside_verus_accept_hole
   1 × external_type_specification_accept_hole
   1 × enum_outside_verus_accept_hole

Structural (info only): 184
   149 × RWLOCK_GHOST
   12 × STD_TRAIT_IMPL
   10 × UNSAFE_SEND_SYNC
   9 × OPAQUE_EXTERNAL
   4 × THREAD_SPAWN
```

The three-bucket model (Holes / Warnings / Accepted) works correctly.
Default exclusions work. EQ_CLONE double-count is fixed. Section 4 has
content. Warning-only files show `⚠` not `❌`. UNSAFE_SEND_SYNC is detected.

## Remaining Work: 3 Items

### Item 1: Assume Subcategories (Enhancement)

The 17 `assume()` + 2 `assume(false)` holes are lumped together. They are
not all the same kind of proof work. Subcategorize them in the per-file
listing and summary:

| Subcategory | Detection Pattern | Example |
|---|---|---|
| `assume_rwlock_ghost` | `assume(...)` in a `*Mt*.rs` file that references `self@` or `spec_*_wf()` after an RwLock read/write | `assume(count == self@.len())` in OrderedSetMtEph.rs |
| `assume_closure_req` | `assume(f.requires(...))` or `assume(closure.requires(...))` | Closure spec propagation gap |
| `assume_algorithmic` | Any other `assume(...)` not matching the above | Real proof target — SMT work needed |
| `assume_false_unreachable` | `assume(false)` in a thread-join error arm or match arm after `diverge()` | Standard idiom, not real proof work |

In the summary, show:
```
Holes Found: 24 (actionable)
   2 × assume(false) [unreachable] (8%)
   5 × assume() [rwlock_ghost] (20%)
   8 × assume() [algorithmic] (33%)
   4 × assume() [closure_req] (16%)
   4 × external_body (16%)
   1 × external (4%)

Real Proof Targets: 13 (24 total - 9 rwlock_ghost - 2 unreachable)
```

The "Real Proof Targets" line is the number agents should work against.
RwLock ghost bridges and unreachable arms are structural — they can't be
closed without Verus language changes.

### Item 2: Root-Cause vs Downstream Holes (Enhancement)

When multiple `external_body` functions exist in one file, often one is the
root cause and the others are downstream (external_body only because they
call the root-cause function and can't get ensures from it).

**Detection (Option A — source annotation)**:
```rust
#[verifier::external_body]
// veracity: blocked_by(expose_internal)
fn split_inner(...) { ... }
```

Veracity reports:
```
error: external_body [blocked_by: expose_internal] - fn split_inner
```

**Detection (Option B — automatic)**:
If function A is external_body AND its body calls function B which is also
external_body in the same file, then A is downstream of B. Root causes are
external_body functions that don't call other external_body functions.

**Summary addition**:
```
External Body Breakdown:
   1 × root cause (fix these first)
   3 × downstream (may resolve when root causes are fixed)
```

With only 4 external_body holes remaining, this is less urgent than it was
at 135. But it will matter again as agents push into Chap47/53 territory.
Implement Option A (annotation-based) first — it's simpler and the user
controls the classification.

### Item 3: Per-Chapter Hole Logs (Bug Fix)

`scripts/all-holes-by-chap.sh` writes per-chapter logs to
`src/ChapNN/analyses/veracity-review-verus-proof-holes.log`. The script
may overwrite the log per-file rather than writing the full chapter.

**Fix**: Run `veracity-review-proof-holes src/ChapNN/` on the whole
directory, not individual files. The per-chapter log should contain all
files in the chapter.

**Validation**: After fix, `src/Chap47/analyses/veracity-review-verus-proof-holes.log`
should contain all Chap47 files (TriangularArrayMtEph.rs, etc.), not just
the last one scanned.

Note: This may be a bug in `scripts/all-holes-by-chap.sh` (APAS-VERUS
side) rather than in veracity itself. Check the script first.

## Priority

| # | Item | Value | Effort |
|---|------|-------|--------|
| 1 | Assume subcategories | High — distinguishes real targets from structural | Medium |
| 2 | Root-cause vs downstream | Medium — useful for orchestration | Medium |
| 3 | Per-chapter logs | Low — global log works fine | Low |

## Validation After Changes

Run against `~/projects/APAS-VERUS/src/` and verify:
- Holes Found still 24 (subcategorization must not change the total)
- Assume subcategories sum to total assume count (17 + 2 = 19)
- "Real Proof Targets" = total - rwlock_ghost - unreachable
- Any `blocked_by` annotations in source are reflected in output
- Per-chapter logs contain all files when regenerated

## Superseded Documents

These plans/ files drove fixes that already landed and can be archived:
- `veracity-hole-counting-fix-prompt.md` — three-bucket model implemented
- `veracity-hole-counting-rules.md` — counting rules implemented
- `veracity-hole-detector-pip.md` — 6 of 9 items done, remaining 3 captured above
- `veracity-proof-holes-fixes.md` — 3 of 4 issues fixed, Issue 4 captured above
- `veracity-root-cause-vs-downstream-holes.md` — captured as Item 2 above
