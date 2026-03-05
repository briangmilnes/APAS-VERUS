//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Table of Contents Standard: the complete APAS-VERUS module template.
//!
//! Every APAS-VERUS source file follows this 13-section ordering. This file
//! contains compilable code in every section, showing where each kind of
//! definition belongs.
//!
//! Sections 1-11 live inside verus!. Sections 12-13 live outside verus! but
//! inside the pub mod.
//!
//! Key pattern demonstrated: spec fns (section 6) can have looser bounds
//! (T: View) than traits/impls (sections 8-9) which need exec bounds
//! (T: View + Copy + PartialEq). This lets spec fns be reusable across
//! more contexts.
//!
//! Reference: src/Chap05/SetStEph.rs (the only chapter file with all 13 sections).
//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4. type definitions
//  5. view impls
//  6. spec fns
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  10. iterators
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!
// 1. module
pub mod table_of_contents_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    // 2. imports
    //
    // use std::... first, then vstd, then crate modules.
    // Verus-only imports go behind #[cfg(verus_keep_ghost)].
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    // 3. broadcast use
    //
    // Import broadcast groups so their lemmas fire automatically.
    broadcast use vstd::seq::group_seq_axioms;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    // Component 1: Custom iterator struct.
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    // Component 5: Ghost iterator struct.
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    // 5. view impls
    //
    // View maps the concrete type to its abstract spec type.
    impl<T> View for ExampleS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    // Component 2: View for iterator.
    impl<'a, T> View for ExampleIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    // Component 8: View for ghost iterator.
    impl<'a, T> View for ExampleGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    // 6. spec fns
    //
    // Spec functions can have LOOSER bounds than the trait (section 8).
    // Here: T: View (spec-only). The trait requires T: View + Copy + PartialEq.
    /// Well-formedness predicate. Bound is T: View, not the full exec bound.
    pub open spec fn spec_tableofcontentsstandard_wf<T: View>(s: &ExampleS<T>) -> bool {
        s@.len() >= 0
    }

    /// Spec-level length. Again, only needs T: View.
    pub open spec fn spec_len<T: View>(s: &ExampleS<T>) -> nat {
        s@.len()
    }

    // Component 3: iter_invariant spec fn.
    pub open spec fn iter_invariant<'a, T>(it: &ExampleIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 7. proof fns/broadcast groups
    //
    // Lemmas and broadcast groups. Proof fns are spec/proof mode only.
    /// View length is always non-negative.
    pub broadcast proof fn lemma_view_len_nat<T>(s: &ExampleS<T>)
        ensures
            #[trigger] s@.len() >= 0,
    {
    }

    pub broadcast group group_example_lemmas {
        lemma_view_len_nat,
    }

    // 8. traits
    //
    // Traits have the full exec bounds needed by their methods.
    // Spec functions in traits are abstract (no body) or open.
    pub trait ExampleTrait<T: View + Copy + PartialEq>: Sized {
        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends
                i < self.spec_len(),
        ;

        fn new(length: usize, init: T) -> (s: Self)
            ensures
                s.spec_len() == length as nat,
        ;

        fn length(&self) -> (len: usize)
            ensures
                len as nat == self.spec_len(),
        ;

        fn nth(&self, i: usize) -> (val: &T)
            requires
                i < self.spec_len(),
            ensures
                *val == self.spec_index(i as int),
        ;

        fn is_empty(&self) -> (empty: bool)
            ensures
                empty == (self.spec_len() == 0),
        ;
    }

    // 9. impls
    impl<T: View + Copy + PartialEq> ExampleTrait<T> for ExampleS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self@[i]
        }

        fn new(length: usize, init: T) -> (s: Self) {
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

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        fn nth(&self, i: usize) -> (val: &T) {
            &self.seq[i]
        }

        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }
    }

    // Component 9: iter() method with ensures.
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

    // PartialEqSpecImpl goes here (section 9) because the style checker sees
    // cfg-gated trait impls as regular impls, which must precede section 10.
    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ExampleS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    // 10. iterators
    //
    // Iterator trait impls. Type definitions and views are in sections 4-5.

    // Component 4: Iterator::next with ensures.
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
    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ExampleIter<'a, T> {
        type GhostIter = ExampleGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> ExampleGhostIterator<'a, T> {
            ExampleGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    // Component 7: ForLoopGhostIterator.
    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ExampleGhostIterator<'a, T> {
        type ExecIter = ExampleIter<'a, T>;

        type Item = T;

        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ExampleIter<'a, T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &ExampleIter<'a, T>) -> ExampleGhostIterator<
            'a,
            T,
        > {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    // Component 10: IntoIterator for &Self.
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

    // Optional: IntoIterator for Self (consuming).
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

    // 11. derive impls in verus!
    //
    // Clone, PartialEq, Eq go inside verus! so they can have ensures.
    impl<T: Clone> Clone for ExampleS<T> {
        fn clone(&self) -> (out: Self) {
            ExampleS { seq: self.seq.clone() }
        }
    }

    impl<T: PartialEq + View> PartialEq for ExampleS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.seq == other.seq;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: Eq + View> Eq for ExampleS<T> {}

    } // verus!

      // 12. macros
      //
      // macro_rules! goes outside verus!, inside the pub mod.
      // Use $crate for fully qualified paths so the macro works from any crate.
    #[macro_export]
    macro_rules! ExampleLit {
        () => {{
            $crate::standards::table_of_contents_standard::table_of_contents_standard::ExampleS {
                seq: vec![],
            }
        }};
        ($($x:expr),* $(,)?) => {{
            $crate::standards::table_of_contents_standard::table_of_contents_standard::ExampleS {
                seq: vec![$($x),*],
            }
        }};
    }

    // 13. derive impls outside verus!
    //
    // Display and Debug go outside verus!, inside the pub mod.

    impl<T: Display> Display for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    impl<T: Debug> Debug for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleS({:?})", self.seq)
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
} // pub mod table_of_contents_standard
