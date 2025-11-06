//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set - verified wrapper around vstd::hash_set::HashSetWithView.
//!
//! NOTE: vstd::hash_set::HashSetWithView doesn't yet expose iterators, so union/intersection
//! operations are not yet implementable. For now, we provide the core operations (empty, singleton,
//! mem, insert) and stub out union/intersection for future implementation.

pub mod SetStEph {
    use vstd::prelude::*;
    use vstd::hash_set::*;
    use vstd::std_specs::hash::obeys_key_model;
    use std::hash::Hash;

    use crate::Types::Types::*;

    verus! {

broadcast use vstd::std_specs::hash::group_hash_axioms;

/// Verified ephemeral Set wrapping HashSetWithView
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
pub struct SetStEph<T: View + Eq + Hash> {
    pub data: HashSetWithView<T>,
}

pub trait SetStEphTrait<T: View + Eq + Hash>: Sized {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self
        requires obeys_key_model::<T>();

    /// APAS: Work Θ(1), Span Θ(1)
    fn singleton(x: T) -> Self
        requires obeys_key_model::<T>();

    /// APAS: Work Θ(1), Span Θ(1)
    fn size(&self) -> (result: N)
        ensures result == self.view().len();

    /// APAS: Work Θ(1), Span Θ(1)
    fn mem(&self, x: &T) -> (result: B)
        ensures result == self.view().contains(x@);

    /// APAS: Work Θ(1), Span Θ(1)
    fn insert(&mut self, x: T)
        ensures self.view() == old(self).view().insert(x@);

    spec fn view(&self) -> Set<<T as View>::V>;
}

impl<T: View + Eq + Hash> SetStEphTrait<T> for SetStEph<T> {
    fn empty() -> (result: SetStEph<T>)
        ensures result.view() == Set::<<T as View>::V>::empty()
    {
        // TODO: Remove assume once we understand how to propagate obeys_key_model from requires
        proof {
            assume(obeys_key_model::<T>());
        }
        SetStEph {
            data: HashSetWithView::new(),
        }
    }

    fn singleton(x: T) -> (result: SetStEph<T>)
        ensures result.view() == Set::<<T as View>::V>::empty().insert(x@)
    {
        // TODO: Remove assume once we understand how to propagate obeys_key_model from requires
        proof {
            assume(obeys_key_model::<T>());
        }
        let mut s = HashSetWithView::new();
        s.insert(x);
        SetStEph { data: s }
    }

    fn size(&self) -> (result: N)
        ensures result == self.view().len()
    {
        self.data.len()
    }

    fn mem(&self, x: &T) -> (result: B)
        ensures result == self.view().contains(x@)
    {
        self.data.contains(x)
    }

    fn insert(&mut self, x: T)
        ensures self.view() == old(self).view().insert(x@)
    {
        self.data.insert(x);
    }

    open spec fn view(&self) -> Set<<T as View>::V> {
        self.data@
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
