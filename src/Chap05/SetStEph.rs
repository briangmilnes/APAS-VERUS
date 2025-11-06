//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set - verified wrapper around vstd::hash_set::HashSetWithView.
//!
//! NOTE: vstd::hash_set::HashSetWithView doesn't yet expose iterators, so union/intersection
//! operations are not yet implementable. For now, we provide the core operations (empty, singleton,
//! mem, insert) and stub out union/intersection for future implementation.

pub mod SetStEph {
    use vstd::prelude::*;
    use vstd::hash_set::*;
    use std::hash::Hash;

    use crate::Types::Types::*;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use vstd::std_specs::hash::group_hash_axioms;

// Experimental: generic axiom for obeys_key_model
// WARNING: This is unsound - not all types actually obey the key model!
pub proof fn axiom_generic_obeys_hash_table_key_model<T>()
    ensures
        #[trigger] obeys_key_model::<T>(),
{
    admit();
}

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

// Concrete implementation for u64
// This should verify because vstd has axiom_u64_obeys_hash_table_key_model
impl SetStEphTrait<u64> for SetStEph<u64> {
    fn empty() -> (result: SetStEph<u64>)
        ensures result.view() == Set::<u64>::empty()
    {
        SetStEph {
            data: HashSetWithView::new(),
        }
    }

    fn singleton(x: u64) -> (result: SetStEph<u64>)
        ensures result.view() == Set::<u64>::empty().insert(x@)
    {
        let mut s = HashSetWithView::new();
        s.insert(x);
        SetStEph { data: s }
    }

    fn size(&self) -> (result: N)
        ensures result == self.view().len()
    {
        self.data.len()
    }

    fn mem(&self, x: &u64) -> (result: B)
        ensures result == self.view().contains(x@)
    {
        self.data.contains(x)
    }

    fn insert(&mut self, x: u64)
        ensures self.view() == old(self).view().insert(x@)
    {
        self.data.insert(x);
    }

    open spec fn view(&self) -> Set<u64> {
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
