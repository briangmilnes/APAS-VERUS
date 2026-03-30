# R110 Agent 3 — Prove AVLTreeSeqMtPer map_values assume. AFK. PBOGH.

## Objective

Eliminate the last assume in Chap37. Line 687 of `src/Chap37/AVLTreeSeqMtPer.rs`:

```rust
assume(out@.map_values(|t: T| t@) =~= self.spec_seq());
```

This is in `values_in_order`, which calls `inorder_collect` to push cloned elements
into a Vec, then claims the views of those elements match `spec_seq()` (which is
`spec_inorder(self.root)`).

## The proof obligation

`inorder_collect` does an in-order traversal, pushing `n.value.clone()` at each node.
After the traversal, `out` contains cloned elements in in-order. We need:

```
out@.map_values(|t: T| t@) =~= spec_inorder(self.root)
```

Where `spec_inorder` is defined recursively (line 116):
```rust
pub open spec fn spec_inorder<T: StTInMtT>(link: Link<T>) -> Seq<T::V>
    decreases link,
{
    match link {
        None => Seq::<T::V>::empty(),
        Some(node) => spec_inorder(node.left) + seq![node.value@] + spec_inorder(node.right),
    }
}
```

## Why this is hard (and why you will do it anyway)

`inorder_collect` currently has `ensures true` (line 499). That's where the real
work is — strengthening `inorder_collect`'s ensures to state what it actually does.

The ensures should say something like:
```
out@.map_values(|t: T| t@) =~=
    old(out)@.map_values(|t: T| t@) + spec_inorder(*cur)
```

i.e., the function appends the in-order sequence of `cur` to whatever was already
in `out`. Then `values_in_order` starts with empty `out` and the conclusion follows.

The proof will need:
1. A loop-free recursive argument (inorder_collect is recursive, not a loop).
2. `clone` preserves view: `cloned(x, y) ==> y@ == x@`. Already used elsewhere
   in this file (line 532).
3. `map_values` distributes over `+` (concatenation) and `push`. This may need
   a helper lemma if vstd doesn't have it. Check vstd first:
   `veracity-search 'map_values.*push\|map_values.*add\|map_values.*concat'`
4. The recursive structure: after left traversal, push, right traversal, the
   output is `old_out + inorder(left) + [val@] + inorder(right)` which equals
   `old_out + inorder(Some(node))`.

## Read first

- `src/Chap37/AVLTreeSeqMtPer.rs` — full file, especially:
  - `spec_inorder` (line 116) — recursive spec
  - `inorder_collect` (line 497) — exec, `ensures true`
  - `values_in_order` (line 680) — caller with the assume
  - `build_balanced_from_slice` (line 509) — working proof of similar property
  - `lemma_size_eq_inorder_len` (line 169) — existing recursive lemma pattern
- `src/standards/mut_standard.rs` — &mut patterns in Verus

## Search before writing lemmas

```bash
veracity-search 'map_values'
veracity-search 'spec fn map_values'
```

Check if vstd has `map_values` distribution lemmas. If not, write them locally.

## Isolation

```bash
scripts/validate.sh isolate Chap37
```

## Rules

- Do NOT add assume or accept. You are REMOVING an assume, not adding new ones.
- Do NOT weaken ensures. The target is `out@.map_values(|t: T| t@) =~= self.spec_seq()`.
- Do NOT leave `inorder_collect` with `ensures true`. Give it a real postcondition.
- This is one assume in one function. It is provable. The recursive structure
  mirrors `spec_inorder` exactly. Do the proof work. PBOGH.
- If you need a `map_values` distribution lemma, write it as a proof fn in this
  file. Keep it minimal.
- No subagents.

## STEP 20

## Report

Write `plans/agent3-r110-map-values-report.md`.
