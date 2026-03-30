# R106 Agent 2 — Fix View type conformance across St/Mt variants, STEP 20

## Objective

`veracity-compare-par-mut` reports 21 errors where St and Mt variants disagree
on View types, return types, or ensures strength. Fix the View type mismatches
first, then the spec weakening. Work bottom-up through the chapter dependency
chain so downstream chapters see the fixed Views.

## Errors to fix

Run the tool yourself first to get current errors:
```bash
~/projects/veracity/target/release/veracity-compare-par-mut /home/milnes/projects/APAS-VERUS 2>&1 | grep " error"
```

### Priority 1: View type mismatches

The convention: St and Mt variants of the same data structure MUST have the same
View type (modulo T vs T::V projection). A Set should view as Set, a Map as Map,
a Graph as GraphView. If the St and Mt Views disagree, one of them is wrong.

To decide which is wrong: the View should match the **abstract data type**. A Set's
view is `Set<T::V>`. A Map's view is `Map<K::V, V::V>`. A Graph's view is
`GraphView<V::V>`. A Seq's view is `Seq<T::V>`.

When the St variant has `View = Seq<T>` but the Mt has `View = Set<T::V>`, the St
is probably wrong (using the backing store type as the View instead of the abstract
type). Fix the St variant.

When the Mt variant has a different View than St, and St's View matches the abstract
type, fix the Mt variant.

### Priority 2: Spec weakening

Where Mt has weaker ensures than St (just `ensures wf` instead of the full
functional spec), copy the St ensures to the Mt variant. The ensures should be
identical after variant-name substitution. The proof may need work to go through
the RwLock.

### Priority 3: Return type mismatches

Lower priority. Some are intentional (different backing stores). Report but don't
fix unless trivial.

## Work order (bottom-up by dependency chain)

1. **Chap37** — BSTRBMtEph: View Link<T> → BalBinTree<T> (fix Mt to match St)
   - Mt file is standalone, no downstream cascade
   - `scripts/validate.sh isolate Chap37`

2. **Chap41** — AVLTreeSetMtEph: View Seq<T::V> → Set<T::V> (fix Mt to match StPer)
   - Mt file is standalone, no downstream cascade
   - `scripts/validate.sh isolate Chap41`

3. **Chap05** — Check if SetStEph/SetMtEph View error is real or false positive
   - The main SetStEph struct already has `View = Set<T::V>` (line 79)
   - The error may be about LockedSetMtEph comparing against the wrong St struct
   - If false positive, report and move on
   - `scripts/validate.sh isolate Chap05`

4. **Chap06** — DirGraphStEph/MtEph View mismatch
   - Check if DirGraphStEph has `View = Seq<V>` (wrong) vs MtEph `View = GraphView<V::V>` (right)
   - If St is wrong, this cascades into Chap52-66. Fix St, then validate downstream.
   - Same for UnDirGraph, LabDirGraph, LabUnDirGraph
   - Also fix LabUnDirGraphMtEph spec weakening on `add_labeled_edge`
   - `scripts/validate.sh isolate Chap06`, then downstream chapters

5. **Chap38** — BSTParaMtEph spec weakening on insert/delete
   - `scripts/validate.sh isolate Chap38`

6. **Chap41** — AVLTreeSetMtPer/MtEph spec weakening on from_seq
   - `scripts/validate.sh isolate Chap41`

7. **Chap43** — OrderedSetStPer View Seq<T> → Set<T::V>, OrderedTableStPer View
   Seq<Pair<K,V>> → Map<K::V,V::V>
   - These cascade into Chap44, 52, 57, 59
   - Also fix OrderedSetStEph spec weakening on filter
   - Also fix AugOrderedTableMtEph spec weakening on get_key_range
   - `scripts/validate.sh isolate Chap43`, then downstream

## Important rules

- Work ONE chapter at a time. Validate clean before moving to the next.
- **Do NOT change View types in files that are correct.** Only change the variant
  that disagrees with the abstract data type.
- When changing a View type, ALL callers that use `self@` must be updated. Search
  with: `grep -rn "self@\[" src/ChapNN/` and `grep -rn "\.view()" src/ChapNN/`
- **Do NOT add assume or accept.**
- If a View change cascades into too many files (>20), stop, report what you found,
  and move to the next item. Do not spend all 20 steps on one View change.
- Mt files are standalone — Mt MUST NOT import from St counterparts.
- The tool may report false positives (comparing wrong struct pairs within a file).
  Triage each error before fixing.
- Read `src/standards/view_standard.rs` before starting.

## Triage first

Before fixing anything, run the tool and classify each error:
- **Real View mismatch**: the same abstract type has different Views across variants
- **False positive**: the tool is comparing different structs (e.g., LockedSetMtEph vs SetStEph)
- **Intentional**: documented reason for divergence (check comments near View impl)

Report the triage table in your report.

## Isolation

Use isolate per chapter:
```bash
scripts/validate.sh isolate Chap37
scripts/validate.sh isolate Chap41
# etc.
```

Full validate after all changes.

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 100

## Report

Write `plans/agent2-r106-view-conformance-report.md`.
