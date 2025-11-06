//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set - verified wrapper around vstd::hash_set::HashSetWithView.
//!
//! NOTE: vstd::hash_set::HashSetWithView doesn't yet expose iterators, so union/intersection
//! operations are not yet implementable. For now, we provide the core operations (empty, singleton,
//! mem, insert) and stub out union/intersection for future implementation.

pub mod SetStEph {
    use vstd::prelude::*;
    use vstd::hash_set::*;

    use crate::Types::Types::*;

    verus! {

/// Verified ephemeral Set wrapping HashSetWithView
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
pub struct SetStEph<T: View + Eq + std::hash::Hash> {
    pub data: HashSetWithView<T>,
}

pub trait SetStEphTrait<T: View> {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self
    where
        Self: Sized;

    /// APAS: Work Θ(1), Span Θ(1)
    fn singleton(x: T) -> Self
    where
        Self: Sized;

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

impl<T> SetStEphTrait<T> for SetStEph<T>
where
    T: View + std::hash::Hash + Eq,
{
    fn empty() -> (result: SetStEph<T>)
        ensures result.view() == Set::<<T as View>::V>::empty()
    {
        SetStEph {
            data: HashSetWithView::new(),
        }
    }

    fn singleton(x: T) -> (result: SetStEph<T>)
        ensures result.view() == Set::<<T as View>::V>::empty().insert(x@)
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
