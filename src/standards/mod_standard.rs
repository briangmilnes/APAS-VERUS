// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Module Standard: how to structure an APAS-VERUS module file.
//!
//! Every APAS-VERUS source file wraps its content in a `pub mod ModuleName` block.
//! The module name matches the file name (without extension). Inside the module,
//! imports come first, then `verus!` wraps sections 2-11. Sections 12-13 go
//! outside `verus!` but inside the module.
//!
//! Table of Contents sections:
//!  1. module          — `pub mod Name {`
//!  2. imports         — `use` statements inside verus!
//!  3. broadcast use   — broadcast group imports
//!  4. type definitions
//!  5. view impls
//!  6. spec fns
//!  7. proof fns / broadcast groups
//!  8. traits
//!  9. impls
//! 10. iterators
//! 11. derive impls in verus!
//! 12. macros          — outside verus!, inside module
//! 13. derive impls outside verus!
//!
//! Reference: src/Chap18/ArraySeqStEph.rs
// 1. module
pub mod mod_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    // 5. view impls
    /// View maps Vec<T> to Seq<T> (the abstract sequence).
    impl<T> View for ExampleS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    impl<'a, T> View for ExampleIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<'a, T> View for ExampleGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    // 6. spec fns
    /// Well-formedness predicate (named spec_<module>_wf per convention).
    pub open spec fn spec_modstandard_wf<T>(s: &ExampleS<T>) -> bool {
        s@.len() >= 0
    }

    pub open spec fn iter_invariant<'a, T>(it: &ExampleIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 8. traits
    pub trait ExampleTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;

        fn length(&self) -> (len: usize)
            ensures
                len as nat == self.spec_len(),
        ;
    }

    // 9. impls
    impl<T> ExampleTrait<T> for ExampleS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
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

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }
    }

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

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ExampleIter<'a, T> {
        type GhostIter = ExampleGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> ExampleGhostIterator<'a, T> {
            ExampleGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

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

        open spec fn ghost_advance(&self, _exec_iter: &ExampleIter<'a, T>) -> ExampleGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

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
} // pub mod mod_standard
