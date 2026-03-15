# Agent 1 — Round 18: Add Ghost(spec_fn) to filter Across Chap38+41+42

## Mission

Add the `Ghost(spec_fn)` parameter to every `filter` function in Chap38, Chap41, and
Chap42. This closes the "no way to specify filter completeness" gap that you punted on
in R17. The pattern already exists in `src/standards/using_closures_standard.rs` — you
are implementing it, not inventing it.

## Required Reading

**Before writing any code**, read `src/standards/using_closures_standard.rs`. Pattern C
(Ghost spec_fn filter) is the exact pattern you need.

## Exact Pattern

From `src/standards/using_closures_standard.rs` lines 91-102:

```rust
fn filter<F: Fn(&T) -> bool>(
    &self,
    pred: &F,
    Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
) -> (filtered: Self) where T: Copy
    requires
        forall|x: &T| #[trigger] pred.requires((x,)),
        forall|x: T, keep: bool|
            pred.ensures((&x,), keep) ==> keep == spec_pred(x),
    ensures
        filtered@ <= self@,  // subset_of (keep existing ensures)
        forall|v: T::V| #[trigger] filtered@.contains(v)
            ==> self@.contains(v) && spec_pred(v),  // forward: every result satisfies pred
        forall|v: T::V| self@.contains(v) && spec_pred(v)
            ==> #[trigger] filtered@.contains(v),  // backward: every satisfying elem is included
;
```

The two key additions beyond what's currently there:
1. The `Ghost(spec_pred)` parameter — a ghost spec function that mirrors the exec closure.
2. The backward completeness ensures: `self@.contains(v) && spec_pred(v) ==> filtered@.contains(v)`.

For table filter (Chap42), the spec_pred takes two parameters (key and value):
```rust
fn filter<F: Fn(&T, &V) -> bool>(
    &self,
    pred: &F,
    Ghost(spec_pred): Ghost<spec_fn(T::V, V::V) -> bool>,
) -> (filtered: Self)
    requires
        forall|k: &T, v: &V| #[trigger] pred.requires((k, v)),
        forall|k: T, v: V, keep: bool|
            pred.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
    ensures
        forall|k: T::V| #[trigger] filtered@.dom().contains(k)
            ==> self@.dom().contains(k),
        forall|k: T::V| filtered@.dom().contains(k)
            ==> filtered@[k] == self@[k],
        forall|k: T::V| self@.dom().contains(k) && spec_pred(k, self@[k])
            ==> #[trigger] filtered@.dom().contains(k),
;
```

## Files to Modify

### Chap38 (1 file)

| # | File | Trait fn | Current |
|---|------|----------|---------|
| 1 | BSTParaStEph.rs | filter | subset_of only, no Ghost |

### Chap41 (3 files)

| # | File | Trait fn | Current |
|---|------|----------|---------|
| 2 | ArraySetStEph.rs | filter | subset_of only, no Ghost |
| 3 | AVLTreeSetStEph.rs | filter | subset_of only, no Ghost |
| 4 | AVLTreeSetStPer.rs | filter | subset_of only, no Ghost |

### Chap42 (2 files)

| # | File | Trait fn | Current |
|---|------|----------|---------|
| 5 | TableStEph.rs | filter | domain subset only, no Ghost |
| 6 | TableStPer.rs | filter | domain subset only, no Ghost |

## Procedure

1. **Read** `src/standards/using_closures_standard.rs` — the filter pattern (Pattern C).
2. For each file:
   a. **Read** the current filter trait signature.
   b. **Add** `Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>` parameter to the trait fn.
   c. **Add** the requires clauses: callable + mirrors spec_pred.
   d. **Add** the backward completeness ensures.
   e. **Update all impl fn signatures** to match the new trait signature.
   f. **Update all call sites** — add `Ghost(spec_pred)` ghost argument.
   g. If the impl body can't prove the new ensures, add `#[verifier::external_body]`.
3. `scripts/validate.sh` — 0 errors.
4. For Mt/MtPer wrappers that delegate to the StEph/StPer filter: update their signatures
   to pass through the Ghost parameter.

## Important

- The trait signature change is the correct fix. Do NOT avoid it.
- If adding Ghost(spec_pred) to the trait breaks call sites, fix the call sites — add
  `Ghost(spec_pred)` as a ghost argument. If no spec_pred is available at the call site
  (e.g., in a test), use `Ghost(|_| true)` or `Ghost(|_: T::V| arbitrary())`.
- Do NOT skip this because "it changes the trait signature." That's the point.
- Do NOT punt with "structural blocker." The standard file shows exactly how to do it.
- Add `external_body` to impl bodies that can't prove the new ensures. Strong spec +
  external_body > weak spec + proved body.
- Chap43 filter is handled by Agent 2 — do NOT modify Chap43 files.

## Deliverables

- Modified filter in 6 trait files + corresponding impl files.
- Updated call sites (including tests, Mt wrappers).
- `plans/agent1-round18-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
