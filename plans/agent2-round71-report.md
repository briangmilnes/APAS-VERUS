# R71 Agent 2: Iterator Standard Review

## Summary

Reviewed 14 files across Chap18, Chap19, Chap23, and Chap37 against the 10-component
iterator standard defined in `src/standards/iterators_standard.rs`.

**Result**: 10 of 14 files are fully compliant. 2 files have justified structural
deviations. 2 files are non-standard and need work.

## 10-Component Checklist

| # | Component | Description |
|---|-----------|-------------|
| C1 | Iterator struct | Custom `ModIter<'a, T>` wrapping `std::slice::Iter<'a, T>` |
| C2 | View for iter | `type V = (int, Seq<T>)`, delegates to `self.inner@` |
| C3 | iter_invariant | `pub open spec fn iter_invariant`: `0 <= it@.0 <= it@.1.len()` |
| C4 | Iterator::next | Two-arm ensures: None (exhausted) / Some (element + advance) |
| C5 | Ghost iterator struct | `pos: int`, `elements: Seq<T>`, `phantom: PhantomData<&'a T>` |
| C6 | ForLoopGhostIteratorNew | Maps exec iter to ghost iter |
| C7 | ForLoopGhostIterator | 6 spec fns: exec_invariant, ghost_invariant, ghost_ensures, ghost_decrease, ghost_peek_next, ghost_advance |
| C8 | View for ghost iter | `type V = Seq<T>`, returns `self.elements.take(self.pos)` |
| C9 | iter() method | On main struct trait, ensures `it@.0 == 0, it@.1 == self@, iter_invariant(&it)` |
| C10 | IntoIterator for &Self | Delegates to iter(), with ensures |

## Per-File Summary Table

| # | Chap | File | Components | Missing | Issues |
|---|------|------|-----------|---------|--------|
| 1 | 18 | ArraySeqStEph.rs | 10/10 | None | None |
| 2 | 18 | ArraySeqStPer.rs | 9.5/10 | None | C10 IntoIterator outside verus! (no ensures) |
| 3 | 18 | ArraySeqMtEph.rs | 10/10 | None | None |
| 4 | 18 | ArraySeqMtPer.rs | 10/10 | None | None |
| 5 | 18 | LinkedListStEph.rs | 10/10 | None | None |
| 6 | 18 | LinkedListStPer.rs | 10/10 | None | None |
| 7 | 19 | ArraySeqStEph.rs | 10/10 | None | Duplicate section headers (minor) |
| 8 | 19 | ArraySeqStPer.rs | 10/10 | None | Duplicate section headers (minor) |
| 9 | 19 | ArraySeqMtEph.rs | 10/10 | None | None |
| 10 | 19 | ArraySeqMtEphSlice.rs | 1/10 | C1-C8 | Raw `std::slice::Iter` — no custom iterator |
| 11 | 23 | BalBinTreeStEph.rs | 7/10 | C1*,C9,C10 | Justified: consuming iterators for tree |
| 12 | 23 | PrimTreeSeqStPer.rs | 10/10 | None | None |
| 13 | 37 | AVLTreeSeq.rs | 10/10 | None | C4 uses external_body on next() |
| 14 | 37 | AVLTreeSeqMtPer.rs | 1/10 | C1-C8,C10 | Hand-rolled iter, ensures just `true` |

## Detailed Per-File Analysis

### 1. Chap18/ArraySeqStEph.rs — COMPLIANT

All 10 components present at lines 910-1031. Reference implementation quality.

- C1: `ArraySeqStEphIter<'a, T>` wrapping `std::slice::Iter<'a, T>` (line 914)
- C2: View `(int, Seq<T>)` delegating to `self.inner@` (line 918-921)
- C3: `iter_invariant` at line 936
- C4: Standard two-arm ensures on `next()` (line 944-963)
- C5: `ArraySeqStEphGhostIterator` with pos/elements/phantom (line 924-929)
- C6: ForLoopGhostIteratorNew (line 966-971)
- C7: ForLoopGhostIterator with all 6 spec fns (line 973-1006)
- C8: Ghost iter View returns `elements.take(pos)` (line 931-934)
- C9: `iter()` at line 893 with standard ensures
- C10: `IntoIterator for &Self` at line 1008 with ensures

Also has consuming `IntoIterator for Self` (line 1021-1031).

### 2. Chap18/ArraySeqStPer.rs — NEARLY COMPLIANT

Components C1-C9 present inside verus! (lines 895-994). C10 is outside verus!.

- C1-C9: All standard, matching ArraySeqStEph pattern exactly.
- C10: `IntoIterator for &Self` moved outside verus! at line 1029 due to Verus AIR bug
  ("ill-typed AIR on proj%%core!iter.traits.collect.IntoIterator./Item"). The impl works
  at runtime but has **no ensures**. Comment explains the workaround (line 996-997).

