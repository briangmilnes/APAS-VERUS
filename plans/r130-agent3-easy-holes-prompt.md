# R130 Agent 3 — Close easy proof holes in Chap43, 52, 53. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- Standard 4 (`spec_wf_standard.rs`) — wf predicates and propagation
- Standard 22 (`capacity_bounds_standard.rs`) — size bounds in requires/ensures

Report file: `plans/r130-agent3-easy-holes-report.md`

## Targets

### Chap43: 2 holes

1. `src/Chap43/OrderedSetMtEph.rs:547` — `assume(inner@.len() + 1 < usize::MAX)`
   This is a capacity bound. The fix is to add `self@.len() < usize::MAX - 1` (or similar)
   to the `requires` of the enclosing function, so the caller proves the bound.
   Read standard 22.

2. `src/Chap43/OrderedTableStPer.rs:3495` — `fn from_sorted_entries` missing wf ensures.
   Add `ensures result.spec_orderedtablestper_wf()` and prove it from the function body.
   Read standard 4.

### Chap52: 1 hole

3. `src/Chap52/AdjTableGraphMtPer.rs:450` — `assume(neighbors.spec_avltreesetmtper_wf())`
   The neighbors set comes from the graph's adjacency table. The wf should follow from the
   graph's own wf predicate. Read the graph's wf spec and propagate the wf through.

### Chap53: 1 hole

4. `src/Chap53/GraphSearchMtPer.rs:231` — `assume(neighbors.spec_avltreesetmtper_wf())`
   Same pattern as Chap52 — neighbors wf should propagate from graph wf.

## Approach

For each hole:
1. Read the function and surrounding context.
2. Identify where the assumed property should come from (wf propagation, requires, ensures).
3. Add the appropriate requires/ensures to propagate the bound.
4. Verify the fix doesn't break callers (check who calls the modified function).

## Validation

Run `scripts/validate.sh isolate Chap43` then `scripts/validate.sh isolate Chap52`
then `scripts/validate.sh isolate Chap53`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add new assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Propagate wf through requires/ensures — that's the fix, not assuming.
