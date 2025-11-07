//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `vstd::hash_set::HashSetWithView`.
//!
//! Maximally verified approach: verify what we can (size, mem, insert)
//! Trust operations that require iteration (HashSetWithView doesn't expose iter())

pub mod SetStEph {

    use vstd::prelude::View;
    use vstd::prelude::{Set, verus, old};
    use vstd::hash_set::HashSetWithView;
    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use crate::Types::Types::*;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use vstd::std_specs::hash::group_hash_axioms;

/// Verified ephemeral Set wrapping HashSetWithView
/// Note: Limited API due to HashSetWithView not exposing iterators
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
pub struct SetStEph<T: vstd::prelude::View + Eq + Hash> {
    pub data: HashSetWithView<T>,
}

impl<T: vstd::prelude::View + Eq + Hash> vstd::prelude::View for SetStEph<T> {
    type V = Set<<T as vstd::prelude::View>::V>;

    open spec fn view(&self) -> Set<<T as vstd::prelude::View>::V> {
        self.data@
    }
}

pub trait SetStEphTrait<T: StT + Hash>: Sized + vstd::prelude::View {
    /// APAS: Work Θ(1), Span Θ(1)
    /// TRUSTED: generic obeys_key_model
    fn empty() -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    /// TRUSTED: generic obeys_key_model
    fn singleton(x: T) -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    /// VERIFIED
    fn size(&self) -> (result: N)
        ensures result == vstd::prelude::View::view(self).len();

    /// APAS: Work Θ(1), Span Θ(1)
    /// VERIFIED
    fn mem(&self, x: &T) -> (result: B)
        ensures result == vstd::prelude::View::view(self).contains(x@);

    /// APAS: Work Θ(1), Span Θ(1)
    /// VERIFIED
    fn insert(&mut self, x: T)
        ensures vstd::prelude::View::view(self) == vstd::prelude::View::view(&old(self)).insert(x@);
}

// Maximally verified implementation
impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
    // TRUSTED: Can't verify generic obeys_key_model
    #[verifier::external_body]
    fn empty() -> (result: SetStEph<T>)
        ensures <SetStEph<T> as vstd::prelude::View>::view(&result) == Set::<<T as vstd::prelude::View>::V>::empty()
    {
        SetStEph {
            data: HashSetWithView::new(),
        }
    }

    // TRUSTED: Can't verify generic obeys_key_model
    #[verifier::external_body]
    fn singleton(x: T) -> (result: SetStEph<T>)
        ensures <SetStEph<T> as vstd::prelude::View>::view(&result) == Set::<<T as vstd::prelude::View>::V>::empty().insert(x@)
    {
        let mut s = HashSetWithView::new();
        s.insert(x);
        SetStEph { data: s }
    }

    // VERIFIED: Direct call to HashSetWithView::len
    fn size(&self) -> (result: N)
        ensures result == self@.len()
    {
        self.data.len()
    }

    // VERIFIED: Direct call to HashSetWithView::contains
    fn mem(&self, x: &T) -> (result: B)
        ensures result == self@.contains(x@)
    {
        self.data.contains(x)
    }

    // VERIFIED: Direct call to HashSetWithView::insert
    fn insert(&mut self, x: T)
        ensures self@ == old(self)@.insert(x@)
    {
        self.data.insert(x);
    }
}

    } // verus!

    #[macro_export]
    macro_rules! SetLit {
        () => {{
            < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
            $( __s.insert($x); )*
            __s
        }};
    }
}
