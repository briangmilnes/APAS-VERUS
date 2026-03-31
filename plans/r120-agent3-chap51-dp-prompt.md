# R119 Agent 3 — Strengthen Chap51 DP specs (BottomUpDP + TopDownDP). AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 13 warnings on Chap51. Two DP framework
modules: BottomUpDP (6 warnings) and TopDownDP (7 warnings). Missing wf
predicates and missing Mt functions.

## Warnings

### BottomUpDP (6 warnings)

**Missing wf (4 warnings):**
- `BottomUpDPStPer.rs`: no spec_*_wf
- `BottomUpDPMtPer.rs`: no spec_*_wf
- `BottomUpDPStEph.rs`: no spec_*_wf
- `BottomUpDPMtEph.rs`: no spec_*_wf

**Missing Mt functions (2 warnings):**
- `BottomUpDPMtPer.rs`: missing `med_bottom_up`, `initialize_base_cases`,
  `compute_cell_value` from StPer
- `BottomUpDPMtEph.rs`: missing same 3 fns from StEph

Check if these are the core DP algorithm functions. If they exist as free
functions or inline code in Mt, lift them. If genuinely unimplemented,
assess complexity and document.

### TopDownDP (7 warnings)

**Missing wf (4 warnings):**
- `TopDownDPStPer.rs`: no spec_*_wf
- `TopDownDPMtPer.rs`: no spec_*_wf
- `TopDownDPStEph.rs`: no spec_*_wf
- `TopDownDPMtEph.rs`: no spec_*_wf

**Missing St function (1 warning):**
- `TopDownDPStEph.rs`: missing `with_memo_table` from StPer

**Missing Mt functions (2 warnings):**
- `TopDownDPMtPer.rs`: missing 9 fns from StPer: `spec_memo`,
  `spec_memo_correct`, `memo_size`, `is_memoized`, `get_memoized`,
  `with_memo_table`, `clear_memo`, `med_memoized`, `med_recursive`
- `TopDownDPMtEph.rs`: missing 9 fns from StEph: same list but
  `with_memo_table` → `insert_memo`

These are the memoization infrastructure functions. Check if MtPer/MtEph
have any of these under different names or as free functions.

## Strategy

1. Read all 8 files (St variants first for reference).
2. Assess wf predicates — DP modules may have struct fields (memo table,
   dimensions) that support meaningful wf. Add if real, skip if vacuous.
3. Check missing Mt functions — are they unimplemented or just unnamed?
4. For genuinely missing functions: implement if straightforward
   (lock-delegate-unlock), document if complex.
5. Validate: `scripts/validate.sh isolate Chap51`.
6. RTT: `scripts/rtt.sh Chap51`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept.
- No vacuous wf predicates.
- Mt standalone: do NOT import from St counterparts.
- No subagents.

## STEP 25

## Report

Write `plans/agent3-r119-chap51-dp-report.md`. Include before/after
warning count per file.
