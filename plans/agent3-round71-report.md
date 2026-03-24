# R71 Agent 3 Report: Iterator Standard Review

## Scope

14 files across Chap37 (partial), Chap41, Chap43 (partial) reviewed against the
iterator standard (`src/standards/iterators_standard.rs`) and wrapping iterator
standard (`src/standards/wrapping_iterators_standard.rs`).

## Summary Table

| # | Chap | File | Pattern | Components Present | Missing | Issues |
|---|------|------|---------|--------------------|---------|--------|
| 1 | 37 | AVLTreeSeqStEph.rs | Tree traversal | 1, 4, 9, 10 | 2, 3, 5, 6, 7, 8 | `ensures true` on next(); no ghost iter |
| 2 | 37 | AVLTreeSeqStPer.rs | Tree traversal | 1, 4, 9, 10 | 2, 3, 5, 6, 7, 8 | `ensures true` on next(); no ghost iter |
| 3 | 37 | BSTSetAVLMtEph.rs | Vec snapshot | All 10 | None | Assume in next() for Clone (acceptable) |
| 4 | 37 | BSTSetBBAlphaMtEph.rs | Vec snapshot | All 10 | None | Assume in next() for Clone (acceptable) |
| 5 | 37 | BSTSetPlainMtEph.rs | Vec snapshot | All 10 | None | Assume in next() for Clone (acceptable) |
| 6 | 37 | BSTSetRBMtEph.rs | Vec snapshot | All 10 | None | Assume in next() for Clone (acceptable) |
| 7 | 37 | BSTSetSplayMtEph.rs | Vec snapshot | All 10 | None | Assume in next() for Clone (acceptable) |
| 8 | 41 | AVLTreeSetMtEph.rs | Vec snapshot | All 10 | None | Assume in next() for Clone (acceptable) |
| 9 | 43 | AugOrderedTableMtEph.rs | Direct delegate | 9, 10 only | N/A (delegate) | iter()/IntoIter outside verus!, no ensures |
| 10 | 43 | AugOrderedTableStEph.rs | Direct delegate | 9, 10 only | N/A (delegate) | Compliant wrapping pattern |
| 11 | 43 | AugOrderedTableStPer.rs | Direct delegate | 9, 10 only | N/A (delegate) | Compliant wrapping pattern |
| 12 | 43 | OrderedSetStEph.rs | Vec + external_body | All 10 | None | external_body + unsafe in next(); iter() missing ensures |
| 13 | 43 | OrderedSetStPer.rs | Borrowed seq + ext_body | All 10 | None | external_body in next(); fully compliant |
| 14 | 43 | OrderedTableMtEph.rs | Vec snapshot | 1, 2, 3, 5, 6, 7, 8 | 4, 9, 10 outside verus! | next()/iter()/IntoIter outside verus!, no ensures |

## Compliance Summary

- **Fully compliant (10/10 components, correct specs):** 7 files
  - BSTSetAVLMtEph, BSTSetBBAlphaMtEph, BSTSetPlainMtEph, BSTSetRBMtEph,
    BSTSetSplayMtEph (Chap37), AVLTreeSetMtEph (Chap41), OrderedSetStPer (Chap43)
- **Compliant wrapping pattern (delegate to base_table):** 2 files
  - AugOrderedTableStEph, AugOrderedTableStPer (Chap43)
- **Partially compliant, needs work:** 5 files
  - AVLTreeSeqStEph, AVLTreeSeqStPer (Chap37) — missing 6 components each
  - OrderedSetStEph (Chap43) — missing ensures on iter()
  - AugOrderedTableMtEph (Chap43) — outside verus!, no ensures
  - OrderedTableMtEph (Chap43) — next()/iter()/IntoIter outside verus!

## Detailed Findings

### 1. AVLTreeSeqStEph.rs (Chap37)

**Iterator struct:** `AVLTreeSeqIterStEph<'a, T>` (line 94) — stack-based in-order
traversal iterator with `stack: Vec<&'a AVLTreeNode<T>>` and `current` fields.

**Present components:**
- (1) Iterator struct: Yes, but wraps stack/current, not `std::slice::Iter`
- (4) Iterator::next: Yes (line 1089), but `ensures true` — no two-arm spec
- (9) iter(): Yes, in trait (line 306) and impl (line 903)
- (10) IntoIterator for &Self: Yes (line 1079)

