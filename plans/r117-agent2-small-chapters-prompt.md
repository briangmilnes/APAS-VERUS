# R117 Agent 2 — Strengthen specs in Chap05, Chap18, Chap27, Chap49. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 18 warnings across four small chapters.
All are mechanical: missing functions, missing wf predicates, and missing
ensures clauses. Four quick wins in one prompt.

## Chapter breakdown

### Chap05 — SetMtEph (3 warnings)

1. **Missing fn `split`** (line 125): StEph has `split`, MtEph doesn't.
   Check if the underlying set supports split through the RwLock pattern.
   Implement if straightforward, document if blocked.

2. **`iter` missing 1 ensures** (line 143): StEph has 5 ensures, MtEph has 4.
   Missing: `forall|j: int| 0 <= j < it@.1.len() ==> self@.contains(#[trigger] ...)`.
   Add if the impl can prove it.

### Chap18 — ArraySeq (5 warnings)

1. **MtPer missing fn `inject`** (line 167): StPer has it, MtPer doesn't.
   Check if inject exists in MtPer under a different name or if it needs
   implementing.

2. **StEph missing ensures vs StPer** (4 warnings): StEph has fewer ensures
   than StPer on `append` (3 vs 4), `filter` (4 vs 5), `update` (3 vs 4),
   `inject` (2 vs 3). Check what the extra StPer ensures clause is in each
   case — likely a `spec_wf` ensures. Add to StEph if the impl proves it.

### Chap27 — ReduceContract/ScanContract (2 warnings)

1. **ReduceContractMtEph missing `reduce_contract`** (line 147): The core
   algorithm function. Check if it exists as a free function or is missing
   entirely. If it's a free function, lift it into the trait.

2. **ScanContractMtEph missing `scan_contract` and `expand_scan`** (line 229):
   Same pattern. Check and lift or implement.

### Chap49 — MinEditDist/SubsetSum (8 warnings)

All 8 warnings are `no spec_*_wf predicate found`. Four files each for
MinEditDist and SubsetSum (StPer, MtPer, StEph, MtEph).

These are DP modules. Read the StPer variant of each to understand the
struct, then add `spec_<module>_wf` predicates following the naming
convention: `spec_mineditdiststper_wf`, `spec_mineditdistmteph_wf`, etc.

The wf predicate should capture the real invariant of the struct — typically
that internal dimensions/indices are consistent. Read the struct fields and
the requires on public functions to understand what the wf should assert.

Do NOT add `requires true` or vacuous wf predicates. If the struct has no
meaningful invariant, report that and skip the wf.

## Work order

1. Read all affected files (St variants first for reference, then Mt variants).
2. Chap49 wf predicates first (8 warnings, most mechanical).
3. Chap18 ensures clauses (4 warnings, likely just adding wf ensures).
4. Chap27 missing functions (2 warnings, may need implementation).
5. Chap05 missing split + iter ensures (3 warnings).
6. Validate each chapter: `scripts/validate.sh isolate ChapNN`.
7. RTTs: `scripts/rtt.sh ChapNN`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from St counterparts.
- Adding requires may break callers — check and fix call sites.
- No subagents.
- Validate each chapter individually with isolate mode.

## STEP 30

## Report

Write `plans/agent2-r117-small-chapters-report.md`. Include before/after
warning count per chapter.
