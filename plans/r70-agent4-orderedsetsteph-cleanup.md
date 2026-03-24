# R70 Agent 4: OrderedSetStEph — Clone Assumes + Iterator Unsafe

## Goal

Eliminate all 3 proof holes in `src/Chap43/OrderedSetStEph.rs`. Target: 3 → 0.

## Current Holes (3)

Run `scripts/holes.sh src/Chap43/OrderedSetStEph.rs` first to get exact current list.

| # | Line | Type | Content |
|---|------|------|---------|
| 1 | 785 | assume | `assume(k1_clone@ == k1@)` in get_range |
| 2 | 801 | assume | `assume(k2_clone@ == k2@)` in get_range |
| 3 | 967 | unsafe | `Some(unsafe { &*ptr })` in Iterator::next |

## Strategy

### Clone Assumes (holes 1-2)

In `get_range`, the code does `let k1_clone = k1.clone()` and needs `k1_clone@ == k1@`.
This is the standard eq/clone workaround.

**Fix options**:

1. **Use `clone_plus` instead of `clone`**. The `clone_plus` trait (from
   `crate::vstdplus::clone_plus::clone_plus`) provides `ensures clone@ == self@`. If `K`
   already has `ClonePlus`, switch to it:
   ```rust
   let k1_clone = k1.clone_plus();
   proof { lemma_cloned_view_eq(k1, k1_clone); }
   ```
   Check if `K: ClonePlus` is available or if the trait bounds need updating.

2. **Check if `K: StT` implies `ClonePlus`**. The `StT` supertrait may already include
   `ClonePlus`. Read the `StT` definition in `src/Types/Types.rs`.

3. If clone_plus is not available for the type, check the
   `partial_eq_eq_clone_standard.rs` for the correct workaround pattern. The assume may
   be legitimate if the type doesn't have ClonePlus.

### Iterator Unsafe (hole 3)

Line 967: `Some(unsafe { &*ptr })` in `Iterator::next` for `OrderedSetStEphIterator`.

The code does:
```rust
let ptr = &self.elements[self.pos] as *const T;
self.pos = self.pos + 1;
Some(unsafe { &*ptr })
```

This is a raw pointer dereference to avoid borrow checker issues with `&self.elements[self.pos]`
while also mutating `self.pos`.

**Fix options**:

1. **Restructure to avoid unsafe**: Store the elements as a `Vec<T>` (owned) instead of
   borrowing, and use safe indexing:
   ```rust
   let elem = &self.elements[self.pos];
   self.pos = self.pos + 1;
   // Return a reference — but lifetime issues may prevent this
   ```
   This may not work due to Rust's borrow rules (can't return `&self.elements[i]` from
   `&mut self`).

2. **Check the iterator standard** (`src/standards/iterators_standard.rs`). The standard
   iterator wraps `std::slice::Iter` and delegates `next()`, avoiding this issue entirely.
   Rewriting to wrap slice::Iter would eliminate both the unsafe AND the need for
   `assume(iter_invariant)` (which was already accepted).

3. **Wrap in `external_body` with tight ensures** if the unsafe cannot be eliminated.
   This is the same pattern used for OrderedSetStPer's iterator (line 1475 is
   `structural_false_positive STD_TRAIT_IMPL next`).

4. **Check if Verus has a safe pattern for this**. Search `src/experiments/` and
   `~/projects/verus/source/rust_verify_test/tests/` for iterator patterns that return
   references without unsafe.

The most practical approach may be option 3 (external_body with tight ensures) since
the iterator already has an `accept(iter_invariant)` — the unsafe is just the exec-level
equivalent of that structural limitation.

### If Time Remains

After fixing the 3 OrderedSetStEph holes, if you have time:
- Run `scripts/holes.sh src/Chap43/` to see if any remaining holes can be addressed
- Check for any style warnings in OrderedSetStEph that can be fixed

## Steps

1. **Read** OrderedSetStEph.rs — understand get_range and iterator implementation
2. **Read** `src/Types/Types.rs` — check StT definition for ClonePlus
3. **Read** `src/standards/partial_eq_eq_clone_standard.rs` — clone workaround pattern
4. **Read** `src/standards/iterators_standard.rs` — standard iterator pattern
5. **Fix** clone assumes using clone_plus if available
6. **Fix** or properly wrap the iterator unsafe
7. **Validate**, **rtt**, **ptt** — run sequentially

## Constraints

- Modify only `src/Chap43/OrderedSetStEph.rs` and possibly `src/Types/Types.rs` if trait
  bounds need adjustment.
- Do NOT modify OrderedTableStEph.rs (Agent 1 owns that).
- Do NOT modify OrderedTableStPer.rs (Agent 2 owns that).
- Do NOT modify BSTTreapStEph.rs (Agent 3 owns that).
- Do NOT add new `assume` or `accept`.
- Do NOT weaken ensures.
- If the unsafe truly cannot be eliminated safely, wrapping in external_body with the
  existing ensures is acceptable (it's a structural false positive).
- Run validate, rtt, ptt sequentially, never in parallel.
- Write report to `plans/agent4-round70-report.md` when done.