**Issue**: IntoIterator for &Self lacks ensures. This is a Verus limitation, not a code
deficiency. The workaround is noted in-line.

### 3. Chap18/ArraySeqMtEph.rs — COMPLIANT

All 10 components present at lines 1441-1562.

- C1: `ArraySeqMtEphIter<'a, T>` wrapping `std::slice::Iter<'a, T>` (line 1444-1447)
- C2: View `(int, Seq<T>)` (line 1449-1451)
- C3: `iter_invariant` at line 1454
- C4: Standard two-arm ensures (line 1462-1481)
- C5: Ghost iterator with pos/elements/phantom (line 1484-1490)
- C6: ForLoopGhostIteratorNew (line 1497-1502)
- C7: ForLoopGhostIterator with all 6 spec fns (line 1504-1537)
- C8: Ghost iter View (line 1492-1494)
- C9: `iter()` at line 1092
- C10: IntoIterator for &Self at line 1539 with ensures

Also has consuming IntoIterator (line 1552-1562).

### 4. Chap18/ArraySeqMtPer.rs — COMPLIANT

All 10 components present at lines 1046-1167. Identical structure to Chap18/ArraySeqMtEph.

- C1-C10: All present and standard.
- Also has consuming IntoIterator (line 1157-1167).

### 5. Chap18/LinkedListStEph.rs — COMPLIANT

All 10 components present (lines 756-865, previously read in full). Reference quality.

- C1: `LinkedListStEphIter<'a, T>` wrapping `std::slice::Iter<'a, T>`
- C5: Ghost iterator with phantom field
- C9: `iter()` in trait with standard ensures
- C10: IntoIterator for both &Self and Self

### 6. Chap18/LinkedListStPer.rs — COMPLIANT

All 10 components present (lines 739-848, previously read in full). Mirrors StEph.

### 7. Chap19/ArraySeqStEph.rs — COMPLIANT

All 10 components present at lines 1052-1176. Structurally identical to Chap18/ArraySeqStEph.

- Minor issue: Duplicate section headers (`// 10. iterators` at both line 1050 and 1052,
  `// 11. derive impls in verus!` at both line 1179 and 1181). Cosmetic only.

### 8. Chap19/ArraySeqStPer.rs — COMPLIANT

All 10 components present at lines 1060-1184. Identical to Chap19/ArraySeqStEph structure.

- Same duplicate section header issue as Chap19/ArraySeqStEph.

### 9. Chap19/ArraySeqMtEph.rs — COMPLIANT

All 10 components present at lines 1336-1457. Standard pattern.

### 10. Chap19/ArraySeqMtEphSlice.rs — NON-STANDARD

**Significantly non-compliant.** This file (328 lines total, previously read in full)
returns a raw `std::slice::Iter<'a, T>` directly from its `iter()` method at line 249
instead of wrapping it in a custom iterator struct.

