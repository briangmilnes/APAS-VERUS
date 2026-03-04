// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0
//! DeepView Standard: how to implement vstd's DeepView trait in APAS-VERUS.
//!
//! DeepView recursively maps through nested container types. While View maps
//! Vec<T> to Seq<T> (keeping T concrete), DeepView maps Vec<T> to Seq<T::V>
//! (recursively abstracting elements).
//!
//! This file shows three patterns:
//! - Pattern A: Simple struct (identity — same as View).
//! - Pattern B: Generic collection (recursive deep_view on elements).
//! - Pattern C: Tuple deep_view (vstd built-in).
//!
//! References:
//! - src/experiments/deep_view_struct.rs
//! - src/experiments/deep_view_2_tuple.rs
//! - src/Chap18/ArraySeq.rs
// 1. module
pub mod deep_view_standard {

    use vstd::prelude::*;

    verus! {

    // Pattern A: Simple Struct
    //
    // For non-generic structs with primitive fields, DeepView is identical to View.
    // Both project to the same type.
    // 4. type definitions
    pub struct SimpleS {
        pub val: Option<usize>,
    }

    // 5. view impls
    /// View and DeepView both project to the same type for simple structs.
    impl View for SimpleS {
        type V = Option<usize>;

        open spec fn view(&self) -> Option<usize> {
            self.val
        }
    }

    impl DeepView for SimpleS {
        type V = Option<usize>;

        open spec fn deep_view(&self) -> Option<usize> {
            self.val
        }
    }

    // Pattern B: Generic Collection
    //
    // For a generic collection wrapping Vec<T>, View and DeepView differ:
    // - View:     type V = Seq<T>       (elements stay concrete)
    // - DeepView: type V = Seq<T::V>    (elements recursively abstracted)
    //
    // The body uses Seq::new with deep_view() on each element.
    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct CollectionS<T> {
        pub seq: Vec<T>,
    }

    // 5. view impls
    /// View keeps elements concrete: Seq<T>.
    impl<T: View> View for CollectionS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    /// DeepView recursively abstracts elements: Seq<T::V> where V = T's DeepView.
    impl<T: DeepView> DeepView for CollectionS<T> {
        type V = Seq<T::V>;

        open spec fn deep_view(&self) -> Seq<T::V> {
            let v = self.seq@;
            Seq::new(v.len(), |i: int| v[i].deep_view())
        }
    }

    // Pattern C: Tuple DeepView
    //
    // vstd provides built-in DeepView for tuples. For Vec<(K, Vec<V>)>,
    // deep_view() produces Seq<(K::V, Seq<V::V>)> automatically.
    // No custom implementation needed — just use it in specs.
    // 7. proof fns
    /// Prove that deep_view preserves length (key bridge lemma).
    proof fn lemma_deep_view_len<T: DeepView>(v: &Vec<T>)
        ensures
            v.deep_view().len() == v@.len(),
    {
    }

    /// Prove that tuple deep_view works on Vec<(u32, Vec<u32>)>.
    proof fn test_tuple_deep_view(v: Vec<(u32, Vec<u32>)>)
        ensures
            v.deep_view().len() == v@.len(),
    {
    }

    } // verus!
} // pub mod deep_view_standard
