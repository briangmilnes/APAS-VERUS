# R101 Agent 3 — OrderedSetStPer View: Seq<T> → Set<T::V>, STEP 20

## Objective

OrderedSetStPer has `View = Seq<T>` but MtEph has `View = Set<T::V>`. Fix StPer.

## The fix

In `src/Chap43/OrderedSetStPer.rs`:

```rust
// Before:
impl<T: StT + Ord> View for OrderedSetStPer<T> {
    type V = Seq<T>;  // WRONG
}

// After:
impl<T: StT + Ord> View for OrderedSetStPer<T> {
    type V = Set<T::V>;
    open spec fn view(&self) -> Set<T::V> {
        // Convert backing ordered sequence to set
        self.backing@.to_set()  // or however the backing maps to Set
    }
}
```

Check how OrderedSetMtEph implements its View — mirror that.

## Cascade

Every `self@[i]` on an OrderedSetStPer becomes invalid. Replace with
`self@.contains(x)`, `self@.len()`, etc. The `to_seq()` method still
returns a `Seq` — callers that need indexing use `to_seq()`.

## Read first

- `src/Chap43/OrderedSetStPer.rs` — your file
- `src/Chap43/OrderedSetMtEph.rs` — View = Set<T::V> (your template)
- `src/Chap43/OrderedSetStEph.rs` — check its View for comparison

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify OrderedSetMtEph or OrderedSetStEph.
- Do NOT modify OrderedTableMtPer (agent2 R100 just changed it).
- If cascade touches too many files, fix the View impl and report callers.

## STEP 20

## Report

Write `plans/agent3-r101-orderedset-view-report.md`.
