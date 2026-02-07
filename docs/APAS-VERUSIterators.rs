// ─────────────────────────────────────────────────────────────────────────────
//  APAS-VERUS Iterator Standard
// ─────────────────────────────────────────────────────────────────────────────
//
//  This file defines the standard iterator implementation pattern for
//  APAS-VERUS data structures.
//  The canonical implementation is ArraySeqStEph (Chap18).
//
//  Contents
//  --------
//   0. Rust's Three Standard For-Loop Patterns
//   1. Required Components (11 items)
//   2. Component Templates
//   3. Proof-Time Test Templates (loop-loop, loop-loop-borrow, loop-loop-consume,
//                                 for-iter, for-borrow, for-consume)
//   4. Compliance Table
//
// ─────────────────────────────────────────────────────────────────────────────

//  0. RUST'S THREE STANDARD FOR-LOOP PATTERNS
//
//  Rust's `for` loop desugars to `IntoIterator::into_iter()`.  The Rust
//  standard library defines three conventional iteration patterns, which
//  the community calls "the three iterators":
//
//   #  Pattern                    Trait / Method          Yields    Ownership
//  ── ────────────────────────── ─────────────────────── ───────── ──────────
//   1  for x in &collection      IntoIterator for &C     &T        borrows
//   2  for x in &mut collection  IntoIterator for &mut C &mut T    mut borrows
//   3  for x in collection       IntoIterator for C      T         consumes
//
//  These are sometimes called "iter", "iter_mut", and "into_iter" after
//  the corresponding explicit method names:
//
//      for x in collection.iter()      // equivalent to for x in &collection
//      for x in collection.iter_mut()  // equivalent to for x in &mut collection
//      for x in collection.into_iter() // equivalent to for x in collection
//
//  See: The Rust Programming Language, Ch.13.2 "Processing a Series of
//  Items with Iterators", and the IntoIterator trait documentation.
//
//  APAS-VERUS support:
//
//   Pattern 1 (borrow / iter):  Required.  This is the primary iteration
//       pattern for APAS-VERUS collections.  Requires IntoIterator for &Self
//       and iter() method, both with ensures.
//
//   Pattern 2 (mut borrow / iter_mut):  Not supported.  vstd does not yet
//       spec IterMut / iter_mut.  Also not needed: APAS-VERUS collections
//       use functional-style updates, not in-place mutation during iteration.
//
//   Pattern 3 (consuming / into_iter):  Supported.  vstd fully specs
//       Vec::IntoIter (View, next ensures, ForLoopGhostIterator).
//       ArraySeqStEph provides IntoIterator for Self with ensures and
//       has loop-loop-consume + for-consume proof-time tests.
//       Note: the consuming pattern yields T (owned), not &T.

//  1. REQUIRED COMPONENTS
//
//  For a collection `CollectionS<T>` with element type `T`, the following
//  eleven components are required.  All items marked (V) must be inside the
//  verus! { } block.
//
//   #  Component                          Location   Purpose
//  ── ──────────────────────────────────  ─────────  ─────────────────────────
//   1  Custom iterator struct             (V)        Wraps underlying Rust iter
//   2  View for iterator                  (V)        type V = (int, Seq<T>)
//   3  iter_invariant spec fn             (V)        Bounds the position index
//   4  Iterator::next with ensures        (V)        Core iteration contract
//   5  Ghost iterator struct              (V)        Spec-level loop state
//   6  ForLoopGhostIteratorNew impl       (V)        Creates ghost from exec
//   7  ForLoopGhostIterator impl          (V)        Full ghost loop protocol
//   8  View for ghost iterator            (V)        items-seen-so-far = take()
//   9  iter() method with ensures         (V)        Entry point with specs
//  10  IntoIterator for &Self             (V)        Enables for-borrow pattern
//  11  Proof-time tests                   tests/     loop-loop + for-iter
//
//  Items 1-10 go in the source file.  Item 11 goes in
//      rust_verify_test/tests/<Chap>/Prove<Collection>.rs

