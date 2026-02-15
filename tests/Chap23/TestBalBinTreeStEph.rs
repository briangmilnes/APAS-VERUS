//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn inorder_and_preorder_traversals_match_definitions() {
    let tree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 2, BalBinTree::leaf()),
        4,
        BalBinTree::node(BalBinTree::leaf(), 6, BalBinTree::leaf()),
    );
    let inorder = tree.in_order();
    let preorder = tree.pre_order();
    assert_eq!(inorder, vec![2, 4, 6]);
    assert_eq!(preorder, vec![4, 2, 6]);
    assert_eq!(tree.size(), 3);
    assert_eq!(tree.height(), 2);
}

#[test]
fn balbintree_empty_leaf_operations() {
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.size(), 0);
    assert_eq!(leaf.height(), 0);
    assert_eq!(leaf.in_order().len(), 0);
    assert_eq!(leaf.pre_order().len(), 0);
}

#[test]
fn balbintree_single_node_operations() {
    let single = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    assert_eq!(single.size(), 1);
    assert_eq!(single.height(), 1);
    assert_eq!(single.in_order(), vec![42]);
    assert_eq!(single.pre_order(), vec![42]);
}

#[test]
fn balbintree_complex_structure() {
    // Build a more complex tree: ((1,2,3),4,(5,6,7))
    let left_subtree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    let right_subtree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
        6,
        BalBinTree::node(BalBinTree::leaf(), 7, BalBinTree::leaf()),
    );
    let tree = BalBinTree::node(left_subtree, 4, right_subtree);

    assert_eq!(tree.size(), 7);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(tree.pre_order(), vec![4, 2, 1, 3, 6, 5, 7]);
}

#[test]
fn balbintree_height_calculation() {
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.height(), 0);

    let single = BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf());
    assert_eq!(single.height(), 1);

    let left_heavy = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::leaf(),
    );
    assert_eq!(left_heavy.height(), 2);

    let right_heavy = BalBinTree::node(
        BalBinTree::leaf(),
        1,
        BalBinTree::node(BalBinTree::leaf(), 2, BalBinTree::leaf()),
    );
    assert_eq!(right_heavy.height(), 2);
}

#[test]
fn balbintree_size_calculation() {
    let leaf = BalBinTree::<N>::leaf();
    assert_eq!(leaf.size(), 0);

    let single = BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf());
    assert_eq!(single.size(), 1);

    let three_nodes = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    assert_eq!(three_nodes.size(), 3);
}

#[test]
fn balbintree_traversal_consistency() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::leaf(),
        ),
        3,
        BalBinTree::node(
            BalBinTree::leaf(),
            4,
            BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
        ),
    );

    let inorder = tree.in_order();
    let preorder = tree.pre_order();

    assert_eq!(inorder, vec![1, 2, 3, 4, 5]);
    assert_eq!(preorder[0], 3); // Root value
    assert_eq!(inorder.len(), preorder.len());
    assert_eq!(inorder.len(), tree.size());
}

#[test]
fn balbintree_is_leaf_check() {
    let leaf = BalBinTree::<N>::leaf();
    assert!(leaf.is_leaf());

    let single = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    assert!(!single.is_leaf());

    let complex = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    assert!(!complex.is_leaf());
}

#[test]
fn balbintree_unbalanced_left() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::leaf(),
        ),
        3,
        BalBinTree::leaf(),
    );

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), vec![1, 2, 3]);
    assert_eq!(tree.pre_order(), vec![3, 2, 1]);
}

#[test]
fn balbintree_unbalanced_right() {
    let tree = BalBinTree::node(
        BalBinTree::leaf(),
        1,
        BalBinTree::node(
            BalBinTree::leaf(),
            2,
            BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
        ),
    );

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), vec![1, 2, 3]);
    assert_eq!(tree.pre_order(), vec![1, 2, 3]);
}

#[test]
fn balbintree_large_balanced_tree() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
        ),
        4,
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
            6,
            BalBinTree::node(BalBinTree::leaf(), 7, BalBinTree::leaf()),
        ),
    );

    assert_eq!(tree.size(), 7);
    assert_eq!(tree.height(), 3);
    assert_eq!(tree.in_order(), vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(tree.pre_order(), vec![4, 2, 1, 3, 6, 5, 7]);
}

#[test]
fn balbintree_only_left_children() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(
                BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
                2,
                BalBinTree::leaf(),
            ),
            3,
            BalBinTree::leaf(),
        ),
        4,
        BalBinTree::leaf(),
    );

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.in_order(), vec![1, 2, 3, 4]);
}

