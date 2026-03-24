# R73 — Prove rank_key_iter in OrderedTableStEph.rs

## Objective

Remove the `external_body` from `rank_key_iter` at line 3409 in
`src/Chap43/OrderedTableStEph.rs` and prove its postcondition.

## The function

```rust
fn rank_key_iter(&self, k: &K) -> (rank: usize)
    where K: TotalOrder
    requires
        self.spec_orderedtablesteph_wf(),
        obeys_view_eq::<K>(),
    ensures
        self@.dom().finite(),
        rank <= self@.dom().len(),
        rank as int == self@.dom().filter(
            |x: K::V| exists|t: K| #![trigger t@] t@ == x
                && TotalOrder::le(t, *k) && t@ != k@
        ).len();
```

## The body (already written, just needs proof)

The implementation iterates `self.tree.in_order()`, counts entries whose key
is strictly less than `k` using `TotalOrder::cmp`. The logic is correct —
the proof needs to connect the loop's count to the filter-based spec.

## Proof strategy

1. Remove `#[verifier::external_body]` and the agent-ownership comment.
2. Add loop invariants to the `while i < len` loop:
   - `count` equals the number of elements in `sorted[0..i]` that are
     strictly less than `k` (by view).
   - Connect `sorted` (the in-order traversal) to `self@.dom()` — the
     in-order sequence projected to keys should equal the domain elements.
   - The filter on `self@.dom()` counting `TotalOrder::le(t, *k) && t@ != k@`
     is equivalent to counting strictly-less-than elements.
3. After the loop, assert the count equals the filter length.
4. You will likely need lemmas from `vstd` about `Seq::filter` and `Set::filter`.
   Search with `veracity-search 'filter.*len'` and in `~/projects/verus/source/vstd/`.

## Dependencies

- `in_order()` returns the sorted sequence of `Pair<K, V>`.
- `self@` is the abstract map `Map<K::V, V::V>`.
- `self@.dom()` is a `Set<K::V>`.
- The connection between `in_order()` elements and `self@.dom()` is key.
  Look at how other proved functions in the same file bridge these.

## Validation

Run `scripts/validate.sh` after removing the external_body. If the proof
is hard, add intermediate assertions and lemma calls. Do not re-add
external_body — leave the corpse if stuck and report what you tried.

## Rules

- Read `CLAUDE.md` on startup.
- Read the full OrderedTableStEph.rs file (or at least sections 6-9) to
  understand the spec functions, lemmas, and proof patterns already in use.
- Do NOT weaken the ensures. The spec is from APAS.
- Do NOT add assume or accept.