//  2. COMPONENT TEMPLATES
//
//  Substitute "Collection" and "T" throughout.  The canonical reference
//  implementation is src/Chap18/ArraySeqStEph.rs.
//
//  ── 1. Custom Iterator Struct ──────────────────────────────────────────────
//
//  Wraps the inner Rust iterator.  Fields are private (not pub) so that
//  users interact only through the View and Iterator trait.
//
//      #[verifier::reject_recursive_types(T)]
//      pub struct CollectionIter<'a, T> {
//          inner: std::slice::Iter<'a, T>,   // or hash_set::Iter, etc.
//      }
//
//  ── 2. View for Iterator ───────────────────────────────────────────────────
//
//  The View is a pair: (position_index, full_sequence).
//  - Position starts at 0 and advances to elements.len().
//  - The sequence is the *full* iteration order, fixed at creation.
//  - Use `closed spec fn` to hide the inner implementation.
//
//      impl<'a, T> View for CollectionIter<'a, T> {
//          type V = (int, Seq<T>);
//          closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
//      }
//
//  ── 3. iter_invariant ──────────────────────────────────────────────────────
//
//  A top-level spec fn bounding the position index.  Users include this
//  in loop invariants.
//
//      pub open spec fn iter_invariant<'a, T>(it: &CollectionIter<'a, T>) -> bool {
//          0 <= it@.0 <= it@.1.len()
//      }
//
//  ── 4. Iterator::next with ensures ─────────────────────────────────────────
//
//  The ensures clause is the key verification contract.  It has two arms:
//  None (exhausted) and Some (produced an element).
//
//      impl<'a, T> std::iter::Iterator for CollectionIter<'a, T> {
//          type Item = &'a T;
//
//          fn next(&mut self) -> (next: Option<&'a T>)
//              ensures ({
//                  let (old_index, old_seq) = old(self)@;
//                  match next {
//                      None => {
//                          &&& self@ == old(self)@
//                          &&& old_index >= old_seq.len()
//                      },
//                      Some(element) => {
//                          let (new_index, new_seq) = self@;
//                          &&& 0 <= old_index < old_seq.len()
//                          &&& new_seq == old_seq
//                          &&& new_index == old_index + 1
//                          &&& element == old_seq[old_index]
//                      },
//                  }
//              })
//          {
//              self.inner.next()
//          }
//      }
//
//  ── 5. Ghost Iterator Struct ───────────────────────────────────────────────
//
//  Pure spec-level state used by the ForLoopGhostIterator protocol.
//  Fields are pub so for-loop invariants can refer to them directly
//  (e.g. `iter.pos`, `iter.elements`).
//
//      #[verifier::reject_recursive_types(T)]
//      pub struct CollectionGhostIterator<'a, T> {
//          pub pos: int,
//          pub elements: Seq<T>,
//          pub phantom: core::marker::PhantomData<&'a T>,
//      }
//
//  ── 6. ForLoopGhostIteratorNew ─────────────────────────────────────────────
//
//  Creates ghost state from the exec iterator.
//
//      impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew
//          for CollectionIter<'a, T>
//      {
//          type GhostIter = CollectionGhostIterator<'a, T>;
//          open spec fn ghost_iter(&self) -> CollectionGhostIterator<'a, T> {
//              CollectionGhostIterator {
//                  pos: self@.0,
//                  elements: self@.1,
//                  phantom: core::marker::PhantomData,
//              }
//          }
//      }
//
//  ── 7. ForLoopGhostIterator ────────────────────────────────────────────────
//
//  The full ghost-loop protocol.  Six spec functions:
//
//      impl<'a, T> vstd::pervasive::ForLoopGhostIterator
//          for CollectionGhostIterator<'a, T>
//      {
//          type ExecIter = CollectionIter<'a, T>;
//          type Item = T;
//          type Decrease = int;
//
//          // Links ghost state to exec iterator.
//          open spec fn exec_invariant(&self, exec_iter: &CollectionIter<'a, T>) -> bool {
//              &&& self.pos == exec_iter@.0
//              &&& self.elements == exec_iter@.1
//          }
//
//          // Maintained across iterations; init is the state before the first iteration.
//          open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
//              init matches Some(init) ==> {
//                  &&& init.pos == 0
//                  &&& init.elements == self.elements
//                  &&& 0 <= self.pos <= self.elements.len()
//              }
//          }
//
//          // Holds after the loop exits normally.
//          open spec fn ghost_ensures(&self) -> bool {
//              self.pos == self.elements.len()
//          }
//
//          // Termination measure.
//          open spec fn ghost_decrease(&self) -> Option<int> {
//              Some(self.elements.len() - self.pos)
//          }
//
//          // What the next call to next() will yield (before the call).
//          open spec fn ghost_peek_next(&self) -> Option<T> {
//              if 0 <= self.pos < self.elements.len() {
//                  Some(self.elements[self.pos])
//              } else {
//                  None
//              }
//          }
//
//          // Ghost state after processing one element.
//          open spec fn ghost_advance(
//              &self, _exec_iter: &CollectionIter<'a, T>,
//          ) -> CollectionGhostIterator<'a, T> {
//              Self { pos: self.pos + 1, ..*self }
//          }
//      }
//
//  ── 8. View for Ghost Iterator ─────────────────────────────────────────────
//
//  The ghost iterator's View is the *items seen so far*: the prefix of
//  length `pos`.  This is what user code asserts against after the loop.
//
//      impl<'a, T> View for CollectionGhostIterator<'a, T> {
//          type V = Seq<T>;
//          open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
//      }
//
//  ── 9. iter() Method ───────────────────────────────────────────────────────
//
//  Entry point for iteration.  Ensures that:
//  - Position starts at 0.
//  - The sequence matches the collection's contents.
//  - iter_invariant holds.
//
//      pub fn iter(&self) -> (it: CollectionIter<'_, T>)
//          ensures
//              it@.0 == 0,
//              it@.1 == self.seq@,     // adapt to collection's data field
//              iter_invariant(&it),
//      {
//          CollectionIter { inner: self.seq.iter() }
//      }
//
//  For Set-like types the ensures additionally includes:
//      it@.1.map(|i: int, k: T| k@).to_set() == self@,
//      it@.1.no_duplicates(),
//
//  ── 10. IntoIterator for &Self ─────────────────────────────────────────────
//
//  Enables `for x in &collection`.  Must return the custom iterator with
//  the same ensures as iter().
//
//      impl<'a, T> std::iter::IntoIterator for &'a CollectionS<T> {
//          type Item = &'a T;
//          type IntoIter = CollectionIter<'a, T>;
//          fn into_iter(self) -> (it: Self::IntoIter)
//              ensures
//                  it@.0 == 0,
//                  it@.1 == self.seq@,
//                  iter_invariant(&it),
//          {
//              CollectionIter { inner: self.seq.iter() }
//          }
//      }