#[test]
fn balbintree_only_right_children() {
    let tree = BalBinTree::node(
        BalBinTree::leaf(),
        1,
        BalBinTree::node(
            BalBinTree::leaf(),
            2,
            BalBinTree::node(
                BalBinTree::leaf(),
                3,
                BalBinTree::node(BalBinTree::leaf(), 4, BalBinTree::leaf()),
            ),
        ),
    );

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.in_order(), vec![1, 2, 3, 4]);
}

#[test]
fn balbintree_trait_methods() {
    use BalBinTreeTrait;

    let leaf = <BalBinTree<N> as BalBinTreeTrait<N>>::leaf();
    assert!(<BalBinTree<N> as BalBinTreeTrait<N>>::is_leaf(&leaf));

    let node = <BalBinTree<N> as BalBinTreeTrait<N>>::node(
        <BalBinTree<N> as BalBinTreeTrait<N>>::leaf(),
        42,
        <BalBinTree<N> as BalBinTreeTrait<N>>::leaf(),
    );

    assert!(!<BalBinTree<N> as BalBinTreeTrait<N>>::is_leaf(&node));
    assert_eq!(<BalBinTree<N> as BalBinTreeTrait<N>>::size(&node), 1);
    assert_eq!(<BalBinTree<N> as BalBinTreeTrait<N>>::height(&node), 1);
    assert_eq!(
        <BalBinTree<N> as BalBinTreeTrait<N>>::in_order(&node),
        vec![42]
    );
    assert_eq!(
        <BalBinTree<N> as BalBinTreeTrait<N>>::pre_order(&node),
        vec![42]
    );
}

#[test]
fn balbintree_clone() {
    let tree = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    let tree2 = tree.clone();
    assert_eq!(tree, tree2);
}

#[test]
fn balbintree_debug() {
    let tree = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    let debug_str = format!("{:?}", tree);
    assert!(debug_str.contains("Node") || debug_str.contains("42"));
}

//		iterator tests

#[test]
fn inorder_iter_collects_correctly() {
    let tree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    let collected: Vec<N> = tree.iter_in_order().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn preorder_iter_collects_correctly() {
    let tree = BalBinTree::node(
        BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
        2,
        BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
    );
    let collected: Vec<N> = tree.iter_pre_order().collect();
    assert_eq!(collected, vec![2, 1, 3]);
}

#[test]
fn inorder_iter_leaf_is_empty() {
    let leaf = BalBinTree::<N>::leaf();
    let collected: Vec<N> = leaf.iter_in_order().collect();
    assert!(collected.is_empty());
}

#[test]
fn preorder_iter_leaf_is_empty() {
    let leaf = BalBinTree::<N>::leaf();
    let collected: Vec<N> = leaf.iter_pre_order().collect();
    assert!(collected.is_empty());
}

#[test]
fn inorder_iter_single_node() {
    let single = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    let collected: Vec<N> = single.iter_in_order().collect();
    assert_eq!(collected, vec![42]);
}

#[test]
fn preorder_iter_single_node() {
    let single = BalBinTree::node(BalBinTree::leaf(), 42, BalBinTree::leaf());
    let collected: Vec<N> = single.iter_pre_order().collect();
    assert_eq!(collected, vec![42]);
}

#[test]
fn inorder_iter_complex() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
        ),
        4,
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
            6,
            BalBinTree::node(BalBinTree::leaf(), 7, BalBinTree::leaf()),
        ),
    );
    let collected: Vec<N> = tree.iter_in_order().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn preorder_iter_complex() {
    let tree = BalBinTree::node(
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 1, BalBinTree::leaf()),
            2,
            BalBinTree::node(BalBinTree::leaf(), 3, BalBinTree::leaf()),
        ),
        4,
        BalBinTree::node(
            BalBinTree::node(BalBinTree::leaf(), 5, BalBinTree::leaf()),
            6,
            BalBinTree::node(BalBinTree::leaf(), 7, BalBinTree::leaf()),
        ),
    );
    let collected: Vec<N> = tree.iter_pre_order().collect();
    assert_eq!(collected, vec![4, 2, 1, 3, 6, 5, 7]);
}

#[test]
fn inorder_iter_manual_next() {
    let tree = BalBinTree::node(
        BalBinTree::leaf(),
        10,
        BalBinTree::node(BalBinTree::leaf(), 20, BalBinTree::leaf()),
    );
    let mut it = tree.iter_in_order();
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(20));
    assert_eq!(it.next(), None);
}

#[test]
fn preorder_iter_manual_next() {
    let tree = BalBinTree::node(
        BalBinTree::leaf(),
        10,
        BalBinTree::node(BalBinTree::leaf(), 20, BalBinTree::leaf()),
    );
    let mut it = tree.iter_pre_order();
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(20));
    assert_eq!(it.next(), None);
}