**Missing components:**
- (2) View for iterator — not implemented
- (3) iter_invariant spec fn — not implemented
- (5) Ghost iterator struct — not implemented
- (6) ForLoopGhostIteratorNew — not implemented
- (7) ForLoopGhostIterator — not implemented
- (8) Ghost iterator View — not implemented

**Consuming IntoIterator:** Not present.
**Debug/Display:** Yes, for iterator (lines 1198-1208).
**Section header:** Yes, `// 10. iterators`.
**Assume/accept/external_body:** None in iterator code itself.

**Assessment:** Major gap. This is a custom tree-traversal iterator that predates the
standard. Adding View/ghost infrastructure requires defining what the View sequence is
(the in-order traversal result), which is non-trivial for a stack-based iterator.

---

### 2. AVLTreeSeqStPer.rs (Chap37)

**Iterator struct:** `AVLTreeSeqStPerIter<'a, T>` (line 69) — same stack-based pattern
as StEph, with `stack: Vec<&'a Node<T>>` and `current`.

**Present/Missing:** Identical pattern to AVLTreeSeqStEph. Components 1, 4, 9, 10
present; 2, 3, 5, 6, 7, 8 missing.

**Consuming IntoIterator:** Not present.
**Debug/Display:** Yes, for iterator (lines 920-930).
**Section header:** Yes.
**Iterator::next ensures:** `ensures true` (line 824).

**Assessment:** Same gap as StEph. These two files are the most non-compliant.

---

### 3. BSTSetAVLMtEph.rs (Chap37)

**Iterator struct:** `BSTSetAVLMtEphIter<T>` (line 41) — `snapshot: Vec<T>`, `pos: usize`.
This is the snapshot pattern: the Mt module acquires a lock, copies elements into a Vec,
and iterates the snapshot.

**All 10 components present:**
- (1) Iterator struct with snapshot Vec (line 41)
- (2) View: `(self.pos as int, self.snapshot@)` — correct `(int, Seq<T>)` type
- (3) iter_invariant: `0 <= it@.0 <= it@.1.len()` (line 67)
- (4) Iterator::next with full two-arm ensures (lines 383-399)
- (5) Ghost iterator: `BSTSetAVLMtEphGhostIter<T>` with pos/elements/phantom
- (6) ForLoopGhostIteratorNew: correct
- (7) ForLoopGhostIterator: all 6 spec fns correct
- (8) Ghost View: `self.elements.take(self.pos)` — correct
- (9) iter() in trait with `ensures it@.0 == 0, bstsetavlmteph_iter_invariant(&it)`
- (10) IntoIterator for &Self with matching ensures

**Consuming IntoIterator:** Yes, returns `std::vec::IntoIter<T>`.
**Debug/Display:** Yes, for iterator and ghost iterator.
**Section header:** Yes.
**Assume in next():** `assume(item == old(self)@.1[old(self)@.0])` — Clone preserves
value. Acceptable per eq/clone workaround pattern (though technically this assume is in
next(), not in clone() itself — a minor deviation).

**Assessment:** Fully compliant. The assume for Clone in next() is a known pattern
across all BSTSet Mt files.

---

### 4–7. BSTSetBBAlphaMtEph, BSTSetPlainMtEph, BSTSetRBMtEph, BSTSetSplayMtEph (Chap37)

All four files follow the **identical** pattern as BSTSetAVLMtEph:
- All 10 components present and correct
- Vec snapshot iterator with pos counter
- Same two-arm ensures on next()
- Same Clone assume in next()
- Same ghost iterator infrastructure
- Consuming IntoIterator present (std::vec::IntoIter<T>)
- Debug/Display for iterator and ghost iterator present
- Section 10 header present

**Assessment:** All fully compliant.

---

### 8. AVLTreeSetMtEph.rs (Chap41)

**Iterator struct:** `AVLTreeSetMtEphIter<T>` (line 70) — `snapshot: Vec<T>`, `pos: usize`.
Same Vec snapshot pattern as the BSTSet files.

