Fix 3 Iterator::next external_body holes by refactoring to delegation.

BACKGROUND

3 Iterator::next implementations have external_body because their bodies
call `nth(pos)`, which requires `spec_wf()` and `index < len`. Iterator::next
(std trait) cannot carry `requires`, so these preconditions can't be stated.

All 30+ other Iterator::next implementations in the codebase verify fine.
They work by either:
  (a) Delegating to an already-proved inner iterator (`self.inner.next()`)
  (b) Using stack-based tree traversal (no preconditions needed)
  (c) Relying on vstd's slice::Iter::next assume_specification

The fix: change the iterator struct to wrap an already-proved iterator
instead of doing manual nth-based indexing.

TARGET 1 — src/Chap43/OrderedSetStEph.rs (line 622)

Current:
  struct OrderedSetStEphIter { seq: &'a AVLTreeSeqStEphS<T>, pos: usize, len: usize }
  fn next() { self.seq.nth(self.pos) }  // nth requires wf + bounds

Fix: wrap AVLTreeSeqStEph's proved iterator (AVLTreeSeqIterStEph):
  struct OrderedSetStEphIter { inner: AVLTreeSeqIterStEph<'a, T> }
  fn next() { self.inner.next() }

Update:
  - iter() method (~line 591): construct inner iterator from self.base_set.elements
  - View impl (~line 610): delegate to inner view
  - iter_invariant (~line 615): delegate to inner invariant
  - IntoIterator impl (~line 709): construct via inner
  - ForLoopGhostIteratorNew (~line 666): adjust ghost_iter
  - ForLoopGhostIterator (~line 674): adjust exec_invariant, ghost_advance
  - Remove external_body from next()
  - ensures clause stays the same — the inner iterator's ensures should
    satisfy it since both iterate the same sequence in order

TARGET 2 — src/Chap43/OrderedSetStPer.rs (line 647)

Same pattern as OrderedSetStEph. The StPer variant wraps AVLTreeSeqStPer
which also has a proved stack-based iterator (AVLTreeSeqStPerIter? or
similar — check the file).

  struct OrderedSetStPerIter { inner: <StPer's proved iterator> }
  fn next() { self.inner.next() }

Update all the same infrastructure (View, iter_invariant, GhostIterator).

TARGET 3 — src/Chap37/AVLTreeSeq.rs (line 1118)

AVLTreeSeq is the base tree module. Its iterator uses nth-based indexing:
  struct AVLTreeSeqIter { tree: &'a AVLTreeS<T>, pos: usize, len: usize }
  fn next() { self.tree.nth(self.pos) }

AVLTreeSeqStEph has a proved stack-based iterator. However, AVLTreeSeq
and AVLTreeSeqStEph are separate types (standalone rule). Two options:

  (a) Duplicate the stack-based iterator pattern from AVLTreeSeqStEph
      into AVLTreeSeq. push_left_iter + stack.pop() traversal.
  (b) Weaken ensures to `ensures true` (like AVLTreeSeqStEph does).

Option (a) preserves the strong ensures. Option (b) is simpler but loses
the element-tracking postcondition.

VERIFICATION

After each file change, check that:
  1. The iterator type's View impl still returns (int, Seq<T::V>)
  2. The ensures clause on next() is unchanged
  3. The GhostIterator infrastructure is consistent
  4. Run scripts/validate.sh — 0 errors
  5. Run scripts/rtt.sh — TestAVLTreeSeq.rs has 9 tests using .iter()

Do NOT add assume, accept, or external_body.
Every quantifier must have explicit #[trigger].
