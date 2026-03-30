# R101 Agent 2 — AVLTreeSetMtEph View: Seq<T> → Set<T::V>, STEP 20

## Objective

AVLTreeSetMtEph has `View = Seq<T>` but StPer has `View = Set<T::V>`. Fix MtEph.

## The fix

In `src/Chap41/AVLTreeSetMtEph.rs`:

```rust
// Before:
impl<T: StTInMtT + Ord> View for AVLTreeSetMtEph<T> {
    type V = Seq<T>;  // WRONG
}

// After:
impl<T: StTInMtT + Ord> View for AVLTreeSetMtEph<T> {
    type V = Set<T::V>;
    open spec fn view(&self) -> Set<T::V> {
        self.tree@  // ParamBST views as Set<T::V>
    }
}
```

Check how StPer (`AVLTreeSetStPer.rs`) implements its View — mirror exactly.

## Cascade

Every `self@[i]` on an AVLTreeSetMtEph becomes invalid (Set has no indexing).
Replace with `self@.contains(x)`, `self@.len()`, etc.

Callers to check:
```bash
grep -rn "AVLTreeSetMtEph" src/ --include="*.rs" | grep -v analyses | grep "@"
```

Key callers likely in Chap52 (graph modules) and Chap61-66 (graph algorithms).
Most already use `self@.contains()` pattern through the trait. The `self@[i]`
usage is probably limited.

## Read first

- `src/Chap41/AVLTreeSetMtEph.rs` — your file
- `src/Chap41/AVLTreeSetStPer.rs` — View = Set<T::V> (your template)
- `src/Chap41/AVLTreeSetStEph.rs` — View = Set<T::V> for comparison

## Isolation

```bash
scripts/validate.sh isolate Chap41
```

Then check callers:
```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify StEph or StPer files.
- Do NOT touch Chap43 or Chap52 source files (other agents working there).
  Only check that Chap52 still compiles after your Chap41 change.
- If cascade is too large, fix Chap41 and report what callers need updating.

## STEP 20

## Report

Write `plans/agent2-r101-avltreeset-view-report.md`.