**All 10 components present:**
- (1) Iterator struct (line 70)
- (2) View: `(self.pos as int, self.snapshot@)` (line 88)
- (3) iter_invariant (line 102)
- (4) Iterator::next with two-arm ensures (lines 479-508)
- (5) Ghost iterator (line 75)
- (6) ForLoopGhostIteratorNew (line 511)
- (7) ForLoopGhostIterator with all 6 fns (line 518)
- (8) Ghost View (line 95)
- (9) iter() in trait (line 212) with ensures
- (10) IntoIterator for &Self (line 553)

**Consuming IntoIterator:** Not present.
**Debug/Display:** Yes, for iterator (lines 612-622) and ghost iterator (lines 624-633).
**Section header:** Yes.
**Assume in next():** Same Clone assume pattern as BSTSet files.

**Assessment:** Fully compliant. Only missing optional consuming IntoIterator.

---

### 9. AugOrderedTableMtEph.rs (Chap43)

**Pattern:** Direct delegation to `OrderedTableMtEph` base_table. No custom iterator
struct — returns `OrderedTableMtEphIter` directly.

**iter() method (line 769):** Outside verus!, no ensures.
```rust
pub fn iter(&self) -> OrderedTableMtEphIter<'_, K, V> {
    self.base_table.iter()
}
```

**IntoIterator (line 774):** Outside verus!, no ensures.

**Assessment:** Follows wrapping pattern by delegating directly, but since both iter()
and IntoIterator are outside verus! with no ensures, this is weaker than the StEph/StPer
counterparts which have proper ensures inside verus!. The lack of ensures means callers
cannot rely on `it@.0 == 0` or `iter_invariant` at the type level.

---

### 10. AugOrderedTableStEph.rs (Chap43)

**Pattern:** Direct delegation to `OrderedTableStEph` base_table. Returns
`OrderedTableStEphIter<K, V>` directly.

**iter() method (line 826):** Inside verus!, with ensures:
```rust
ensures it@.0 == 0, it@.1.len() == self.base_table.tree@.len(), iter_invariant(&it)
```

**IntoIterator (line 837):** Inside verus!, matching ensures.

**Assessment:** Fully compliant wrapping pattern. Has proper ensures.

---

### 11. AugOrderedTableStPer.rs (Chap43)

**Pattern:** Direct delegation to `OrderedTableStPer` base_table. Returns
`OrderedTableStPerIter<K, V>` directly.

**iter() method (line 892):** Inside verus!, with ensures:
```rust
ensures it@.0 == 0, it@.1.len() == self.base_table.tree@.len(), iter_invariant(&it)
```

**IntoIterator (line 903):** Inside verus!, matching ensures.

**Assessment:** Fully compliant wrapping pattern. Has proper ensures.

---

### 12. OrderedSetStEph.rs (Chap43)

**Iterator struct:** `OrderedSetStEphIter<'a, T>` (line 922) — `elements: Vec<T>`,
`pos: usize`, `len: usize`, `phantom`. Owns a Vec built by `collect_in_order()`.

**All 10 components present:**
- (1) Iterator struct (line 922)
- (2) View: `(self.pos as int, self.elements@.map_values(|t: T| t@))` — uses
  `map_values` to convert T to T::V. Type is `(int, Seq<T::V>)`.
- (3) iter_invariant (line 936)
- (4) Iterator::next with `#[verifier::external_body]` and full two-arm ensures (line 940)
- (5) Ghost iterator (line 975)
- (6) ForLoopGhostIteratorNew (line 989)
- (7) ForLoopGhostIterator with all 6 fns (line 996)
- (8) Ghost View: `self.elements.take(self.pos)` (line 981)
- (9) iter() in bare impl (line 909) — **missing ensures**
- (10) IntoIterator for &Self (line 1031) — **missing ensures** (has requires only)

**Consuming IntoIterator:** Not present.
**Debug/Display:** Not found for iterator or ghost iterator structs.
**Section header:** Yes, `// 10. iterators` (line 907).
**external_body:** Yes, on Iterator::next (line 943).
**unsafe:** Yes, `unsafe { &*ptr }` in next() body (line 967) — raw pointer dereference
to return `&'a T` from owned Vec.

**Issues:**
1. **iter() has no ensures clause** (line 911-913). Standard requires
   `ensures it@.0 == 0, it@.1 == self.<data>@, iter_invariant(&it)`.
2. **IntoIterator has no ensures clause** (line 1034-1036). Only has `requires`.
3. **Missing Debug/Display** for iterator and ghost iterator structs.
4. **View uses `map_values`** instead of direct delegation — acceptable since the
   element type has a View that differs from the raw type.

