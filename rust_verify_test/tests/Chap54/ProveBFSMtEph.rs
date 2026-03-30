//! Proof tests for Chap54 BFSMtEph BFS tree iterators
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
    #[test] chap54_bfsmteph_top_down_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
        use apas_verus::Chap54::BFSMtEph::BFSMtEph::*;

        fn test_top_down_loop(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfsmteph_wf(graph),
        {
            let tree = BFSMtEph::bfs_tree(graph, source);
            let td = tree.top_down_order();

            let mut it: ArraySeqMtEphIter<usize> = td.iter();
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
    #[test] chap54_bfsmteph_bottom_up_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
        use apas_verus::Chap54::BFSMtEph::BFSMtEph::*;

        fn test_bottom_up_loop(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfsmteph_wf(graph),
        {
            let tree = BFSMtEph::bfs_tree(graph, source);
            let _n = tree.order.length();
            let bu = tree.bottom_up_order();

            let mut it: ArraySeqMtEphIter<usize> = bu.iter();
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

// for-borrow-iter on top_down_order
test_verify_one_file! {
    #[test] chap54_bfsmteph_top_down_for verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
        use apas_verus::Chap54::BFSMtEph::BFSMtEph::*;

        fn test_top_down_for(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfsmteph_wf(graph),
        {
            let tree = BFSMtEph::bfs_tree(graph, source);
            let td = tree.top_down_order();

            let it: ArraySeqMtEphIter<usize> = td.iter();
            let ghost iter_seq: Seq<usize> = it@.1;
            let ghost mut count: int = 0;

            for v in iter: it
                invariant
                    count == iter.pos,
                    iter.elements == iter_seq,
                    iter.pos <= iter_seq.len(),
                    iter_seq.len() == tree.order.spec_len(),
            {
                proof { count = count + 1; }
            }

            assert(count == iter_seq.len());
        }
    } => Ok(())
}

// for-borrow-iter on bottom_up_order
test_verify_one_file! {
    #[test] chap54_bfsmteph_bottom_up_for verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
        use apas_verus::Chap54::BFSMtEph::BFSMtEph::*;

        fn test_bottom_up_for(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfsmteph_wf(graph),
        {
            let tree = BFSMtEph::bfs_tree(graph, source);
            let _n = tree.order.length();
            let bu = tree.bottom_up_order();

            let it: ArraySeqMtEphIter<usize> = bu.iter();
            let ghost iter_seq: Seq<usize> = it@.1;
            let ghost mut count: int = 0;

            for v in iter: it
                invariant
                    count == iter.pos,
                    iter.elements == iter_seq,
                    iter.pos <= iter_seq.len(),
                    iter_seq.len() == tree.order.spec_len(),
            {
                proof { count = count + 1; }
            }

            assert(count == iter_seq.len());
        }
    } => Ok(())
}
