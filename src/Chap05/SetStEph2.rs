//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set implementing SetTraitWithView

pub mod SetStEph2 {

    use vstd::prelude::*;
    use vstd::hash_set::HashSetWithView;
    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use crate::Types::Types::*;
    use crate::vstdplus::SetTrait::SetTrait::{SetTrait, SetTraitWithView};

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use vstd::std_specs::hash::group_hash_axioms;

/// Verified ephemeral Set wrapping HashSetWithView
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
pub struct SetStEph2<T: View + Eq + Hash> {
    data: HashSetWithView<T>,
}

// Implement View for SetStEph2
impl<T: View + Eq + Hash> View for SetStEph2<T> {
    type V = Set<<T as View>::V>;

    open spec fn view(&self) -> Set<<T as View>::V> {
        self.data@
    }
}

// Implement SetTrait (just the view method)
impl<T: StT + Hash> SetTrait<T> for SetStEph2<T> {
    open spec fn view(&self) -> Set<<T as View>::V> {
        self.data@
    }
}

// Implement SetTraitWithView - methods inherit ensures clauses from trait
impl<T: StT + Hash> SetTraitWithView<T> for SetStEph2<T> {
    #[verifier::external_body]
    fn empty() -> (result: Self) {
        SetStEph2 {
            data: HashSetWithView::new(),
        }
    }

    fn contains(&self, x: &T) -> (result: bool) {
        self.data.contains(x)
    }

    fn insert(&mut self, x: T) {
        self.data.insert(x);
    }

    fn remove(&mut self, x: &T) {
        self.data.remove(x);
    }

    #[verifier::external_body]
    fn union(&self, other: &Self) -> (result: Self) {
        let mut out_data = self.data.clone();
        for x in other.data.iter() {
            out_data.insert(x.clone());
        }
        SetStEph2 { data: out_data }
    }

    #[verifier::external_body]
    fn intersect(&self, other: &Self) -> (result: Self) {
        let mut out_data = HashSetWithView::new();
        for x in self.data.iter() {
            if other.data.contains(x) {
                out_data.insert(x.clone());
            }
        }
        SetStEph2 { data: out_data }
    }

    #[verifier::external_body]
    fn difference(&self, other: &Self) -> (result: Self) {
        let mut out_data = HashSetWithView::new();
        for x in self.data.iter() {
            if !other.data.contains(x) {
                out_data.insert(x.clone());
            }
        }
        SetStEph2 { data: out_data }
    }

    fn len(&self) -> (result: usize) {
        self.data.len()
    }

    #[verifier::external_body]
    fn is_empty(&self) -> (result: bool) {
        self.data.len() == 0
    }
}

    } // verus!
}