**Assessment:** Structurally complete but missing ensures on iter()/IntoIterator,
which means callers cannot prove iterator invariants. The external_body + unsafe pattern
is necessary because the iterator owns a Vec but needs to return `&'a T` references.

---

### 13. OrderedSetStPer.rs (Chap43)

**Iterator struct:** `OrderedSetStPerIter<'a, T>` (line 1457) — `seq: &'a
AVLTreeSeqStPerS<T>`, `pos: usize`, `len: usize`. Borrows the persistent sequence.

**All 10 components present:**
- (1) Iterator struct (line 1457)
- (2) View: `(self.pos as int, self.seq@)` — type `(int, Seq<T::V>)` (line 1463)
- (3) iter_invariant (line 1468)
- (4) Iterator::next with `#[verifier::external_body]` and full two-arm ensures (line 1472)
- (5) Ghost iterator (line 1505)
- (6) ForLoopGhostIteratorNew (line 1519)
- (7) ForLoopGhostIterator with all 6 fns (line 1526)
- (8) Ghost View: `self.elements.take(self.pos)` (line 1511)
- (9) iter() with ensures: `it@.0 == 0, it@.1 == self.base_set.elements@, iter_invariant(&it)` (line 1442)
- (10) IntoIterator for &Self (line 1561)

**Consuming IntoIterator:** Not present.
**Debug/Display:** Not found for iterator or ghost iterator structs.
**Section header:** Yes, `// 10. iterators` (line 1440).
**external_body:** Yes, on Iterator::next (line 1475).
**unsafe:** No — next() calls `self.seq.nth(self.pos)` (a verified method).

**Issues:**
1. **Missing Debug/Display** for iterator and ghost iterator structs.

**Assessment:** Fully compliant in terms of the 10 components and ensures clauses. The
only gap is missing Debug/Display impls.

---

### 14. OrderedTableMtEph.rs (Chap43)

**Iterator struct:** `OrderedTableMtEphIter<'a, K, V>` (line 667) — `snapshot: Vec<Pair<K, V>>`,
`pos: usize`, `_phantom`. Same snapshot pattern as BSTSet Mt files.

**Components inside verus! (lines 660-800):**
- (1) Iterator struct (line 667)
- (2) View: `(self.pos as int, self.snapshot@)` — correct (line 673)
- (3) iter_invariant (line 680)
- (5) Ghost iterator (line 686)
- (6) ForLoopGhostIteratorNew (line 700)
- (7) ForLoopGhostIterator with all 6 fns (line 707)
- (8) Ghost View: `self.elements.take(self.pos)` (line 692)

**Components OUTSIDE verus! (lines 814-851) — non-compliant:**
- (4) Iterator::next (line 814) — **no ensures clause, no return-value name, bare
  Rust impl**. Just `fn next(&mut self) -> Option<Pair<K, V>>` with manual
  pos/snapshot indexing.
- (9) iter() (line 829) — **no ensures, no return-value name**. Acquires RwLock read,
  collects entries, releases lock, returns iterator.
- (10) IntoIterator for &Self (line 844) — **no ensures**.

**Consuming IntoIterator:** Not present.
**Debug/Display:** Not found for iterator or ghost iterator structs.
**Section header:** Yes, `// 10. iterators` (line 660).
**Assume/external_body:** None — but the code is outside verus! entirely, so Verus
does not verify it at all.

**Issues:**
1. **Iterator::next is outside verus!** — should be inside verus! with two-arm ensures.
2. **iter() is outside verus!** — should be inside verus! with proper ensures.
3. **IntoIterator is outside verus!** — should be inside verus! with ensures.
4. **Missing Debug/Display** for iterator and ghost iterator structs.
5. The comment at line 661-663 says "All iterator infrastructure uses external_body"
   but the actual code is outside verus! entirely — worse than external_body because
   Verus cannot even type-check the signatures.

**Assessment:** The ghost infrastructure (components 1-3, 5-8) is correctly inside
verus!, but the exec-facing components (next, iter, IntoIterator) are entirely outside
Verus scope. This file needs the most work of all 14 files.

---

## Issue Categories

### Category A: Missing Ghost Iterator Infrastructure (6 missing components)

