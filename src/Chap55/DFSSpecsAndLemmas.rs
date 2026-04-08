//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared DFS specs and lemmas used by all Chap55 files.
//! Contains: spec_num_false (termination measure), bool-sequence lemmas,
//! and ArraySeq view-bridge lemmas for both Eph and Per representations.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups

//		Section 1. module

pub mod DFSSpecsAndLemmas {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;

    verus! {

    //		Section 6. spec fns


    /// Counts false entries in a boolean sequence (termination measure for DFS).
    pub open spec fn spec_num_false(s: Seq<bool>) -> nat
        decreases s.len()
    {
        if s.len() == 0 { 0 }
        else if !s.last() { 1 + spec_num_false(s.drop_last()) }
        else { spec_num_false(s.drop_last()) }
    }

    //		Section 7. proof fns/broadcast groups


    /// Setting a false entry to true strictly decreases the count of false entries.
    pub proof fn lemma_set_true_decreases_num_false(s: Seq<bool>, idx: int)
        requires
            0 <= idx < s.len(),
            !s[idx],
        ensures
            spec_num_false(s.update(idx, true)) < spec_num_false(s),
        decreases s.len(),
    {
        if s.len() == 1 {
            assert(s.drop_last() =~= Seq::<bool>::empty());
            assert(s.update(0, true).drop_last() =~= Seq::<bool>::empty());
            assert(!s.last());
            assert(s.update(0, true).last());
        } else if idx == s.len() - 1 {
            assert(!s.last());
            assert(s.update(idx, true).last());
            assert(s.update(idx, true).drop_last() =~= s.drop_last());
        } else {
            assert(s.update(idx, true).drop_last() =~= s.drop_last().update(idx, true));
            lemma_set_true_decreases_num_false(s.drop_last(), idx);
            assert(!s.last() <==> !s.update(idx, true).last());
        }
    }

    /// Setting a false entry to true decreases the count by exactly one.
    pub proof fn lemma_set_true_num_false_eq(s: Seq<bool>, idx: int)
        requires
            0 <= idx < s.len(),
            !s[idx],
        ensures
            spec_num_false(s.update(idx, true)) == spec_num_false(s) - 1,
        decreases s.len(),
    {
        if s.len() == 1 {
            assert(s.drop_last() =~= Seq::<bool>::empty());
            assert(s.update(0, true).drop_last() =~= Seq::<bool>::empty());
            assert(!s.last());
            assert(s.update(0, true).last());
        } else if idx == s.len() - 1 {
            assert(!s.last());
            assert(s.update(idx, true).last());
            assert(s.update(idx, true).drop_last() =~= s.drop_last());
        } else {
            assert(s.update(idx, true).drop_last() =~= s.drop_last().update(idx, true));
            lemma_set_true_num_false_eq(s.drop_last(), idx);
            assert(!s.last() <==> !s.update(idx, true).last());
        }
    }

    /// An all-true sequence has zero false entries.
    pub proof fn lemma_all_true_num_false_zero(s: Seq<bool>)
        requires forall|j: int| 0 <= j < s.len() ==> #[trigger] s[j],
        ensures spec_num_false(s) == 0,
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_true_num_false_zero(s.drop_last());
        }
    }

    /// An all-false sequence has num_false equal to its length.
    pub proof fn lemma_all_false_num_false_eq_len(s: Seq<bool>)
        requires forall|j: int| #![trigger s[j]] 0 <= j < s.len() ==> !s[j],
        ensures spec_num_false(s) == s.len(),
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_false_num_false_eq_len(s.drop_last());
        }
    }

    /// Bridge: for ArraySeqStEphS<bool>, view index equals spec_index.
    pub proof fn lemma_bool_view_eq_spec_index(a: &ArraySeqStEphS<bool>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Collapses the forall bridge after `a.set(vertex, val)` into one call.
    /// Proves `a@ =~= old_view.update(vertex, val)` from the set ensures.
    pub proof fn lemma_bool_array_set_view(
        a: &ArraySeqStEphS<bool>,
        old_view: Seq<bool>,
        vertex: int,
        val: bool,
    )
        requires
            a@.len() == old_view.len(),
            0 <= vertex < a@.len(),
            a.spec_index(vertex) == val,
            forall|i: int| 0 <= i < old_view.len() && i != vertex
                ==> #[trigger] a.spec_index(i) == old_view[i],
        ensures
            a@ =~= old_view.update(vertex, val),
    {
        lemma_bool_view_eq_spec_index(a);
        assert forall|j: int| 0 <= j < a@.len()
            implies #[trigger] a@[j] == old_view.update(vertex, val)[j] by {
            assert(a@[j] == a.spec_index(j));
        };
    }

    /// Bridge: for ArraySeqStEphS<usize>, view index equals spec_index.
    pub proof fn lemma_usize_view_eq_spec_index(a: &ArraySeqStEphS<usize>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: graph adjacency list view at vertex equals spec_index view (ephemeral).
    pub proof fn lemma_graph_view_bridge(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        neighbors: &ArraySeqStEphS<usize>,
        vertex: int,
    )
        requires
            0 <= vertex < graph@.len(),
            *neighbors == graph.spec_index(vertex),
        ensures
            neighbors@ =~= graph@[vertex],
    {
    }

    /// Bridge: for ArraySeqStPerS<usize>, view index equals spec_index.
    pub proof fn lemma_usize_per_view_eq_spec_index(a: &ArraySeqStPerS<usize>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: persistent graph adjacency list view at vertex equals spec_index view.
    pub proof fn lemma_graph_per_view_bridge(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        neighbors: &ArraySeqStPerS<usize>,
        vertex: int,
    )
        requires
            0 <= vertex < graph@.len(),
            *neighbors == graph.spec_index(vertex),
        ensures
            neighbors@ =~= graph@[vertex],
    {
    }

    } // verus!
}
