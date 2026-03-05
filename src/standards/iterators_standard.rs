//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Iterator Standard: how to implement verified iterators in APAS-VERUS.
//!
//! Every APAS-VERUS collection implements the iterator standard, which enables
//! verified iteration via both manual `loop` and Verus `for x in iter: it`.
//!
//! This file shows all 10 required components for a Vec<T>-backed collection.
//! The canonical reference implementation is src/Chap18/ArraySeqStEph.rs.
//! The full specification is in docs/APAS-VERUSIterators.rs.
//!
//! Components (all inside verus!):
//!  1. Custom iterator struct (section 4)
//!  2. View for iterator: (int, Seq<T>) (section 5)
//!  3. iter_invariant spec fn (section 6)
//!  4. Iterator::next with two-arm ensures (section 10)
//!  5. Ghost iterator struct (section 4)
//!  6. ForLoopGhostIteratorNew impl (section 10)
//!  7. ForLoopGhostIterator impl (6 spec fns) (section 10)
//!  8. View for ghost iterator: elements.take(pos) (section 5)
//!  9. iter() method with ensures (section 10)
//! 10. IntoIterator for &Self (section 10)
//!
//! Optional: IntoIterator for Self (consuming pattern).
// 1. module
pub mod iterators_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    // Component 1: Custom iterator struct.
    // Wraps the underlying Rust iterator. The inner field is pub for vstd access.
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    // Component 5: Ghost iterator struct.
    // Pure spec-level state for the ForLoopGhostIterator protocol.
    // Fields are pub so for-loop invariants can reference iter.pos, iter.elements.
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    // 5. view impls

    // Component 2: View for iterator.
    // The View is a pair: (position_index, full_sequence).
    // Position starts at 0 and advances to elements.len().
    // The sequence is fixed at creation time.
    impl<'a, T> View for ExampleIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    // Component 8: View for ghost iterator.
    // Items seen so far: the prefix of length pos.
    // This is what user code asserts against after the loop completes.
    impl<'a, T> View for ExampleGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    // 6. spec fns

    // Component 3: iter_invariant spec fn.
    // Bounds the position index. Users include this in loop invariants.
    pub open spec fn iter_invariant<'a, T>(it: &ExampleIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 8. traits

    pub trait ExampleTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends
                i < self.spec_len(),
        ;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;
    }

    // 9. impls

    impl<T> ExampleTrait<T> for ExampleS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq@[i]
        }

        fn new(length: usize, init: T) -> (s: Self) where T: Copy {
            let mut v: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    v@.len() == i as int,
                decreases length - i,
            {
                v.push(init);
                i = i + 1;
            }
            ExampleS { seq: v }
        }
    }

    // Component 9: iter() method with ensures.
    // Entry point for iteration. Position starts at 0, sequence matches data.
    impl<T> ExampleS<T> {
        pub fn iter(&self) -> (it: ExampleIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ExampleIter { inner: self.seq.iter() }
        }
    }

    // 10. iterators

    // Component 4: Iterator::next with ensures.
    // Two arms: None (exhausted) and Some (produced an element).
    // None: iterator unchanged, position at/past end.
    // Some: sequence unchanged, position advances by 1, element at old position.
    impl<'a, T> std::iter::Iterator for ExampleIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (next: Option<&'a T>)
            ensures
                ({
                    let (old_index, old_seq) = old(self)@;
                    match next {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    // Component 6: ForLoopGhostIteratorNew.
    // Creates ghost state from the exec iterator at loop start.
    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ExampleIter<'a, T> {
        type GhostIter = ExampleGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> ExampleGhostIterator<'a, T> {
            ExampleGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    // Component 7: ForLoopGhostIterator.
    // The full ghost-loop protocol with six spec functions.
    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ExampleGhostIterator<'a, T> {
        type ExecIter = ExampleIter<'a, T>;

        type Item = T;

        type Decrease = int;

        /// Links ghost state to exec iterator.
        open spec fn exec_invariant(&self, exec_iter: &ExampleIter<'a, T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        /// Maintained across iterations; init is state before first iteration.
        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        /// Holds after the loop exits normally.
        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        /// Termination measure.
        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        /// What the next call to next() will yield (before the call).
        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        /// Ghost state after processing one element.
        open spec fn ghost_advance(&self, _exec_iter: &ExampleIter<'a, T>) -> ExampleGhostIterator<
            'a,
            T,
        > {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    // Component 10: IntoIterator for &Self.
    // Enables `for x in &collection` syntax.
    impl<'a, T> std::iter::IntoIterator for &'a ExampleS<T> {
        type Item = &'a T;

        type IntoIter = ExampleIter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ExampleIter { inner: self.seq.iter() }
        }
    }

    // Optional: IntoIterator for Self (consuming pattern).
    // Yields owned T, not &T. Uses vstd's built-in IntoIter View and ghost support.
    impl<T> std::iter::IntoIterator for ExampleS<T> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        {
            self.seq.into_iter()
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: Debug> Debug for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleS({:?})", self.seq)
        }
    }

    impl<T: Display> Display for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    impl<'a, T: Debug> Debug for ExampleIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for ExampleIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleIter")
        }
    }

    impl<'a, T> Debug for ExampleGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleGhostIterator")
        }
    }

    impl<'a, T> Display for ExampleGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleGhostIterator")
        }
    }
} // pub mod iterators_standard
