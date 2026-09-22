//  APAS-VERUS Iterator Standard (verus 0.2026.09.13 prophetic iterator model)
//
//  This file defines the standard iterator implementation pattern for
//  APAS-VERUS data structures as a component checklist. The verified
//  reference implementations are the standards themselves:
//      src/standards/iterators_standard.rs            delegated style, loop idioms
//      src/standards/prophetic_iterators_standard.rs  delegated and custom styles
//      src/standards/wrapping_iterators_standard.rs   re-expose and adaptor
//  The prose reference is docs/PropheticIterators.md. The 09.13 measurements
//  behind each rule are in docs/StandardsUpgrade.md.
//
//  Contents
//   0. Rust's three standard for-loop patterns
//   1. Required components, by style
//   2. Constructor postconditions
//   3. Loop invariants and the decreases rule
//   4. Proof-time tests (six patterns)
//   5. Compliance

//  0. RUST'S THREE STANDARD FOR-LOOP PATTERNS
//
//  Rust's `for` loop desugars to `IntoIterator::into_iter()`.  The Rust
//  standard library defines three conventional iteration patterns:
//
//   #  Pattern                    Trait / Method          Yields    Ownership
//   1  for x in &collection      IntoIterator for &C     &T        borrows
//   2  for x in &mut collection  IntoIterator for &mut C &mut T    mut borrows
//   3  for x in collection       IntoIterator for C      T         consumes
//
//  APAS-VERUS support:
//
//   Pattern 1 (borrow / iter):  Required.  `iter()` and `IntoIterator for
//       &Self`, both with the constructor postconditions of section 2.
//
//   Pattern 2 (mut borrow / iter_mut):  Not adopted.  vstd 0.2026.09.13
//       specifies `std::slice::IterMut` (vstd/std_specs/slice.rs), so it is
//       now specifiable; modules that carry an unspecified `iter_mut` keep it
//       as-is until a standard covers mutable iteration.
//
//   Pattern 3 (consuming / into_iter):  Supported.  `IntoIterator for Self`
//       returning `std::vec::IntoIter<T>` (or the custom type), with the
//       same postconditions.  Yields owned `T`, not `&T`.

//  1. REQUIRED COMPONENTS, BY STYLE
//
//  Delegated (a Vec-backed collection; 68 of the 71 APAS iterators).  All
//  inside verus!.  No iterator struct, no iterator View, no ghost iterator:
//  vstd supplies `IteratorSpecImpl` for `std::slice::Iter` and
//  `std::vec::IntoIter`.
//
//   #  Component                                       Section
//   1  fn iter(&self) -> std::slice::Iter<'_, T>        8 (trait) / 9 (impl)
//   2  impl IntoIterator for &Self  (same type, same    10
//      postconditions)
//   3  impl IntoIterator for Self -> std::vec::IntoIter 10 (optional)
//
//  Custom (a collection with no slice underneath; the three lazy AVLTreeSeq
//  iterators).  All inside verus!, in section 10 of the parent type.
//
//   #  Component
//   1  The iterator struct, private fields, `#[verifier::type_invariant]`
//   2  A closed spec fn `elts()` for the stable creation-time contents
//   3  impl Iterator: `next` with no `ensures` (the spec is the trait impl)
//   4  impl IteratorSpecImpl: the five spec fns
//        obeys_prophetic_iter_laws   -> true for a verified iterator
//        remaining (prophetic)       -> the items still to be returned
//        will_return_none (prophetic)-> true for a terminating iterator
//        decrease                    -> Some(non-prophetic metric)
//        peek(index)                 -> Some(elts()[index]) in range
//   5  A constructor: exec fn with `#[verifier::when_used_as_spec]` naming an
//      open spec form, and the postconditions of section 2
//   Optional: ExactSizeIteratorSpecImpl::exact_len,
//             DoubleEndedIteratorSpecImpl::peek_back.
//
//  Wrapping (a collection that wraps another).  Re-expose the inner
//  collection's iterator: `OuterS::iter()` returns the same std type
//  `InnerS::iter()` returns, restating the postconditions over the outer
//  view.  Only a module that must own its iterator type writes an adaptor
//  `OuterIter { inner }` whose `remaining`, `will_return_none`, `decrease`
//  and `peek` forward to the inner iterator's and whose
//  `obeys_prophetic_iter_laws` is the inner value written out (`true` for a
//  std iterator).