//  3. PROOF-TIME TEST TEMPLATES
//
//  Every collection with an iterator must have both of these tests in
//      rust_verify_test/tests/<Chap>/Prove<Collection>.rs
//
//  The tests verify that the ghost-level accumulation protocol works:
//  after iterating the full collection, the ghost Seq equals the full
//  iterator sequence.
//
//  ── Test 1: loop-loop ──────────────────────────────────────────────────────
//
//  Manual iteration using `loop` + `if let`.  This is the fundamental
//  pattern; it works with only Iterator::next ensures (no ghost iterator
//  required).
//
//      test_verify_one_file! {
//          #[test] collection_loop_loop verus_code! {
//              use vstd::prelude::*;
//              use apas_verus::<Chap>::<Collection>::<Collection>::*;
//
//              fn test_loop_loop() {
//                  let a: CollectionS<u64> = /* construct */;
//
//                  let mut it: CollectionIter<u64> = a.iter();
//                  let ghost iter_seq: Seq<u64> = it@.1;
//                  let ghost mut items: Seq<u64> = Seq::empty();
//
//                  #[verifier::loop_isolation(false)]
//                  loop
//                      invariant
//                          items =~= iter_seq.take(it@.0 as int),
//                          iter_invariant(&it),
//                          iter_seq == it@.1,
//                          it@.0 <= iter_seq.len(),
//                      decreases iter_seq.len() - it@.0,
//                  {
//                      if let Some(x) = it.next() {
//                          proof { items = items.push(*x); }
//                      } else {
//                          break;
//                      }
//                  }
//
//                  assert(it@.0 == iter_seq.len());
//                  assert(items =~= iter_seq);
//              }
//          } => Ok(())
//      }
//
//  Key invariants:
//    - items =~= iter_seq.take(it@.0)    ghost accumulates the prefix
//    - iter_invariant(&it)               position stays in bounds
//    - iter_seq == it@.1                 sequence doesn't change
//    - it@.0 <= iter_seq.len()           needed for decreases
//
//  ── Test 2: for-iter ───────────────────────────────────────────────────────
//
//  Uses Verus's `for x in iter: it` syntax with the ForLoopGhostIterator
//  protocol.  Cleaner than loop-loop; the ghost iterator fields (iter.pos,
//  iter.elements) are available directly in invariants.
//
//      test_verify_one_file! {
//          #[test] collection_for_iter verus_code! {
//              use vstd::prelude::*;
//              use apas_verus::<Chap>::<Collection>::<Collection>::*;
//
//              fn test_for_iter() {
//                  let a: CollectionS<u64> = /* construct */;
//
//                  let it: CollectionIter<u64> = a.iter();
//                  let ghost iter_seq: Seq<u64> = it@.1;
//                  let ghost mut items: Seq<u64> = Seq::empty();
//
//                  for x in iter: it
//                      invariant
//                          iter.elements == iter_seq,
//                          items =~= iter_seq.take(iter.pos),
//                          iter.pos <= iter_seq.len(),
//                  {
//                      proof { items = items.push(*x); }
//                  }
//
//                  assert(items =~= iter_seq);
//              }
//          } => Ok(())
//      }
//
//  Key invariants:
//    - iter.elements == iter_seq          sequence doesn't change
//    - items =~= iter_seq.take(iter.pos)  ghost accumulates the prefix
//    - iter.pos <= iter_seq.len()          position stays in bounds
//
//  After the loop, ghost_ensures gives iter.pos == iter.elements.len(),
//  so items == iter_seq.take(len) == iter_seq.
//
//  ── Difference between the two tests ───────────────────────────────────────
//
//  | Aspect            | loop-loop                  | for-iter                  |
//  |-------------------|----------------------------|---------------------------|
//  | Syntax            | loop + if-let + break      | for x in iter: it         |
//  | Ghost state       | Manual via it@             | Automatic via iter.*      |
//  | Requires          | Iterator::next ensures     | + ForLoopGhostIterator    |
//  | loop_isolation    | Needs #[...(false)]        | Not needed                |
//  | Termination       | explicit decreases         | Automatic via Decrease    |
//  | Postcondition     | assert after break         | Follows from ghost_ensures|

//  4. COMPLIANCE TABLE
//
//  As of this writing.  ✅ = present and verified.  ⚠ = present but needs
//  improvement.  ❌ = missing.
//
//  | Collection       | 1-8  | 9:iter | 10:Into | 11:tests          |
//  |                  | Infra| ensures| ensures | loop  | for-iter  |
//  |------------------|------|--------|---------|-------|-----------|
//  | SetStEph         |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | SetMtEph         |  ❌  |   ❌   |   ❌    |  ❌   |    ❌     |
//  | RelationStEph    |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | MappingStEph     |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | ArraySeqStEph    |  ✅  |   ✅   |   ✅    |  ✅   |    ✅     |
//  | ArraySeqStPer    |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | ArraySeqMtEph    |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | ArraySeqMtPer    |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | ArraySeq         |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | LinkedListStEph  |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | LinkedListStPer  |  ✅  |   ✅   |   ❌    |  ✅   |    ✅     |
//  | MathSeq          |  ❌  |   ❌   |   ❌    |  ❌   |    ❌     |
//  | DirGraphStEph    |  ❌  |   ❌   |   ❌    |  ❌   |    ❌     |
//  | UnDirGraphStEph  |  ❌  |   ❌   |   ❌    |  ❌   |    ❌     |