Missing components:
- C1: No custom iterator struct (returns raw `std::slice::Iter`)
- C2: No View impl for the iterator
- C3: No iter_invariant spec fn
- C4: No Iterator::next impl (uses std's directly)
- C5: No ghost iterator struct
- C6: No ForLoopGhostIteratorNew
- C7: No ForLoopGhostIterator
- C8: No View for ghost iterator

Present:
- C9: Has `iter()` method but ensures uses `forall` element-wise equality pattern instead
  of the standard `it@.1 == self.data@` pattern
- C10: Has `IntoIterator for &Self` at line 272 but with non-standard ensures

**Note**: This file appears to be a minimal "slice-based" variant. The non-standard
iterator approach means for-loop ghost iteration (verified loop invariants) is not
available for this module.

### 11. Chap23/BalBinTreeStEph.rs — JUSTIFIED DEVIATIONS

Has 3 iterator types (InOrder, PreOrder, PostOrder), all using consuming iteration.
Previously read in full (882 lines).

Present components (adapted):
- C1*: Iterator structs exist but wrap `IntoIter<T>` (consuming) not `slice::Iter<'a, T>`
  (borrowing). Item type is `T` not `&'a T`.
- C2: View impls present with `(int, Seq<T>)` pattern
- C3: `iter_invariant` present for each iterator type
- C4: Iterator::next with standard two-arm ensures
- C5: Ghost iterator structs present but **missing `phantom` field** (no lifetime to anchor)
- C6: ForLoopGhostIteratorNew impls present
- C7: ForLoopGhostIterator with all 6 spec fns
- C8: Ghost iter View with `elements.take(pos)` pattern

Missing:
- C9: No `iter()` method. Has `iter_in_order()`, `iter_pre_order()`, `iter_post_order()`
  instead (3 traversal orders require 3 entry points).
- C10: No `IntoIterator for &Self`. Trees cannot cheaply produce a borrow iterator without
  first collecting to Vec, which these iterators do internally.

**Justification**: Tree structures legitimately differ from linear collections. The
consuming pattern (collect-to-Vec then iterate) is the correct approach for trees. The
three traversal orders justify separate entry points rather than a single `iter()`.
Missing phantom is because the iterator owns its data (no lifetime parameter needed).

### 12. Chap23/PrimTreeSeqStPer.rs — COMPLIANT

All 10 components present (lines 754-875, previously read in full).

- Uses `prim_tree_seq_iter_invariant` name instead of `iter_invariant` (module-specific
  naming, acceptable).
- Both borrow and consuming IntoIterator present.

### 13. Chap37/AVLTreeSeq.rs — COMPLIANT (with external_body on next)

All 10 components present.

- C1: `AVLTreeSeqIter<'a, T>` at line 66, but stores `tree: &'a AVLTreeS<T>`, `pos: usize`,
  `len: usize` instead of wrapping `std::slice::Iter`. This is correct for tree iteration
  by index.
- C2: View `(int, Seq<T>)` at line 88-93, computed from `(self.pos, inorder_values(tree.root))`
- C3: `iter_invariant` at line 180
- C4: `next()` at line 1119 has standard two-arm ensures but uses **`#[verifier::external_body]`**
  (line 1122). The body calls `self.tree.nth(self.pos)` which is O(log n) per step.
- C5: `AVLTreeSeqGhostIterator` with pos/elements/phantom (line 73-77)
- C6: ForLoopGhostIteratorNew (line 1151-1156)
- C7: ForLoopGhostIterator with all 6 spec fns (line 1158-1191)
- C8: Ghost iter View (line 95-98)
- C9: `iter()` in trait at line 299 with ensures, impl at line 947
- C10: No `IntoIterator for &Self` or `IntoIterator for Self` found.

**Issues**:
- `external_body` on Iterator::next is a proof hole. The ensures are strong (standard
  two-arm pattern) but the body is not verified.
- Missing IntoIterator impls (both &Self and Self). The `iter()` method exists but
  `for x in &tree` syntax won't work without IntoIterator.

### 14. Chap37/AVLTreeSeqMtPer.rs — NON-STANDARD

**Significantly non-compliant.** Previously read in full (828 lines).

- C1: `AVLTreeSeqMtPerIter<T>` at line 94-97 stores `values: Vec<T>` and `index: usize`.
  This is a hand-rolled consuming iterator that pre-collects all values, not a standard
  wrapper around `std::slice::Iter`.
- C2: No View impl for the iterator
- C3: No iter_invariant spec fn
- C4: `next()` ensures just `true` (line 691) — no postcondition at all
- C5: No ghost iterator struct
- C6: No ForLoopGhostIteratorNew
- C7: No ForLoopGhostIterator
- C8: No View for ghost iterator
- C9: No `iter()` method on main struct
- C10: `IntoIterator for Self` only (no `&Self`)

**Assessment**: This file needs a complete iterator rewrite to match the standard. The
current iterator provides no verified postconditions and no ghost iteration support.

## Summary Statistics

| Category | Count | Files |
|----------|-------|-------|
| Fully compliant | 10 | Chap18: ArraySeqStEph, ArraySeqMtEph, ArraySeqMtPer, LinkedListStEph, LinkedListStPer; Chap19: ArraySeqStEph, ArraySeqStPer, ArraySeqMtEph; Chap23: PrimTreeSeqStPer; Chap37: AVLTreeSeq |
| Nearly compliant | 1 | Chap18: ArraySeqStPer (IntoIterator outside verus! due to Verus bug) |
| Justified deviations | 1 | Chap23: BalBinTreeStEph (tree-specific consuming pattern) |
| Non-standard | 2 | Chap19: ArraySeqMtEphSlice, Chap37: AVLTreeSeqMtPer |

## Recommended Actions

1. **Chap19/ArraySeqMtEphSlice.rs**: Add standard iterator wrapper around `std::slice::Iter`,
   ghost iterator, ForLoopGhostIterator impls, and iter_invariant. Low effort — follow
   the Chap18/ArraySeqStEph pattern.

2. **Chap37/AVLTreeSeqMtPer.rs**: Rewrite iterator to match standard. Add View for iterator,
   ghost iterator with ForLoopGhostIterator, iter_invariant, proper next() ensures, and
   iter() method. Medium effort — can adapt AVLTreeSeq.rs pattern.

3. **Chap37/AVLTreeSeq.rs**: Remove external_body from Iterator::next (proof hole). Add
   IntoIterator for &Self. Low-medium effort.

4. **Chap18/ArraySeqStPer.rs**: IntoIterator outside verus! is a known Verus limitation.
   No action needed unless Verus fixes the AIR bug.

5. **Chap23/BalBinTreeStEph.rs**: Add `phantom` field to ghost iterator structs for
   consistency. Very low effort.

6. **Chap19/ArraySeqStEph.rs, ArraySeqStPer.rs**: Remove duplicate section headers.
   Cosmetic only.