| # | Chap | File | Missing Components |
|---|------|------|--------------------|
| 1 | 37 | AVLTreeSeqStEph.rs | 2, 3, 5, 6, 7, 8 |
| 2 | 37 | AVLTreeSeqStPer.rs | 2, 3, 5, 6, 7, 8 |

These files use custom stack-based in-order traversal iterators and entirely lack the
verification layer. Adding View and ghost iterators requires defining a spec-level
sequence for the in-order traversal, which is non-trivial.

### Category B: Exec Iterator Code Outside verus!

| # | Chap | File | Components Outside |
|---|------|------|--------------------|
| 9 | 43 | AugOrderedTableMtEph.rs | 9, 10 (iter, IntoIter) |
| 14 | 43 | OrderedTableMtEph.rs | 4, 9, 10 (next, iter, IntoIter) |

Moving these inside verus! requires dealing with RwLock acquire/release in the iter()
body. OrderedTableMtEph is the harder case since its next() is also outside.

### Category C: Missing Ensures on iter()/IntoIterator

| # | Chap | File | Missing Ensures On |
|---|------|------|--------------------|
| 12 | 43 | OrderedSetStEph.rs | iter(), IntoIterator |

iter() and IntoIterator are inside verus! but have no ensures clause. Adding
`ensures it@.0 == 0, iter_invariant(&it)` should be straightforward.

### Category D: Missing Debug/Display

| # | Chap | File |
|---|------|------|
| 12 | 43 | OrderedSetStEph.rs |
| 13 | 43 | OrderedSetStPer.rs |
| 14 | 43 | OrderedTableMtEph.rs |

### Category E: Clone Assume in next() (Acceptable)

All 6 compliant Mt files (BSTSet*MtEph + AVLTreeSetMtEph) use `assume(item ==
old(self)@.1[old(self)@.0])` after cloning from the snapshot Vec. This is the standard
Clone-preserves-value workaround — acceptable per project conventions but technically
a hole in iterator verification.

## Totals

- **Files fully compliant:** 9 of 14 (7 full standard + 2 wrapping delegates)
- **Files needing work:** 5 of 14
- **Total missing components:** 12 (6 each from AVLTreeSeqStEph/StPer)
- **Components outside verus!:** 5 (3 from OrderedTableMtEph + 2 from AugOrderedTableMtEph)
- **Missing ensures:** 2 (OrderedSetStEph iter() and IntoIterator)
- **Missing Debug/Display:** 3 files

---

## R71 Fixes: All 5 Non-Compliant Files Fixed

### Verification

- validate: 4428 verified, 0 errors
- RTT: 2528 passed, 0 skipped
- PTT: 145 passed, 0 skipped

### Changes by File

| # | Chap | File | Issue | Fix |
|---|------|------|-------|-----|
| 1 | 37 | AVLTreeSeqStEph.rs | Missing 6 of 10 components | Added Ghost fields, View, iter_invariant, external_body next/iter, ghost iterator, ForLoopGhostIterator*, Debug/Display |
| 2 | 37 | AVLTreeSeqStPer.rs | Missing 6 of 10 components | Same pattern as StEph, adapted for Arc/Node types |
| 3 | 43 | OrderedTableMtEph.rs | next/iter/IntoIter outside verus! | Moved inside verus! section 10, added external_body + two-arm ensures, added to trait |
| 4 | 43 | AugOrderedTableMtEph.rs | iter/IntoIter outside verus! | Moved inside verus!, wrapping pattern delegates to base_table.iter() |
| 5 | 43 | OrderedSetStEph.rs | iter/IntoIter missing ensures | Added external_body + ensures, added Debug/Display for iterator types |

### Technical Notes

- **Ghost\<T\> vs `pub ghost`**: `pub ghost field: Type` doesn't work in `external_body` fns. Use `pub field: Ghost<Type>` instead.
- **Trait `self@` limitation**: Use trait-defined spec fns (e.g., `self.spec_seq()`) in ensures instead of `self@` when the trait doesn't extend View.
- **IntoIterator `self@` limitation**: For `impl IntoIterator for &'a Type`, use field-level access (e.g., `spec_inorder(self.root)`) instead of `self@`.
- Verification count dropped 4433 → 4428: expected, as `ensures true` functions became `external_body`.
