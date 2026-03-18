# Veracity: Downgrade Structural False Positives from error to info

## Problem

The hole detector correctly identifies 41 structural false positives (SFPs)
and subtracts them from the "Real Actionable" count. But in the per-file
output (Section 1), these SFPs still display as `error:` lines:

```
/home/.../OrderedTableMtPer.rs:408: error: assume() [rwlock:reader] - proof { assume(inner@ =~= self@); }
```

AI agents reading this output see `error:` and try to fix these holes,
wasting entire rounds discovering they're structurally unfixable. The
`[rwlock:reader]` subcategory tag is there but agents don't reliably
parse it as "skip this."

## Fix

Change SFP entries from `error:` to `info:` severity in Section 1 output.
They already have `structural_false_positive` info lines — just stop
double-reporting them as errors.

### Current output (two lines per SFP):
```
/path/file.rs:408: error: assume() [rwlock:reader] - proof { assume(inner@ =~= self@); }
/path/file.rs:408: info: structural_false_positive RWLOCK_GHOST reader [high]
```

### Desired output (one line per SFP):
```
/path/file.rs:408: info: structural_false_positive RWLOCK_GHOST reader — assume(inner@ =~= self@) [high]
```

The SFP should appear ONLY as an `info:` line, NOT also as an `error:`.
Merge the assume text into the info line so context isn't lost.

## All 5 SFP categories to downgrade

| Category | Count | Pattern |
|----------|-------|---------|
| RWLOCK_GHOST | 22 | `assume(inner@ =~= self@)` and `assume(x.spec_*_wf())` after RwLock acquire |
| STD_TRAIT_IMPL | 10 | `assume` inside PartialEq::eq and Clone::clone bodies |
| THREAD_SPAWN | 4 | `external_body` on spawn/join in HFScheduler |
| OPAQUE_EXTERNAL | 3 | `external_body` on external_type_specification |
| UNSAFE_SEND_SYNC | 2 | `unsafe impl Send/Sync` |

## Impact on Summary (Section 3)

The "Holes Found" count should only include real actionable holes.
Currently it says:

```
Holes Found: 108 total
   30 × assume() (27%)
Real Actionable Holes: 67 (108 total - 41 structural FPs in count)
```

Change to:

```
Holes Found: 67 (actionable)
   7 × assume() [algorithmic]
   1 × assume() [closure]
   ...
Structural (info only): 41
   22 × RWLOCK_GHOST
   10 × STD_TRAIT_IMPL
   ...
```

The top-line "Holes Found" number should be the real actionable count.
SFPs get their own subsection. This way agents see "67 holes" not "108
holes" and don't mentally have to subtract.

## Per-File Summaries

The per-file hole counts (e.g., `13 × assume()`) should also exclude SFPs.
For OrderedSetMtEph.rs the current output says:

```
      13 × assume()
      1 × external_body
```

It should say:

```
      1 × external_body
      Info: 13 × structural_false_positive (RWLOCK_GHOST)
```

Because all 13 assumes in that file are RwLock bridges. The agent should
see "1 hole" not "14 holes."

## Testing

After the change, run on APAS-VERUS and verify:
1. Top-line "Holes Found" = 67 (not 108)
2. OrderedSetMtEph shows 1 hole (not 14)
3. OrderedTableMtPer shows 0 holes (not 8)
4. SFP details still visible as `info:` lines
5. No SFP line appears as `error:`
