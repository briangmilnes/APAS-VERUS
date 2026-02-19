//! Proof tests for BalBinTree iterators
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-in-order:    `loop { ... it.next() ... }` over in-order iter
//!   - loop-pre-order:   `loop { ... it.next() ... }` over pre-order iter
//!   - loop-post-order:  `loop { ... it.next() ... }` over post-order iter
//!   - for-in-order:     `for x in iter: tree.iter_in_order()`
//!   - for-pre-order:    `for x in iter: tree.iter_pre_order()`
//!   - for-post-order:   `for x in iter: tree.iter_post_order()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-in-order: Manual iteration with loop + next()
test_verify_one_file! {
    #[test] balbintree_loop_inorder verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_loop_inorder()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
            );

            let mut it: InOrderIter<u64> = tree.iter_in_order();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0),
                    in_order_iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof {
                        items = items.push(x);
                    }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// loop-pre-order: Manual iteration with loop + next()
test_verify_one_file! {
    #[test] balbintree_loop_preorder verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_loop_preorder()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
            );

            let mut it: PreOrderIter<u64> = tree.iter_pre_order();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0),
                    pre_order_iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof {
                        items = items.push(x);
                    }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-in-order: `for x in iter: tree.iter_in_order()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] balbintree_for_inorder verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_for_inorder()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
            );

            let it: InOrderIter<u64> = tree.iter_in_order();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(x);
                }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-pre-order: `for x in iter: tree.iter_pre_order()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] balbintree_for_preorder verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_for_preorder()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
            );

            let it: PreOrderIter<u64> = tree.iter_pre_order();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(x);
                }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// loop-post-order: Manual iteration with loop + next()
test_verify_one_file! {
    #[test] balbintree_loop_postorder verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_loop_postorder()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
            );

            let mut it: PostOrderIter<u64> = tree.iter_post_order();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0),
                    post_order_iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof {
                        items = items.push(x);
                    }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-post-order: `for x in iter: tree.iter_post_order()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] balbintree_for_postorder verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_for_postorder()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
            );

            let it: PostOrderIter<u64> = tree.iter_post_order();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(x);
                }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// post-order traversal properties: result matches spec
test_verify_one_file! {
    #[test] balbintree_postorder_spec_match verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_postorder_spec()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::leaf(),
                42,
                BalBinTree::leaf(),
            );
            let traversal: Vec<u64> = tree.post_order();
            assert(traversal@ =~= tree.spec_post_order());
            assert(traversal@.len() == 1);
        }
    } => Ok(())
}

// in-order traversal properties: result matches spec
test_verify_one_file! {
    #[test] balbintree_inorder_spec_match verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_inorder_spec()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::leaf(),
                42,
                BalBinTree::leaf(),
            );
            let traversal: Vec<u64> = tree.in_order();
            assert(traversal@ =~= tree.spec_in_order());
            assert(traversal@.len() == 1);
        }
    } => Ok(())
}

// pre-order traversal properties: result matches spec
test_verify_one_file! {
    #[test] balbintree_preorder_spec_match verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_preorder_spec()
            requires obeys_feq_clone::<u64>(),
        {
            let tree: BalBinTree<u64> = BalBinTree::node(
                BalBinTree::leaf(),
                42,
                BalBinTree::leaf(),
            );
            let traversal: Vec<u64> = tree.pre_order();
            assert(traversal@ =~= tree.spec_pre_order());
            assert(traversal@.len() == 1);
        }
    } => Ok(())
}

// leaf traversals are empty
test_verify_one_file! {
    #[test] balbintree_leaf_traversals_empty verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_leaf_empty()
            requires obeys_feq_clone::<u64>(),
        {
            let leaf = BalBinTree::<u64>::leaf();
            let inorder: Vec<u64> = leaf.in_order();
            let preorder: Vec<u64> = leaf.pre_order();
            assert(inorder@.len() == 0);
            assert(preorder@.len() == 0);
        }
    } => Ok(())
}

// iterator creation from leaf: immediately exhausted
test_verify_one_file! {
    #[test] balbintree_leaf_iter_exhausted verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_leaf_iter()
            requires obeys_feq_clone::<u64>(),
        {
            let leaf = BalBinTree::<u64>::leaf();
            let mut it = leaf.iter_in_order();
            let next = it.next();
            assert(next.is_none());
        }
    } => Ok(())
}
