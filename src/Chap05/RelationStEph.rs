//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation - verified wrapper around HashSetWithView<Pair<T,U>>.

pub mod RelationStEph {
    use vstd::prelude::*;
    use vstd::hash_set::*;
    use vstd::std_specs::hash::obeys_key_model;
    use std::hash::Hash;

    use crate::Types::Types::*;

    verus! {

/// Verified ephemeral Relation wrapping HashSetWithView<Pair<T,U>>
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
#[verifier::reject_recursive_types(U)]
pub struct RelationStEph<T: View + Eq + Hash, U: View + Eq + Hash> {
    pub pairs: HashSetWithView<Pair<T, U>>,
}

pub trait RelationStEphTrait<T: View + Eq + Hash + Copy, U: View + Eq + Hash + Copy>: Sized {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self
        requires obeys_key_model::<Pair<T, U>>();

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

impl<T: View + Eq + std::hash::Hash + Copy, U: View + Eq + std::hash::Hash + Copy> RelationStEphTrait<T, U> for RelationStEph<T, U> {
    fn empty() -> (result: Self)
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
        self.pairs.contains(&Pair(*t, *u))
    }

    fn insert(&mut self, t: T, u: U)
        ensures self.view() == old(self).view().insert((t@, u@))
    {
        self.pairs.insert(Pair(t, u));
    }

    open spec fn view(&self) -> Set<(<T as View>::V, <U as View>::V)> {
        self.pairs@
    }
}

    } // verus!

    // Simplified RelationLit macro for our verified wrapper
    // Note: Does not support literal syntax yet - need to add FromSet/FromVec methods
    #[macro_export]
    macro_rules! RelationLit {
        () => {{
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::empty()
        }};
        // TODO: Add syntax for ($( ($a:expr, $b:expr) ),*) once we have FromVec
    }
}
