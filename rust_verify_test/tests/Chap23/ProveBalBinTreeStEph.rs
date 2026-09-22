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
        use vstd::std_specs::iter::*;
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

            let ghost orig: Seq<u64> = tree.spec_in_order();
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::vec::IntoIter<u64> = tree.iter_in_order();
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == x);
                        }
                        collected.push(x);
                    },
                    None => {
                        assert(pos == orig.len());
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// loop-pre-order: Manual iteration with loop + next()
test_verify_one_file! {
    #[test] balbintree_loop_preorder verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
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

            let ghost orig: Seq<u64> = tree.spec_pre_order();
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::vec::IntoIter<u64> = tree.iter_pre_order();
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == x);
                        }
                        collected.push(x);
                    },
                    None => {
                        assert(pos == orig.len());
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// for-in-order: `for x in iter: tree.iter_in_order()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] balbintree_for_inorder verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
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

            let ghost orig: Seq<u64> = tree.spec_in_order();
            let mut collected: Vec<u64> = Vec::new();
            for x in it: tree.iter_in_order()
                invariant
                    it.seq() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
            {
                collected.push(x);
            }
            assert(collected@ =~= orig);
        }
    } => Ok(())
}

// for-pre-order: `for x in iter: tree.iter_pre_order()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] balbintree_for_preorder verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
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

            let ghost orig: Seq<u64> = tree.spec_pre_order();
            let mut collected: Vec<u64> = Vec::new();
            for x in it: tree.iter_pre_order()
                invariant
                    it.seq() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
            {
                collected.push(x);
            }
            assert(collected@ =~= orig);
        }
    } => Ok(())
}

// loop-post-order: Manual iteration with loop + next()
test_verify_one_file! {
    #[test] balbintree_loop_postorder verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
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

            let ghost orig: Seq<u64> = tree.spec_post_order();
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::vec::IntoIter<u64> = tree.iter_post_order();
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == x);
                        }
                        collected.push(x);
                    },
                    None => {
                        assert(pos == orig.len());
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// for-post-order: `for x in iter: tree.iter_post_order()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] balbintree_for_postorder verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
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

            let ghost orig: Seq<u64> = tree.spec_post_order();
            let mut collected: Vec<u64> = Vec::new();
            for x in it: tree.iter_post_order()
                invariant
                    it.seq() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
            {
                collected.push(x);
            }
            assert(collected@ =~= orig);
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
