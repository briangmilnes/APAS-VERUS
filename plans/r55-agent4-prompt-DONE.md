<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 4 — Round 55 Prompt

## Branch

Work on `agent4/ready`. Base: `045bf2ce9`.

## DO NOT TOUCH

- Chap47 (any file)
- Chap37 (any file — Agent 3 is working there)
- Chap41/AVLTreeSetStEph.rs (Agent 2)
- Chap41/AVLTreeSetStPer.rs (Agent 3)
- Chap43/OrderedTableMtPer.rs (Agent 3)
- Chap43/OrderedSetStEph.rs (blocked, needs Chap41 sortedness first)
- Chap43/OrderedSetStPer.rs (blocked, needs Chap41 sortedness first)

## Assignment: Chap43 AugOrderedTable holes (attempt 2 hard holes)

### Task 1: AugOrderedTableMtEph.rs — remove external_body from reduce_range_parallel

**File:** `src/Chap43/AugOrderedTableMtEph.rs`
**Line:** 672
**Hole:** `#[verifier::external_body]` on `reduce_range_parallel`

This function does parallel recursive range reduction. It is currently `external_body`
because:
1. It uses `ParaPair!` with inline closures (specs don't propagate)
2. `get_key_range` doesn't ensure wf on the result
3. Closure-clone-requires issue for `MtReduceFn`

**Strategy:**

Read the function body first. Then:

1. Convert `ParaPair!` to HFScheduler `join()` with named closures that have explicit
   `ensures`. Read `src/standards/using_closures_standard.rs` and
   `src/standards/hfscheduler_standard.rs` first.

2. Capture ghost views before the `move`:
   ```
   let ghost left_view = left_table@;
   let f1 = move || -> (r: V) ensures ... { left_table.reduce_val() };
   ```

3. For the `get_key_range` wf issue: if `get_key_range` doesn't ensure wf, you may need
   to add wf to its ensures in the trait (check `AugOrderedTableMtEphTrait`). This may
   cascade to the StEph version.

4. For the closure-clone-requires issue on the reducer function: if the function clones
   the reducer to pass to subcalls, you may need an `assume(obeys_feq_clone)` pattern
   at the entry point — ask yourself whether this is a structural boundary or algorithmic
   logic before adding any assume.

**If this proves too blocked, document exactly what you tried and what failed.**

### Task 2: AugOrderedTableStPer.rs — closure-clone in lemma_reducer_clone_total

**File:** `src/Chap43/AugOrderedTableStPer.rs`
**Line:** 124
**Hole:** `assume(forall|v1: &V, v2: &V| #[trigger] cloned.requires((v1, v2)))`

This is a proof lemma that says "cloning a total reducer preserves totality." The assume
is because Verus cannot prove that `Clone::clone()` on an `Fn` trait object preserves
`requires`.

**Strategy options:**

(a) **Concrete reducer type**: Instead of generic `F: Fn(&V, &V) -> V + Clone`, define a
    `ReducerFn` struct wrapping a function pointer, implement Clone manually with a verified
    ensures, and use that instead of the generic. This avoids the Fn-clone opacity.

(b) **External axiom**: Write a small `#[verifier::external_body]` proof helper that axiomatizes
    `Fn::clone` preserving requires. This is a tight, well-scoped trust boundary.

(c) **Accept the hole**: If neither (a) nor (b) works, document why and leave the hole with
    a clear explanation. Do NOT add `accept()` without asking.

Try (a) first — it's the cleanest solution. If the AugOrderedTable API is locked to
`F: Fn(&V, &V) -> V + Clone`, then try (b).

## Validation

Run `scripts/validate.sh` after each change. Show full output. Fix all warnings.
Do not leave trigger warnings.

## Report

Write `plans/agent4-round55-report.md` with holes before/after table including
Chap column. For each hole attempted, document what you tried, what worked/failed,
and what blocks further progress.
