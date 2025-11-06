//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation - verified wrapper around HashSetWithView<(T,U)>.
//!
//! NOTE: Uses (T,U) tuples instead of Pair<T,U> because vstd's View trait is implemented
//! for Rust tuples but not for custom Pair types.

pub mod RelationStEph {
    use vstd::prelude::*;
    use vstd::hash_set::*;

    use crate::Types::Types::*;

    verus! {

/// Verified ephemeral Relation wrapping HashSetWithView<(T,U)>
pub struct RelationStEph<T: View + Eq + std::hash::Hash, U: View + Eq + std::hash::Hash> {
    pairs: HashSetWithView<(T, U)>,
}

pub trait RelationStEphTrait<T: View, U: View> {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    fn size(&self) -> (result: N)
        ensures result == self.view().len();

    /// APAS: Work Θ(1), Span Θ(1)
    /// Note: Requires T and U to be Copy
    fn mem(&self, t: &T, u: &U) -> (result: B)
        ensures result == self.view().contains((t@, u@));

    /// APAS: Work Θ(1), Span Θ(1)
    fn insert(&mut self, t: T, u: U)
        ensures self.view() == old(self).view().insert((t@, u@));

    spec fn view(&self) -> Set<(<T as View>::V, <U as View>::V)>;
}

impl<T, U> RelationStEph<T, U>
where
    T: View + std::hash::Hash + Eq,
    U: View + std::hash::Hash + Eq,
{
    pub open spec fn view(&self) -> Set<(<T as View>::V, <U as View>::V)> {
        self.pairs@
    }
}

impl<T, U> RelationStEphTrait<T, U> for RelationStEph<T, U>
where
    T: View + std::hash::Hash + Eq + Copy,
    U: View + std::hash::Hash + Eq + Copy,
{
    fn empty() -> (result: RelationStEph<T, U>)
        ensures result.view() == Set::<(<T as View>::V, <U as View>::V)>::empty()
    {
        RelationStEph {
            pairs: HashSetWithView::new(),
        }
    }

    fn size(&self) -> (result: N)
        ensures result == self.view().len()
    {
        self.pairs.len()
    }

    fn mem(&self, t: &T, u: &U) -> (result: B)
        ensures result == self.view().contains((t@, u@))
    {
        self.pairs.contains(&(*t, *u))
    }

    fn insert(&mut self, t: T, u: U)
        ensures self.view() == old(self).view().insert((t@, u@))
    {
        self.pairs.insert((t, u));
    }
}

    } // verus!
}
