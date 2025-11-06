//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation built on SetStEph<Pair<A,B>>.

pub mod RelationStEph {
    use vstd::prelude::*;
    use std::hash::Hash;
    use std::fmt::{Display, Debug};

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

    verus! {

/// Relation built on SetStEph<Pair<T, U>>
pub struct RelationStEph<T: View + Eq + Hash + Copy + Display + Debug, U: View + Eq + Hash + Copy + Display + Debug> {
    pairs: SetStEph<Pair<T, U>>,
}

pub trait RelationStEphTrait<T: View + Eq + Hash + Copy + Display + Debug, U: View + Eq + Hash + Copy + Display + Debug>: Sized {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    fn size(&self) -> (result: N)
        ensures result == self.view().len();

    /// APAS: Work Θ(1), Span Θ(1)
    fn mem(&self, t: &T, u: &U) -> (result: B)
        ensures result == self.view().contains((t@, u@));

    /// APAS: Work Θ(1), Span Θ(1)
    fn insert(&mut self, t: T, u: U)
        ensures self.view() == old(self).view().insert((t@, u@));

    spec fn view(&self) -> Set<(<T as View>::V, <U as View>::V)>;
}

impl<T: View + Eq + Hash + Copy + Display + Debug, U: View + Eq + Hash + Copy + Display + Debug> RelationStEphTrait<T, U> for RelationStEph<T, U> {
    fn empty() -> (result: Self)
        ensures result.view() == Set::<(<T as View>::V, <U as View>::V)>::empty()
    {
        RelationStEph {
            pairs: SetStEph::empty(),
        }
    }

    fn size(&self) -> (result: N)
        ensures result == self.view().len()
    {
        self.pairs.size()
    }

    fn mem(&self, t: &T, u: &U) -> (result: B)
        ensures result == self.view().contains((t@, u@))
    {
        self.pairs.mem(&Pair(*t, *u))
    }

    fn insert(&mut self, t: T, u: U)
        ensures self.view() == old(self).view().insert((t@, u@))
    {
        self.pairs.insert(Pair(t, u));
    }

    open spec fn view(&self) -> Set<(<T as View>::V, <U as View>::V)> {
        self.pairs.view()
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
