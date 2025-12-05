//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 — Multi-threaded ephemeral Set.
//!
//! Parallel set operations using verified ParaPairDisjoint for fork-join parallelism.

#[cfg(verus_keep_ghost)]
pub mod SetMtEph {

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;
    use std::hash::Hash;

    verus! {

    broadcast use {
        vstd::set::group_set_axioms,
        vstd::seq_lib::group_seq_lib_default,
        crate::Types::Types::group_Pair_axioms,
    };

    /// Parallel cartesian_product: {(a, b) | a ∈ s1, b ∈ s2}
    ///
    /// Uses parallel fork-join: (first × s2) || (rest × s2), then disjoint_union.
    /// 
    /// TODO: Full proof requires:
    /// 1. Lemma: for no_duplicates seq, seq.len() == seq.map.to_set.len()
    /// 2. Tracked witnesses to connect closure captures to outer ghost state
    /// 3. Clone-preserves-view axiom instantiation
    #[verifier::external_body]
    pub fn cartesian_product_mt<T, U>(s1: SetStEph<T>, s2: SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
        where
            T: StT + Hash + Clone + Send + Sync + 'static,
            U: StT + Hash + Clone + Send + Sync + 'static,
            Pair<T, U>: StT + Hash + View<V = (T::V, U::V)>,
        requires
            valid_key_type::<T>(),
            valid_key_type::<U>(),
            valid_key_type::<Pair<T, U>>(),
        ensures
            product@.finite(),
            forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (s1@.contains(av) && s2@.contains(bv)),
    {
        // Parallel implementation using fork-join
        if s1.size() == 0 {
            SetStEph::empty()
        } else {
            let mut it = s1.iter();
            match it.next() {
                Some(first) => {
                    // Build rest from remaining elements
                    let mut rest: SetStEph<T> = SetStEph::empty();
                    for x in it {
                        let _ = rest.insert(x.clone());
                    }
                    
                    // Parallel: (first × s2) || (rest × s2)
                    let first_clone = first.clone();
                    let s2_clone = s2.clone();
                    
                    let pair = crate::ParaPairs::ParaPairs::para_pair(
                        move || SetStEph::elt_cross_set(&first_clone, &s2_clone),
                        move || cartesian_product_mt(rest, s2)
                    );
                    
                    // Disjoint union since first ∉ rest
                    pair.0.disjoint_union(&pair.1)
                },
                None => SetStEph::empty(),
            }
        }
    }

    } // verus!
}

// Non-verified version for cargo test
#[cfg(not(verus_keep_ghost))]
pub mod SetMtEph {

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;
    use crate::ParaPair;
    use std::hash::Hash;

    pub fn cartesian_product_mt<T, U>(s1: SetStEph<T>, s2: SetStEph<U>) -> SetStEph<Pair<T, U>>
    where
        T: StT + Hash + Clone + Send + Sync + 'static,
        U: StT + Hash + Clone + Send + Sync + 'static,
        Pair<T, U>: StT + Hash,
    {
        if s1.size() == 0 {
            SetStEph::empty()
        } else {
            let mut it = s1.iter();
            match it.next() {
                Some(first) => {
                    let mut rest: SetStEph<T> = SetStEph::empty();
                    for x in it {
                        let _ = rest.insert(x.clone());
                    }
                    
                    let first_clone = first.clone();
                    let s2_clone = s2.clone();
                    
                    let Pair(first_cross, rest_cross) = ParaPair!(
                        move || SetStEph::elt_cross_set(&first_clone, &s2_clone),
                        move || cartesian_product_mt(rest, s2)
                    );
                    
                    first_cross.union(&rest_cross)
                },
                None => SetStEph::empty(),
            }
        }
    }
}
