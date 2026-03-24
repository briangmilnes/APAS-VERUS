# R69 Agent 4: OrderedSetStEph + BSTTreapStEph Cleanup

## Goal

Clean up remaining holes in two small files. Target: 3 + 1 → 0 + 0.

## File 1: `src/Chap43/OrderedSetStEph.rs` (3 holes)

### Hole 1: Unsafe iterator (line ~975)

```rust
Some(unsafe { &*ptr })
```

The iterator uses a raw pointer dereference. The standard iterator pattern (see
`src/standards/iterators_standard.rs`) uses `std::slice::Iter<'a, T>` — no unsafe needed.

**Fix**: Rewrite the iterator to wrap `std::slice::Iter` instead of using raw pointers.
Reference implementation: `src/Chap18/ArraySeqStEph.rs` iterator section.

The current iterator struct likely has manual `pos` and `elements` fields. Replace with:
```rust
pub struct OrderedSetStEphIter<'a, T> {
    pub inner: std::slice::Iter<'a, T>,
}
```

Update `Iterator::next`, `View`, ghost iterator, and `ForLoopGhostIterator` impls to
match the standard pattern.

### Holes 2-3: Clone assumes in algorithmic code (lines ~793, ~809)

```rust
let k1_clone = k1.clone();
proof { assume(k1_clone@ == k1@); }
```

These are in `get_key_range`, NOT inside `Clone::clone` body. Per the standard, clone
assumes belong ONLY in `Clone::clone` bodies. In algorithmic code, `k1_clone@ == k1@`
should flow from Clone's `ensures cloned@ == self@`.

**Fix**: Check if the `Clone` impl for `T` has `ensures cloned@ == self@`. If so, the
assume is redundant — delete it. If Clone lacks that ensures, add it to Clone (following
`src/standards/partial_eq_eq_clone_standard.rs`), then delete the assume.

The `T: StT` bound includes `Clone`. Check what `StT`'s Clone impl ensures.

## File 2: `src/Chap39/BSTTreapStEph.rs` (1 hole)

### Hole: Clone assume in reduce_inner_st (line ~2652)

```rust
proof { assume(left_base == identity); } // eq/clone workaround
```

This is in `reduce_inner_st`, not in a Clone body. `left_base` is the result of
`identity.clone()`. If `Clone::clone` ensures `cloned@ == self@` for the identity
type, this assume is redundant.

**Fix**: Same approach — check Clone ensures, delete assume if covered.

## Steps

1. **Read** `src/standards/iterators_standard.rs` and `src/Chap18/ArraySeqStEph.rs`
   (reference iterator implementation)
2. **Read** `src/standards/partial_eq_eq_clone_standard.rs` (clone ensures pattern)
3. **Read** `src/Chap43/OrderedSetStEph.rs` — understand current iterator and
   get_key_range
4. **Rewrite** OrderedSetStEph iterator to standard `std::slice::Iter` pattern
5. **Fix** or **remove** clone assumes in get_key_range (lines ~793, ~809)
6. **Read** `src/Chap39/BSTTreapStEph.rs` — fix reduce_inner_st clone assume
7. **Validate**, **rtt**, **ptt**

## Constraints

- Only modify `src/Chap43/OrderedSetStEph.rs` and `src/Chap39/BSTTreapStEph.rs`.
- Do NOT modify other files.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially.
