# Agent3 spec_wf Work Plan

Date: 2026-03-11
Branch: agent3/ready
Standards: spec_wf_standard.rs, multi_struct_standard.rs, spec_naming_convention.rs

## Survey Results

The plan assigned 23 files across Chap52-59. After surveying every file:

| # | Chap | Files | Plan says | Actual state | Work needed |
|---|------|-------|-----------|--------------|-------------|
| 1 | 52 | 10 AdjSeq/AdjTable/EdgeSet | Need wf added | All 10 DONE: real wf bodies + full threading | None |
| 2 | 53 | PQMinStEph, PQMinStPer | Need wf added | No wf at all | Add wf from scratch |
| 3 | 56 | 4 F64 result files | Need wf added | wf exists as static method, not threaded | Convert to &self, thread |
| 4 | 57 | StackStEph | Need wf added | No wf; Vec wrapper, no invariant | See note below |
| 5 | 58 | BellmanFordStEph{F64,I64} | Need wf added | F64=placeholder, I64=algorithm-only | None |
| 6 | 59 | Johnson{St,Mt}Eph{F64,I64} | Need wf added | F64=placeholder, I64=algorithm-only | None |

**Actual file count: 7 files need work** (not 23).

## Phase 1: Chap56 F64 Result Files (4 files)

These files have wf predicates defined as static trait methods:
```rust
spec fn spec_ssspresultstephf64_wf(s: &SSSPResultStEphF64) -> bool;
```
Standard requires `&self` pattern:
```rust
spec fn spec_ssspresultstephf64_wf(&self) -> bool;
```

### Work per file:

**SSSPResultStEphF64.rs** (`src/Chap56/SSSPResultStEphF64.rs`)
- Convert `spec fn spec_ssspresultstephf64_wf(s: &SSSPResultStEphF64)` → `spec fn spec_ssspresultstephf64_wf(&self)`
- Convert `open spec fn` impl body from `s.` to `self.`
- Thread wf into: `new` ensures, `get_distance`/`set_distance`/`get_predecessor`/`set_predecessor` requires, `is_reachable` requires, `extract_path` requires
- For `&mut self` methods: `old(self).spec_*_wf()` in requires, `self.spec_*_wf()` in ensures
- For persistent `self` returns: `updated.spec_*_wf()` in ensures

**SSSPResultStPerF64.rs** (`src/Chap56/SSSPResultStPerF64.rs`)
- Same conversion: static → &self
- Thread wf: `new` ensures, all method requires
- Persistent methods (`set_distance`, `set_predecessor`) take `self` and return `Self` → `self.spec_*_wf()` in requires, `updated.spec_*_wf()` in ensures

**AllPairsResultStEphF64.rs** (`src/Chap56/AllPairsResultStEphF64.rs`)
- Convert static → &self
- Already has `new` ensures via `Self::spec_allpairsresultstephf64_wf(&empty)` — update syntax
- Thread wf into all other methods

**AllPairsResultStPerF64.rs** (`src/Chap56/AllPairsResultStPerF64.rs`)
- Same as StEph conversion
- Already has `new` ensures — update syntax
- Thread wf into all other methods

**Note**: The I64 counterparts use the same static method pattern but aren't in the plan. This creates a divergence. I will match the standard (&self) for F64 files as assigned.

### Validate after Phase 1

## Phase 2: Chap53 PQMin (2 files)

**PQMinStEph.rs** (`src/Chap53/PQMinStEph.rs`)
- Add to trait: `spec fn spec_pqminsteph_wf(&self) -> bool;`
- Add to impl: `open spec fn spec_pqminsteph_wf(&self) -> bool { ... }`
- wf body: `self.visited@.finite()` (AVLTreeSet is always finite, but this documents the expectation)
- Thread into: `pq_min` ensures `search.spec_pqminsteph_wf()`, `pq_min_multi` ensures `search.spec_pqminsteph_wf()`
- Also add wf to the free `pub fn pq_min` and `pub fn pq_min_multi` ensures
- Note: both main functions are `external_body`, so ensures are claims

**PQMinStPer.rs** (`src/Chap53/PQMinStPer.rs`)
- Mirror PQMinStEph pattern with `spec_pqminstper_wf`

### Validate after Phase 2

## Phase 3: Chap57 StackStEph (1 file) — DECISION NEEDED

**StackStEph.rs** (`src/Chap57/StackStEph.rs`)

The plan lists this file, but:
- It's a Vec wrapper with no structural invariant
- `spec_stacksteph_wf` would be `true` for all values
- The standard says "The inv function must carry a real invariant — never just true" (for RwLockPredicate, but the spirit applies)
- Adding `true` wf + threading adds noise without value

**Options**:
a. Add trivially-true wf for consistency (every data structure module gets one)
b. Skip — no meaningful invariant exists
c. Add wf with `self@.len() <= usize::MAX` (technically true for Vec, documents the bound)

**Additional issue**: The trait has NO specs — requires/ensures are only on the impl, which is non-standard. The spec_wf_standard says specs belong in the trait. This is a separate fix from wf.

### Validate after Phase 3

## Phase 4: Final validation

- `scripts/validate.sh` — zero errors
- `scripts/rtt.sh` — all runtime tests pass
- `scripts/ptt.sh` — all proof time tests pass
- Fix any trigger warnings found during validation
- Commit to agent3/ready, push

## Risk Assessment

- **Chap56 F64 conversion**: Low risk. The wf body doesn't change, just the calling convention. Threading adds requirements that may tighten caller constraints, but since callers (BellmanFord, Dijkstra, Johnson) are mostly `external_body`, no verification regressions expected.
- **Chap53 PQMin**: Low risk. Functions are `external_body`, so wf is just a claim in ensures.
- **Chap57 Stack**: Zero risk if skipped. Minimal risk if added.
- **No new `assume`, `admit`, or `external_body` will be introduced.**
