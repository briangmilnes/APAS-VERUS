# R100 Agent 3 — Fix AVLTreeSeqMtPer View: Seq<T> → Seq<T::V>, STEP 20

## Objective

AVLTreeSeqMtPer has `View = Seq<T>` but StPer has `View = Seq<T::V>`. Fix
MtPer to match.

## The fix

In `src/Chap37/AVLTreeSeqMtPer.rs`:

```rust
// Before:
impl<T: StTInMtT> View for AVLTreeSeqMtPerS<T> {
    type V = Seq<T>;
    open spec fn view(&self) -> Seq<T> { ... }
}

// After:
impl<T: StTInMtT> View for AVLTreeSeqMtPerS<T> {
    type V = Seq<T::V>;
    open spec fn view(&self) -> Seq<T::V> {
        self.backing@.map(|_i: int, t: T| t@)
    }
}
```

Check how StPer implements its View — mirror exactly.

## Cascade

After changing the View, every `self@[i]` returns `T::V` instead of `T`.
Callers that used `self@[i].field` need `self@[i]` (the view already applied).
Callers that used `self@[i]@` (double view) become `self@[i]` (single).

Find all callers:
```bash
grep -rn "AVLTreeSeqMtPer" src/ --include="*.rs" | grep -v analyses | grep -v "pub mod\|pub use"
```

Key callers likely in:
- `src/Chap41/AVLTreeSetMtPer.rs` — uses AVLTreeSeqMtPer as backing
- `src/Chap43/OrderedTableMtPer.rs` — may use it for entries

## Read first

- `src/Chap37/AVLTreeSeqMtPer.rs` — your file
- `src/Chap37/AVLTreeSeqStPer.rs` — StPer version (View = Seq<T::V>, your template)
- `src/Chap41/AVLTreeSetMtPer.rs` — primary caller

## Isolation

```bash
scripts/validate.sh isolate Chap41
```

(Pulls in Chap37 transitively.)

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify StPer files — they're correct.
- Do NOT touch Chap43 or Chap52 (other agents working there).
- If Chap41 cascade is too large, fix Chap37 first and report the
  Chap41 changes needed.
- This is a View refactor dry run for the bigger SetStEph refactor later.

## STEP 20

## Report

Write `plans/agent3-r100-avltreeseq-view-report.md`.
