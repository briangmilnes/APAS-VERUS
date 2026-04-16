// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Table of Contents Standard: the complete APAS-VERUS module template.
//!
//! Every APAS-VERUS source file follows this bottom-up, per-type ordering.
//! Each type gets a complete 4-10 cycle (struct, view, specs, proofs, trait,
//! impl, iterators) grouped together, ordered leaf-first (bottom-up). This
//! makes it easy for a human reader to skip to the type they care about.
//!
//! Sections 1-3 are global (one per file). Sections 4-10 repeat per type,
//! with letter suffixes (a, b, c...) and the type name in the header comment.
//! Section 11 (coarse locking) appears once, right before end of verus!.
//! Sections 12-14 also repeat per type, bottom-up.
//!
//! This file demonstrates a module with two structs: InnerS (leaf) and
//! ExampleS (depends on InnerS). InnerS comes first (bottom-up). ExampleS
//! has iterators, showing that iterator structs, views, ghost structs, and
//! all iterator impls live together in section 10 with their parent type.
//!
//! Section 11 (top level coarse locking) is for Mt modules that wrap a verified
//! St struct in an RwLock. It contains the complete Layer 2: Inv struct,
//! RwLockPredicate impl, Locked struct, type_invariant inherent impl, Locked
//! View, LockedTrait, and LockedTrait impl. Inside verus!, after all type
//! groups, before derive impls. Omit for files that do not use this pattern.
//! See toplevel_coarse_rwlocks_for_mt_modules.rs for the full standard.
//!
//! Reference: src/Chap05/SetStEph.rs (the only chapter file with all 14 sections).
//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions — struct InnerS
//	Section 5a. view impls — struct InnerS
//	Section 6a. spec fns — struct InnerS
//	Section 7a. proof fns/broadcast groups — struct InnerS
//	Section 8a. traits — struct InnerS
//	Section 9a. impls — struct InnerS
//	Section 4b. type definitions — struct ExampleS
//	Section 5b. view impls — struct ExampleS
//	Section 6b. spec fns — struct ExampleS
//	Section 7b. proof fns/broadcast groups — struct ExampleS
//	Section 8b. traits — struct ExampleS
//	Section 9b. impls — struct ExampleS
//	Section 10b. iterators — struct ExampleS
//	Section 11. top level coarse locking
//	Section 12a. derive impls in verus! — struct InnerS
//	Section 12b. derive impls in verus! — struct ExampleS
//	Section 13a. macros — struct InnerS
//	Section 13b. macros — struct ExampleS
//	Section 14a. derive impls outside verus! — struct InnerS
//	Section 14b. derive impls outside verus! — struct ExampleS
//		Section 1. module
pub mod table_of_contents_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    //		Section 2. imports
    //
    // use std::... first, then vstd, then crate modules.
    // Verus-only imports go behind #[cfg(verus_keep_ghost)].
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    //		Section 3. broadcast use
    //
    // Import broadcast groups so their lemmas fire automatically.
    broadcast use vstd::seq::group_seq_axioms;

    // ========================================================================
    // Type group a: InnerS (leaf type, no dependencies)
    // ========================================================================

    //		Section 4a. type definitions — struct InnerS

    pub struct InnerS {
        pub val: u64,
    }

    //		Section 5a. view impls — struct InnerS

    impl View for InnerS {
        type V = int;

        open spec fn view(&self) -> int {
            self.val as int
        }
    }

    //		Section 6a. spec fns — struct InnerS

    pub open spec fn spec_inners_wf(s: &InnerS) -> bool {
        s@ >= 0
    }

    //		Section 7a. proof fns/broadcast groups — struct InnerS

    pub broadcast proof fn lemma_inners_view_nat(s: &InnerS)
        ensures
            #[trigger] s@ >= 0,
    {
    }

    pub broadcast group group_inners_lemmas {
        lemma_inners_view_nat,
    }

    //		Section 8a. traits — struct InnerS

    pub trait InnerSTrait: Sized {
        spec fn spec_val(&self) -> int;

        fn new(val: u64) -> (s: Self)
            ensures
                s.spec_val() == val as int,
        ;

        fn get_val(&self) -> (v: u64)
            ensures
                v as int == self.spec_val(),
        ;
    }

    //		Section 9a. impls — struct InnerS

    impl InnerSTrait for InnerS {
        open spec fn spec_val(&self) -> int {
            self@
        }

        fn new(val: u64) -> (s: Self) {
            InnerS { val }
        }

        fn get_val(&self) -> (v: u64) {
            self.val
        }
    }

    // InnerS has no iterators, so no 10a section.

    // ========================================================================
    // Type group b: ExampleS (depends on InnerS, has iterators)
    // ========================================================================

    //		Section 4b. type definitions — struct ExampleS

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    //		Section 5b. view impls — struct ExampleS

    impl<T> View for ExampleS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    //		Section 6b. spec fns — struct ExampleS
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

    //		Section 7b. proof fns/broadcast groups — struct ExampleS

    /// View length is always non-negative.
    pub broadcast proof fn lemma_view_len_nat<T>(s: &ExampleS<T>)
        ensures
            #[trigger] s@.len() >= 0,
    {
    }

    pub broadcast group group_example_lemmas {
        lemma_view_len_nat,
    }

    //		Section 8b. traits — struct ExampleS
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

    //		Section 9b. impls — struct ExampleS

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

    // iter() method — lives in 9b (impls) because it's an inherent method on ExampleS.
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

    // PartialEqSpecImpl goes here (section 9b) because the style checker sees
    // cfg-gated trait impls as regular impls, which must precede section 10.
    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ExampleS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    //		Section 10b. iterators — struct ExampleS
    //
    // Everything iterator-related for ExampleS lives here: the iterator struct,
    // its view, the ghost iterator struct, its view, iter_invariant, all trait
    // impls (Iterator, ForLoopGhostIteratorNew, ForLoopGhostIterator,
    // IntoIterator). This keeps the full iterator story in one place.

    // Component 1: Custom iterator struct.
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    // Component 2: View for iterator.
    impl<'a, T> View for ExampleIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    // Component 3: iter_invariant spec fn.
    pub open spec fn iter_invariant<'a, T>(it: &ExampleIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

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

    // Component 5: Ghost iterator struct.
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
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

    // Component 8: View for ghost iterator.
    impl<'a, T> View for ExampleGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
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

    //		Section 11. top level coarse locking
    //
    // Mt modules only. Contains the complete Layer 2 locking wrapper:
    // Inv struct, RwLockPredicate impl, Locked struct, type_invariant
    // inherent impl (with closed accessor), Locked View, LockedTrait,
    // and LockedTrait impl. Omit for St modules and Mt modules that
    // do not use coarse locking.
    // See: src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs

    //		Section 12a. derive impls in verus! — struct InnerS

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for InnerS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl Clone for InnerS {
        fn clone(&self) -> (out: Self) {
            InnerS { val: self.val }
        }
    }

    impl PartialEq for InnerS {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = (self.val == other.val);
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl Eq for InnerS {}

    //		Section 12b. derive impls in verus! — struct ExampleS
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

    //		Section 13a. macros — struct InnerS
    //
    // (InnerS has no macro in this example.)

    //		Section 13b. macros — struct ExampleS
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

    //		Section 14a. derive impls outside verus! — struct InnerS

    impl Display for InnerS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }

    impl Debug for InnerS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerS({})", self.val)
        }
    }

    //		Section 14b. derive impls outside verus! — struct ExampleS
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
