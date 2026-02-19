//! Proof tests for Chap54 BFSStEph BFS tree iterators
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
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
        use apas_verus::Chap54::BFSStEph::BFSStEph::*;

        fn test_top_down_loop(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_wf_graph(graph),
        {
            let tree = bfs_tree(graph, source);
            let td = tree.top_down_order();

            let mut it: ArraySeqStEphIter<usize> = td.iter();
            let ghost iter_seq: Seq<usize> = it@.1;
            let ghost mut count: int = 0;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    count == it@.0,
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                    iter_seq.len() == tree.order.spec_len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(v) = it.next() {
                    proof { count = count + 1; }
                } else {
                    break;
                }
            }

            assert(count == iter_seq.len());
        }
    } => Ok(())
}

// loop-borrow-iter on bottom_up_order
test_verify_one_file! {
    #[test] chap54_bfssteph_bottom_up_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
        use apas_verus::Chap54::BFSStEph::BFSStEph::*;

        fn test_bottom_up_loop(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_wf_graph(graph),
        {
            let tree = bfs_tree(graph, source);
            let _n = tree.order.length();
            let bu = tree.bottom_up_order();

            let mut it: ArraySeqStEphIter<usize> = bu.iter();
            let ghost iter_seq: Seq<usize> = it@.1;
            let ghost mut count: int = 0;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    count == it@.0,
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                    iter_seq.len() == tree.order.spec_len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(v) = it.next() {
                    proof { count = count + 1; }
                } else {
                    break;
                }
            }

            assert(count == iter_seq.len());
        }
    } => Ok(())
}
