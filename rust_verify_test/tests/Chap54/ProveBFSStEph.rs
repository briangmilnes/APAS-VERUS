//! Proof tests for Chap54 BFSStEph BFS tree iterators
//!
//! Loop patterns tested:
//!   - loop-borrow-iter:   `loop { ... td.iter() ... }` (top_down and bottom_up)
//!   - for-borrow-iter:    `for x in iter: td.iter()` (top_down and bottom_up)
//!
//! Tests that BFS tree results can be iterated with verified loop invariants,
//! confirming the spec contracts on top_down_order and bottom_up_order.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter on top_down_order
test_verify_one_file! {
    #[test] chap54_bfssteph_top_down_loop verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
        use apas_verus::Chap54::BFSStEph::BFSStEph::*;

        fn test_top_down_loop(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfssteph_wf(graph),
        {
            let tree = BFSStEph::bfs_tree(graph, source);
            let td = tree.top_down_order();

            let it0 = td.iter();
            let ghost orig: Seq<usize> = vstd::std_specs::slice::into_iter_elts(it0);
            let mut it: std::slice::Iter<'_, usize> = it0;
            let ghost mut count: int = 0;

            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= count <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - count,
                    orig.len() == tree.order.spec_len(),
                decreases IteratorSpec::decrease(&it)->0,
            {
                match it.next() {
                    Some(v) => {
                        proof { count = count + 1; }
                    },
                    None => {
                        assert(count == orig.len());
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// loop-borrow-iter on bottom_up_order
test_verify_one_file! {
    #[test] chap54_bfssteph_bottom_up_loop verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
        use apas_verus::Chap54::BFSStEph::BFSStEph::*;

        fn test_bottom_up_loop(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfssteph_wf(graph),
        {
            let tree = BFSStEph::bfs_tree(graph, source);
            let _n = tree.order.length();
            let bu = tree.bottom_up_order();

            let it0 = bu.iter();
            let ghost orig: Seq<usize> = vstd::std_specs::slice::into_iter_elts(it0);
            let mut it: std::slice::Iter<'_, usize> = it0;
            let ghost mut count: int = 0;

            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= count <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - count,
                    orig.len() == tree.order.spec_len(),
                decreases IteratorSpec::decrease(&it)->0,
            {
                match it.next() {
                    Some(v) => {
                        proof { count = count + 1; }
                    },
                    None => {
                        assert(count == orig.len());
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// for-borrow-iter on top_down_order
test_verify_one_file! {
    #[test] chap54_bfssteph_top_down_for verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
        use apas_verus::Chap54::BFSStEph::BFSStEph::*;

        fn test_top_down_for(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfssteph_wf(graph),
        {
            let tree = BFSStEph::bfs_tree(graph, source);
            let td = tree.top_down_order();

            let it0 = td.iter();
            let ghost orig: Seq<usize> = vstd::std_specs::slice::into_iter_elts(it0);
            let ghost mut count: int = 0;

            for v in it: it0
                invariant
                    it.seq() == orig.as_ref(),
                    count == it.index(),
                    orig.len() == tree.order.spec_len(),
            {
                proof { count = count + 1; }
            }

            assert(count == orig.len());
        }
    } => Ok(())
}

// for-borrow-iter on bottom_up_order
test_verify_one_file! {
    #[test] chap54_bfssteph_bottom_up_for verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
        use apas_verus::Chap54::BFSStEph::BFSStEph::*;

        fn test_bottom_up_for(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfssteph_wf(graph),
        {
            let tree = BFSStEph::bfs_tree(graph, source);
            let _n = tree.order.length();
            let bu = tree.bottom_up_order();

            let it0 = bu.iter();
            let ghost orig: Seq<usize> = vstd::std_specs::slice::into_iter_elts(it0);
            let ghost mut count: int = 0;

            for v in it: it0
                invariant
                    it.seq() == orig.as_ref(),
                    count == it.index(),
                    orig.len() == tree.order.spec_len(),
            {
                proof { count = count + 1; }
            }

            assert(count == orig.len());
        }
    } => Ok(())
}