//  2. CONSTRUCTOR POSTCONDITIONS
//
//  Every `iter()`, `into_iter()` and custom constructor ensures three things
//  (the guide's triple, examples/guide/iterators.rs):
//
//      IteratorSpec::remaining(&it) == self@.as_ref(),          // 1. prophetic seq
//      vstd::std_specs::slice::into_iter_elts(it) == self@,     // 2. what peek reads
//      IteratorSpec::decrease(&it) is Some,                     // 3. for termination
//
//  For a consuming iterator drop `.as_ref()` and use
//  `vstd::std_specs::vec::into_iter_elts`.  For a custom type clause 2 is
//  `IteratorSpec::remaining(&it) == it.elts()`.
//
//  Rule for trait methods: when the constructor is a trait method and returns
//  a type whose `IteratorSpecImpl` and `next` are verified in this crate,
//  clauses 1 and 3 name the inner std iterator (`&it.inner`), not the
//  returned type; naming the returned type makes its `next` check fail on
//  09.13.  Inherent methods and free functions may name the returned type.

//  3. LOOP INVARIANTS AND THE DECREASES RULE
//
//  A `for` loop names its wrapper and reasons through `it.index()` (items
//  consumed), the prophetic `it.seq()` (the whole sequence) and
//  `it.history()` (items consumed so far):
//
//      for x in it: coll.iter()
//          invariant
//              it.seq() == orig.as_ref(),
//              collected.len() == it.index(),
//              forall|i: int| 0 <= i < collected.len()
//                  ==> #[trigger] collected@[i] == *it.seq()[i],
//      {
//          collected.push(*x);
//      }
//      assert(collected@ =~= orig);   // it.index() == it.seq().len() afterwards
//
//  A manual loop runs on the iterator's own `next()` (not on
//  `VerusForLoopWrapper`, which vstd declares only under `verus_keep_ghost`
//  and which does not compile under cargo), with a ghost consumed-count
//  `pos` and the wrapper's wf_inner written out over `remaining()`:
//
//      let mut it = coll.iter();
//      let ghost mut pos: int = 0;
//      loop
//          invariant
//              IteratorSpec::obeys_prophetic_iter_laws(&it),
//              IteratorSpec::decrease(&it) is Some,
//              0 <= pos <= orig.len(),
//              IteratorSpec::remaining(&it).len() == orig.len() - pos,
//              forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
//                  ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
//              collected.len() == pos,
//              forall|i: int| 0 <= i < collected.len()
//                  ==> #[trigger] collected@[i] == orig[i],
//          decreases IteratorSpec::decrease(&it)->0,
//      {
//          let ghost old_pos = pos;
//          match it.next() {
//              Some(x) => {
//                  proof { pos = pos + 1; assert(orig[old_pos] == *x); }
//                  collected.push(*x);
//              },
//              None => { assert(pos == orig.len()); assert(collected@ =~= orig); break; },
//          }
//      }
//
//  The decreases rule: `remaining()` and `it.seq()` are prophetic and may not
//  appear in `decreases`; measure the non-prophetic
//  `IteratorSpec::decrease(&it)->0`. A manual loop draws its conclusion
//  before `break`, because the prophetic equality does not survive the
//  break.  `it` is not in scope after a `for` loop.

//  4. PROOF-TIME TESTS (SIX PATTERNS)
//
//  Every collection with an iterator has these tests in
//      rust_verify_test/tests/<Chap>/Prove<Collection>.rs
//
//   #  Pattern            Creates the iterator via                       Yields
//   1  loop-borrow-iter   let mut it = a.iter(); loop { it.next() }      &T
//   2  loop-borrow-into   let mut it = (&a).into_iter(); loop { .. }     &T
//   3  for-borrow-iter    for x in it: a.iter()                          &T
//   4  for-borrow-into    for x in it: (&a).into_iter()                  &T
//   5  loop-consume       let mut it = a.into_iter(); loop { .. }        T
//   6  for-consume        for x in it: a.into_iter()                     T
//
//  Patterns 5 and 6 apply only to collections with `IntoIterator for Self`.
//  The templates are in src/standards/iterator_ptt_standard.rs; the verified
//  instances are rust_verify_test/tests/standards/Proveiterators_standard.rs
//  and Proveprophetic_iterators_standard.rs (the latter adds `for-custom` and
//  `loop-custom` over a custom iterator).

//  5. COMPLIANCE
//
//  The per-file inventory of all 71 APAS iterators, their style (delegated
//  or custom) and their cost profile is the table in
//  docs/PropheticIterators.md.  The migration of the chapter files to this
//  model is planned in plans/verus-0.2026.05.21-iterator-migration.md.
