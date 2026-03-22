<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
# Agent 1 — Round 50 (Revised)

## Primary Target: Chap47 ParaHashTableStEph — 3 Holes + 8 Warnings

Target: `src/Chap47/ParaHashTableStEph.rs` (787 lines, 3 holes, 8 warnings)

### Holes

| # | Line | Type | Description |
|---|------|------|-------------|
| 1 | 116 | assume (algorithmic) | `assume(c == *x)` — Clone bridge for T::clone |
| 2 | 494 | external_body | Function body not verified |
| 3 | 112 | fn_missing_requires | `clone_elem` missing requires |

### Warnings (fn_missing_wf)

| # | Line | Function | Warning |
|---|------|----------|---------|
| 1 | 618 | createTable | fn_missing_wf_ensures — needs spec_hashtable_wf() |
| 2 | 669 | insert | fn_missing_wf_requires — needs spec_hashtable_wf() |
| 3 | 686 | lookup | fn_missing_wf_requires — needs spec_hashtable_wf() |
| 4 | 696 | delete | fn_missing_wf_requires — needs spec_hashtable_wf() |
| 5 | 709 | metrics | fn_missing_wf_requires — needs spec_hashtable_wf() |
| 6 | 718 | loadAndSize | fn_missing_wf_requires — needs spec_hashtable_wf() |
| 7 | 735 | resize | fn_missing_wf_requires — needs spec_hashtable_wf() |

### Approach

1. **Read the file** thoroughly first. Understand the hash table design and existing specs.
2. **Fix fn_missing_wf warnings**: Add `self.spec_hashtable_wf()` (or the correct wf
   predicate name) to requires/ensures where veracity flags it. These are real
   preconditions, not tautologies. The trait signature is the source of truth — update
   trait declarations first, then impls.
3. **Hole #1 (clone_elem assume)**: This is the same Clone-view pattern seen elsewhere.
   Use the `obeys_feq_clone` propagation pattern from `partial_eq_eq_clone_standard.rs`
   if applicable, or add the assume to the function's requires and propagate up.
4. **Hole #2 (external_body at line 494)**: Read the function, understand what it does,
   and provide a verified body. May need loop invariants.
5. **Hole #3 (fn_missing_requires on clone_elem)**: Add the real requires once you
   understand what clone_elem needs.

### Secondary: Chap65 Kruskal + Prim (2 holes + 1 warning)

If Chap47 work completes early:

- `src/Chap65/KruskalStEph.rs:58` — external_body on sort_edges (238 lines total)
- `src/Chap65/PrimStEph.rs:95` — external_body on prim_mst (202 lines total)
- `src/Chap65/PrimStEph.rs:72` — fn_missing_requires on pq_entry_new

### Standards to Read

- `src/standards/partial_eq_eq_clone_standard.rs` (Clone patterns)
- `src/standards/capacity_bounds_standard.rs` (if insert needs capacity)
- `src/standards/spec_wf_standard.rs` (wf naming)

### Validation

Run each step separately, fix errors between steps:
- `scripts/validate.sh` — 0 errors, no trigger warnings
- `scripts/rtt.sh` — all pass
- `scripts/ptt.sh` — all pass

### Constraints

- Do NOT add new assumes, external_body, or accept()
- Do NOT weaken ensures clauses
- Do NOT add `requires true` or tautological requires
- Do NOT add `// veracity: no_requires` annotations

### Current State

- Main at post-merge of agents 1-4 R49/R50
- 4450 verified, 2613 RTT, 147 PTT
