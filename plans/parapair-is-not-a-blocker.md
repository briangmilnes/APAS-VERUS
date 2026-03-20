# ParaPair! Is NOT a Blocker — Proof Pattern for Agents

## The Misconception

Agents have claimed that functions using `ParaPair!` are "blocked" or "structural
limits" that can't be proved. This is wrong. Chap06 and Chap36 — both clean
chapters with zero holes — use `ParaPair!` extensively and verify fully.

## The Pattern

`ParaPair!` works with Verus when you use **named closures with explicit ensures**.
Verus verifies each closure arm independently and propagates the ensures to the
`Pair` result.

### Example 1: Chap36/QuickSortMtEph.rs (verified, 0 holes)

```rust
let ghost left_view = left_seq@;
let ghost right_view = right_seq@;

let f1 = move || -> (v: ArraySeqMtEphS<T>)
    ensures sorted_by(v@, leq), v@.to_multiset() == left_view.to_multiset()
{
    Self::quick_sort_first(&mut la);
    la.seq
};

let f2 = move || -> (v: ArraySeqMtEphS<T>)
    ensures sorted_by(v@, leq), v@.to_multiset() == right_view.to_multiset()
{
    Self::quick_sort_first(&mut ra);
    ra.seq
};

let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

// After this line, Verus knows:
//   sorted_by(sorted_left@, leq)
//   sorted_left@.to_multiset() == left_view.to_multiset()
//   sorted_by(sorted_right@, leq)
//   sorted_right@.to_multiset() == right_view.to_multiset()
```

### Example 2: Chap06/LabDirGraphMtEph.rs (verified, 0 holes)

```rust
let ghost v_left = v@;
let ghost v_right = v@;

let f1 = move || -> (out: SetStEph<V>)
    ensures out.spec_setsteph_wf(),
            out@ == g_left.spec_n_plus_from_set(v_left, left_arcs@)
{ g_left.n_plus_par(v_left, left_arcs) };

let f2 = move || -> (out: SetStEph<V>)
    ensures out.spec_setsteph_wf(),
            out@ == g_right.spec_n_plus_from_set(v_right, right_arcs@)
{ g_right.n_plus_par(v_right, right_arcs) };

let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
```

## Steps to Prove a ParaPair Function

1. **Read `src/standards/using_closures_standard.rs`** before writing any closure code.

2. **Capture ghost state before the closures.** Closures capture by move, so any
   spec-level values needed in ensures must be captured as `let ghost`:
   ```rust
   let ghost left_view = left@;
   let ghost right_view = right@;
   ```

3. **Write named closures with explicit ensures.** Never use inline closures.
   The ensures must be provable from the closure body:
   ```rust
   let f1 = move || -> (result: ReturnType)
       ensures result@ == some_spec(left_view)
   { compute_left(left) };
   ```

4. **Call `ParaPair!` or `crate::ParaPair!`:**
   ```rust
   let Pair(left_result, right_result) = crate::ParaPair!(f1, f2);
   ```

5. **Use the ensures in subsequent proof blocks.** Verus propagates the closure
   ensures to the pair results automatically.

## What Goes Wrong

- **Missing ensures on closures**: Verus can't propagate what isn't stated.
- **Inline closures**: `ParaPair!(move || foo(), move || bar())` — Verus can't
  attach ensures to inline closures.
- **Ghost capture**: If the ensures references `left@` but `left` was moved into
  the closure, the ensures can't see it. Capture `let ghost left_view = left@`
  before the closure and use `left_view` in ensures.
- **Weak callee ensures**: If the function called inside the closure doesn't have
  strong enough ensures, the closure can't state strong ensures either. Fix the
  callee's ensures first.

## Application to Chap66/BoruvkaMtEph.rs

The 9 "blocked" holes in Boruvka all use `ParaPair!` for divide-and-conquer.
They are NOT blocked. To prove them:

1. Ensure helper functions called inside ParaPair arms have ensures.
2. Write named closures with ghost captures and explicit ensures.
3. Remove external_body.

The pattern is identical to QuickSort (Chap36) and Graph neighbors (Chap06).
